extern crate clonable_vec_of_trait_objects;

use clonable_vec_of_trait_objects::*;

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
    println!("Solution B");
    use solution_b::Enum;
    let vec: Vec<Enum> = vec![Enum::A(A::new()), Enum::B(B::new())];
    for elem in &vec {
        elem.print();
    }
    let vec2 = vec.clone();
    for elem in &vec2 {
        elem.print();
    }

    // Solution C: BoxClone -> Clone to new trait object
    println!("Solution C");
    use solution_c::{BoxCloneableTrait, clone_vec};
    let vec: Vec<Box<BoxCloneableTrait>> = vec![Box::new(A::new()), Box::new(B::new())];
    for elem in &vec {
        elem.print();
    }
    let vec2 = clone_vec(&vec);
    for elem in &vec2 {
        elem.print();
    }
}
