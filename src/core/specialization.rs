pub use crate::generated::specialization::{
    Specialization, SpecializationMethods, SpecializationRef, SpecializationRefMethods, SpecializationRefMut,
    SpecializationRefMutMethods,
};

impl SpecializationMethods for Specialization {}

impl SpecializationRefMutMethods for SpecializationRefMut<'_> {}

impl SpecializationRefMethods for SpecializationRef<'_> {}

