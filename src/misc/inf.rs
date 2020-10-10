use std::any::{Any, TypeId};
use std::cmp::{Eq, PartialEq, PartialOrd};

pub enum Infinity {
    PosInfinity,
    NegInfinity,
}

impl Infinity {
    pub fn from_ref<T: Any>(other_type: &T) -> Option<&Infinity> {
        if TypeId::of::<Infinity>() == other_type.type_id() {
            unsafe { (other_type as *const T).cast::<Infinity>().as_ref() }
        } else {
            None
        }
    }
}

impl<T: Any> PartialEq<T> for Infinity {
    fn eq(&self, other: &T) -> bool {
        if other.type_id() == self.type_id() {
            let other_inf = Infinity::from_ref(other).unwrap();

            match (self, other_inf) {
                (Infinity::PosInfinity, Infinity::PosInfinity) => {
                    panic!("Operator == can not be applied to two PosInfinity")
                }
                (Infinity::PosInfinity, Infinity::NegInfinity) => false,
                (Infinity::NegInfinity, Infinity::PosInfinity) => false,
                (Infinity::NegInfinity, Infinity::NegInfinity) => {
                    panic!("Operator == can not be applied to two NegInfinity")
                }
            };
        }
        false
    }
}

impl Eq for Infinity {}

impl<T: Any> PartialOrd<T> for Infinity {
    fn partial_cmp(&self, _: &T) -> Option<std::cmp::Ordering> {
        panic!("Not all comparison operators are supported for Infinity")
    }

    fn lt(&self, other: &T) -> bool {
        if other.type_id() == self.type_id() {
            let other_inf = Infinity::from_ref(other).unwrap();

            match (self, other_inf) {
                (Infinity::PosInfinity, Infinity::PosInfinity) => {
                    panic!("Operator < can not be applied to two PosInfinity")
                }
                (Infinity::PosInfinity, Infinity::NegInfinity) => false,
                (Infinity::NegInfinity, Infinity::PosInfinity) => true,
                (Infinity::NegInfinity, Infinity::NegInfinity) => {
                    panic!("Operator < can not be applied to two NegInfinity")
                }
            };
        }
        matches!(&self, Infinity::NegInfinity)
    }

    fn le(&self, other: &T) -> bool {
        self.lt(other)
    }

    fn gt(&self, other: &T) -> bool {
        if other.type_id() == self.type_id() {
            let other_inf = Infinity::from_ref(other).unwrap();

            match (self, other_inf) {
                (Infinity::PosInfinity, Infinity::PosInfinity) => {
                    panic!("Operator < can not be applied to two PosInfinity")
                }
                (Infinity::PosInfinity, Infinity::NegInfinity) => true,
                (Infinity::NegInfinity, Infinity::PosInfinity) => false,
                (Infinity::NegInfinity, Infinity::NegInfinity) => {
                    panic!("Operator < can not be applied to two NegInfinity")
                }
            };
        }
        matches!(&self, Infinity::PosInfinity)
    }

    fn ge(&self, other: &T) -> bool {
        self.gt(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let pos_inf = Infinity::PosInfinity;
        let neg_inf = Infinity::NegInfinity;
        assert!(pos_inf > 0);
        assert!(neg_inf < 0);
        assert!(neg_inf < pos_inf);
    }
}
