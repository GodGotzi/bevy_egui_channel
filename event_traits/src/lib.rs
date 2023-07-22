pub trait EventCollection<T> {
    fn event_eq_type(&self, other: T) -> bool;
}

pub trait EnumVec<T> {
    fn as_vec() -> Vec<T>;
}
