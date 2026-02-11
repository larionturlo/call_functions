use std::error::Error;

pub mod cases;
pub mod extract;
pub mod reqwest;
pub mod statistic;
pub mod translate;

pub type BoxError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, BoxError>;

pub use extract::company;
