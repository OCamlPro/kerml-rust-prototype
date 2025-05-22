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
use super::classifier::{Classifier, ClassifierRefMut, ClassifierRef};
use super::feature::{Feature, FeatureRefMut, FeatureRef};
pub struct TypeInner {
    pub(super) sup_namespace: NamespaceInner,
    pub(super) is_abstract: bool,
    pub(super) is_sufficient: bool,
}
pub trait TypeStruct
where
    Self: TypeStructRefMut,
    Self: TypeStructRef,
    Self: NamespaceStruct,
{
    fn is_abstract(self) -> bool;
    fn is_sufficient(self) -> bool;
}
pub trait TypeStructRefMut
where
    Self: TypeStructRef,
    Self: NamespaceStructRefMut,
{
    fn is_abstract_ref_mut(&mut self) -> &mut bool;
    fn is_sufficient_ref_mut(&mut self) -> &mut bool;
}
pub trait TypeStructRef
where
    Self: NamespaceStructRef,
{
    fn is_abstract_ref(&self) -> &bool;
    fn is_sufficient_ref(&self) -> &bool;
}
pub trait TypeUpcast: TypeStruct {
    fn into_type_(self) -> Type;
}
pub trait TypeUpcastRefMut<'a>: TypeStructRefMut {
    fn as_type_ref_mut(self) -> TypeRefMut<'a>;
}
pub trait TypeUpcastRef<'a>: TypeStructRef {
    fn as_type_ref(self) -> TypeRef<'a>;
}
impl TypeStruct for TypeInner {
    fn is_abstract(self) -> bool {
        self.is_abstract
    }
    fn is_sufficient(self) -> bool {
        self.is_sufficient
    }
}
impl TypeStructRefMut for TypeInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        &mut self.is_abstract
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        &mut self.is_sufficient
    }
}
impl TypeStructRef for TypeInner {
    fn is_abstract_ref(&self) -> &bool {
        &self.is_abstract
    }
    fn is_sufficient_ref(&self) -> &bool {
        &self.is_sufficient
    }
}
impl NamespaceStruct for TypeInner {}
impl NamespaceStructRefMut for TypeInner {}
impl NamespaceStructRef for TypeInner {}
impl ElementStruct for TypeInner {
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
impl ElementStructRefMut for TypeInner {
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
impl ElementStructRef for TypeInner {
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
pub enum Type {
    Itself(TypeInner),
    Classifier(Classifier),
    Feature(Feature),
}
pub enum TypeRefMut<'a> {
    Itself(&'a mut TypeInner),
    Classifier(ClassifierRefMut<'a>),
    Feature(FeatureRefMut<'a>),
}
pub enum TypeRef<'a> {
    Itself(&'a TypeInner),
    Classifier(ClassifierRef<'a>),
    Feature(FeatureRef<'a>),
}
impl Type {
    pub fn as_ref(&self) -> TypeRef {
        match self {
            Type::Itself(inner) => TypeRef::Itself(&inner),
            Type::Classifier(inner) => TypeRef::Classifier(inner.as_ref()),
            Type::Feature(inner) => TypeRef::Feature(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> TypeRefMut {
        match self {
            Type::Itself(inner) => TypeRefMut::Itself(inner),
            Type::Classifier(inner) => TypeRefMut::Classifier(inner.as_ref_mut()),
            Type::Feature(inner) => TypeRefMut::Feature(inner.as_ref_mut()),
        }
    }
}
impl<'a> TypeRefMut<'a> {
    pub fn as_ref(self) -> TypeRef<'a> {
        match self {
            TypeRefMut::Itself(inner) => TypeRef::Itself(inner),
            TypeRefMut::Classifier(inner) => TypeRef::Classifier(inner.as_ref()),
            TypeRefMut::Feature(inner) => TypeRef::Feature(inner.as_ref()),
        }
    }
}
impl TypeStruct for Type {
    fn is_abstract(self) -> bool {
        match self {
            Type::Itself(x) => x.is_abstract(),
            Type::Classifier(x) => x.is_abstract(),
            Type::Feature(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Type::Itself(x) => x.is_sufficient(),
            Type::Classifier(x) => x.is_sufficient(),
            Type::Feature(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Type {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Type::Itself(x) => x.is_abstract_ref_mut(),
            Type::Classifier(x) => x.is_abstract_ref_mut(),
            Type::Feature(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Type::Itself(x) => x.is_sufficient_ref_mut(),
            Type::Classifier(x) => x.is_sufficient_ref_mut(),
            Type::Feature(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Type {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Type::Itself(x) => x.is_abstract_ref(),
            Type::Classifier(x) => x.is_abstract_ref(),
            Type::Feature(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Type::Itself(x) => x.is_sufficient_ref(),
            Type::Classifier(x) => x.is_sufficient_ref(),
            Type::Feature(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for TypeRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeRefMut::Itself(x) => x.is_abstract_ref_mut(),
            TypeRefMut::Classifier(x) => x.is_abstract_ref_mut(),
            TypeRefMut::Feature(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            TypeRefMut::Classifier(x) => x.is_sufficient_ref_mut(),
            TypeRefMut::Feature(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for TypeRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            TypeRefMut::Itself(x) => x.is_abstract_ref(),
            TypeRefMut::Classifier(x) => x.is_abstract_ref(),
            TypeRefMut::Feature(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            TypeRefMut::Itself(x) => x.is_sufficient_ref(),
            TypeRefMut::Classifier(x) => x.is_sufficient_ref(),
            TypeRefMut::Feature(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for TypeRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            TypeRef::Itself(x) => x.is_abstract_ref(),
            TypeRef::Classifier(x) => x.is_abstract_ref(),
            TypeRef::Feature(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            TypeRef::Itself(x) => x.is_sufficient_ref(),
            TypeRef::Classifier(x) => x.is_sufficient_ref(),
            TypeRef::Feature(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Type {}
impl NamespaceStructRefMut for Type {}
impl NamespaceStructRef for Type {}
impl<'a> NamespaceStructRefMut for TypeRefMut<'a> {}
impl<'a> NamespaceStructRef for TypeRefMut<'a> {}
impl<'a> NamespaceStructRef for TypeRef<'a> {}
impl ElementStruct for Type {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Type::Itself(x) => x.owned_relationship(),
            Type::Classifier(x) => x.owned_relationship(),
            Type::Feature(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Type::Itself(x) => x.owning_relationship(),
            Type::Classifier(x) => x.owning_relationship(),
            Type::Feature(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Type::Itself(x) => x.element_id(),
            Type::Classifier(x) => x.element_id(),
            Type::Feature(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Type::Itself(x) => x.alias_ids(),
            Type::Classifier(x) => x.alias_ids(),
            Type::Feature(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Type::Itself(x) => x.declared_short_name(),
            Type::Classifier(x) => x.declared_short_name(),
            Type::Feature(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Type::Itself(x) => x.declared_name(),
            Type::Classifier(x) => x.declared_name(),
            Type::Feature(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Type::Itself(x) => x.is_implied_included(),
            Type::Classifier(x) => x.is_implied_included(),
            Type::Feature(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Type {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Type::Itself(x) => x.owned_relationship_ref_mut(),
            Type::Classifier(x) => x.owned_relationship_ref_mut(),
            Type::Feature(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Type::Itself(x) => x.owning_relationship_ref_mut(),
            Type::Classifier(x) => x.owning_relationship_ref_mut(),
            Type::Feature(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Type::Itself(x) => x.element_id_ref_mut(),
            Type::Classifier(x) => x.element_id_ref_mut(),
            Type::Feature(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Type::Itself(x) => x.alias_ids_ref_mut(),
            Type::Classifier(x) => x.alias_ids_ref_mut(),
            Type::Feature(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Type::Itself(x) => x.declared_short_name_ref_mut(),
            Type::Classifier(x) => x.declared_short_name_ref_mut(),
            Type::Feature(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Type::Itself(x) => x.declared_name_ref_mut(),
            Type::Classifier(x) => x.declared_name_ref_mut(),
            Type::Feature(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Type::Itself(x) => x.is_implied_included_ref_mut(),
            Type::Classifier(x) => x.is_implied_included_ref_mut(),
            Type::Feature(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Type {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Type::Itself(x) => x.owned_relationship_ref(),
            Type::Classifier(x) => x.owned_relationship_ref(),
            Type::Feature(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Type::Itself(x) => x.owning_relationship_ref(),
            Type::Classifier(x) => x.owning_relationship_ref(),
            Type::Feature(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Type::Itself(x) => x.element_id_ref(),
            Type::Classifier(x) => x.element_id_ref(),
            Type::Feature(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Type::Itself(x) => x.alias_ids_ref(),
            Type::Classifier(x) => x.alias_ids_ref(),
            Type::Feature(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Type::Itself(x) => x.declared_short_name_ref(),
            Type::Classifier(x) => x.declared_short_name_ref(),
            Type::Feature(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Type::Itself(x) => x.declared_name_ref(),
            Type::Classifier(x) => x.declared_name_ref(),
            Type::Feature(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Type::Itself(x) => x.is_implied_included_ref(),
            Type::Classifier(x) => x.is_implied_included_ref(),
            Type::Feature(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for TypeRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            TypeRefMut::Classifier(x) => x.owned_relationship_ref_mut(),
            TypeRefMut::Feature(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            TypeRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            TypeRefMut::Classifier(x) => x.owning_relationship_ref_mut(),
            TypeRefMut::Feature(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            TypeRefMut::Itself(x) => x.element_id_ref_mut(),
            TypeRefMut::Classifier(x) => x.element_id_ref_mut(),
            TypeRefMut::Feature(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            TypeRefMut::Itself(x) => x.alias_ids_ref_mut(),
            TypeRefMut::Classifier(x) => x.alias_ids_ref_mut(),
            TypeRefMut::Feature(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            TypeRefMut::Classifier(x) => x.declared_short_name_ref_mut(),
            TypeRefMut::Feature(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeRefMut::Itself(x) => x.declared_name_ref_mut(),
            TypeRefMut::Classifier(x) => x.declared_name_ref_mut(),
            TypeRefMut::Feature(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            TypeRefMut::Classifier(x) => x.is_implied_included_ref_mut(),
            TypeRefMut::Feature(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for TypeRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeRefMut::Itself(x) => x.owned_relationship_ref(),
            TypeRefMut::Classifier(x) => x.owned_relationship_ref(),
            TypeRefMut::Feature(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeRefMut::Itself(x) => x.owning_relationship_ref(),
            TypeRefMut::Classifier(x) => x.owning_relationship_ref(),
            TypeRefMut::Feature(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TypeRefMut::Itself(x) => x.element_id_ref(),
            TypeRefMut::Classifier(x) => x.element_id_ref(),
            TypeRefMut::Feature(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TypeRefMut::Itself(x) => x.alias_ids_ref(),
            TypeRefMut::Classifier(x) => x.alias_ids_ref(),
            TypeRefMut::Feature(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TypeRefMut::Itself(x) => x.declared_short_name_ref(),
            TypeRefMut::Classifier(x) => x.declared_short_name_ref(),
            TypeRefMut::Feature(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TypeRefMut::Itself(x) => x.declared_name_ref(),
            TypeRefMut::Classifier(x) => x.declared_name_ref(),
            TypeRefMut::Feature(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TypeRefMut::Itself(x) => x.is_implied_included_ref(),
            TypeRefMut::Classifier(x) => x.is_implied_included_ref(),
            TypeRefMut::Feature(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for TypeRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeRef::Itself(x) => x.owned_relationship_ref(),
            TypeRef::Classifier(x) => x.owned_relationship_ref(),
            TypeRef::Feature(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeRef::Itself(x) => x.owning_relationship_ref(),
            TypeRef::Classifier(x) => x.owning_relationship_ref(),
            TypeRef::Feature(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TypeRef::Itself(x) => x.element_id_ref(),
            TypeRef::Classifier(x) => x.element_id_ref(),
            TypeRef::Feature(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TypeRef::Itself(x) => x.alias_ids_ref(),
            TypeRef::Classifier(x) => x.alias_ids_ref(),
            TypeRef::Feature(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TypeRef::Itself(x) => x.declared_short_name_ref(),
            TypeRef::Classifier(x) => x.declared_short_name_ref(),
            TypeRef::Feature(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TypeRef::Itself(x) => x.declared_name_ref(),
            TypeRef::Classifier(x) => x.declared_name_ref(),
            TypeRef::Feature(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TypeRef::Itself(x) => x.is_implied_included_ref(),
            TypeRef::Classifier(x) => x.is_implied_included_ref(),
            TypeRef::Feature(x) => x.is_implied_included_ref(),
        }
    }
}
impl TypeUpcast for Type {
    fn into_type_(self) -> Type {
        self
    }
}
impl<'a> TypeUpcastRefMut<'a> for TypeRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        self
    }
}
impl<'a> TypeUpcastRef<'a> for TypeRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        self
    }
}
impl NamespaceUpcast for Type {
    fn into_namespace(self) -> Namespace {
        Namespace::Type(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for TypeRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        NamespaceRefMut::Type(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for TypeRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        NamespaceRef::Type(self).as_namespace_ref()
    }
}
impl ElementUpcast for Type {
    fn into_element(self) -> Element {
        Namespace::Type(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for TypeRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        NamespaceRefMut::Type(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for TypeRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        NamespaceRef::Type(self).as_element_ref()
    }
}
pub trait TypeDowncast {
    fn try_into_classifier(self) -> Result<Classifier, String>;
    fn try_into_feature(self) -> Result<Feature, String>;
}
pub trait TypeDowncastRefMut<'a> {
    fn try_as_classifier_ref_mut(self) -> Result<ClassifierRefMut<'a>, String>;
    fn try_as_feature_ref_mut(self) -> Result<FeatureRefMut<'a>, String>;
}
pub trait TypeDowncastRef<'a> {
    fn try_as_classifier_ref(self) -> Result<ClassifierRef<'a>, String>;
    fn try_as_feature_ref(self) -> Result<FeatureRef<'a>, String>;
}
impl TypeDowncast for Type {
    fn try_into_classifier(self) -> Result<Classifier, String> {
        match self {
            Type::Classifier(e) => Ok(e),
            _ => Err("Not a Classifier".into()),
        }
    }
    fn try_into_feature(self) -> Result<Feature, String> {
        match self {
            Type::Feature(e) => Ok(e),
            _ => Err("Not a Feature".into()),
        }
    }
}
impl<'a> TypeDowncastRefMut<'a> for TypeRefMut<'a> {
    fn try_as_classifier_ref_mut(self) -> Result<ClassifierRefMut<'a>, String> {
        match self {
            TypeRefMut::Classifier(e) => Ok(e),
            _ => Err("Not a Classifier".into()),
        }
    }
    fn try_as_feature_ref_mut(self) -> Result<FeatureRefMut<'a>, String> {
        match self {
            TypeRefMut::Feature(e) => Ok(e),
            _ => Err("Not a Feature".into()),
        }
    }
}
impl<'a> TypeDowncastRef<'a> for TypeRef<'a> {
    fn try_as_classifier_ref(self) -> Result<ClassifierRef<'a>, String> {
        match self {
            TypeRef::Classifier(e) => Ok(e),
            _ => Err("Not a Classifier".into()),
        }
    }
    fn try_as_feature_ref(self) -> Result<FeatureRef<'a>, String> {
        match self {
            TypeRef::Feature(e) => Ok(e),
            _ => Err("Not a Feature".into()),
        }
    }
}
pub trait TypeMethodsDescendants
where
    Self: DescendantOf<Type>,
    Self::Via: TypeMethods,
    Self: Sized,
{}
pub trait TypeMethods: Sized {}
impl<T: TypeMethodsDescendants> TypeMethods for T
where
    T::Via: TypeMethods,
{}
impl DescendantOf<Namespace> for Type {
    type Via = Namespace;
    fn into_via(self) -> Self::Via {
        self.into_namespace()
    }
}
impl NamespaceMethodsDescendants for Type {}
impl DescendantOf<Element> for Type {
    type Via = Namespace;
    fn into_via(self) -> Self::Via {
        self.into_namespace()
    }
}
impl ElementMethodsDescendants for Type {}
pub trait TypeRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<TypeRefMut<'a>>,
    Self::Via: TypeRefMutMethods,
    Self: Sized,
{}
pub trait TypeRefMutMethods: Sized {}
impl<'a, T: TypeRefMutMethodsDescendants<'a>> TypeRefMutMethods for T
where
    T::Via: TypeRefMutMethods,
{}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for TypeRefMut<'a> {
    type Via = NamespaceRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for TypeRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for TypeRefMut<'a> {
    type Via = NamespaceRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for TypeRefMut<'a> {}
pub trait TypeRefMethodsDescendants<'a>
where
    Self: DescendantOf<TypeRef<'a>>,
    Self::Via: TypeRefMethods,
    Self: Sized,
{
    fn inherited_memberships_impl(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via()
            .inherited_memberships(excluded_namespaces, excluded_types, exclude_implied)
    }
    fn inheritable_memberships_impl(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via()
            .inheritable_memberships(
                excluded_namespaces,
                excluded_types,
                exclude_implied,
            )
    }
    fn non_private_memberships_impl(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via()
            .non_private_memberships(
                excluded_namespaces,
                excluded_types,
                exclude_implied,
            )
    }
    fn remove_redefined_features_impl(
        self,
        memberships: Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().remove_redefined_features(memberships)
    }
    fn all_redefined_features_of_impl(
        self,
        membership: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().all_redefined_features_of(membership)
    }
    fn direction_of_impl(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.into_via().direction_of(feature)
    }
    fn direction_of_excluding_impl(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.into_via().direction_of_excluding(feature, excluded)
    }
    fn supertypes_impl(
        self,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        self.into_via().supertypes(exclude_implied)
    }
    fn all_supertypes_impl(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        self.into_via().all_supertypes()
    }
    fn specializes_impl(
        self,
        supertype: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool {
        self.into_via().specializes(supertype)
    }
    fn specializes_from_library_impl(self, library_type_name: String) -> bool {
        self.into_via().specializes_from_library(library_type_name)
    }
    fn multiplicities_impl(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::multiplicity::Multiplicity>>> {
        self.into_via().multiplicities()
    }
}
pub trait TypeRefMethods: Sized {
    fn inherited_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn inheritable_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn non_private_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn remove_redefined_features(
        self,
        memberships: Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
    fn all_redefined_features_of(
        self,
        membership: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
    fn direction_of(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
    fn direction_of_excluding(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
    fn supertypes(
        self,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>;
    fn all_supertypes(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>;
    fn specializes(
        self,
        supertype: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool;
    fn specializes_from_library(self, library_type_name: String) -> bool;
    fn multiplicities(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::multiplicity::Multiplicity>>>;
}
impl<'a, T: TypeRefMethodsDescendants<'a>> TypeRefMethods for T
where
    T::Via: TypeRefMethods,
{
    fn inherited_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        TypeRefMethodsDescendants::inherited_memberships_impl(
            self,
            excluded_namespaces,
            excluded_types,
            exclude_implied,
        )
    }
    fn inheritable_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        TypeRefMethodsDescendants::inheritable_memberships_impl(
            self,
            excluded_namespaces,
            excluded_types,
            exclude_implied,
        )
    }
    fn non_private_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        TypeRefMethodsDescendants::non_private_memberships_impl(
            self,
            excluded_namespaces,
            excluded_types,
            exclude_implied,
        )
    }
    fn remove_redefined_features(
        self,
        memberships: Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        TypeRefMethodsDescendants::remove_redefined_features_impl(self, memberships)
    }
    fn all_redefined_features_of(
        self,
        membership: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        TypeRefMethodsDescendants::all_redefined_features_of_impl(self, membership)
    }
    fn direction_of(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        TypeRefMethodsDescendants::direction_of_impl(self, feature)
    }
    fn direction_of_excluding(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        TypeRefMethodsDescendants::direction_of_excluding_impl(self, feature, excluded)
    }
    fn supertypes(
        self,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        TypeRefMethodsDescendants::supertypes_impl(self, exclude_implied)
    }
    fn all_supertypes(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        TypeRefMethodsDescendants::all_supertypes_impl(self)
    }
    fn specializes(
        self,
        supertype: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool {
        TypeRefMethodsDescendants::specializes_impl(self, supertype)
    }
    fn specializes_from_library(self, library_type_name: String) -> bool {
        TypeRefMethodsDescendants::specializes_from_library_impl(self, library_type_name)
    }
    fn multiplicities(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::multiplicity::Multiplicity>>> {
        TypeRefMethodsDescendants::multiplicities_impl(self)
    }
}
impl<'a> DescendantOf<NamespaceRef<'a>> for TypeRef<'a> {
    type Via = NamespaceRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for TypeRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for TypeRef<'a> {
    type Via = NamespaceRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_namespace_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for TypeRef<'a> {}

