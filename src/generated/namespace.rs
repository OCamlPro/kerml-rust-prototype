#![allow(unused)]
use super::utils::DescendantOf;
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
use super::type_::{Type, TypeRefMut, TypeRef};
use super::package::{Package, PackageRefMut, PackageRef};
pub struct NamespaceInner {
    pub(super) sup_element: ElementInner,
}
pub trait NamespaceStruct
where
    Self: NamespaceStructRefMut,
    Self: NamespaceStructRef,
    Self: ElementStruct,
{}
pub trait NamespaceStructRefMut
where
    Self: NamespaceStructRef,
    Self: ElementStructRefMut,
{}
pub trait NamespaceStructRef
where
    Self: ElementStructRef,
{}
pub trait NamespaceUpcast: NamespaceStruct {
    fn into_namespace(self) -> Namespace;
}
pub trait NamespaceUpcastRefMut<'a>: NamespaceStructRefMut {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a>;
}
pub trait NamespaceUpcastRef<'a>: NamespaceStructRef {
    fn as_namespace_ref(self) -> NamespaceRef<'a>;
}
impl NamespaceStruct for NamespaceInner {}
impl NamespaceStructRefMut for NamespaceInner {}
impl NamespaceStructRef for NamespaceInner {}
impl ElementStruct for NamespaceInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_element.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_element.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_element.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_element.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_element.is_implied_included()
    }
}
impl ElementStructRefMut for NamespaceInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_element.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_element.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_element.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_element.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for NamespaceInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_element.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_element.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_element.is_implied_included_ref()
    }
}
pub enum Namespace {
    Itself(NamespaceInner),
    Type(Type),
    Package(Package),
}
pub enum NamespaceRefMut<'a> {
    Itself(&'a mut NamespaceInner),
    Type(TypeRefMut<'a>),
    Package(PackageRefMut<'a>),
}
pub enum NamespaceRef<'a> {
    Itself(&'a NamespaceInner),
    Type(TypeRef<'a>),
    Package(PackageRef<'a>),
}
impl Namespace {
    pub fn as_ref(&self) -> NamespaceRef {
        match self {
            Namespace::Itself(inner) => NamespaceRef::Itself(&inner),
            Namespace::Type(inner) => NamespaceRef::Type(inner.as_ref()),
            Namespace::Package(inner) => NamespaceRef::Package(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> NamespaceRefMut {
        match self {
            Namespace::Itself(inner) => NamespaceRefMut::Itself(inner),
            Namespace::Type(inner) => NamespaceRefMut::Type(inner.as_ref_mut()),
            Namespace::Package(inner) => NamespaceRefMut::Package(inner.as_ref_mut()),
        }
    }
}
impl<'a> NamespaceRefMut<'a> {
    pub fn as_ref(self) -> NamespaceRef<'a> {
        match self {
            NamespaceRefMut::Itself(inner) => NamespaceRef::Itself(inner),
            NamespaceRefMut::Type(inner) => NamespaceRef::Type(inner.as_ref()),
            NamespaceRefMut::Package(inner) => NamespaceRef::Package(inner.as_ref()),
        }
    }
}
impl NamespaceStruct for Namespace {}
impl NamespaceStructRefMut for Namespace {}
impl NamespaceStructRef for Namespace {}
impl<'a> NamespaceStructRefMut for NamespaceRefMut<'a> {}
impl<'a> NamespaceStructRef for NamespaceRefMut<'a> {}
impl<'a> NamespaceStructRef for NamespaceRef<'a> {}
impl ElementStruct for Namespace {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Namespace::Itself(x) => x.owned_relationship(),
            Namespace::Type(x) => x.owned_relationship(),
            Namespace::Package(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Namespace::Itself(x) => x.owning_relationship(),
            Namespace::Type(x) => x.owning_relationship(),
            Namespace::Package(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Namespace::Itself(x) => x.element_id(),
            Namespace::Type(x) => x.element_id(),
            Namespace::Package(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Namespace::Itself(x) => x.alias_ids(),
            Namespace::Type(x) => x.alias_ids(),
            Namespace::Package(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_short_name(),
            Namespace::Type(x) => x.declared_short_name(),
            Namespace::Package(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_name(),
            Namespace::Type(x) => x.declared_name(),
            Namespace::Package(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Namespace::Itself(x) => x.is_implied_included(),
            Namespace::Type(x) => x.is_implied_included(),
            Namespace::Package(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Namespace {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Namespace::Itself(x) => x.owned_relationship_ref_mut(),
            Namespace::Type(x) => x.owned_relationship_ref_mut(),
            Namespace::Package(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Namespace::Itself(x) => x.owning_relationship_ref_mut(),
            Namespace::Type(x) => x.owning_relationship_ref_mut(),
            Namespace::Package(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Namespace::Itself(x) => x.element_id_ref_mut(),
            Namespace::Type(x) => x.element_id_ref_mut(),
            Namespace::Package(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Namespace::Itself(x) => x.alias_ids_ref_mut(),
            Namespace::Type(x) => x.alias_ids_ref_mut(),
            Namespace::Package(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_short_name_ref_mut(),
            Namespace::Type(x) => x.declared_short_name_ref_mut(),
            Namespace::Package(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_name_ref_mut(),
            Namespace::Type(x) => x.declared_name_ref_mut(),
            Namespace::Package(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Namespace::Itself(x) => x.is_implied_included_ref_mut(),
            Namespace::Type(x) => x.is_implied_included_ref_mut(),
            Namespace::Package(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Namespace {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Namespace::Itself(x) => x.owned_relationship_ref(),
            Namespace::Type(x) => x.owned_relationship_ref(),
            Namespace::Package(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Namespace::Itself(x) => x.owning_relationship_ref(),
            Namespace::Type(x) => x.owning_relationship_ref(),
            Namespace::Package(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Namespace::Itself(x) => x.element_id_ref(),
            Namespace::Type(x) => x.element_id_ref(),
            Namespace::Package(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Namespace::Itself(x) => x.alias_ids_ref(),
            Namespace::Type(x) => x.alias_ids_ref(),
            Namespace::Package(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_short_name_ref(),
            Namespace::Type(x) => x.declared_short_name_ref(),
            Namespace::Package(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Namespace::Itself(x) => x.declared_name_ref(),
            Namespace::Type(x) => x.declared_name_ref(),
            Namespace::Package(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Namespace::Itself(x) => x.is_implied_included_ref(),
            Namespace::Type(x) => x.is_implied_included_ref(),
            Namespace::Package(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for NamespaceRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            NamespaceRefMut::Type(x) => x.owned_relationship_ref_mut(),
            NamespaceRefMut::Package(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            NamespaceRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            NamespaceRefMut::Type(x) => x.owning_relationship_ref_mut(),
            NamespaceRefMut::Package(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            NamespaceRefMut::Itself(x) => x.element_id_ref_mut(),
            NamespaceRefMut::Type(x) => x.element_id_ref_mut(),
            NamespaceRefMut::Package(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.alias_ids_ref_mut(),
            NamespaceRefMut::Type(x) => x.alias_ids_ref_mut(),
            NamespaceRefMut::Package(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            NamespaceRefMut::Type(x) => x.declared_short_name_ref_mut(),
            NamespaceRefMut::Package(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.declared_name_ref_mut(),
            NamespaceRefMut::Type(x) => x.declared_name_ref_mut(),
            NamespaceRefMut::Package(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            NamespaceRefMut::Type(x) => x.is_implied_included_ref_mut(),
            NamespaceRefMut::Package(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for NamespaceRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceRefMut::Itself(x) => x.owned_relationship_ref(),
            NamespaceRefMut::Type(x) => x.owned_relationship_ref(),
            NamespaceRefMut::Package(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceRefMut::Itself(x) => x.owning_relationship_ref(),
            NamespaceRefMut::Type(x) => x.owning_relationship_ref(),
            NamespaceRefMut::Package(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NamespaceRefMut::Itself(x) => x.element_id_ref(),
            NamespaceRefMut::Type(x) => x.element_id_ref(),
            NamespaceRefMut::Package(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.alias_ids_ref(),
            NamespaceRefMut::Type(x) => x.alias_ids_ref(),
            NamespaceRefMut::Package(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.declared_short_name_ref(),
            NamespaceRefMut::Type(x) => x.declared_short_name_ref(),
            NamespaceRefMut::Package(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceRefMut::Itself(x) => x.declared_name_ref(),
            NamespaceRefMut::Type(x) => x.declared_name_ref(),
            NamespaceRefMut::Package(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NamespaceRefMut::Itself(x) => x.is_implied_included_ref(),
            NamespaceRefMut::Type(x) => x.is_implied_included_ref(),
            NamespaceRefMut::Package(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for NamespaceRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceRef::Itself(x) => x.owned_relationship_ref(),
            NamespaceRef::Type(x) => x.owned_relationship_ref(),
            NamespaceRef::Package(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceRef::Itself(x) => x.owning_relationship_ref(),
            NamespaceRef::Type(x) => x.owning_relationship_ref(),
            NamespaceRef::Package(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NamespaceRef::Itself(x) => x.element_id_ref(),
            NamespaceRef::Type(x) => x.element_id_ref(),
            NamespaceRef::Package(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NamespaceRef::Itself(x) => x.alias_ids_ref(),
            NamespaceRef::Type(x) => x.alias_ids_ref(),
            NamespaceRef::Package(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceRef::Itself(x) => x.declared_short_name_ref(),
            NamespaceRef::Type(x) => x.declared_short_name_ref(),
            NamespaceRef::Package(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceRef::Itself(x) => x.declared_name_ref(),
            NamespaceRef::Type(x) => x.declared_name_ref(),
            NamespaceRef::Package(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NamespaceRef::Itself(x) => x.is_implied_included_ref(),
            NamespaceRef::Type(x) => x.is_implied_included_ref(),
            NamespaceRef::Package(x) => x.is_implied_included_ref(),
        }
    }
}
impl NamespaceUpcast for Namespace {
    fn into_namespace(self) -> Namespace {
        self
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for NamespaceRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        self
    }
}
impl<'a> NamespaceUpcastRef<'a> for NamespaceRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        self
    }
}
impl ElementUpcast for Namespace {
    fn into_element(self) -> Element {
        Element::Namespace(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for NamespaceRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ElementRefMut::Namespace(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for NamespaceRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ElementRef::Namespace(self).as_element_ref()
    }
}
pub trait NamespaceDowncast {
    fn try_into_type_(self) -> Result<Type, String>;
    fn try_into_package(self) -> Result<Package, String>;
}
pub trait NamespaceDowncastRefMut<'a> {
    fn try_as_type_ref_mut(self) -> Result<TypeRefMut<'a>, String>;
    fn try_as_package_ref_mut(self) -> Result<PackageRefMut<'a>, String>;
}
pub trait NamespaceDowncastRef<'a> {
    fn try_as_type_ref(self) -> Result<TypeRef<'a>, String>;
    fn try_as_package_ref(self) -> Result<PackageRef<'a>, String>;
}
impl NamespaceDowncast for Namespace {
    fn try_into_type_(self) -> Result<Type, String> {
        match self {
            Namespace::Type(e) => Ok(e),
            _ => Err("Not a Type".into()),
        }
    }
    fn try_into_package(self) -> Result<Package, String> {
        match self {
            Namespace::Package(e) => Ok(e),
            _ => Err("Not a Package".into()),
        }
    }
}
impl<'a> NamespaceDowncastRefMut<'a> for NamespaceRefMut<'a> {
    fn try_as_type_ref_mut(self) -> Result<TypeRefMut<'a>, String> {
        match self {
            NamespaceRefMut::Type(e) => Ok(e),
            _ => Err("Not a Type".into()),
        }
    }
    fn try_as_package_ref_mut(self) -> Result<PackageRefMut<'a>, String> {
        match self {
            NamespaceRefMut::Package(e) => Ok(e),
            _ => Err("Not a Package".into()),
        }
    }
}
impl<'a> NamespaceDowncastRef<'a> for NamespaceRef<'a> {
    fn try_as_type_ref(self) -> Result<TypeRef<'a>, String> {
        match self {
            NamespaceRef::Type(e) => Ok(e),
            _ => Err("Not a Type".into()),
        }
    }
    fn try_as_package_ref(self) -> Result<PackageRef<'a>, String> {
        match self {
            NamespaceRef::Package(e) => Ok(e),
            _ => Err("Not a Package".into()),
        }
    }
}
pub trait NamespaceMethodsDescendants
where
    Self: DescendantOf<Namespace>,
    Self::Via: NamespaceMethods,
    Self: Sized,
{}
pub trait NamespaceMethods: Sized {}
impl<T: NamespaceMethodsDescendants> NamespaceMethods for T
where
    T::Via: NamespaceMethods,
{}
impl DescendantOf<Element> for Namespace {
    type Via = Element;
    fn into_via(self) -> Self::Via {
        self.into_element()
    }
}
impl ElementMethodsDescendants for Namespace {}
pub trait NamespaceRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<NamespaceRefMut<'a>>,
    Self::Via: NamespaceRefMutMethods,
    Self: Sized,
{}
pub trait NamespaceRefMutMethods: Sized {}
impl<'a, T: NamespaceRefMutMethodsDescendants<'a>> NamespaceRefMutMethods for T
where
    T::Via: NamespaceRefMutMethods,
{}
impl<'a> DescendantOf<ElementRefMut<'a>> for NamespaceRefMut<'a> {
    type Via = ElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for NamespaceRefMut<'a> {}
pub trait NamespaceRefMethodsDescendants<'a>
where
    Self: DescendantOf<NamespaceRef<'a>>,
    Self::Via: NamespaceRefMethods,
    Self: Sized,
{
    fn names_of_impl(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<String> {
        self.into_via().names_of(element)
    }
    fn visibility_of_impl(
        self,
        mem: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.into_via().visibility_of(mem)
    }
    fn visible_memberships_impl(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
        is_recursive: bool,
        include_all: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().visible_memberships(excluded, is_recursive, include_all)
    }
    fn imported_memberships_impl(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().imported_memberships(excluded)
    }
    fn memberships_of_visibility_impl(
        self,
        visibility: Option<
            std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>,
        >,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().memberships_of_visibility(visibility, excluded)
    }
    fn resolve_impl(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().resolve(qualified_name)
    }
    fn resolve_global_impl(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().resolve_global(qualified_name)
    }
    fn resolve_local_impl(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().resolve_local(name)
    }
    fn resolve_visible_impl(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().resolve_visible(name)
    }
    fn qualification_of_impl(self, qualified_name: String) -> Option<String> {
        self.into_via().qualification_of(qualified_name)
    }
    fn unqualified_name_of_impl(self, qualified_name: String) -> String {
        self.into_via().unqualified_name_of(qualified_name)
    }
}
pub trait NamespaceRefMethods: Sized {
    fn names_of(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<String>;
    fn visibility_of(
        self,
        mem: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
    fn visible_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
        is_recursive: bool,
        include_all: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn memberships_of_visibility(
        self,
        visibility: Option<
            std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>,
        >,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn resolve(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn resolve_global(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn resolve_local(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn resolve_visible(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn qualification_of(self, qualified_name: String) -> Option<String>;
    fn unqualified_name_of(self, qualified_name: String) -> String;
}
impl<'a, T: NamespaceRefMethodsDescendants<'a>> NamespaceRefMethods for T
where
    T::Via: NamespaceRefMethods,
{
    fn names_of(
        self,
        element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<String> {
        NamespaceRefMethodsDescendants::names_of_impl(self, element)
    }
    fn visibility_of(
        self,
        mem: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        NamespaceRefMethodsDescendants::visibility_of_impl(self, mem)
    }
    fn visible_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
        is_recursive: bool,
        include_all: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::visible_memberships_impl(
            self,
            excluded,
            is_recursive,
            include_all,
        )
    }
    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::imported_memberships_impl(self, excluded)
    }
    fn memberships_of_visibility(
        self,
        visibility: Option<
            std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>,
        >,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::memberships_of_visibility_impl(
            self,
            visibility,
            excluded,
        )
    }
    fn resolve(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::resolve_impl(self, qualified_name)
    }
    fn resolve_global(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::resolve_global_impl(self, qualified_name)
    }
    fn resolve_local(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::resolve_local_impl(self, name)
    }
    fn resolve_visible(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        NamespaceRefMethodsDescendants::resolve_visible_impl(self, name)
    }
    fn qualification_of(self, qualified_name: String) -> Option<String> {
        NamespaceRefMethodsDescendants::qualification_of_impl(self, qualified_name)
    }
    fn unqualified_name_of(self, qualified_name: String) -> String {
        NamespaceRefMethodsDescendants::unqualified_name_of_impl(self, qualified_name)
    }
}
impl<'a> DescendantOf<ElementRef<'a>> for NamespaceRef<'a> {
    type Via = ElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for NamespaceRef<'a> {}

