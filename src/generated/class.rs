#![allow(unused)]
use super::utils::DescendantOf;
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
use super::behavior::{Behavior, BehaviorRefMut, BehaviorRef};
use super::structure::{Structure, StructureRefMut, StructureRef};
pub struct ClassInner {
    pub(super) sup_classifier: ClassifierInner,
}
pub trait ClassStruct
where
    Self: ClassStructRefMut,
    Self: ClassStructRef,
    Self: ClassifierStruct,
{}
pub trait ClassStructRefMut
where
    Self: ClassStructRef,
    Self: ClassifierStructRefMut,
{}
pub trait ClassStructRef
where
    Self: ClassifierStructRef,
{}
pub trait ClassUpcast: ClassStruct {
    fn into_class(self) -> Class;
}
pub trait ClassUpcastRefMut<'a>: ClassStructRefMut {
    fn as_class_ref_mut(self) -> ClassRefMut<'a>;
}
pub trait ClassUpcastRef<'a>: ClassStructRef {
    fn as_class_ref(self) -> ClassRef<'a>;
}
impl ClassStruct for ClassInner {}
impl ClassStructRefMut for ClassInner {}
impl ClassStructRef for ClassInner {}
impl ClassifierStruct for ClassInner {}
impl ClassifierStructRefMut for ClassInner {}
impl ClassifierStructRef for ClassInner {}
impl TypeStruct for ClassInner {
    fn is_abstract(self) -> bool {
        self.sup_classifier.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_classifier.is_sufficient()
    }
}
impl TypeStructRefMut for ClassInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for ClassInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_classifier.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_classifier.is_sufficient_ref()
    }
}
impl NamespaceStruct for ClassInner {}
impl NamespaceStructRefMut for ClassInner {}
impl NamespaceStructRef for ClassInner {}
impl ElementStruct for ClassInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_classifier.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_classifier.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_classifier.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_classifier.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_classifier.is_implied_included()
    }
}
impl ElementStructRefMut for ClassInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_classifier.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_classifier.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_classifier.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_classifier.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_classifier.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for ClassInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_classifier.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_classifier.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_classifier.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_classifier.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_classifier.is_implied_included_ref()
    }
}
pub enum Class {
    Itself(ClassInner),
    Behavior(Behavior),
    Structure(Structure),
}
pub enum ClassRefMut<'a> {
    Itself(&'a mut ClassInner),
    Behavior(BehaviorRefMut<'a>),
    Structure(StructureRefMut<'a>),
}
pub enum ClassRef<'a> {
    Itself(&'a ClassInner),
    Behavior(BehaviorRef<'a>),
    Structure(StructureRef<'a>),
}
impl Class {
    pub fn as_ref(&self) -> ClassRef {
        match self {
            Class::Itself(inner) => ClassRef::Itself(&inner),
            Class::Behavior(inner) => ClassRef::Behavior(inner.as_ref()),
            Class::Structure(inner) => ClassRef::Structure(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> ClassRefMut {
        match self {
            Class::Itself(inner) => ClassRefMut::Itself(inner),
            Class::Behavior(inner) => ClassRefMut::Behavior(inner.as_ref_mut()),
            Class::Structure(inner) => ClassRefMut::Structure(inner.as_ref_mut()),
        }
    }
}
impl<'a> ClassRefMut<'a> {
    pub fn as_ref(self) -> ClassRef<'a> {
        match self {
            ClassRefMut::Itself(inner) => ClassRef::Itself(inner),
            ClassRefMut::Behavior(inner) => ClassRef::Behavior(inner.as_ref()),
            ClassRefMut::Structure(inner) => ClassRef::Structure(inner.as_ref()),
        }
    }
}
impl ClassStruct for Class {}
impl ClassStructRefMut for Class {}
impl ClassStructRef for Class {}
impl<'a> ClassStructRefMut for ClassRefMut<'a> {}
impl<'a> ClassStructRef for ClassRefMut<'a> {}
impl<'a> ClassStructRef for ClassRef<'a> {}
impl ClassifierStruct for Class {}
impl ClassifierStructRefMut for Class {}
impl ClassifierStructRef for Class {}
impl<'a> ClassifierStructRefMut for ClassRefMut<'a> {}
impl<'a> ClassifierStructRef for ClassRefMut<'a> {}
impl<'a> ClassifierStructRef for ClassRef<'a> {}
impl TypeStruct for Class {
    fn is_abstract(self) -> bool {
        match self {
            Class::Itself(x) => x.is_abstract(),
            Class::Behavior(x) => x.is_abstract(),
            Class::Structure(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Class::Itself(x) => x.is_sufficient(),
            Class::Behavior(x) => x.is_sufficient(),
            Class::Structure(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Class {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Class::Itself(x) => x.is_abstract_ref_mut(),
            Class::Behavior(x) => x.is_abstract_ref_mut(),
            Class::Structure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Class::Itself(x) => x.is_sufficient_ref_mut(),
            Class::Behavior(x) => x.is_sufficient_ref_mut(),
            Class::Structure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Class {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Class::Itself(x) => x.is_abstract_ref(),
            Class::Behavior(x) => x.is_abstract_ref(),
            Class::Structure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Class::Itself(x) => x.is_sufficient_ref(),
            Class::Behavior(x) => x.is_sufficient_ref(),
            Class::Structure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for ClassRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassRefMut::Itself(x) => x.is_abstract_ref_mut(),
            ClassRefMut::Behavior(x) => x.is_abstract_ref_mut(),
            ClassRefMut::Structure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            ClassRefMut::Behavior(x) => x.is_sufficient_ref_mut(),
            ClassRefMut::Structure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for ClassRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ClassRefMut::Itself(x) => x.is_abstract_ref(),
            ClassRefMut::Behavior(x) => x.is_abstract_ref(),
            ClassRefMut::Structure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ClassRefMut::Itself(x) => x.is_sufficient_ref(),
            ClassRefMut::Behavior(x) => x.is_sufficient_ref(),
            ClassRefMut::Structure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for ClassRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ClassRef::Itself(x) => x.is_abstract_ref(),
            ClassRef::Behavior(x) => x.is_abstract_ref(),
            ClassRef::Structure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ClassRef::Itself(x) => x.is_sufficient_ref(),
            ClassRef::Behavior(x) => x.is_sufficient_ref(),
            ClassRef::Structure(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Class {}
impl NamespaceStructRefMut for Class {}
impl NamespaceStructRef for Class {}
impl<'a> NamespaceStructRefMut for ClassRefMut<'a> {}
impl<'a> NamespaceStructRef for ClassRefMut<'a> {}
impl<'a> NamespaceStructRef for ClassRef<'a> {}
impl ElementStruct for Class {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Class::Itself(x) => x.owned_relationship(),
            Class::Behavior(x) => x.owned_relationship(),
            Class::Structure(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Class::Itself(x) => x.owning_relationship(),
            Class::Behavior(x) => x.owning_relationship(),
            Class::Structure(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Class::Itself(x) => x.element_id(),
            Class::Behavior(x) => x.element_id(),
            Class::Structure(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Class::Itself(x) => x.alias_ids(),
            Class::Behavior(x) => x.alias_ids(),
            Class::Structure(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Class::Itself(x) => x.declared_short_name(),
            Class::Behavior(x) => x.declared_short_name(),
            Class::Structure(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Class::Itself(x) => x.declared_name(),
            Class::Behavior(x) => x.declared_name(),
            Class::Structure(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Class::Itself(x) => x.is_implied_included(),
            Class::Behavior(x) => x.is_implied_included(),
            Class::Structure(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Class {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Class::Itself(x) => x.owned_relationship_ref_mut(),
            Class::Behavior(x) => x.owned_relationship_ref_mut(),
            Class::Structure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Class::Itself(x) => x.owning_relationship_ref_mut(),
            Class::Behavior(x) => x.owning_relationship_ref_mut(),
            Class::Structure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Class::Itself(x) => x.element_id_ref_mut(),
            Class::Behavior(x) => x.element_id_ref_mut(),
            Class::Structure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Class::Itself(x) => x.alias_ids_ref_mut(),
            Class::Behavior(x) => x.alias_ids_ref_mut(),
            Class::Structure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Class::Itself(x) => x.declared_short_name_ref_mut(),
            Class::Behavior(x) => x.declared_short_name_ref_mut(),
            Class::Structure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Class::Itself(x) => x.declared_name_ref_mut(),
            Class::Behavior(x) => x.declared_name_ref_mut(),
            Class::Structure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Class::Itself(x) => x.is_implied_included_ref_mut(),
            Class::Behavior(x) => x.is_implied_included_ref_mut(),
            Class::Structure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Class {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Class::Itself(x) => x.owned_relationship_ref(),
            Class::Behavior(x) => x.owned_relationship_ref(),
            Class::Structure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Class::Itself(x) => x.owning_relationship_ref(),
            Class::Behavior(x) => x.owning_relationship_ref(),
            Class::Structure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Class::Itself(x) => x.element_id_ref(),
            Class::Behavior(x) => x.element_id_ref(),
            Class::Structure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Class::Itself(x) => x.alias_ids_ref(),
            Class::Behavior(x) => x.alias_ids_ref(),
            Class::Structure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Class::Itself(x) => x.declared_short_name_ref(),
            Class::Behavior(x) => x.declared_short_name_ref(),
            Class::Structure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Class::Itself(x) => x.declared_name_ref(),
            Class::Behavior(x) => x.declared_name_ref(),
            Class::Structure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Class::Itself(x) => x.is_implied_included_ref(),
            Class::Behavior(x) => x.is_implied_included_ref(),
            Class::Structure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ClassRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ClassRefMut::Behavior(x) => x.owned_relationship_ref_mut(),
            ClassRefMut::Structure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ClassRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ClassRefMut::Behavior(x) => x.owning_relationship_ref_mut(),
            ClassRefMut::Structure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ClassRefMut::Itself(x) => x.element_id_ref_mut(),
            ClassRefMut::Behavior(x) => x.element_id_ref_mut(),
            ClassRefMut::Structure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ClassRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ClassRefMut::Behavior(x) => x.alias_ids_ref_mut(),
            ClassRefMut::Structure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ClassRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ClassRefMut::Behavior(x) => x.declared_short_name_ref_mut(),
            ClassRefMut::Structure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ClassRefMut::Itself(x) => x.declared_name_ref_mut(),
            ClassRefMut::Behavior(x) => x.declared_name_ref_mut(),
            ClassRefMut::Structure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ClassRefMut::Behavior(x) => x.is_implied_included_ref_mut(),
            ClassRefMut::Structure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ClassRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassRefMut::Itself(x) => x.owned_relationship_ref(),
            ClassRefMut::Behavior(x) => x.owned_relationship_ref(),
            ClassRefMut::Structure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassRefMut::Itself(x) => x.owning_relationship_ref(),
            ClassRefMut::Behavior(x) => x.owning_relationship_ref(),
            ClassRefMut::Structure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ClassRefMut::Itself(x) => x.element_id_ref(),
            ClassRefMut::Behavior(x) => x.element_id_ref(),
            ClassRefMut::Structure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ClassRefMut::Itself(x) => x.alias_ids_ref(),
            ClassRefMut::Behavior(x) => x.alias_ids_ref(),
            ClassRefMut::Structure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ClassRefMut::Itself(x) => x.declared_short_name_ref(),
            ClassRefMut::Behavior(x) => x.declared_short_name_ref(),
            ClassRefMut::Structure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ClassRefMut::Itself(x) => x.declared_name_ref(),
            ClassRefMut::Behavior(x) => x.declared_name_ref(),
            ClassRefMut::Structure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ClassRefMut::Itself(x) => x.is_implied_included_ref(),
            ClassRefMut::Behavior(x) => x.is_implied_included_ref(),
            ClassRefMut::Structure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ClassRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassRef::Itself(x) => x.owned_relationship_ref(),
            ClassRef::Behavior(x) => x.owned_relationship_ref(),
            ClassRef::Structure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassRef::Itself(x) => x.owning_relationship_ref(),
            ClassRef::Behavior(x) => x.owning_relationship_ref(),
            ClassRef::Structure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ClassRef::Itself(x) => x.element_id_ref(),
            ClassRef::Behavior(x) => x.element_id_ref(),
            ClassRef::Structure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ClassRef::Itself(x) => x.alias_ids_ref(),
            ClassRef::Behavior(x) => x.alias_ids_ref(),
            ClassRef::Structure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ClassRef::Itself(x) => x.declared_short_name_ref(),
            ClassRef::Behavior(x) => x.declared_short_name_ref(),
            ClassRef::Structure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ClassRef::Itself(x) => x.declared_name_ref(),
            ClassRef::Behavior(x) => x.declared_name_ref(),
            ClassRef::Structure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ClassRef::Itself(x) => x.is_implied_included_ref(),
            ClassRef::Behavior(x) => x.is_implied_included_ref(),
            ClassRef::Structure(x) => x.is_implied_included_ref(),
        }
    }
}
impl ClassUpcast for Class {
    fn into_class(self) -> Class {
        self
    }
}
impl<'a> ClassUpcastRefMut<'a> for ClassRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        self
    }
}
impl<'a> ClassUpcastRef<'a> for ClassRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        self
    }
}
impl ClassifierUpcast for Class {
    fn into_classifier(self) -> Classifier {
        Classifier::Class(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for ClassRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        ClassifierRefMut::Class(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for ClassRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        ClassifierRef::Class(self).as_classifier_ref()
    }
}
impl TypeUpcast for Class {
    fn into_type_(self) -> Type {
        Classifier::Class(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for ClassRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ClassifierRefMut::Class(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for ClassRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ClassifierRef::Class(self).as_type_ref()
    }
}
impl NamespaceUpcast for Class {
    fn into_namespace(self) -> Namespace {
        Classifier::Class(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for ClassRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ClassifierRefMut::Class(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for ClassRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ClassifierRef::Class(self).as_namespace_ref()
    }
}
impl ElementUpcast for Class {
    fn into_element(self) -> Element {
        Classifier::Class(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ClassRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ClassifierRefMut::Class(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ClassRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ClassifierRef::Class(self).as_element_ref()
    }
}
pub trait ClassDowncast {
    fn try_into_behavior(self) -> Result<Behavior, String>;
    fn try_into_structure(self) -> Result<Structure, String>;
}
pub trait ClassDowncastRefMut<'a> {
    fn try_as_behavior_ref_mut(self) -> Result<BehaviorRefMut<'a>, String>;
    fn try_as_structure_ref_mut(self) -> Result<StructureRefMut<'a>, String>;
}
pub trait ClassDowncastRef<'a> {
    fn try_as_behavior_ref(self) -> Result<BehaviorRef<'a>, String>;
    fn try_as_structure_ref(self) -> Result<StructureRef<'a>, String>;
}
impl ClassDowncast for Class {
    fn try_into_behavior(self) -> Result<Behavior, String> {
        match self {
            Class::Behavior(e) => Ok(e),
            _ => Err("Not a Behavior".into()),
        }
    }
    fn try_into_structure(self) -> Result<Structure, String> {
        match self {
            Class::Structure(e) => Ok(e),
            _ => Err("Not a Structure".into()),
        }
    }
}
impl<'a> ClassDowncastRefMut<'a> for ClassRefMut<'a> {
    fn try_as_behavior_ref_mut(self) -> Result<BehaviorRefMut<'a>, String> {
        match self {
            ClassRefMut::Behavior(e) => Ok(e),
            _ => Err("Not a Behavior".into()),
        }
    }
    fn try_as_structure_ref_mut(self) -> Result<StructureRefMut<'a>, String> {
        match self {
            ClassRefMut::Structure(e) => Ok(e),
            _ => Err("Not a Structure".into()),
        }
    }
}
impl<'a> ClassDowncastRef<'a> for ClassRef<'a> {
    fn try_as_behavior_ref(self) -> Result<BehaviorRef<'a>, String> {
        match self {
            ClassRef::Behavior(e) => Ok(e),
            _ => Err("Not a Behavior".into()),
        }
    }
    fn try_as_structure_ref(self) -> Result<StructureRef<'a>, String> {
        match self {
            ClassRef::Structure(e) => Ok(e),
            _ => Err("Not a Structure".into()),
        }
    }
}
pub trait ClassMethodsDescendants
where
    Self: DescendantOf<Class>,
    Self::Via: ClassMethods,
    Self: Sized,
{}
pub trait ClassMethods: Sized {}
impl<T: ClassMethodsDescendants> ClassMethods for T
where
    T::Via: ClassMethods,
{}
impl DescendantOf<Classifier> for Class {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl ClassifierMethodsDescendants for Class {}
impl DescendantOf<Type> for Class {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl TypeMethodsDescendants for Class {}
impl DescendantOf<Namespace> for Class {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl NamespaceMethodsDescendants for Class {}
impl DescendantOf<Element> for Class {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl ElementMethodsDescendants for Class {}
pub trait ClassRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ClassRefMut<'a>>,
    Self::Via: ClassRefMutMethods,
    Self: Sized,
{}
pub trait ClassRefMutMethods: Sized {}
impl<'a, T: ClassRefMutMethodsDescendants<'a>> ClassRefMutMethods for T
where
    T::Via: ClassRefMutMethods,
{}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for ClassRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for ClassRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for ClassRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for ClassRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for ClassRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for ClassRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ClassRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ClassRefMut<'a> {}
pub trait ClassRefMethodsDescendants<'a>
where
    Self: DescendantOf<ClassRef<'a>>,
    Self::Via: ClassRefMethods,
    Self: Sized,
{}
pub trait ClassRefMethods: Sized {}
impl<'a, T: ClassRefMethodsDescendants<'a>> ClassRefMethods for T
where
    T::Via: ClassRefMethods,
{}
impl<'a> DescendantOf<ClassifierRef<'a>> for ClassRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for ClassRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for ClassRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for ClassRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for ClassRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for ClassRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ClassRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ClassRef<'a> {}

