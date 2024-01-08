pub trait Fixture {
    fn gen() -> Self;
}

pub trait TryFixture {
    fn try_gen() -> crate::error::JfResult<Self>
    where
        Self: Sized;
}

#[cfg_attr(coverage, coverage(off))]
pub fn async_test<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("Failed to initialize tokio::runtime::Runtime")
        .block_on(f)
}
