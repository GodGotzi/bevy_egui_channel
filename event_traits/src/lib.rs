pub trait EventCollection<T> {
    fn event_eq_type(&self, other: T) -> bool;
}
