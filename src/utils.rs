//! Useful stuff related to the spaceapi.

/// An ``Optional`` can contain ``Optional::Value<T>`` or ``Optional::Absent``.
/// It is similar to an ``Option``, but ``Optional::Absent`` means it will be
/// omitted when serialized, while ``None`` will be serialized to ``null``.
#[derive(Debug, Copy, Clone)]
pub enum Optional<T> {
    Value(T),
    Absent,
}

impl<T> Optional<T> {

    /// Applies a function to the contained value or returns a default.  see
    /// [`std::option::Option<T>::map_or`](http://doc.rust-lang.org/std/option/enum.Option.html#method.map_or)
    pub fn map_or<U, F: FnOnce(T) -> U>(self, def: U, f: F) -> U {
        match self {
            Optional::Value(v) => f(v),
            Optional::Absent => def,
        }
    }

    pub fn as_mut<'r>(&'r mut self) -> Optional<&'r mut T> {
        match *self {
            Optional::Value(ref mut x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }

    pub fn as_ref<'r>(&'r self) -> Optional<&'r T> {
        match *self {
            Optional::Value(ref x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }

}
