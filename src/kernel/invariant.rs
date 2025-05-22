pub use crate::generated::invariant::{
    Invariant, InvariantMethods, InvariantRef, InvariantRefMethods, InvariantRefMut,
    InvariantRefMutMethods,
};

impl InvariantMethods for Invariant {}

impl InvariantRefMutMethods for InvariantRefMut<'_> {}

impl InvariantRefMethods for InvariantRef<'_> {}
