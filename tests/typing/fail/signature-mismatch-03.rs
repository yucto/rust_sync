use rustre::sync;

sync! {
    #![pass(1)]

    node oui() = (a)
    where
        a : int = 3;

    node non(b : bool) = ()
    where
        o : int = oui(b);
}

fn main() {}
