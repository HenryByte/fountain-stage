//! Stage extensions for Fountain syntax.

/// Returns the greeting printed by the `fountain-stage` binary.
///
/// ```
/// assert_eq!(fountain_stage::greeting(), "Hello, world!");
/// ```
#[must_use]
pub fn greeting() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_hello_world() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
