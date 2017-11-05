/// Tracked value for Copy types.
#[derive(Copy, Clone)]
pub struct Tracked<T: PartialEq + Copy> {
    fresh: bool,
    val: T
}

impl<T: PartialEq + Copy> Tracked<T> {
    pub fn new(val: T) -> Self {
        Tracked {
            fresh: true,
            val
        }
    }

    /// Set a new value, marked as fresh if not equal to the existing value.
    pub fn set(&mut self, val: T) {
        if self.val != val {
            self.val = val;
            self.fresh = true;
        }
    }

    /// Get the current value, marking it as stale.
    pub fn get(&mut self) -> T {
        self.fresh = false;
        self.val
    }

    /// Get the current value if it's been modified since last time we checked, marking it as stale.
    pub fn get_if_fresh(&mut self) -> Option<T> {
        if self.fresh {
            Some(self.get())
        } else {
            None
        }
    }

    // Get the current value without marking it.
    pub fn peek(&self) -> T {
        self.val
    }

    pub fn is_fresh(&self) -> bool {
        self.fresh
    }
}
impl<T: Default + Copy + PartialEq> Default for Tracked<T> {
    fn default() -> Self {
        Tracked::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Tracked::new(5);

        assert!(t.is_fresh());
        assert_eq!(t.peek(), 5);
    }

    #[test]
    fn test_set_same_val() {
        let mut t = Tracked::new(5);

        // Reset to stale
        t.get();

        // Same value - still stale
        t.set(5);

        assert!(!t.is_fresh());
    }

    #[test]
    fn test_set_different_val() {
        let mut t = Tracked::new(5);

        // Reset to stale
        t.get();

        // Different value - fresh again
        t.set(6);

        assert!(t.is_fresh());
        assert_eq!(t.peek(), 6);
    }

    #[test]
    fn test_get() {
        let mut t = Tracked::new(5);

        let v = t.get();

        assert!(!t.is_fresh());
        assert_eq!(v, 5);
    }

    #[test]
    fn test_get_if_fresh() {
        let mut t = Tracked::new("hello");

        let v = t.get_if_fresh();

        assert!(!t.is_fresh());
        assert!(v.is_some());
        assert_eq!(v.unwrap(), "hello");
    }

    #[test]
    fn test_get_if_fresh_stale() {
        let mut t = Tracked::new("hello");

        // Mark as stale
        t.get();

        let v = t.get_if_fresh();

        assert!(v.is_none());
    }

    #[test]
    fn test_peek() {
        let t = Tracked::new(777);

        let v = t.peek();

        assert!(t.is_fresh());
        assert_eq!(v, 777);
    }

    #[test]
    fn test_is_fresh() {
        let t = Tracked {
            fresh: true,
            val: 5
        };

        assert!(t.is_fresh());
    }

    #[test]
    fn test_is_stale() {
        let t = Tracked {
            fresh: false,
            val: 5
        };

        assert!(!t.is_fresh());
    }
}
