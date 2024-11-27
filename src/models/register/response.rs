pub enum RegisterResponse {
    UserAlreadyExists,
    UserSuccessfullyRegistered,
    Error(String),
}
