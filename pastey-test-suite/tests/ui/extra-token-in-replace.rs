use pastey::paste;

macro_rules! m {
    ($name:ident) => {
        paste! {
            struct [< $name:replace("a", "b", "c") >];
        }
    };
}

m!(Pastey);

fn main() {}
