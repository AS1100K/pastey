use pastey::paste;

macro_rules! m {
    ($name:ident) => {
        paste! {
            struct [< $name:camel # >];
        }
    };
}

m!(r#loop);

fn main() {}
