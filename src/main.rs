extern crate reference_wrapper;

use reference_wrapper::*;

fn main() {
    // Problem: Vec of IFace is not copyable
    let vec: Vec<Box<Iface>> = vec![Box::new(A::new()), Box::new(B::new())];
    for elem in &vec {
        elem.print();
    }
    //let vec2 = vec.clone(); // fails to compile

    // Solution A: Just require copy
    //use solution_a::CloneAbleIFace;
    // fails to compile: the trait cannot require that `Self : Sized`
    //let vec: Vec<Box<CloneAbleIFace>> = vec![Box::new(A::new()), Box::new(B::new())];
}
