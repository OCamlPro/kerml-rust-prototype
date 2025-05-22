pub use crate::generated::multiplicity::{
    Multiplicity, MultiplicityMethods, MultiplicityRef, MultiplicityRefMethods, MultiplicityRefMut,
    MultiplicityRefMutMethods,
};

impl MultiplicityMethods for Multiplicity {}

impl MultiplicityRefMutMethods for MultiplicityRefMut<'_> {}

impl MultiplicityRefMethods for MultiplicityRef<'_> {}
