#![allow(unused)]
use super::utils::DescendantOf;
use super::package::{
    Package, PackageRefMut, PackageRef, PackageStruct, PackageStructRefMut,
    PackageStructRef, PackageInner, PackageUpcast, PackageUpcastRefMut, PackageUpcastRef,
    PackageMethodsDescendants, PackageRefMutMethodsDescendants,
    PackageRefMethodsDescendants,
};
use super::namespace::{
    Namespace, NamespaceRefMut, NamespaceRef, NamespaceStruct, NamespaceStructRefMut,
    NamespaceStructRef, NamespaceInner, NamespaceUpcast, NamespaceUpcastRefMut,
    NamespaceUpcastRef, NamespaceMethodsDescendants, NamespaceRefMutMethodsDescendants,
    NamespaceRefMethodsDescendants,
};
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
pub struct LibraryPackageInner {
    pub(super) sup_package: PackageInner,
    pub(super) is_standard: bool,
}
pub trait LibraryPackageStruct
where
    Self: LibraryPackageStructRefMut,
    Self: LibraryPackageStructRef,
    Self: PackageStruct,
{
    fn is_standard(self) -> bool;
}
pub trait LibraryPackageStructRefMut
where
    Self: LibraryPackageStructRef,
    Self: PackageStructRefMut,
{
    fn is_standard_ref_mut(&mut self) -> &mut bool;
}
pub trait LibraryPackageStructRef
where
    Self: PackageStructRef,
{
    fn is_standard_ref(&self) -> &bool;
}
pub trait LibraryPackageUpcast: LibraryPackageStruct {
    fn into_library_package(self) -> LibraryPackage;
}
pub trait LibraryPackageUpcastRefMut<'a>: LibraryPackageStructRefMut {
    fn as_library_package_ref_mut(self) -> LibraryPackageRefMut<'a>;
}
pub trait LibraryPackageUpcastRef<'a>: LibraryPackageStructRef {
    fn as_library_package_ref(self) -> LibraryPackageRef<'a>;
}
impl LibraryPackageStruct for LibraryPackageInner {
    fn is_standard(self) -> bool {
        self.is_standard
    }
}
impl LibraryPackageStructRefMut for LibraryPackageInner {
    fn is_standard_ref_mut(&mut self) -> &mut bool {
        &mut self.is_standard
    }
}
impl LibraryPackageStructRef for LibraryPackageInner {
    fn is_standard_ref(&self) -> &bool {
        &self.is_standard
    }
}
impl PackageStruct for LibraryPackageInner {}
impl PackageStructRefMut for LibraryPackageInner {}
impl PackageStructRef for LibraryPackageInner {}
impl NamespaceStruct for LibraryPackageInner {}
impl NamespaceStructRefMut for LibraryPackageInner {}
impl NamespaceStructRef for LibraryPackageInner {}
impl ElementStruct for LibraryPackageInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_package.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_package.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_package.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_package.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_package.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_package.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_package.is_implied_included()
    }
}
impl ElementStructRefMut for LibraryPackageInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_package.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_package.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_package.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_package.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_package.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_package.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_package.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for LibraryPackageInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_package.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_package.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_package.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_package.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_package.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_package.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_package.is_implied_included_ref()
    }
}
pub enum LibraryPackage {
    Itself(LibraryPackageInner),
}
pub enum LibraryPackageRefMut<'a> {
    Itself(&'a mut LibraryPackageInner),
}
pub enum LibraryPackageRef<'a> {
    Itself(&'a LibraryPackageInner),
}
impl LibraryPackage {
    pub fn as_ref(&self) -> LibraryPackageRef {
        match self {
            LibraryPackage::Itself(inner) => LibraryPackageRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LibraryPackageRefMut {
        match self {
            LibraryPackage::Itself(inner) => LibraryPackageRefMut::Itself(inner),
        }
    }
}
impl<'a> LibraryPackageRefMut<'a> {
    pub fn as_ref(self) -> LibraryPackageRef<'a> {
        match self {
            LibraryPackageRefMut::Itself(inner) => LibraryPackageRef::Itself(inner),
        }
    }
}
impl LibraryPackageStruct for LibraryPackage {
    fn is_standard(self) -> bool {
        match self {
            LibraryPackage::Itself(x) => x.is_standard(),
        }
    }
}
impl LibraryPackageStructRefMut for LibraryPackage {
    fn is_standard_ref_mut(&mut self) -> &mut bool {
        match self {
            LibraryPackage::Itself(x) => x.is_standard_ref_mut(),
        }
    }
}
impl LibraryPackageStructRef for LibraryPackage {
    fn is_standard_ref(&self) -> &bool {
        match self {
            LibraryPackage::Itself(x) => x.is_standard_ref(),
        }
    }
}
impl<'a> LibraryPackageStructRefMut for LibraryPackageRefMut<'a> {
    fn is_standard_ref_mut(&mut self) -> &mut bool {
        match self {
            LibraryPackageRefMut::Itself(x) => x.is_standard_ref_mut(),
        }
    }
}
impl<'a> LibraryPackageStructRef for LibraryPackageRefMut<'a> {
    fn is_standard_ref(&self) -> &bool {
        match self {
            LibraryPackageRefMut::Itself(x) => x.is_standard_ref(),
        }
    }
}
impl<'a> LibraryPackageStructRef for LibraryPackageRef<'a> {
    fn is_standard_ref(&self) -> &bool {
        match self {
            LibraryPackageRef::Itself(x) => x.is_standard_ref(),
        }
    }
}
impl PackageStruct for LibraryPackage {}
impl PackageStructRefMut for LibraryPackage {}
impl PackageStructRef for LibraryPackage {}
impl<'a> PackageStructRefMut for LibraryPackageRefMut<'a> {}
impl<'a> PackageStructRef for LibraryPackageRefMut<'a> {}
impl<'a> PackageStructRef for LibraryPackageRef<'a> {}
impl NamespaceStruct for LibraryPackage {}
impl NamespaceStructRefMut for LibraryPackage {}
impl NamespaceStructRef for LibraryPackage {}
impl<'a> NamespaceStructRefMut for LibraryPackageRefMut<'a> {}
impl<'a> NamespaceStructRef for LibraryPackageRefMut<'a> {}
impl<'a> NamespaceStructRef for LibraryPackageRef<'a> {}
impl ElementStruct for LibraryPackage {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackage::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackage::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LibraryPackage::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LibraryPackage::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LibraryPackage::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LibraryPackage {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackage::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LibraryPackage::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LibraryPackage::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LibraryPackage::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LibraryPackage::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LibraryPackage {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackage::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackage::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LibraryPackage::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LibraryPackage::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackage::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LibraryPackage::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LibraryPackageRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LibraryPackageRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LibraryPackageRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LibraryPackageRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LibraryPackageRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LibraryPackageRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackageRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LibraryPackageRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LibraryPackageRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackageRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LibraryPackageRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LibraryPackageRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LibraryPackageRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackageRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LibraryPackageRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LibraryPackageRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LibraryPackageUpcast for LibraryPackage {
    fn into_library_package(self) -> LibraryPackage {
        self
    }
}
impl<'a> LibraryPackageUpcastRefMut<'a> for LibraryPackageRefMut<'a> {
    fn as_library_package_ref_mut(self) -> LibraryPackageRefMut<'a> {
        self
    }
}
impl<'a> LibraryPackageUpcastRef<'a> for LibraryPackageRef<'a> {
    fn as_library_package_ref(self) -> LibraryPackageRef<'a> {
        self
    }
}
impl PackageUpcast for LibraryPackage {
    fn into_package(self) -> Package {
        Package::LibraryPackage(self).into_package()
    }
}
impl<'a> PackageUpcastRefMut<'a> for LibraryPackageRefMut<'a> {
    fn as_package_ref_mut(self) -> PackageRefMut<'a> {
        PackageRefMut::LibraryPackage(self).as_package_ref_mut()
    }
}
impl<'a> PackageUpcastRef<'a> for LibraryPackageRef<'a> {
    fn as_package_ref(self) -> PackageRef<'a> {
        PackageRef::LibraryPackage(self).as_package_ref()
    }
}
impl NamespaceUpcast for LibraryPackage {
    fn into_namespace(self) -> Namespace {
        Package::LibraryPackage(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LibraryPackageRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        PackageRefMut::LibraryPackage(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LibraryPackageRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        PackageRef::LibraryPackage(self).as_namespace_ref()
    }
}
impl ElementUpcast for LibraryPackage {
    fn into_element(self) -> Element {
        Package::LibraryPackage(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LibraryPackageRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        PackageRefMut::LibraryPackage(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LibraryPackageRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        PackageRef::LibraryPackage(self).as_element_ref()
    }
}
pub trait LibraryPackageDowncast {}
pub trait LibraryPackageDowncastRefMut<'a> {}
pub trait LibraryPackageDowncastRef<'a> {}
impl LibraryPackageDowncast for LibraryPackage {}
impl<'a> LibraryPackageDowncastRefMut<'a> for LibraryPackageRefMut<'a> {}
impl<'a> LibraryPackageDowncastRef<'a> for LibraryPackageRef<'a> {}
pub trait LibraryPackageMethodsDescendants
where
    Self: DescendantOf<LibraryPackage>,
    Self::Via: LibraryPackageMethods,
    Self: Sized,
{}
pub trait LibraryPackageMethods: Sized {}
impl<T: LibraryPackageMethodsDescendants> LibraryPackageMethods for T
where
    T::Via: LibraryPackageMethods,
{}
impl DescendantOf<Package> for LibraryPackage {
    type Via = Package;
    fn into_via(self) -> Self::Via {
        self.into_package()
    }
}
impl PackageMethodsDescendants for LibraryPackage {}
impl DescendantOf<Namespace> for LibraryPackage {
    type Via = Package;
    fn into_via(self) -> Self::Via {
        self.into_package()
    }
}
impl NamespaceMethodsDescendants for LibraryPackage {}
impl DescendantOf<Element> for LibraryPackage {
    type Via = Package;
    fn into_via(self) -> Self::Via {
        self.into_package()
    }
}
impl ElementMethodsDescendants for LibraryPackage {}
pub trait LibraryPackageRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LibraryPackageRefMut<'a>>,
    Self::Via: LibraryPackageRefMutMethods,
    Self: Sized,
{}
pub trait LibraryPackageRefMutMethods: Sized {}
impl<'a, T: LibraryPackageRefMutMethodsDescendants<'a>> LibraryPackageRefMutMethods for T
where
    T::Via: LibraryPackageRefMutMethods,
{}
impl<'a> DescendantOf<PackageRefMut<'a>> for LibraryPackageRefMut<'a> {
    type Via = PackageRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref_mut()
    }
}
impl<'a> PackageRefMutMethodsDescendants<'a> for LibraryPackageRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LibraryPackageRefMut<'a> {
    type Via = PackageRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LibraryPackageRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LibraryPackageRefMut<'a> {
    type Via = PackageRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LibraryPackageRefMut<'a> {}
pub trait LibraryPackageRefMethodsDescendants<'a>
where
    Self: DescendantOf<LibraryPackageRef<'a>>,
    Self::Via: LibraryPackageRefMethods,
    Self: Sized,
{}
pub trait LibraryPackageRefMethods: Sized {}
impl<'a, T: LibraryPackageRefMethodsDescendants<'a>> LibraryPackageRefMethods for T
where
    T::Via: LibraryPackageRefMethods,
{}
impl<'a> DescendantOf<PackageRef<'a>> for LibraryPackageRef<'a> {
    type Via = PackageRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref()
    }
}
impl<'a> PackageRefMethodsDescendants<'a> for LibraryPackageRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LibraryPackageRef<'a> {
    type Via = PackageRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LibraryPackageRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LibraryPackageRef<'a> {
    type Via = PackageRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_package_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LibraryPackageRef<'a> {}

