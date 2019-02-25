pub trait Trait {
    fn print(&self);
}

#[derive(Clone, Debug)]
pub struct A {
}

impl A {
    pub fn new() -> A {
        A{}
    }
}


impl Trait for A {
    fn print(&self) {
        println!("A ");
    }
}

#[derive(Clone, Debug)]
pub struct B {
}

impl B {
    pub fn new() -> B {
        B{}
    }
}

impl Trait for B {
    fn print(&self) {
        println!("B ");
    }
}

pub mod solution_a;
pub mod solution_b;
pub mod solution_c;
