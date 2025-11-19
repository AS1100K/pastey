use pastey::paste;

macro_rules! m {
    ($name:ident) => {
        paste! {
            struct [< $name:replace >]
        }
    };
}

m!(Pastey);

fn main() {}
