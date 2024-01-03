pub trait Fixture {
    fn fixture() -> Self;
}

pub fn tuple_fixture<T: Fixture, U: Fixture>() -> (T, U) {
    (Fixture::fixture(), Fixture::fixture())
}
