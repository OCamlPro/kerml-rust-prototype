pub use crate::generated::predicate::{
    Predicate, PredicateMethods, PredicateRef, PredicateRefMethods, PredicateRefMut,
    PredicateRefMutMethods,
};

impl PredicateMethods for Predicate {}

impl PredicateRefMutMethods for PredicateRefMut<'_> {}

impl PredicateRefMethods for PredicateRef<'_> {}
