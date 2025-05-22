#![allow(unused)]
use super::utils::DescendantOf;
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
use super::library_package::{LibraryPackage, LibraryPackageRefMut, LibraryPackageRef};
pub struct PackageInner {
    pub(super) sup_namespace: NamespaceInner,
}
pub trait PackageStruct
where
    Self: PackageStructRefMut,
    Self: PackageStructRef,
    Self: NamespaceStruct,
{}
pub trait PackageStructRefMut
where
    Self: PackageStructRef,
    Self: NamespaceStructRefMut,
{}
pub trait PackageStructRef
where
    Self: NamespaceStructRef,
{}
pub trait PackageUpcast: PackageStruct {
    fn into_package(self) -> Package;
}
pub trait PackageUpcastRefMut<'a>: PackageStructRefMut {
    fn as_package_ref_mut(self) -> PackageRefMut<'a>;
}
pub trait PackageUpcastRef<'a>: PackageStructRef {
    fn as_package_ref(self) -> PackageRef<'a>;
}
impl PackageStruct for PackageInner {}
impl PackageStructRefMut for PackageInner {}
impl PackageStructRef for PackageInner {}
impl NamespaceStruct for PackageInner {}
impl NamespaceStructRefMut for PackageInner {}
impl NamespaceStructRef for PackageInner {}
impl ElementStruct for PackageInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_namespace.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_namespace.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_namespace.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_namespace.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_namespace.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_namespace.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_namespace.is_implied_included()
    }
}
impl ElementStructRefMut for PackageInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_namespace.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_namespace.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_namespace.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_namespace.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_namespace.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_namespace.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_namespace.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for PackageInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_namespace.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_namespace.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_namespace.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_namespace.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_namespace.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_namespace.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_namespace.is_implied_included_ref()
    }
}
pub enum Package {
    Itself(PackageInner),
    LibraryPackage(LibraryPackage),
}
pub enum PackageRefMut<'a> {
    Itself(&'a mut PackageInner),
    LibraryPackage(LibraryPackageRefMut<'a>),
}
pub enum PackageRef<'a> {
    Itself(&'a PackageInner),
    LibraryPackage(LibraryPackageRef<'a>),
}
impl Package {
    pub fn as_ref(&self) -> PackageRef {
        match self {
            Package::Itself(inner) => PackageRef::Itself(&inner),
            Package::LibraryPackage(inner) => PackageRef::LibraryPackage(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> PackageRefMut {
        match self {
            Package::Itself(inner) => PackageRefMut::Itself(inner),
            Package::LibraryPackage(inner) => {
                PackageRefMut::LibraryPackage(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> PackageRefMut<'a> {
    pub fn as_ref(self) -> PackageRef<'a> {
        match self {
            PackageRefMut::Itself(inner) => PackageRef::Itself(inner),
            PackageRefMut::LibraryPackage(inner) => {
                PackageRef::LibraryPackage(inner.as_ref())
            }
        }
    }
}
impl PackageStruct for Package {}
impl PackageStructRefMut for Package {}
impl PackageStructRef for Package {}
impl<'a> PackageStructRefMut for PackageRefMut<'a> {}
impl<'a> PackageStructRef for PackageRefMut<'a> {}
impl<'a> PackageStructRef for PackageRef<'a> {}
impl NamespaceStruct for Package {}
impl NamespaceStructRefMut for Package {}
impl NamespaceStructRef for Package {}
impl<'a> NamespaceStructRefMut for PackageRefMut<'a> {}
impl<'a> NamespaceStructRef for PackageRefMut<'a> {}
impl<'a> NamespaceStructRef for PackageRef<'a> {}
impl ElementStruct for Package {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Package::Itself(x) => x.owned_relationship(),
            Package::LibraryPackage(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Package::Itself(x) => x.owning_relationship(),
            Package::LibraryPackage(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Package::Itself(x) => x.element_id(),
            Package::LibraryPackage(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Package::Itself(x) => x.alias_ids(),
            Package::LibraryPackage(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Package::Itself(x) => x.declared_short_name(),
            Package::LibraryPackage(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Package::Itself(x) => x.declared_name(),
            Package::LibraryPackage(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Package::Itself(x) => x.is_implied_included(),
            Package::LibraryPackage(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Package {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Package::Itself(x) => x.owned_relationship_ref_mut(),
            Package::LibraryPackage(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Package::Itself(x) => x.owning_relationship_ref_mut(),
            Package::LibraryPackage(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Package::Itself(x) => x.element_id_ref_mut(),
            Package::LibraryPackage(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Package::Itself(x) => x.alias_ids_ref_mut(),
            Package::LibraryPackage(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Package::Itself(x) => x.declared_short_name_ref_mut(),
            Package::LibraryPackage(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Package::Itself(x) => x.declared_name_ref_mut(),
            Package::LibraryPackage(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Package::Itself(x) => x.is_implied_included_ref_mut(),
            Package::LibraryPackage(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Package {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Package::Itself(x) => x.owned_relationship_ref(),
            Package::LibraryPackage(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Package::Itself(x) => x.owning_relationship_ref(),
            Package::LibraryPackage(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Package::Itself(x) => x.element_id_ref(),
            Package::LibraryPackage(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Package::Itself(x) => x.alias_ids_ref(),
            Package::LibraryPackage(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Package::Itself(x) => x.declared_short_name_ref(),
            Package::LibraryPackage(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Package::Itself(x) => x.declared_name_ref(),
            Package::LibraryPackage(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Package::Itself(x) => x.is_implied_included_ref(),
            Package::LibraryPackage(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for PackageRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PackageRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            PackageRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            PackageRefMut::Itself(x) => x.element_id_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            PackageRefMut::Itself(x) => x.alias_ids_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PackageRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PackageRefMut::Itself(x) => x.declared_name_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            PackageRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            PackageRefMut::LibraryPackage(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for PackageRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PackageRefMut::Itself(x) => x.owned_relationship_ref(),
            PackageRefMut::LibraryPackage(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PackageRefMut::Itself(x) => x.owning_relationship_ref(),
            PackageRefMut::LibraryPackage(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PackageRefMut::Itself(x) => x.element_id_ref(),
            PackageRefMut::LibraryPackage(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PackageRefMut::Itself(x) => x.alias_ids_ref(),
            PackageRefMut::LibraryPackage(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PackageRefMut::Itself(x) => x.declared_short_name_ref(),
            PackageRefMut::LibraryPackage(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PackageRefMut::Itself(x) => x.declared_name_ref(),
            PackageRefMut::LibraryPackage(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PackageRefMut::Itself(x) => x.is_implied_included_ref(),
            PackageRefMut::LibraryPackage(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for PackageRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PackageRef::Itself(x) => x.owned_relationship_ref(),
            PackageRef::LibraryPackage(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PackageRef::Itself(x) => x.owning_relationship_ref(),
            PackageRef::LibraryPackage(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PackageRef::Itself(x) => x.element_id_ref(),
            PackageRef::LibraryPackage(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PackageRef::Itself(x) => x.alias_ids_ref(),
            PackageRef::LibraryPackage(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PackageRef::Itself(x) => x.declared_short_name_ref(),
            PackageRef::LibraryPackage(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PackageRef::Itself(x) => x.declared_name_ref(),
            PackageRef::LibraryPackage(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PackageRef::Itself(x) => x.is_implied_included_ref(),
            PackageRef::LibraryPackage(x) => x.is_implied_included_ref(),
        }
    }
}
impl PackageUpcast for Package {
    fn into_package(self) -> Package {
        self
    }
}
impl<'a> PackageUpcastRefMut<'a> for PackageRefMut<'a> {
    fn as_package_ref_mut(self) -> PackageRefMut<'a> {
        self
    }
}
impl<'a> PackageUpcastRef<'a> for PackageRef<'a> {
    fn as_package_ref(self) -> PackageRef<'a> {
        self
    }
}
impl NamespaceUpcast for Package {
    fn into_namespace(self) -> Namespace {
        Namespace::Package(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for PackageRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        NamespaceRefMut::Package(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for PackageRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        NamespaceRef::Package(self).as_namespace_ref()
    }
}
impl ElementUpcast for Package {
    fn into_element(self) -> Element {
        Namespace::Package(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for PackageRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        NamespaceRefMut::Package(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for PackageRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        NamespaceRef::Package(self).as_element_ref()
    }
}
pub trait PackageDowncast {
    fn try_into_library_package(self) -> Result<LibraryPackage, String>;
}
pub trait PackageDowncastRefMut<'a> {
    fn try_as_library_package_ref_mut(self) -> Result<LibraryPackageRefMut<'a>, String>;
}
pub trait PackageDowncastRef<'a> {
    fn try_as_library_package_ref(self) -> Result<LibraryPackageRef<'a>, String>;
}
impl PackageDowncast for Package {
    fn try_into_library_package(self) -> Result<LibraryPackage, String> {
        match self {
            Package::LibraryPackage(e) => Ok(e),
            _ => Err("Not a LibraryPackage".into()),
        }
    }
}
impl<'a> PackageDowncastRefMut<'a> for PackageRefMut<'a> {
    fn try_as_library_package_ref_mut(self) -> Result<LibraryPackageRefMut<'a>, String> {
        match self {
            PackageRefMut::LibraryPackage(e) => Ok(e),
            _ => Err("Not a LibraryPackage".into()),
        }
    }
}
impl<'a> PackageDowncastRef<'a> for PackageRef<'a> {
    fn try_as_library_package_ref(self) -> Result<LibraryPackageRef<'a>, String> {
        match self {
            PackageRef::LibraryPackage(e) => Ok(e),
            _ => Err("Not a LibraryPackage".into()),
        }
    }
}
pub trait PackageMethodsDescendants
where
    Self: DescendantOf<Package>,
    Self::Via: PackageMethods,
    Self: Sized,
{}
pub trait PackageMethods: Sized {}
impl<T: PackageMethodsDescendants> PackageMethods for T
where
    T::Via: PackageMethods,
{}
impl DescendantOf<Namespace> for Package {
    type Via = Namespace;
    fn into_via(self) -> Self::Via {
        self.into_namespace()
    }
}
impl NamespaceMethodsDescendants for Package {}
impl DescendantOf<Element> for Package {
    type Via = Namespace;
    fn into_via(self) -> Self::Via {
        self.into_namespace()
    }
}
impl ElementMethodsDescendants for Package {}
pub trait PackageRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<PackageRefMut<'a>>,
    Self::Via: PackageRefMutMethods,
    Self: Sized,
{}
pub trait PackageRefMutMethods: Sized {}
impl<'a, T: PackageRefMutMethodsDescendants<'a>> PackageRefMutMethods for T
where
    T::Via: PackageRefMutMethods,
{}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for PackageRefMut<'a> {
    type Via = NamespaceRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for PackageRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for PackageRefMut<'a> {
    type Via = NamespaceRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for PackageRefMut<'a> {}
pub trait PackageRefMethodsDescendants<'a>
where
    Self: DescendantOf<PackageRef<'a>>,
    Self::Via: PackageRefMethods,
    Self: Sized,
{
    fn include_as_member_impl(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool {
        self.into_via().include_as_member(element)
    }
}
pub trait PackageRefMethods: Sized {
    fn include_as_member(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool;
}
impl<'a, T: PackageRefMethodsDescendants<'a>> PackageRefMethods for T
where
    T::Via: PackageRefMethods,
{
    fn include_as_member(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool {
        PackageRefMethodsDescendants::include_as_member_impl(self, element)
    }
}
impl<'a> DescendantOf<NamespaceRef<'a>> for PackageRef<'a> {
    type Via = NamespaceRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for PackageRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for PackageRef<'a> {
    type Via = NamespaceRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for PackageRef<'a> {}

