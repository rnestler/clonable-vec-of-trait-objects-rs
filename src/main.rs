extern crate reference_wrapper;

use reference_wrapper::*;

fn main() {
    // Problem: Vec of Trait is not copyable
    let vec: Vec<Box<Trait>> = vec![Box::new(A::new()), Box::new(B::new())];
    for elem in &vec {
        elem.print();
    }
    //let vec2 = vec.clone(); // fails to compile

    // Solution A: Just require Clone
    //use solution_a::CloneAbleTrait;
    // fails to compile: the trait cannot require that `Self : Sized`
    //let vec: Vec<Box<CloneAbleTrait>> = vec![Box::new(A::new()), Box::new(B::new())];

    // Solution B: Enum -> static dispatch
    use solution_b::Enum;
    let vec: Vec<Enum> = vec![Enum::A(A::new()), Enum::B(B::new())];
    for elem in &vec {
        elem.print();
    }
    let vec2 = vec.clone();
    for elem in &vec2 {
        elem.print();
    }
}
