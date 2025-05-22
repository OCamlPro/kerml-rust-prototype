#![allow(unused)]
use super::utils::DescendantOf;
use super::structure::{
    Structure, StructureRefMut, StructureRef, StructureStruct, StructureStructRefMut,
    StructureStructRef, StructureInner, StructureUpcast, StructureUpcastRefMut,
    StructureUpcastRef, StructureMethodsDescendants, StructureRefMutMethodsDescendants,
    StructureRefMethodsDescendants,
};
use super::class::{
    Class, ClassRefMut, ClassRef, ClassStruct, ClassStructRefMut, ClassStructRef,
    ClassInner, ClassUpcast, ClassUpcastRefMut, ClassUpcastRef, ClassMethodsDescendants,
    ClassRefMutMethodsDescendants, ClassRefMethodsDescendants,
};
use super::classifier::{
    Classifier, ClassifierRefMut, ClassifierRef, ClassifierStruct,
    ClassifierStructRefMut, ClassifierStructRef, ClassifierInner, ClassifierUpcast,
    ClassifierUpcastRefMut, ClassifierUpcastRef, ClassifierMethodsDescendants,
    ClassifierRefMutMethodsDescendants, ClassifierRefMethodsDescendants,
};
use super::type_::{
    Type, TypeRefMut, TypeRef, TypeStruct, TypeStructRefMut, TypeStructRef, TypeInner,
    TypeUpcast, TypeUpcastRefMut, TypeUpcastRef, TypeMethodsDescendants,
    TypeRefMutMethodsDescendants, TypeRefMethodsDescendants,
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
pub struct MetaclassInner {
    pub(super) sup_structure: StructureInner,
}
pub trait MetaclassStruct
where
    Self: MetaclassStructRefMut,
    Self: MetaclassStructRef,
    Self: StructureStruct,
{}
pub trait MetaclassStructRefMut
where
    Self: MetaclassStructRef,
    Self: StructureStructRefMut,
{}
pub trait MetaclassStructRef
where
    Self: StructureStructRef,
{}
pub trait MetaclassUpcast: MetaclassStruct {
    fn into_metaclass(self) -> Metaclass;
}
pub trait MetaclassUpcastRefMut<'a>: MetaclassStructRefMut {
    fn as_metaclass_ref_mut(self) -> MetaclassRefMut<'a>;
}
pub trait MetaclassUpcastRef<'a>: MetaclassStructRef {
    fn as_metaclass_ref(self) -> MetaclassRef<'a>;
}
impl MetaclassStruct for MetaclassInner {}
impl MetaclassStructRefMut for MetaclassInner {}
impl MetaclassStructRef for MetaclassInner {}
impl StructureStruct for MetaclassInner {}
impl StructureStructRefMut for MetaclassInner {}
impl StructureStructRef for MetaclassInner {}
impl ClassStruct for MetaclassInner {}
impl ClassStructRefMut for MetaclassInner {}
impl ClassStructRef for MetaclassInner {}
impl ClassifierStruct for MetaclassInner {}
impl ClassifierStructRefMut for MetaclassInner {}
impl ClassifierStructRef for MetaclassInner {}
impl TypeStruct for MetaclassInner {
    fn is_abstract(self) -> bool {
        self.sup_structure.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_structure.is_sufficient()
    }
}
impl TypeStructRefMut for MetaclassInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_structure.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_structure.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for MetaclassInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_structure.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_structure.is_sufficient_ref()
    }
}
impl NamespaceStruct for MetaclassInner {}
impl NamespaceStructRefMut for MetaclassInner {}
impl NamespaceStructRef for MetaclassInner {}
impl ElementStruct for MetaclassInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_structure.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_structure.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_structure.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_structure.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_structure.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_structure.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_structure.is_implied_included()
    }
}
impl ElementStructRefMut for MetaclassInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_structure.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_structure.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_structure.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_structure.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_structure.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_structure.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_structure.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for MetaclassInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_structure.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_structure.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_structure.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_structure.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_structure.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_structure.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_structure.is_implied_included_ref()
    }
}
pub enum Metaclass {
    Itself(MetaclassInner),
}
pub enum MetaclassRefMut<'a> {
    Itself(&'a mut MetaclassInner),
}
pub enum MetaclassRef<'a> {
    Itself(&'a MetaclassInner),
}
impl Metaclass {
    pub fn as_ref(&self) -> MetaclassRef {
        match self {
            Metaclass::Itself(inner) => MetaclassRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> MetaclassRefMut {
        match self {
            Metaclass::Itself(inner) => MetaclassRefMut::Itself(inner),
        }
    }
}
impl<'a> MetaclassRefMut<'a> {
    pub fn as_ref(self) -> MetaclassRef<'a> {
        match self {
            MetaclassRefMut::Itself(inner) => MetaclassRef::Itself(inner),
        }
    }
}
impl MetaclassStruct for Metaclass {}
impl MetaclassStructRefMut for Metaclass {}
impl MetaclassStructRef for Metaclass {}
impl<'a> MetaclassStructRefMut for MetaclassRefMut<'a> {}
impl<'a> MetaclassStructRef for MetaclassRefMut<'a> {}
impl<'a> MetaclassStructRef for MetaclassRef<'a> {}
impl StructureStruct for Metaclass {}
impl StructureStructRefMut for Metaclass {}
impl StructureStructRef for Metaclass {}
impl<'a> StructureStructRefMut for MetaclassRefMut<'a> {}
impl<'a> StructureStructRef for MetaclassRefMut<'a> {}
impl<'a> StructureStructRef for MetaclassRef<'a> {}
impl ClassStruct for Metaclass {}
impl ClassStructRefMut for Metaclass {}
impl ClassStructRef for Metaclass {}
impl<'a> ClassStructRefMut for MetaclassRefMut<'a> {}
impl<'a> ClassStructRef for MetaclassRefMut<'a> {}
impl<'a> ClassStructRef for MetaclassRef<'a> {}
impl ClassifierStruct for Metaclass {}
impl ClassifierStructRefMut for Metaclass {}
impl ClassifierStructRef for Metaclass {}
impl<'a> ClassifierStructRefMut for MetaclassRefMut<'a> {}
impl<'a> ClassifierStructRef for MetaclassRefMut<'a> {}
impl<'a> ClassifierStructRef for MetaclassRef<'a> {}
impl TypeStruct for Metaclass {
    fn is_abstract(self) -> bool {
        match self {
            Metaclass::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Metaclass::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Metaclass {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Metaclass::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Metaclass::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Metaclass {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Metaclass::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Metaclass::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for MetaclassRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for MetaclassRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for MetaclassRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetaclassRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetaclassRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Metaclass {}
impl NamespaceStructRefMut for Metaclass {}
impl NamespaceStructRef for Metaclass {}
impl<'a> NamespaceStructRefMut for MetaclassRefMut<'a> {}
impl<'a> NamespaceStructRef for MetaclassRefMut<'a> {}
impl<'a> NamespaceStructRef for MetaclassRef<'a> {}
impl ElementStruct for Metaclass {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Metaclass::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Metaclass::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Metaclass::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Metaclass::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Metaclass::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Metaclass {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Metaclass::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Metaclass::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Metaclass::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Metaclass::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Metaclass::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Metaclass {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Metaclass::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Metaclass::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Metaclass::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Metaclass::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Metaclass::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Metaclass::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MetaclassRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetaclassRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MetaclassRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MetaclassRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MetaclassRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetaclassRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetaclassRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetaclassRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetaclassRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetaclassRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MetaclassRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetaclassRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetaclassRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetaclassRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetaclassRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetaclassRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetaclassRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetaclassRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl MetaclassUpcast for Metaclass {
    fn into_metaclass(self) -> Metaclass {
        self
    }
}
impl<'a> MetaclassUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_metaclass_ref_mut(self) -> MetaclassRefMut<'a> {
        self
    }
}
impl<'a> MetaclassUpcastRef<'a> for MetaclassRef<'a> {
    fn as_metaclass_ref(self) -> MetaclassRef<'a> {
        self
    }
}
impl StructureUpcast for Metaclass {
    fn into_structure(self) -> Structure {
        Structure::Metaclass(self).into_structure()
    }
}
impl<'a> StructureUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_structure_ref_mut(self) -> StructureRefMut<'a> {
        StructureRefMut::Metaclass(self).as_structure_ref_mut()
    }
}
impl<'a> StructureUpcastRef<'a> for MetaclassRef<'a> {
    fn as_structure_ref(self) -> StructureRef<'a> {
        StructureRef::Metaclass(self).as_structure_ref()
    }
}
impl ClassUpcast for Metaclass {
    fn into_class(self) -> Class {
        Structure::Metaclass(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        StructureRefMut::Metaclass(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for MetaclassRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        StructureRef::Metaclass(self).as_class_ref()
    }
}
impl ClassifierUpcast for Metaclass {
    fn into_classifier(self) -> Classifier {
        Structure::Metaclass(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        StructureRefMut::Metaclass(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for MetaclassRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        StructureRef::Metaclass(self).as_classifier_ref()
    }
}
impl TypeUpcast for Metaclass {
    fn into_type_(self) -> Type {
        Structure::Metaclass(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        StructureRefMut::Metaclass(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for MetaclassRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        StructureRef::Metaclass(self).as_type_ref()
    }
}
impl NamespaceUpcast for Metaclass {
    fn into_namespace(self) -> Namespace {
        Structure::Metaclass(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        StructureRefMut::Metaclass(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for MetaclassRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        StructureRef::Metaclass(self).as_namespace_ref()
    }
}
impl ElementUpcast for Metaclass {
    fn into_element(self) -> Element {
        Structure::Metaclass(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MetaclassRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        StructureRefMut::Metaclass(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MetaclassRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        StructureRef::Metaclass(self).as_element_ref()
    }
}
pub trait MetaclassDowncast {}
pub trait MetaclassDowncastRefMut<'a> {}
pub trait MetaclassDowncastRef<'a> {}
impl MetaclassDowncast for Metaclass {}
impl<'a> MetaclassDowncastRefMut<'a> for MetaclassRefMut<'a> {}
impl<'a> MetaclassDowncastRef<'a> for MetaclassRef<'a> {}
pub trait MetaclassMethodsDescendants
where
    Self: DescendantOf<Metaclass>,
    Self::Via: MetaclassMethods,
    Self: Sized,
{}
pub trait MetaclassMethods: Sized {}
impl<T: MetaclassMethodsDescendants> MetaclassMethods for T
where
    T::Via: MetaclassMethods,
{}
impl DescendantOf<Structure> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl StructureMethodsDescendants for Metaclass {}
impl DescendantOf<Class> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ClassMethodsDescendants for Metaclass {}
impl DescendantOf<Classifier> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ClassifierMethodsDescendants for Metaclass {}
impl DescendantOf<Type> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl TypeMethodsDescendants for Metaclass {}
impl DescendantOf<Namespace> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl NamespaceMethodsDescendants for Metaclass {}
impl DescendantOf<Element> for Metaclass {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ElementMethodsDescendants for Metaclass {}
pub trait MetaclassRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MetaclassRefMut<'a>>,
    Self::Via: MetaclassRefMutMethods,
    Self: Sized,
{}
pub trait MetaclassRefMutMethods: Sized {}
impl<'a, T: MetaclassRefMutMethodsDescendants<'a>> MetaclassRefMutMethods for T
where
    T::Via: MetaclassRefMutMethods,
{}
impl<'a> DescendantOf<StructureRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> StructureRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
impl<'a> DescendantOf<ClassRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MetaclassRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MetaclassRefMut<'a> {}
pub trait MetaclassRefMethodsDescendants<'a>
where
    Self: DescendantOf<MetaclassRef<'a>>,
    Self::Via: MetaclassRefMethods,
    Self: Sized,
{}
pub trait MetaclassRefMethods: Sized {}
impl<'a, T: MetaclassRefMethodsDescendants<'a>> MetaclassRefMethods for T
where
    T::Via: MetaclassRefMethods,
{}
impl<'a> DescendantOf<StructureRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> StructureRefMethodsDescendants<'a> for MetaclassRef<'a> {}
impl<'a> DescendantOf<ClassRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for MetaclassRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for MetaclassRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for MetaclassRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for MetaclassRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MetaclassRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MetaclassRef<'a> {}

