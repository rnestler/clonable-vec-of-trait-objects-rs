use crate::*;

pub trait BoxCloneableTrait: Trait {
    fn box_clone(&self) -> Box<BoxCloneableTrait>; 
}

impl BoxCloneableTrait for A {
    fn box_clone(&self) -> Box<BoxCloneableTrait> {
        Box::new(self.clone())
    }
}

impl BoxCloneableTrait for B {
    fn box_clone(&self) -> Box<BoxCloneableTrait> {
        Box::new(self.clone())
    }
}

pub fn clone_vec(vec: &Vec<Box<BoxCloneableTrait>>) -> Vec<Box<BoxCloneableTrait>>{
    let mut result = vec![];
    for v in vec.iter() {
        result.push(v.box_clone());
    }
    result
}
