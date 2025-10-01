#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ux::u3;

    // arbitrarily sized integers.
    // new panics if the integer is too large, though, instead of at compile time
    #[test]
    fn test_u3() {
        assert_eq!(u3::MAX, u3::new(7));
    }
}
