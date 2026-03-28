use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemFn, LitInt, Token};

struct ZmqResponseHandlerArgs {
    variant: Ident,
    error: Ident,
    poll_timeout_ms: LitInt,
    retries: Option<LitInt>,
}

impl Parse for ZmqResponseHandlerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variant = None;
        let mut error = None;
        let mut poll_timeout_ms = None;
        let mut retries = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "variant" => variant = Some(input.parse::<Ident>()?),
                "error" => error = Some(input.parse::<Ident>()?),
                "poll_timeout_ms" => poll_timeout_ms = Some(input.parse::<LitInt>()?),
                "retries" => retries = Some(input.parse::<LitInt>()?),
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "unknown attribute `{key}`, expected one of: \
                             variant, error, poll_timeout_ms, retries"
                        ),
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            variant: variant.ok_or_else(|| input.error("missing required attribute `variant`"))?,
            error: error.ok_or_else(|| input.error("missing required attribute `error`"))?,
            poll_timeout_ms: poll_timeout_ms
                .ok_or_else(|| input.error("missing required attribute `poll_timeout_ms`"))?,
            retries,
        })
    }
}

/// Attribute macro that generates ZMQ response handler boilerplate:
/// poll → recv → `parse_from_bytes` → match `Msgtype` variant → log.
///
/// # Attributes
///
/// | Name | Required | Description |
/// |------|----------|-------------|
/// | `variant` | yes | `envelope::Msgtype` variant to match (e.g. `AddUserResp`) |
/// | `error` | yes | `ResponseError` variant returned on failure |
/// | `poll_timeout_ms` | yes | Socket poll timeout in milliseconds |
/// | `retries` | no | Retry count — generates a retry loop instead of a single poll |
///
/// # Examples
///
/// Single-poll handler:
///
/// ```ignore
/// #[zmq_response_handler(variant = AddUserResp, error = AddUserRespException, poll_timeout_ms = 10)]
/// pub async fn handle_add_user_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
/// ```
///
/// Retry-loop handler:
///
/// ```ignore
/// #[zmq_response_handler(
///     variant = HeartbeatResp,
///     error = HeartBeatException,
///     poll_timeout_ms = 1000,
///     retries = 3,
/// )]
/// pub async fn handle_heart_beat_response(socket: &zmq::Socket) -> Result<(), ResponseError> {}
/// ```
#[proc_macro_attribute]
pub fn zmq_response_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ZmqResponseHandlerArgs);
    let mut func = parse_macro_input!(item as ItemFn);

    let variant = &args.variant;
    let error = &args.error;
    let poll_timeout = &args.poll_timeout_ms;

    let log_msg = syn::LitStr::new(
        &(String::from("Received ") + &variant.to_string() + " from the server {{{}}}"),
        variant.span(),
    );

    let socket_ident = match extract_first_param_ident(&func) {
        Ok(ident) => ident,
        Err(e) => return e.to_compile_error().into(),
    };

    let new_body: syn::Block = if let Some(ref retries) = args.retries {
        syn::parse_quote!({
            let mut retries_left: i8 = #retries;

            while retries_left > 0 {
                if #socket_ident.poll(::async_zmq::zmq::POLLIN, #poll_timeout) != Ok(0) {
                    let Ok(resp) = #socket_ident.recv_msg(0) else {
                        return Err(ResponseError::#error);
                    };

                    match <::generated::communication::Envelope
                        as ::protobuf::Message>::parse_from_bytes(&resp)
                    {
                        Ok(msg) => match msg.msgtype {
                            Some(
                                ::generated::communication::envelope::Msgtype::#variant(_),
                            ) => {
                                logger::debug!(#log_msg, msg);
                                return Ok(());
                            }
                            _ => {
                                logger::info!("Received unexpected response: {:?}", msg);
                            }
                        },
                        Err(e) => {
                            logger::warn!("Unable to deserialize response: {:?}", e);
                        }
                    }
                }

                retries_left -= 1;
                logger::info!("Number of retries left: {}", retries_left);
            }

            Ok(())
        })
    } else {
        syn::parse_quote!({
            if #socket_ident.poll(::async_zmq::zmq::POLLIN, #poll_timeout) != Ok(0) {
                let Ok(resp) = #socket_ident.recv_msg(0) else {
                    return Err(ResponseError::#error);
                };

                match <::generated::communication::Envelope
                    as ::protobuf::Message>::parse_from_bytes(&resp)
                {
                    Ok(msg) => match msg.msgtype {
                        Some(
                            ::generated::communication::envelope::Msgtype::#variant(_),
                        ) => {
                            logger::debug!(#log_msg, msg);
                        }
                        _ => {
                            logger::warn!("Received unexpected response: {:?}", msg);
                        }
                    },
                    Err(e) => {
                        logger::warn!("Unable to deserialize response: {:?}", e);
                        return Err(ResponseError::#error);
                    }
                }
            }

            Ok(())
        })
    };

    *func.block = new_body;

    quote! { #func }.into()
}

fn extract_first_param_ident(func: &ItemFn) -> syn::Result<Ident> {
    func.sig
        .inputs
        .first()
        .and_then(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &func.sig,
                "zmq_response_handler requires at least one named parameter (socket)",
            )
        })
}
