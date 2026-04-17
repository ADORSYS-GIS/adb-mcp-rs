use rust_mcp_sdk::IdGenerator;

/// An [`IdGenerator`] implementation using CUID2 (collision-resistant unique IDs).
///
/// CUID2 is optimized for horizontal scaling and performance, designed to be
/// collision-resistant and sortable, making it superior to UUID for many use cases.
///
/// See: <https://github.com/paralleldrive/cuid2>
#[derive(Clone, Copy, Debug, Default)]
pub struct CuidGenerator;

impl CuidGenerator {
    /// Creates a new CUID generator.
    pub fn new() -> Self {
        Self
    }
}

impl<T> IdGenerator<T> for CuidGenerator
where
    T: From<String>,
{
    /// Generates a new CUID using the cuid2 crate.
    ///
    /// CUIDs are 24 characters by default and are:
    /// - Collision-resistant
    /// - Sortable (by time)
    /// - URL-safe
    /// - Monotonically increasing (within a single process)
    fn generate(&self) -> T {
        T::from(cuid2::create_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuid_generator() {
        let generator = CuidGenerator::new();
        let id1: String = generator.generate();
        let id2: String = generator.generate();

        // CUIDs should be 24 characters
        assert_eq!(id1.len(), 24);
        assert_eq!(id2.len(), 24);

        // Should be different (extremely unlikely to collide)
        assert_ne!(id1, id2);
    }
}
