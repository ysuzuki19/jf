pub use crate::ctx::logger::MockLogWriter;

pub trait Fixture {
    fn fixture() -> Self;
}

pub trait TryFixture {
    fn try_fixture() -> crate::error::JfResult<Self>
    where
        Self: Sized;
}

#[cfg_attr(coverage, coverage(off))]
pub fn async_test<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("Failed to initialize tokio::runtime::Runtime")
        .block_on(f)
}
