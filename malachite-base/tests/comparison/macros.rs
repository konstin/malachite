use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_gen, signed_pair_gen, signed_triple_gen, unsigned_gen, unsigned_pair_gen,
    unsigned_triple_gen,
};
use std::cmp::{max, min};

#[test]
pub fn test_min() {
    assert_eq!(min!(4), 4);
    assert_eq!(min!(4, 5, 6), 4);
}

#[test]
pub fn test_max() {
    assert_eq!(max!(4), 4);
    assert_eq!(max!(4, 5, 6), 6);
}

fn unsigned_max_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(max!(x), x);
    });

    unsigned_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(max!(x, y), max(x, y));
    });

    unsigned_triple_gen::<T>().test_properties(|(x, y, z)| {
        assert_eq!(max!(x, y, z), max(max(x, y), z));
        assert_eq!(max!(x, y, z), max(x, max(y, z)));
    });
}

fn signed_max_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        assert_eq!(max!(x), x);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(max!(x, y), max(x, y));
    });

    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        assert_eq!(max!(x, y, z), max(max(x, y), z));
        assert_eq!(max!(x, y, z), max(x, max(y, z)));
    });
}

#[test]
fn max_properties() {
    apply_fn_to_unsigneds!(unsigned_max_properties_helper);
    apply_fn_to_signeds!(signed_max_properties_helper);
}

fn unsigned_min_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(min!(x), x);
    });

    unsigned_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(min!(x, y), min(x, y));
    });

    unsigned_triple_gen::<T>().test_properties(|(x, y, z)| {
        assert_eq!(min!(x, y, z), min(min(x, y), z));
        assert_eq!(min!(x, y, z), min(x, min(y, z)));
    });
}

fn signed_min_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        assert_eq!(min!(x), x);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(min!(x, y), min(x, y));
    });

    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        assert_eq!(min!(x, y, z), min(min(x, y), z));
        assert_eq!(min!(x, y, z), min(x, min(y, z)));
    });
}

#[test]
fn min_properties() {
    apply_fn_to_unsigneds!(unsigned_min_properties_helper);
    apply_fn_to_signeds!(signed_min_properties_helper);
}
