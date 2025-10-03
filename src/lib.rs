pub mod anyhow;
pub mod async_trait;
pub mod axum;
pub mod delegate;
pub mod derive_more;
pub mod errors;
pub mod facet;
pub mod faux;
pub mod futures;
pub mod http;
pub mod httpmock;
pub mod ordered_float;
pub mod proptest;
pub mod qcell;
pub mod snafu;
pub mod sqlx;
pub mod thiserror;
pub mod tokio;
pub mod tower;
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
}
