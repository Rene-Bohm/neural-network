use std::ops::Add;

use f64;

pub fn vec_add(lhs: &Vec<f64>, rhs: &Vec<f64>) -> Vec<f64> {
    let mut new: Vec<f64> = Vec::new();

    if (lhs.len() == 0 || rhs.len() == 0 || lhs.len() != rhs.len()) {
        panic!("input vecs is one");
    } else {
        for i in 0..lhs.len() {
            new.push(lhs[i] + rhs[i]);
        }
    }

    new
}

/*

use std::any::{Any, TypeId};

trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}

// implement this trait for every type that implements `Any` (which is most types)
impl<T: ?Sized + Any> InstanceOf for T {}


*/
