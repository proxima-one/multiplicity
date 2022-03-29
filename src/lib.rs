use ff::Field;

/// A Multiset hash. This is a just a finite field element.
/// We use a slightly different notion of "multiset" than elsewhere. The biggest difference is we allow negative multiplicities. See the readme for more information.
pub struct MultisetHash<F>(pub(crate) F);

impl<F: Field> MultisetHash<F> {
    /// constructs a multiset hash for an empty multiset
    pub fn new() -> Self {
        MultisetHash(F::one())
    }

    /// compute the hash that would result when "adding" `count` instances of `elem` to the underlying multiset.
    pub fn add(&mut self, elem: F, count: u64) -> Self {
        let term = elem.pow_vartime([count]);
        MultisetHash(self.0 * term)
    }

    /// same as [`MultisetHash::add`], but works on any type that implements [`HashToField`] and hashes it to the field before adding it. Just a convenience wrapper.
    pub fn add_elem<T: HashToField<F>>(&mut self, elem: T, count: u64) -> Self {
        let term = HashToField::hash_to_field(&elem).pow_vartime([count]);
        MultisetHash(self.0 * term)
    }

    /// compute the hash that would result when "removing" `count` instances of `elem` from the underlying multiset. Note that this can result in negative multiplicities.
    pub fn remove(&mut self, elem: F, count: u64) -> Self {
        let inv_term = elem.pow_vartime([count]).invert();
        if bool::from(inv_term.is_none()) {
            panic!("elements must be nonzero");
        }
        MultisetHash(self.0 * inv_term.unwrap())
    }

    /// same as [`MultisetHash::remove`], but works on any type that implements [`HashToField`] and hashes it to the field before removing it. Just a convenience wrapper.
    pub fn remove_elem<T: HashToField<F>>(&mut self, elem: T, count: u64) -> Self {
        let inv_term = HashToField::hash_to_field(&elem).pow_vartime([count]).invert();
        if bool::from(inv_term.is_none()) {
            panic!("elements must be nonzero");
        }
        MultisetHash(self.0 * inv_term.unwrap())
    }


    /// returns the hash that would result when performing the "multiset union" between the underlying multisets of `self` and `other`.
    /// Here, we define the "multiset union" as a multiset in which each element's multiplicity is the sum of its multiplicities in the initial multisets.
    pub fn multiset_union(&self, other: &Self) -> Self {
        MultisetHash(self.0 * other.0)
    }

    /// returns the hash that would result when performing the "multiset difference" between the underlying multisets of `self` and `other`.
    /// Here, we define the "multiset difference" as a multiset in which each element's multiplicity is the difference of its multiplicities in the initial multisets.
    /// That means, if an element appears in the underlying multiset for `other` with a higher multiplicity than it does in `self`, the returned hash will respect a multiset in which that element has negative multiplicity.
    pub fn multiset_difference(&self, other: &Self) -> Self {
        let inv = other.0.invert();
        if bool::from(inv.is_none()) {
            panic!("multiset hash of `other` must be nonzero");
        }

        MultisetHash(self.0 * inv.unwrap())
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

    #[test]
    fn test_union() {
        let a: Vec<(Scalar, u64)> = vec![(2.into(), 1), (10.into(), 4), (4.into(), 1), (7.into(), 3), (3.into(), 7)];
        let b: Vec<(Scalar, u64)> = vec![(2.into(), 4), (6.into(), 1), (4.into(), 1), (7.into(), 7), (3.into(), 7)];
        let mut left = MultisetHash::new();
        for &(elem, count) in a.iter() {
            left = left.add(elem, count);
        }

        let mut right = MultisetHash::new();
        for &(elem, count) in b.iter() {
            right = right.add(elem, count);
        }

        let u = left.multiset_union(&right);
        let mut check = MultisetHash::new();
        for &(elem, count) in a.iter() {
            check = check.add(elem, count);
        }
        for &(elem, count) in b.iter() {
            check = check.add(elem, count);
        }
        assert_eq!(u.0, check.0);
    }

    #[test]
    fn test_difference() {
        let a: Vec<(Scalar, u64)> = vec![(50.into(), 1), (10.into(), 4), (4.into(), 1), (7.into(), 3), (3.into(), 7)];
        let b: Vec<(Scalar, u64)> = vec![(2.into(), 4), (6.into(), 1), (4.into(), 1), (7.into(), 7), (3.into(), 7)];

        let mut left = MultisetHash::new();
        let mut right = MultisetHash::new();
        for &(elem, count) in a.iter() {
            left = left.add(elem, count);
        }
        for &(elem, count) in b.iter() {
            right = right.add(elem, count);
        }

        let intersection = left.multiset_difference(&right);
        let mut check = MultisetHash::new();
        check = check.add(50.into(), 1);
        check = check.add(10.into(), 4);
        check = check.remove(7.into(), 4);
        check = check.remove(2.into(), 4);
        check = check.remove(6.into(), 1);
        assert_eq!(intersection.0, check.0);
    }
}