//! Implementation of the custom `Optional` type. See `Optional`
//! docs for more information.

use rustc_serialize::{Decodable, Decoder};

/// An `Optional` can contain either `Value<T>` or `Absent`.
/// It is similar to an `Option`, but `None` will be serialized to `null`
/// while `Absent` means that both the key and the value will be omitted
/// when serialized.
///
/// An `Optional` implements the `Default` trait, it is `Absent` by default.
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
            Value(val) => val,
            Absent => panic!("called `Optional::unwrap()` on a `Absent` value"),
        }
    }

    /// Returns the contained value or a default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spaceapi::optional::Optional::{Value,Absent};
    /// assert_eq!(Value("car").unwrap_or("bike"), "car");
    /// assert_eq!(Absent.unwrap_or("bike"), "bike");
    /// ```
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Value(x) => x,
            Absent => def,
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
            Value(x) => x,
            Absent => f()
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
            Value(x) => Value(f(x)),
            Absent => Absent
        }
    }

    /// Applies a function to the contained value or returns a default.  see
    /// [`std::option::Option<T>::map_or`](http://doc.rust-lang.org/std/option/enum.Option.html#method.map_or)
    pub fn map_or<U, F: FnOnce(T) -> U>(self, def: U, f: F) -> U {
        match self {
            Value(v) => f(v),
            Absent => def,
        }
    }

    /// Converts from `Optional<T>` to `Optional<&mut T>`
    pub fn as_mut<'r>(&'r mut self) -> Optional<&'r mut T> {
        match *self {
            Value(ref mut x) => Value(x),
            Absent => Absent
        }
    }

    /// Converts from `Optional<T>` to `Optional<&T>`
    pub fn as_ref<'r>(&'r self) -> Optional<&'r T> {
        match *self {
            Value(ref x) => Value(x),
            Absent => Absent
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
            Value(x) => f(x),
            Absent => Absent,
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
            Absent => true,
            _ => false
        }
    }
}

impl<T> Default for Optional<T> {
    /// An optional value is absent by default.
    fn default() -> Optional<T> {
        Absent
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
            Value(x) => Some(x),
            Absent => None,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let something: Optional<u8> = Default::default();
        assert_eq!(something, Optional::Absent);
    }

    #[test]
    fn test_default_in_struct() {
        #[derive(Default)]
        struct Foo {
            something: Optional<u8>,
            somenum: u8,
        }

        let foo1: Foo = Default::default();
        assert_eq!(foo1.something, Optional::Absent);
        assert_eq!(foo1.somenum, 0);

        let foo2 = Foo { somenum: 7, ..Default::default() };
        assert_eq!(foo2.something, Optional::Absent);
        assert_eq!(foo2.somenum, 7);
    }

}
