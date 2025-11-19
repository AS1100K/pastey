use pastey::paste;

macro_rules! m {
    ($name:ident) => {
        paste! {
            struct [< $name:replace("P" "LibP") >];
        }
    };
}

m!(Pastey);

fn main() {}
