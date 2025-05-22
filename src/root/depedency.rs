pub use crate::generated::dependency::{
    Dependency, DependencyMethods, DependencyRef, DependencyRefMethods, DependencyRefMut,
    DependencyRefMutMethods,
};

impl DependencyMethods for Dependency {}

impl<'a> DependencyRefMutMethods for DependencyRefMut<'a> {}

impl<'a> DependencyRefMethods for DependencyRef<'a> {}
