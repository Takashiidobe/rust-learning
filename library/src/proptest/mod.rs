#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    #[derive(Arbitrary, Debug)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    // we can use proptest with deriving arbitrary for our own struct
    proptest! {
        #[allow(clippy::absurd_extreme_comparisons)]
        #[test]
        fn test_color_values(c: Color) {
            prop_assert!(c.r <= u8::MAX);
            prop_assert!(c.g <= u8::MAX);
            prop_assert!(c.b <= u8::MAX);
        }
    }
}
