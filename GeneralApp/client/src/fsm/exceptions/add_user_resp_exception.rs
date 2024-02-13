#[derive(Debug)]
pub struct AddUserRespException;

impl std::fmt::Display for AddUserRespException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddUserRespException occurs")
    }
}
