#[cfg(not(feature = "development"))]
pub use tracing_orchestra_macros::*;
#[cfg(feature = "development")]
pub use tracing_orchestra_macros_development::*;

pub trait Orchestra {}
