/*!
Wildcard values.

Type `OrAny` represents a value that maybe _wildcarded_. As such, it is akin to Option, except that
rather than _value or none_ it represents _value or any value_.

# Example

*/

use std::mem;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum OrAny<T> {
    Any,
    Some(T),
}

use self::OrAny::{Any, Some};

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> OrAny<T> {
    // ********************************************************************************************
    // Accessors
    // ********************************************************************************************

    #[inline]
    pub fn as_ref(&self) -> OrAny<&T> {
        match *self {
            Some(ref v) => Some(v),
            _ => Any,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> OrAny<&mut T> {
        match *self {
            Some(ref mut x) => Some(x),
            _ => Any,
        }
    }

    #[inline]
    pub fn some(&self) -> Option<&T> {
        match self {
            Some(v) => Option::Some(v),
            _ => Option::None,
        }
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> OrAny<T> {
        mem::replace(self, Some(value))
    }

    // ********************************************************************************************
    // Predicates
    // ********************************************************************************************

    #[inline]
    pub fn is_any(&self) -> bool {
        match self {
            Any => true,
            _ => false,
        }
    }

    /// Returns `true` if the option is a [`Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use package_name::{Any, OrAny, OrAny::Some};
    ///
    /// let x: OrAny<u32> = Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: OrAny<u32> = Any;
    /// assert_eq!(x.is_some(), false);
    /// ```
    ///
    /// [`Some`]: #variant.Some
    #[inline]
    pub fn is_some(&self) -> bool {
        match self {
            Some(_) => true,
            _ => false,
        }
    }

    // ********************************************************************************************
    // Operators
    // ********************************************************************************************

    #[inline]
    pub fn map<U, F>(self, f: F) -> OrAny<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Some(v) => Some(f(v)),
            _ => Any,
        }
    }

    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Some(t) => f(t),
            Any => default,
        }
    }

    #[inline]
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
        if let Some(x) = self {
            if predicate(&x) {
                return Some(x);
            }
        }
        Any
    }

    // ********************************************************************************************
    // Unwrap-Like
    // ********************************************************************************************

    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            _ => panic!("{}", msg),
        }
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Some(v) => v,
            _ => panic!("called `OrAny::unwrap()` on a `None` value"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Some(v) => v,
            _ => def,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T: Default> OrAny<T> {
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        match self {
            Some(x) => x,
            _ => Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T: PartialEq<T>> OrAny<T> {
    #[inline]
    pub fn contains<T>(&self, x: &T) -> bool {
        match self {
            Some(y) => x == y,
            Any => true,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T: PartialEq> PartialEq for OrAny<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(v1), Some(v2)) => v1 == v2,
            _ => true,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {}
}
