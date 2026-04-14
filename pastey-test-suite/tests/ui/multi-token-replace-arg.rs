use pastey::paste;

macro_rules! m {
    ($e:expr) => {
        paste! {
            fn [< foo :replace($e, bar) >]() {}
        }
    };
}

m!(1 + 2);

fn main() {}
