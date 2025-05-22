pub use crate::generated::conjugation::{
    Conjugation, ConjugationMethods, ConjugationRef, ConjugationRefMethods, ConjugationRefMut,
    ConjugationRefMutMethods,
};

impl ConjugationMethods for Conjugation {}

impl ConjugationRefMutMethods for ConjugationRefMut<'_> {}

impl ConjugationRefMethods for ConjugationRef<'_> {}
