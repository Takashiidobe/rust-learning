#[cfg(test)]
mod tests {
    use facet::{Facet, VTableView};

    #[derive(Facet, Debug)]
    struct FooBar {
        foo: u32,
        bar: String,
    }

    #[test]
    fn test_reflection() {
        // we can manipulate the info of the struct, provided here
        let shape = FooBar::SHAPE;
        assert_eq!(shape, FOO_BAR_SHAPE);
    }
}
