use pastey::paste;

paste! {
    fn [<env!(foo)>]() {}
}

fn main() {}
