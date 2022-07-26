/// A helper trait to convert a domain to a persistence model or dao `T`
pub trait ToPersistence<T> {
    /// Converts itself into a persistence model or dao
    fn to_persistence(&self) -> T;
}

/// A helper trait to convert a persistence model or dao into a domain `T`
pub trait ToDomain<T> {
    /// Converts itself into a domain `T`
    ///
    /// # Arguments
    ///
    /// * `persistence` - The persistence model or dao
    fn to_domain(persistence: Self) -> T;
}

