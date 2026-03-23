use envelope_macro::envelope_builder;
use generated::communication::Envelope;

#[must_use]
#[envelope_builder(mut_delete_user_request)]
pub fn build_delete_user_req(id: u32) -> Envelope {
    inner.user_id = id;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_delete_user_req() {
        let id: u32 = 123;

        let envelope = build_delete_user_req(id);

        assert!(envelope.has_delete_user_request());

        let delete_user_request = envelope.delete_user_request();

        assert_eq!(delete_user_request.user_id, id);
    }
}
