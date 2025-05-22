pub use crate::generated::classifier::{
    Classifier, ClassifierMethods, ClassifierRef, ClassifierRefMethods, ClassifierRefMut,
    ClassifierRefMutMethods,
};

impl ClassifierMethods for Classifier {}

impl ClassifierRefMutMethods for ClassifierRefMut<'_> {}

impl ClassifierRefMethods for ClassifierRef<'_> {}

