pub use crate::generated::library_package::{
    LibraryPackage, LibraryPackageMethods, LibraryPackageRef, LibraryPackageRefMethods,
    LibraryPackageRefMut, LibraryPackageRefMutMethods,
};

impl LibraryPackageMethods for LibraryPackage {}

impl LibraryPackageRefMutMethods for LibraryPackageRefMut<'_> {}

impl LibraryPackageRefMethods for LibraryPackageRef<'_> {}
