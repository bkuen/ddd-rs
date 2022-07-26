/// A helper trait to enable persistence conversions
pub trait Persistence<T> {
    fn to_domain(persistence: T) -> Self;
    fn to_persistence(&self) -> T;
}