use pastey::paste;

macro_rules! m {
    () => {
        paste! {
            struct [< :replace("H", "W") >];
        }
    };
}

m!();

fn main() {}
