use pastey::paste;

paste! {
    fn [< foo :replace(!, "bar") >]() {}
}

fn main() {}
