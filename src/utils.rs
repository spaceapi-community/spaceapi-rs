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
    
    /// Returns the contained value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::utils::Optional;
    /// # use spaceapi::utils::Optional::{Value,Absent};
    /// let k = 10;
    /// assert_eq!(Value(4).unwrap_or_else(|| 2 * k), 4);
    /// assert_eq!(Absent.unwrap_or_else(|| 2 * k), 20);
    /// ```
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Optional::Value(x) => x,
            Optional::Absent => f()
        }
    }

    /// Applies a function to the contained value or returns a default.  see
    /// [`std::option::Option<T>::map_or`](http://doc.rust-lang.org/std/option/enum.Option.html#method.map_or)
    pub fn map_or<U, F: FnOnce(T) -> U>(self, def: U, f: F) -> U {
        match self {
            Optional::Value(v) => f(v),
            Optional::Absent => def,
        }
    }

    /// Converts from `Optional<T>` to `Optional<&mut T>`
    pub fn as_mut<'r>(&'r mut self) -> Optional<&'r mut T> {
        match *self {
            Optional::Value(ref mut x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }

    /// Converts from `Optional<T>` to `Optional<&T>`
    pub fn as_ref<'r>(&'r self) -> Optional<&'r T> {
        match *self {
            Optional::Value(ref x) => Optional::Value(x),
            Optional::Absent => Optional::Absent
        }
    }


    /// Returns `true` if the optional is a `Absent` value
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::utils::Optional;
    /// # use spaceapi::utils::Optional::{Value,Absent};
    /// let x: Optional<u32> = Value(2);
    /// assert_eq!(x.is_absent(), false);
    ///
    /// let x: Optional<u32> = Absent;
    /// assert_eq!(x.is_absent(), true);
    /// ```
    pub fn is_absent(&self) -> bool {
        match *self {
            Optional::Absent => true,
            _ => false
        }
    }

}
