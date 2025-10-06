#[cfg(test)]
mod tests {
    use derive_more::{Add, Constructor, Display, From, Into, IntoIterator, IsVariant};
    use pretty_assertions::assert_eq;

    #[derive(PartialEq, From, Add, Debug, Display)]
    #[display("something: {_0}")]
    struct MyInt(i32);

    #[derive(PartialEq, From, Into, Debug, Display)]
    #[display("Point2D: {x}, {y}")]
    struct Point2D {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, From, Add, Debug, Display)]
    enum MyEnum {
        #[display("int: {_0}")]
        Int(i32),
        Uint(u32),
        #[display("nothing")]
        Nothing,
    }

    #[test]
    fn derives() {
        assert_eq!(MyInt(11).to_string(), "something: 11");
        assert_eq!(MyInt(11), MyInt(5) + 6.into());
        assert_eq!(Point2D { x: 10, y: 20 }.to_string(), "Point2D: 10, 20");
        assert_eq!((5, 6), Point2D { x: 5, y: 6 }.into());
        assert_eq!(MyEnum::Int(15), (MyEnum::Int(8) + 7.into()).unwrap());
        assert_eq!(MyEnum::Int(15).to_string(), "int: 15");
        assert_eq!(MyEnum::Uint(42).to_string(), "42");
        assert_eq!(MyEnum::Nothing.to_string(), "nothing");
    }

    #[test]
    fn derive_into_iterator() {
        #[derive(IntoIterator)]
        struct MyVec(Vec<i32>);

        #[derive(IntoIterator)]
        struct Numbers {
            #[into_iterator(owned, ref, ref_mut)]
            numbers: Vec<i32>,
            #[allow(unused)]
            useless: bool,
        }

        assert_eq!(Some(5), MyVec(vec![5, 8]).into_iter().next());

        let mut nums = Numbers {
            numbers: vec![100, 200],
            useless: false,
        };
        assert_eq!(Some(&100), (&nums).into_iter().next());
        assert_eq!(Some(&mut 100), (&mut nums).into_iter().next());
        assert_eq!(Some(100), nums.into_iter().next());
    }

    #[test]
    fn derive_is_variant() {
        #[derive(IsVariant)]
        enum Maybe<T> {
            #[allow(unused)]
            Just(T),
            #[allow(unused)]
            Nothing,
        }

        assert!(Maybe::<()>::Nothing.is_nothing());
        assert!(!Maybe::<()>::Nothing.is_just());
    }

    #[test]
    fn derive_constructor() {
        #[derive(Constructor, PartialEq, Debug)]
        struct Point2D {
            x: i32,
            y: i32,
        }

        assert_eq!(Point2D::new(1, 2), Point2D { x: 1, y: 2 });
    }
}
