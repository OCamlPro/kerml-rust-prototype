pub use crate::generated::metaclass::{
    Metaclass, MetaclassMethods, MetaclassRef, MetaclassRefMethods, MetaclassRefMut,
    MetaclassRefMutMethods,
};

impl MetaclassMethods for Metaclass {}

impl MetaclassRefMutMethods for MetaclassRefMut<'_> {}

impl MetaclassRefMethods for MetaclassRef<'_> {}
