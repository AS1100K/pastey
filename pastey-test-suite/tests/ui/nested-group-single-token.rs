use pastey::paste;

macro_rules! m {
    ($tokens:tt) => {
        paste! {
            fn [< foo :replace(($tokens), "bar") >]() {}
        }
    };
}

m!(ident);

fn main() {}
