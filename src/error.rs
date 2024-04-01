#[derive (Debug)]
pub struct SetupError(pub &'static str);
impl std::error::Error for SetupError {}
impl std::fmt::Display for SetupError {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
