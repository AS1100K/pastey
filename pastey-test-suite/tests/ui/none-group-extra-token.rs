use pastey::paste;

macro_rules! m {
    ($e:expr) => {
        paste! { fn [< $e foo >]() {} }
    };
}

m!(a > b);

fn main() {}
