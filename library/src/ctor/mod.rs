#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ctor::{ctor, dtor};
    // use ctor and dtor to run hooks on program initialization/cleanup.
    // however, be careful -- if static initialization order is non-deterministic, this can lead to
    // program crashes. It's generally best to use the Once structs for lazy setup at call time.

    #[ctor]
    fn my_constructor() {
        println!("This runs at program startup!");
    }

    // This is like Once's get_or_init, except it initializes at program start.
    #[ctor]
    static STATIC_CTOR: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };

    #[dtor]
    fn my_destructor() {
        println!("This runs at program shutdown!");
    }
}
