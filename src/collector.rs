/// A trait for collecting items and producing a result.
pub trait Collector<T, U> {
    /// Collects an item.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to collect.
    fn collect(&mut self, item: T);

    /// Returns the result of the collection.
    ///
    /// # Returns
    ///
    /// The result of the collection.
    fn result(&mut self) -> U;
}

/// A trait for collecting items with a specified collector.
pub trait CollectWith<T>: Iterator {
    /// Collects items with the specified collector.
    ///
    /// # Arguments
    ///
    /// * `self` - The iterator.
    /// * `collector` - The collector to use.
    ///
    /// # Returns
    ///
    /// The result of the collection.
    ///
    /// # Generic Parameters
    ///
    /// * `C` - The type of the collector.
    ///
    /// # Constraints
    ///
    /// * `Self: Sized` - The iterator must be sized.
    /// * `C: Collector<Self::Item, T>` - The collector must implement the `Collector` trait.
    fn collect_with<C>(self, collector: &mut C) -> T
    where
        Self: Sized,
        C: Collector<Self::Item, T>;
}

/// Implements the `CollectWith` trait for any iterator.
impl<I: Iterator, T> CollectWith<T> for I {
    /// Collects items with the specified collector.
    ///
    /// # Arguments
    ///
    /// * `self` - The iterator.
    /// * `collector` - The collector to use.
    ///
    /// # Returns
    ///
    /// The result of the collection.
    fn collect_with<C>(self, collector: &mut C) -> T
    where
        Self: Sized,
        C: Collector<Self::Item, T>,
    {
        self.for_each(|item| collector.collect(item));
        collector.result()
    }
}
