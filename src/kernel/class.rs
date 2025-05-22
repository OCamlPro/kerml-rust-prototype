pub use crate::generated::class::{
    Class, ClassMethods, ClassRef, ClassRefMethods, ClassRefMut,
    ClassRefMutMethods,
};

impl ClassMethods for Class {}

impl ClassRefMutMethods for ClassRefMut<'_> {}

impl ClassRefMethods for ClassRef<'_> {}



