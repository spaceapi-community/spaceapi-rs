//! Implementation of the custom `Optional` type. See `Optional`
//! docs for more information.

use rustc_serialize::{Decodable, Decoder};

/// An `Optional` can contain either `Value<T>` or `Absent`.
/// It is similar to an `Option`, but `None` will be serialized to `null`
/// while `Absent` means the value will be omitted when serialized.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Optional<T> {
    Value(T),
    Absent,
}
use Optional::*;

impl<T> Optional<T> {

    /// Moves the value `v` out of the `Optional<T>` if it is `Value(v)`.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `Absent`.
    ///
    /// # Safety note
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the `Absent`
    /// case explicitly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// let x = Value("air");
    /// assert_eq!(x.unwrap(), "air");
    /// ```
    ///
    /// ```{.should_panic}
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// let x: Optional<&str> = Absent;
    /// assert_eq!(x.unwrap(), "air"); // fails
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            Optional::Value(val) => val,
            Optional::Absent => panic!("called `Optional::unwrap()` on a `Absent` value"),
        }
    }

    /// Returns the contained value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
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

    /// Maps an `Optional<T>` to `Optional<U>` by applying a function to a contained value
    ///
    /// # Examples
    ///
    /// Convert an `Optional<String>` into an `Optional<usize>`, consuming the original:
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// let num_as_str: Optional<String> = Value("10".to_string());
    /// // `Optional::map` takes self *by value*, consuming `num_as_str`
    /// let num_as_int: Optional<usize> = num_as_str.map(|n| n.len());
    /// ```
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Optional<U> {
        match self {
            Optional::Value(x) => Optional::Value(f(x)),
            Optional::Absent => Optional::Absent
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

    /// Returns `Absent` if the optional is `Absent`, otherwise calls `f` with the
    /// wrapped value and returns the result.
    ///
    /// Some languages call this operation flatmap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// fn sq(x: u32) -> Optional<u32> { Value(x * x) }
    /// fn nope(_: u32) -> Optional<u32> { Absent }
    ///
    /// assert_eq!(Value(2).and_then(sq).and_then(sq), Value(16));
    /// assert_eq!(Value(2).and_then(sq).and_then(nope), Absent);
    /// assert_eq!(Value(2).and_then(nope).and_then(sq), Absent);
    /// assert_eq!(Absent.and_then(sq).and_then(sq), Absent);
    /// ```
    pub fn and_then<U, F: FnOnce(T) -> Optional<U>>(self, f: F) -> Optional<U> {
        match self {
            Optional::Value(x) => f(x),
            Optional::Absent => Optional::Absent,
        }
    }

    /// Returns `true` if the optional is a `Absent` value
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
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

impl<T> Into<Option<T>> for Optional<T> {
    /// Convert Optional<T> into Option<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional;
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// let x: Optional<u32> = Value(2);
    /// assert_eq!(Some(2), x.into());
    ///
    /// let x: Optional<u32> = Absent;
    /// assert_eq!(None, x.into());
    /// ```
    fn into(self) -> Option<T> {
        match self {
            Optional::Value(x) => Some(x),
            Optional::Absent => None,
        }
    }
}

impl<T:Decodable> Decodable for Optional<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Optional<T>, D::Error> {
        d.read_option(|d, b| {
            if b {
                Ok(Value(try!(Decodable::decode(d))))
            } else {
                Ok(Absent)
            }
        })
    }
}
