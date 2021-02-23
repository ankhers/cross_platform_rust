#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error executing SQL: {0}")]
    SqlError(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
