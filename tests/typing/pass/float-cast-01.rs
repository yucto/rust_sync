use rust_sync::sync;

sync! {
    node oui() = (a)
    where
        a : float = 3 as float;
}

fn main() {}
