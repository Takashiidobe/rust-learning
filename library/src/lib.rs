pub mod anyhow;
pub mod async_trait;
pub mod axum;
pub mod ctor;
pub mod delegate;
pub mod derive_more;
pub mod errors;
pub mod facet;
pub mod faux;
pub mod futures;
pub mod http;
pub mod httpmock;
pub mod itertools;
pub mod ordered_float;
pub mod parking_lot;
pub mod proptest;
pub mod qcell;
pub mod snafu;
pub mod sqlx;
pub mod thiserror;
pub mod thread_local;
pub mod tokio;
pub mod tower;
pub mod typeshare;
pub mod ux;
pub mod validator;

#[cfg(test)]
mod tests {
    use std::mem::transmute;

    #[test]
    fn transmute_fn() {
        // transmuting is reinterpreting the bits as another type.
        fn foo() -> i32 {
            0
        }
        // first have to transmute to a raw pointer to avoid an integer to pointer transmute
        let pointer = foo as *const ();
        // next transmute from *const() to the fn pointer.
        let function = unsafe { transmute::<*const (), fn() -> i32>(pointer) };
        assert_eq!(function(), 0);
    }

    use is_enum::{IsEnum, log_fn};

    #[derive(IsEnum)]
    enum Fruit {
        Apple,
        Banana,
        Pear,
    }

    #[test]
    fn test_is_enum() {
        let f = Fruit::Pear;
        assert!(f.is_pear());
        assert!(!f.is_banana());
        assert!(!f.is_apple());

        let f = Fruit::Apple;
        assert!(!f.is_pear());
        assert!(!f.is_banana());
        assert!(f.is_apple());

        let f = Fruit::Banana;
        assert!(!f.is_pear());
        assert!(f.is_banana());
        assert!(!f.is_apple());
    }

    #[log_fn]
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
