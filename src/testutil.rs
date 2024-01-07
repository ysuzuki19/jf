pub trait Fixture {
    fn gen() -> Self;
}

pub trait TryFixture {
    fn try_gen() -> crate::error::JfResult<Self>
    where
        Self: Sized;
}
