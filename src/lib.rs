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
pub mod typeshare;
pub mod ux;
pub mod validator;

use std::alloc;
use zeroizing_alloc::ZeroAlloc;

#[global_allocator]
static ALLOC: ZeroAlloc<alloc::System> = ZeroAlloc(alloc::System);

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

    use core::alloc::{GlobalAlloc, Layout};
    use std::sync::{Mutex, OnceLock};

    use zeroizing_alloc::ZeroAlloc;

    struct LogGuard;

    impl LogGuard {
        fn new() -> Self {
            LogAlloc::clear_log();
            LogGuard
        }
    }

    impl Drop for LogGuard {
        fn drop(&mut self) {
            LogAlloc::clear_log();
        }
    }

    pub struct LogAlloc;

    impl LogAlloc {
        fn log() -> &'static Mutex<Vec<Vec<u8>>> {
            static LOG: OnceLock<Mutex<Vec<Vec<u8>>>> = OnceLock::new();
            LOG.get_or_init(|| Mutex::new(Vec::new()))
        }

        pub fn allocs() -> Vec<Vec<u8>> {
            Self::log().lock().unwrap().clone()
        }

        pub fn clear_log() {
            Self::log().lock().unwrap().clear();
        }
    }

    unsafe impl GlobalAlloc for LogAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            unsafe { std::alloc::System.alloc(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let slice = unsafe { core::slice::from_raw_parts(ptr, layout.size()) };
            let mut log = Self::log().lock().unwrap();
            log.push(slice.to_vec());
            unsafe {
                std::alloc::System.dealloc(ptr, layout);
            }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            unsafe { std::alloc::System.alloc_zeroed(layout) }
        }
    }

    #[test]
    fn freed_memory_is_zeroed() {
        let _guard = LogGuard::new();

        let alloc = ZeroAlloc(LogAlloc);
        const LEN: usize = 16;
        const ALIGN: usize = 8;

        for _ in 0..5 {
            unsafe {
                let layout = Layout::from_size_align(LEN, ALIGN).unwrap();
                let ptr = alloc.alloc(layout);

                for i in 0..LEN {
                    ptr.add(i).write(i as u8);
                }

                let slice = core::slice::from_raw_parts(ptr, LEN);
                let expected: Vec<u8> = (0..LEN as u8).collect();
                assert_eq!(slice, expected);

                alloc.dealloc(ptr, layout);
            }
        }

        // check that all allocations have been zeroed out.
        for (i, alloc) in LogAlloc::allocs().iter().enumerate() {
            assert!(
                alloc.iter().all(|&b| b == 0),
                "alloc {i} not zeroed: {alloc:?}"
            );
        }
    }

    #[test]
    #[should_panic]
    fn system_allocator_does_not_zero() {
        let _guard = LogGuard::new();

        let alloc = LogAlloc;

        const LEN: usize = 16;
        const ALIGN: usize = 8;

        unsafe {
            let layout = Layout::from_size_align(LEN, ALIGN).unwrap();
            let ptr = alloc.alloc(layout);

            for i in 0..LEN {
                ptr.add(i).write(i as u8);
            }

            let slice = core::slice::from_raw_parts(ptr, LEN);
            let expected: Vec<u8> = (0..LEN as u8).collect();
            assert_eq!(slice, expected);

            alloc.dealloc(ptr, layout);
        }

        // The system allocator does not zero out memory.
        for (i, alloc) in LogAlloc::allocs().iter().enumerate() {
            assert!(
                alloc.iter().all(|&b| b == 0),
                "alloc {i} not zeroed: {alloc:?}"
            );
        }
    }
}
