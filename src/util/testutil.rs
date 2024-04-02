// SPDX-License-Identifier: MPL-2.0
use super::error::JfResult;

pub trait AsyncFixture {
    async fn async_fixture() -> Self
    where
        Self: Sized;
}

pub trait TryAsyncFixture {
    async fn try_async_fixture() -> JfResult<Self>
    where
        Self: Sized;
}

pub trait Fixture {
    fn fixture() -> Self;
}

pub trait TryFixture {
    fn try_fixture() -> crate::util::error::JfResult<Self>
    where
        Self: Sized;
}

#[cfg_attr(coverage, coverage(off))]
pub fn async_test<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("Failed to initialize tokio::runtime::Runtime")
        .block_on(f)
}
