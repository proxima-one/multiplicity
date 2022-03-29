use std::ops::Deref;

use ff::Field;

pub struct MultisetHash<F>(pub(crate) F);

impl<F: Field> MultisetHash<F> {
    pub fn new() -> Self {
        MultisetHash(F::one())
    }

    pub fn add(&mut self, elem: F, count: u64) -> Self {
        let term = elem.pow_vartime([count]);
        MultisetHash(self.0 * term)
    }

    pub fn add_elem<T: HashToField<F>>(&mut self, elem: T, count: u64) -> Self {
        let term = HashToField::hash_to_field(&elem).pow_vartime([count]);
        MultisetHash(self.0 * term)
    }

    pub fn remove(&mut self, elem: F, count: u64) -> Self {
        let inv_term = elem.pow_vartime([count]).invert();
        if bool::from(inv_term.is_none()) {
            panic!("elements must be nonzero");
        }
        MultisetHash(self.0 * inv_term.unwrap())
    }

    pub fn remove_elem<T: HashToField<F>>(&mut self, elem: T, count: u64) -> Self {
        let inv_term = HashToField::hash_to_field(&elem).pow_vartime([count]).invert();
        if bool::from(inv_term.is_none()) {
            panic!("elements must be nonzero");
        }
        MultisetHash(self.0 * inv_term.unwrap())
    }

    pub fn set_union(&self, other: &Self) -> Self {
        MultisetHash(self.0 * other.0)
    }

    pub fn set_difference(&self, other: &Self) -> Self {
        let inv = other.0.invert();
        if bool::from(inv.is_none()) {
            panic!("multiset hash of `other` must be nonzero");
        }

        MultisetHash(self.0 * inv.unwrap())
    }

    pub fn set_intersection(&self, other: &Self) -> Self {
        let diff = self.set_difference(other);
        self.set_difference(&diff)
    } 

    pub fn set_symmetric_difference(&self, other: &Self) -> Self {
        let left = self.set_difference(other);
        let right = other.set_difference(self);
        left.set_union(&right)
    }
}

pub trait HashToField<F: Field> {
    fn hash_to_field(&self) -> F;
}

#[cfg(test)]
mod tests {
    use bls12_381::Scalar;
    use super::*;

    #[test]
    fn test_single_ops() {
        let mut mh = MultisetHash::<Scalar>::new();
        assert_eq!(mh.0, Scalar::one());
        
        mh = mh.add(2.into(), 1);
        mh = mh.remove(2.into(), 1);
        assert_eq!(mh.0, Scalar::one());

        mh = mh.add(5.into(), 4);
        for _ in 0..4 {
            mh = mh.remove(5.into(), 1);
        }
        assert_eq!(mh.0, Scalar::one());

        for _ in 0..27 {
            mh = mh.add(3.into(), 1);
        }
        mh = mh.remove(3.into(), 27);
        assert_eq!(mh.0, Scalar::one());
    }
}