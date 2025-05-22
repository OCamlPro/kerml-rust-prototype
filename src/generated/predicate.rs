#![allow(unused)]
use super::utils::DescendantOf;
use super::function::{
    Function, FunctionRefMut, FunctionRef, FunctionStruct, FunctionStructRefMut,
    FunctionStructRef, FunctionInner, FunctionUpcast, FunctionUpcastRefMut,
    FunctionUpcastRef, FunctionMethodsDescendants, FunctionRefMutMethodsDescendants,
    FunctionRefMethodsDescendants,
};
use super::behavior::{
    Behavior, BehaviorRefMut, BehaviorRef, BehaviorStruct, BehaviorStructRefMut,
    BehaviorStructRef, BehaviorInner, BehaviorUpcast, BehaviorUpcastRefMut,
    BehaviorUpcastRef, BehaviorMethodsDescendants, BehaviorRefMutMethodsDescendants,
    BehaviorRefMethodsDescendants,
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
pub struct PredicateInner {
    pub(super) sup_function: FunctionInner,
}
pub trait PredicateStruct
where
    Self: PredicateStructRefMut,
    Self: PredicateStructRef,
    Self: FunctionStruct,
{}
pub trait PredicateStructRefMut
where
    Self: PredicateStructRef,
    Self: FunctionStructRefMut,
{}
pub trait PredicateStructRef
where
    Self: FunctionStructRef,
{}
pub trait PredicateUpcast: PredicateStruct {
    fn into_predicate(self) -> Predicate;
}
pub trait PredicateUpcastRefMut<'a>: PredicateStructRefMut {
    fn as_predicate_ref_mut(self) -> PredicateRefMut<'a>;
}
pub trait PredicateUpcastRef<'a>: PredicateStructRef {
    fn as_predicate_ref(self) -> PredicateRef<'a>;
}
impl PredicateStruct for PredicateInner {}
impl PredicateStructRefMut for PredicateInner {}
impl PredicateStructRef for PredicateInner {}
impl FunctionStruct for PredicateInner {}
impl FunctionStructRefMut for PredicateInner {}
impl FunctionStructRef for PredicateInner {}
impl BehaviorStruct for PredicateInner {}
impl BehaviorStructRefMut for PredicateInner {}
impl BehaviorStructRef for PredicateInner {}
impl ClassStruct for PredicateInner {}
impl ClassStructRefMut for PredicateInner {}
impl ClassStructRef for PredicateInner {}
impl ClassifierStruct for PredicateInner {}
impl ClassifierStructRefMut for PredicateInner {}
impl ClassifierStructRef for PredicateInner {}
impl TypeStruct for PredicateInner {
    fn is_abstract(self) -> bool {
        self.sup_function.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_function.is_sufficient()
    }
}
impl TypeStructRefMut for PredicateInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_function.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_function.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for PredicateInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_function.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_function.is_sufficient_ref()
    }
}
impl NamespaceStruct for PredicateInner {}
impl NamespaceStructRefMut for PredicateInner {}
impl NamespaceStructRef for PredicateInner {}
impl ElementStruct for PredicateInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_function.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_function.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_function.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_function.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_function.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_function.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_function.is_implied_included()
    }
}
impl ElementStructRefMut for PredicateInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_function.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_function.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_function.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_function.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_function.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_function.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_function.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for PredicateInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_function.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_function.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_function.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_function.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_function.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_function.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_function.is_implied_included_ref()
    }
}
pub enum Predicate {
    Itself(PredicateInner),
}
pub enum PredicateRefMut<'a> {
    Itself(&'a mut PredicateInner),
}
pub enum PredicateRef<'a> {
    Itself(&'a PredicateInner),
}
impl Predicate {
    pub fn as_ref(&self) -> PredicateRef {
        match self {
            Predicate::Itself(inner) => PredicateRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> PredicateRefMut {
        match self {
            Predicate::Itself(inner) => PredicateRefMut::Itself(inner),
        }
    }
}
impl<'a> PredicateRefMut<'a> {
    pub fn as_ref(self) -> PredicateRef<'a> {
        match self {
            PredicateRefMut::Itself(inner) => PredicateRef::Itself(inner),
        }
    }
}
impl PredicateStruct for Predicate {}
impl PredicateStructRefMut for Predicate {}
impl PredicateStructRef for Predicate {}
impl<'a> PredicateStructRefMut for PredicateRefMut<'a> {}
impl<'a> PredicateStructRef for PredicateRefMut<'a> {}
impl<'a> PredicateStructRef for PredicateRef<'a> {}
impl FunctionStruct for Predicate {}
impl FunctionStructRefMut for Predicate {}
impl FunctionStructRef for Predicate {}
impl<'a> FunctionStructRefMut for PredicateRefMut<'a> {}
impl<'a> FunctionStructRef for PredicateRefMut<'a> {}
impl<'a> FunctionStructRef for PredicateRef<'a> {}
impl BehaviorStruct for Predicate {}
impl BehaviorStructRefMut for Predicate {}
impl BehaviorStructRef for Predicate {}
impl<'a> BehaviorStructRefMut for PredicateRefMut<'a> {}
impl<'a> BehaviorStructRef for PredicateRefMut<'a> {}
impl<'a> BehaviorStructRef for PredicateRef<'a> {}
impl ClassStruct for Predicate {}
impl ClassStructRefMut for Predicate {}
impl ClassStructRef for Predicate {}
impl<'a> ClassStructRefMut for PredicateRefMut<'a> {}
impl<'a> ClassStructRef for PredicateRefMut<'a> {}
impl<'a> ClassStructRef for PredicateRef<'a> {}
impl ClassifierStruct for Predicate {}
impl ClassifierStructRefMut for Predicate {}
impl ClassifierStructRef for Predicate {}
impl<'a> ClassifierStructRefMut for PredicateRefMut<'a> {}
impl<'a> ClassifierStructRef for PredicateRefMut<'a> {}
impl<'a> ClassifierStructRef for PredicateRef<'a> {}
impl TypeStruct for Predicate {
    fn is_abstract(self) -> bool {
        match self {
            Predicate::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Predicate::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Predicate {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Predicate::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Predicate::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Predicate {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Predicate::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Predicate::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for PredicateRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for PredicateRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for PredicateRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            PredicateRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            PredicateRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Predicate {}
impl NamespaceStructRefMut for Predicate {}
impl NamespaceStructRef for Predicate {}
impl<'a> NamespaceStructRefMut for PredicateRefMut<'a> {}
impl<'a> NamespaceStructRef for PredicateRefMut<'a> {}
impl<'a> NamespaceStructRef for PredicateRef<'a> {}
impl ElementStruct for Predicate {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Predicate::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Predicate::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Predicate::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Predicate::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Predicate::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Predicate {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Predicate::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Predicate::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Predicate::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Predicate::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Predicate::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Predicate {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Predicate::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Predicate::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Predicate::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Predicate::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Predicate::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Predicate::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for PredicateRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PredicateRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            PredicateRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            PredicateRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            PredicateRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PredicateRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PredicateRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for PredicateRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PredicateRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PredicateRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PredicateRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PredicateRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PredicateRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PredicateRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PredicateRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for PredicateRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PredicateRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PredicateRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PredicateRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PredicateRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PredicateRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PredicateRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PredicateRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl PredicateUpcast for Predicate {
    fn into_predicate(self) -> Predicate {
        self
    }
}
impl<'a> PredicateUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_predicate_ref_mut(self) -> PredicateRefMut<'a> {
        self
    }
}
impl<'a> PredicateUpcastRef<'a> for PredicateRef<'a> {
    fn as_predicate_ref(self) -> PredicateRef<'a> {
        self
    }
}
impl FunctionUpcast for Predicate {
    fn into_function(self) -> Function {
        Function::Predicate(self).into_function()
    }
}
impl<'a> FunctionUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_function_ref_mut(self) -> FunctionRefMut<'a> {
        FunctionRefMut::Predicate(self).as_function_ref_mut()
    }
}
impl<'a> FunctionUpcastRef<'a> for PredicateRef<'a> {
    fn as_function_ref(self) -> FunctionRef<'a> {
        FunctionRef::Predicate(self).as_function_ref()
    }
}
impl BehaviorUpcast for Predicate {
    fn into_behavior(self) -> Behavior {
        Function::Predicate(self).into_behavior()
    }
}
impl<'a> BehaviorUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_behavior_ref_mut(self) -> BehaviorRefMut<'a> {
        FunctionRefMut::Predicate(self).as_behavior_ref_mut()
    }
}
impl<'a> BehaviorUpcastRef<'a> for PredicateRef<'a> {
    fn as_behavior_ref(self) -> BehaviorRef<'a> {
        FunctionRef::Predicate(self).as_behavior_ref()
    }
}
impl ClassUpcast for Predicate {
    fn into_class(self) -> Class {
        Function::Predicate(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        FunctionRefMut::Predicate(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for PredicateRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        FunctionRef::Predicate(self).as_class_ref()
    }
}
impl ClassifierUpcast for Predicate {
    fn into_classifier(self) -> Classifier {
        Function::Predicate(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        FunctionRefMut::Predicate(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for PredicateRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        FunctionRef::Predicate(self).as_classifier_ref()
    }
}
impl TypeUpcast for Predicate {
    fn into_type_(self) -> Type {
        Function::Predicate(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FunctionRefMut::Predicate(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for PredicateRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FunctionRef::Predicate(self).as_type_ref()
    }
}
impl NamespaceUpcast for Predicate {
    fn into_namespace(self) -> Namespace {
        Function::Predicate(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FunctionRefMut::Predicate(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for PredicateRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FunctionRef::Predicate(self).as_namespace_ref()
    }
}
impl ElementUpcast for Predicate {
    fn into_element(self) -> Element {
        Function::Predicate(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for PredicateRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FunctionRefMut::Predicate(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for PredicateRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FunctionRef::Predicate(self).as_element_ref()
    }
}
pub trait PredicateDowncast {}
pub trait PredicateDowncastRefMut<'a> {}
pub trait PredicateDowncastRef<'a> {}
impl PredicateDowncast for Predicate {}
impl<'a> PredicateDowncastRefMut<'a> for PredicateRefMut<'a> {}
impl<'a> PredicateDowncastRef<'a> for PredicateRef<'a> {}
pub trait PredicateMethodsDescendants
where
    Self: DescendantOf<Predicate>,
    Self::Via: PredicateMethods,
    Self: Sized,
{}
pub trait PredicateMethods: Sized {}
impl<T: PredicateMethodsDescendants> PredicateMethods for T
where
    T::Via: PredicateMethods,
{}
impl DescendantOf<Function> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl FunctionMethodsDescendants for Predicate {}
impl DescendantOf<Behavior> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl BehaviorMethodsDescendants for Predicate {}
impl DescendantOf<Class> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl ClassMethodsDescendants for Predicate {}
impl DescendantOf<Classifier> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl ClassifierMethodsDescendants for Predicate {}
impl DescendantOf<Type> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl TypeMethodsDescendants for Predicate {}
impl DescendantOf<Namespace> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl NamespaceMethodsDescendants for Predicate {}
impl DescendantOf<Element> for Predicate {
    type Via = Function;
    fn into_via(self) -> Self::Via {
        self.into_function()
    }
}
impl ElementMethodsDescendants for Predicate {}
pub trait PredicateRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<PredicateRefMut<'a>>,
    Self::Via: PredicateRefMutMethods,
    Self: Sized,
{}
pub trait PredicateRefMutMethods: Sized {}
impl<'a, T: PredicateRefMutMethodsDescendants<'a>> PredicateRefMutMethods for T
where
    T::Via: PredicateRefMutMethods,
{}
impl<'a> DescendantOf<FunctionRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> FunctionRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<BehaviorRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> BehaviorRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<ClassRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for PredicateRefMut<'a> {
    type Via = FunctionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for PredicateRefMut<'a> {}
pub trait PredicateRefMethodsDescendants<'a>
where
    Self: DescendantOf<PredicateRef<'a>>,
    Self::Via: PredicateRefMethods,
    Self: Sized,
{}
pub trait PredicateRefMethods: Sized {}
impl<'a, T: PredicateRefMethodsDescendants<'a>> PredicateRefMethods for T
where
    T::Via: PredicateRefMethods,
{}
impl<'a> DescendantOf<FunctionRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> FunctionRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<BehaviorRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> BehaviorRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<ClassRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for PredicateRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for PredicateRef<'a> {
    type Via = FunctionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_function_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for PredicateRef<'a> {}

