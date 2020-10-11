use num_traits::sign::Signed;
use std::any::{Any, TypeId};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
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
                (Infinity::PosInfinity, Infinity::NegInfinity)
                | (Infinity::NegInfinity, Infinity::PosInfinity) => false,

                (Infinity::PosInfinity, Infinity::PosInfinity)
                | (Infinity::NegInfinity, Infinity::NegInfinity) => {
                    panic!("Two PosInfinity or two NegInfinity are not comparable")
                }
            };
        }
        false
    }
}

impl Eq for Infinity {}

impl<T: Any> PartialOrd<T> for Infinity {
    fn partial_cmp(&self, _: &T) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn lt(&self, other: &T) -> bool {
        if other.type_id() == self.type_id() {
            let other_inf = Infinity::from_ref(other).unwrap();

            match (self, other_inf) {
                (Infinity::PosInfinity, Infinity::NegInfinity) => false,
                (Infinity::NegInfinity, Infinity::PosInfinity) => true,

                (Infinity::PosInfinity, Infinity::PosInfinity)
                | (Infinity::NegInfinity, Infinity::NegInfinity) => {
                    panic!("Two PosInfinity or two NegInfinity are not comparable")
                }
            };
        }
        matches!(&self, Infinity::NegInfinity)
    }

    fn le(&self, other: &T) -> bool {
        self.lt(other)
    }

    fn gt(&self, other: &T) -> bool {
        !self.le(other)
    }

    fn ge(&self, other: &T) -> bool {
        self.gt(other)
    }
}

impl<T: Any> Add<T> for Infinity {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        if self.type_id() == rhs.type_id() {
            let rhs_inf = Infinity::from_ref(&rhs).unwrap();

            return match (self, rhs_inf) {
                (Infinity::PosInfinity, Infinity::NegInfinity)
                | (Infinity::NegInfinity, Infinity::PosInfinity) => {
                    panic!("Can not add PosInfinity and NegInfinity with each other")
                }

                (Infinity::PosInfinity, Infinity::PosInfinity) => Infinity::PosInfinity,
                (Infinity::NegInfinity, Infinity::NegInfinity) => Infinity::NegInfinity,
            };
        }

        self.clone()
    }
}

impl<T: Any> Sub<T> for Infinity {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        if self.type_id() == rhs.type_id() {
            let rhs_inf = Infinity::from_ref(&rhs).unwrap();

            return match (self, rhs_inf) {
                (Infinity::PosInfinity, Infinity::NegInfinity) => Infinity::PosInfinity,
                (Infinity::NegInfinity, Infinity::PosInfinity) => Infinity::NegInfinity,

                (Infinity::PosInfinity, Infinity::PosInfinity)
                | (Infinity::NegInfinity, Infinity::NegInfinity) => panic!(
                    "Can not subtract two PosInfinities or two NegInfinities from each  other"
                ),
            };
        }

        self.clone()
    }
}

impl<T: Any + Signed> Mul<T> for Infinity {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        if rhs.is_negative() {
            -self.clone()
        } else if rhs.is_positive() {
            self.clone()
        } else {
            panic!("Multiplication of zero and Infinity is not supported")
        }
    }
}

impl<T: Any + Signed> Div<T> for Infinity {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        if rhs.is_zero() {
            panic!("Division by zero")
        }

        self.mul(rhs)
    }
}

impl Neg for Infinity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Infinity::PosInfinity => Infinity::NegInfinity,
            Infinity::NegInfinity => Infinity::PosInfinity,
        }
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
        assert!(pos_inf > neg_inf);
        assert!(matches!(pos_inf + 1, Infinity::PosInfinity));
        assert!(matches!(pos_inf * -10, Infinity::NegInfinity));
        assert!(matches!(neg_inf * -10, Infinity::PosInfinity));
        assert!(matches!(pos_inf / 10, Infinity::PosInfinity));
        assert!(matches!(pos_inf / -10, Infinity::NegInfinity));
    }
}
