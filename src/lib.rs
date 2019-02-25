pub trait Iface {
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


impl Iface for A {
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

impl Iface for B {
    fn print(&self) {
        println!("B ");
    }
}

pub mod solution_a;
