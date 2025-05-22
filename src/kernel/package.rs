pub use crate::generated::package::{
    Package, PackageMethods, PackageRef, PackageRefMethods, PackageRefMut, PackageRefMutMethods,
};

impl PackageMethods for Package {}

impl PackageRefMutMethods for PackageRefMut<'_> {}

impl PackageRefMethods for PackageRef<'_> {
    fn include_as_member(
        self,
        element: std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>,
    ) -> bool {
        todo!()
    }
}

