use rust_sync::sync;

sync! {
    #![pass(1)]
    
    node oui() = (a)
    where
        a : float = 3.0 as float;
}

fn main() {}
