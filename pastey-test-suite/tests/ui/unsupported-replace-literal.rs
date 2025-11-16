use pastey::paste;

macro_rules! m {
    ($name:ident) => {
        paste! {
            struct [< $name:replace(0, "Zero") >];
        }
    };
}

m!(Year2000);

fn main() {}
