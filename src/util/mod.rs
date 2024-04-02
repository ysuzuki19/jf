// SPDX-License-Identifier: MPL-2.0
pub mod error;
mod read_only;
#[cfg(test)]
pub mod testutil;

pub use read_only::ReadOnly;
