use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemFn};

struct EnvelopeBuilderArgs {
    mut_method: Ident,
}

impl Parse for EnvelopeBuilderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut_method = input.parse()?;
        Ok(Self { mut_method })
    }
}

/// Attribute macro that wraps a function body with Envelope builder boilerplate.
///
/// The macro injects `let mut msg = Envelope::new()` and `let req = msg.<mut_method>()`
/// before the function body, and returns `msg` at the end.
///
/// Inside the function body, use `req` to set fields on the inner protobuf message.
///
/// # Example
///
/// ```ignore
/// #[envelope_builder(mut_add_user_resp)]
/// pub fn build_add_user_response(add_user_req: &AddUserReq, result: Result) -> Envelope {
///     inner.user_id = add_user_req.user_id;
///     inner.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);
///     inner.result = result.into();
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// pub fn build_add_user_response(add_user_req: &AddUserReq, result: Result) -> Envelope {
///     let mut msg = Envelope::new();
///     let inner = msg.#mut_method();
///     inner.user_id = add_user_req.user_id;
///     inner.user_name = format!("OK RECEIVED for {}", add_user_req.user_name);
///     inner.result = result.into();
///     msg
/// }
/// ```
#[proc_macro_attribute]
pub fn envelope_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EnvelopeBuilderArgs);
    let mut func = parse_macro_input!(item as ItemFn);

    let mut_method = &args.mut_method;
    let original_stmts = &func.block.stmts;

    let new_body: syn::Block = syn::parse_quote!({
        let mut msg = Envelope::new();
        let inner = msg.#mut_method();
        #(#original_stmts)*
        msg
    });

    *func.block = new_body;

    quote! { #func }.into()
}
