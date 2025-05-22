pub use crate::generated::subclassification::{
    Subclassification, SubclassificationMethods, SubclassificationRef, SubclassificationRefMethods,
    SubclassificationRefMut, SubclassificationRefMutMethods,
};

impl SubclassificationMethods for Subclassification {}

impl SubclassificationRefMutMethods for SubclassificationRefMut<'_> {}

impl SubclassificationRefMethods for SubclassificationRef<'_> {}
