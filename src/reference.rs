/// Tracked value wrapper for types to be modified by reference.
pub struct TrackedRef<T> {
    fresh: bool,
    val: T
}

impl<T> TrackedRef<T> {
    pub fn new(val: T) -> Self {
        TrackedRef {
            fresh: true,
            val
        }
    }

    /// Get a reference to the current value, marking it as stale.
    pub fn get(&mut self) -> &T {
        self.fresh = false;
        &self.val
    }

    /// Get a mutable reference to the current value, marking it as fresh.
    pub fn get_mut(&mut self) -> &mut T {
        self.fresh = true;
        &mut self.val
    }

    /// Get the current value if it's been modified since last time we checked, marking it as stale.
    pub fn get_if_fresh(&mut self) -> Option<&T> {
        if self.fresh {
            Some(self.get())
        } else {
            None
        }
    }

    // Get the current value without marking it.
    pub fn peek(&self) -> &T {
        &self.val
    }

    pub fn is_fresh(&self) -> bool {
        self.fresh
    }
}

pub trait TrackedRefSet<T: PartialEq> {
    /// Set a new value, marked as fresh if not equal to the existing value.
    ///
    /// Set is only defined for `TrackedRef` if `T` implements `PartialEq`.
    fn set(&mut self, val: T);
}

impl<T: PartialEq> TrackedRefSet<T> for TrackedRef<T> {
    fn set(&mut self, val: T) {
        if self.val != val {
            self.val = val;
            self.fresh = true;
        }
    }
}

impl<T: Clone + PartialEq> Clone for TrackedRef<T> {
    fn clone(&self) -> Self {
        TrackedRef {
            fresh: self.fresh,
            val: self.val.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = TrackedRef::new(5);

        assert!(t.is_fresh());
        assert_eq!(t.peek(), &5);
    }

    #[test]
    fn test_set_same_val() {
        let mut t = TrackedRef::new(5);

        // Reset to stale
        t.get();

        // Same value - still stale
        t.set(5);

        assert!(!t.is_fresh());
    }

    #[test]
    fn test_set_different_val() {
        let mut t = TrackedRef::new(5);

        // Reset to stale
        t.get();

        // Different value - fresh again
        t.set(6);

        assert!(t.is_fresh());
        assert_eq!(t.peek(), &6);
    }

    #[test]
    fn test_get() {
        let mut t = TrackedRef::new(5);

        {
            let v = t.get();

            assert_eq!(v, &5);
        }

        assert!(!t.is_fresh());
    }

    #[test]
    fn test_get_mut() {
        #[derive(Debug, PartialEq)]
        struct Wrapper(i32);

        let mut t = TrackedRef::new(Wrapper(777));

        // Mark as stale
        t.get();

        // Modify wrapped value, marking as fresh again
        t.get_mut().0 = 888;

        assert!(t.is_fresh());
        assert_eq!(t.peek(), &Wrapper(888));
    }

    #[test]
    fn test_get_if_fresh() {
        let mut t = TrackedRef::new("hello");

        {
            let v = t.get_if_fresh();

            assert!(v.is_some());
            assert_eq!(v.unwrap(), &"hello");
        }

        assert!(!t.is_fresh());
    }

    #[test]
    fn test_get_if_fresh_stale() {
        let mut t = TrackedRef::new("hello");

        // Mark as stale
        t.get();

        let v = t.get_if_fresh();

        assert!(v.is_none());
    }

    #[test]
    fn test_peek() {
        let t = TrackedRef::new(777);

        let v = t.peek();

        assert!(t.is_fresh());
        assert_eq!(v, &777);
    }

    #[test]
    fn test_is_fresh() {
        let t = TrackedRef {
            fresh: true,
            val: 5
        };

        assert!(t.is_fresh());
    }

    #[test]
    fn test_is_stale() {
        let t = TrackedRef {
            fresh: false,
            val: 5
        };

        assert!(!t.is_fresh());
    }
}

