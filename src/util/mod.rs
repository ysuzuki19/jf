pub mod error;
mod read_only;
#[cfg(test)]
pub mod testutil;

pub use read_only::ReadOnly;
