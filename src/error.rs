pub type Result<T> = core::result::Result<T, GameError>;

pub enum GameError {
    Error(String),
}
