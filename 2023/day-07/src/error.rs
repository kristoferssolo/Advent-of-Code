use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandParseError {
    #[error("Invalid card: {0}")]
    InvalidCard(char),

    #[error("Invalid bid: {0}")]
    InvalidBid(#[from] std::num::ParseIntError),

    #[error("Invalid card count")]
    InvalidCardCount,
}

#[derive(Debug, Error)]
pub enum CombinationError {
    #[error("Invalid card count")]
    InvalidCardCount,
}
