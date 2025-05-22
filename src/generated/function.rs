#![allow(unused)]
use super::utils::DescendantOf;
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
use super::predicate::{Predicate, PredicateRefMut, PredicateRef};
pub struct FunctionInner {
    pub(super) sup_behavior: BehaviorInner,
}
pub trait FunctionStruct
where
    Self: FunctionStructRefMut,
    Self: FunctionStructRef,
    Self: BehaviorStruct,
{}
pub trait FunctionStructRefMut
where
    Self: FunctionStructRef,
    Self: BehaviorStructRefMut,
{}
pub trait FunctionStructRef
where
    Self: BehaviorStructRef,
{}
pub trait FunctionUpcast: FunctionStruct {
    fn into_function(self) -> Function;
}
pub trait FunctionUpcastRefMut<'a>: FunctionStructRefMut {
    fn as_function_ref_mut(self) -> FunctionRefMut<'a>;
}
pub trait FunctionUpcastRef<'a>: FunctionStructRef {
    fn as_function_ref(self) -> FunctionRef<'a>;
}
impl FunctionStruct for FunctionInner {}
impl FunctionStructRefMut for FunctionInner {}
impl FunctionStructRef for FunctionInner {}
impl BehaviorStruct for FunctionInner {}
impl BehaviorStructRefMut for FunctionInner {}
impl BehaviorStructRef for FunctionInner {}
impl ClassStruct for FunctionInner {}
impl ClassStructRefMut for FunctionInner {}
impl ClassStructRef for FunctionInner {}
impl ClassifierStruct for FunctionInner {}
impl ClassifierStructRefMut for FunctionInner {}
impl ClassifierStructRef for FunctionInner {}
impl TypeStruct for FunctionInner {
    fn is_abstract(self) -> bool {
        self.sup_behavior.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_behavior.is_sufficient()
    }
}
impl TypeStructRefMut for FunctionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_behavior.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_behavior.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FunctionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_behavior.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_behavior.is_sufficient_ref()
    }
}
impl NamespaceStruct for FunctionInner {}
impl NamespaceStructRefMut for FunctionInner {}
impl NamespaceStructRef for FunctionInner {}
impl ElementStruct for FunctionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_behavior.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_behavior.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_behavior.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_behavior.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_behavior.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_behavior.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_behavior.is_implied_included()
    }
}
impl ElementStructRefMut for FunctionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_behavior.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_behavior.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_behavior.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_behavior.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_behavior.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_behavior.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_behavior.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FunctionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_behavior.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_behavior.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_behavior.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_behavior.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_behavior.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_behavior.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_behavior.is_implied_included_ref()
    }
}
pub enum Function {
    Itself(FunctionInner),
    Predicate(Predicate),
}
pub enum FunctionRefMut<'a> {
    Itself(&'a mut FunctionInner),
    Predicate(PredicateRefMut<'a>),
}
pub enum FunctionRef<'a> {
    Itself(&'a FunctionInner),
    Predicate(PredicateRef<'a>),
}
impl Function {
    pub fn as_ref(&self) -> FunctionRef {
        match self {
            Function::Itself(inner) => FunctionRef::Itself(&inner),
            Function::Predicate(inner) => FunctionRef::Predicate(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> FunctionRefMut {
        match self {
            Function::Itself(inner) => FunctionRefMut::Itself(inner),
            Function::Predicate(inner) => FunctionRefMut::Predicate(inner.as_ref_mut()),
        }
    }
}
impl<'a> FunctionRefMut<'a> {
    pub fn as_ref(self) -> FunctionRef<'a> {
        match self {
            FunctionRefMut::Itself(inner) => FunctionRef::Itself(inner),
            FunctionRefMut::Predicate(inner) => FunctionRef::Predicate(inner.as_ref()),
        }
    }
}
impl FunctionStruct for Function {}
impl FunctionStructRefMut for Function {}
impl FunctionStructRef for Function {}
impl<'a> FunctionStructRefMut for FunctionRefMut<'a> {}
impl<'a> FunctionStructRef for FunctionRefMut<'a> {}
impl<'a> FunctionStructRef for FunctionRef<'a> {}
impl BehaviorStruct for Function {}
impl BehaviorStructRefMut for Function {}
impl BehaviorStructRef for Function {}
impl<'a> BehaviorStructRefMut for FunctionRefMut<'a> {}
impl<'a> BehaviorStructRef for FunctionRefMut<'a> {}
impl<'a> BehaviorStructRef for FunctionRef<'a> {}
impl ClassStruct for Function {}
impl ClassStructRefMut for Function {}
impl ClassStructRef for Function {}
impl<'a> ClassStructRefMut for FunctionRefMut<'a> {}
impl<'a> ClassStructRef for FunctionRefMut<'a> {}
impl<'a> ClassStructRef for FunctionRef<'a> {}
impl ClassifierStruct for Function {}
impl ClassifierStructRefMut for Function {}
impl ClassifierStructRef for Function {}
impl<'a> ClassifierStructRefMut for FunctionRefMut<'a> {}
impl<'a> ClassifierStructRef for FunctionRefMut<'a> {}
impl<'a> ClassifierStructRef for FunctionRef<'a> {}
impl TypeStruct for Function {
    fn is_abstract(self) -> bool {
        match self {
            Function::Itself(x) => x.is_abstract(),
            Function::Predicate(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Function::Itself(x) => x.is_sufficient(),
            Function::Predicate(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Function {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Function::Itself(x) => x.is_abstract_ref_mut(),
            Function::Predicate(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Function::Itself(x) => x.is_sufficient_ref_mut(),
            Function::Predicate(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Function {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Function::Itself(x) => x.is_abstract_ref(),
            Function::Predicate(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Function::Itself(x) => x.is_sufficient_ref(),
            Function::Predicate(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FunctionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            FunctionRefMut::Predicate(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            FunctionRefMut::Predicate(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FunctionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_abstract_ref(),
            FunctionRefMut::Predicate(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_sufficient_ref(),
            FunctionRefMut::Predicate(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FunctionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FunctionRef::Itself(x) => x.is_abstract_ref(),
            FunctionRef::Predicate(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FunctionRef::Itself(x) => x.is_sufficient_ref(),
            FunctionRef::Predicate(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Function {}
impl NamespaceStructRefMut for Function {}
impl NamespaceStructRef for Function {}
impl<'a> NamespaceStructRefMut for FunctionRefMut<'a> {}
impl<'a> NamespaceStructRef for FunctionRefMut<'a> {}
impl<'a> NamespaceStructRef for FunctionRef<'a> {}
impl ElementStruct for Function {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Function::Itself(x) => x.owned_relationship(),
            Function::Predicate(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Function::Itself(x) => x.owning_relationship(),
            Function::Predicate(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Function::Itself(x) => x.element_id(),
            Function::Predicate(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Function::Itself(x) => x.alias_ids(),
            Function::Predicate(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Function::Itself(x) => x.declared_short_name(),
            Function::Predicate(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Function::Itself(x) => x.declared_name(),
            Function::Predicate(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Function::Itself(x) => x.is_implied_included(),
            Function::Predicate(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Function {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Function::Itself(x) => x.owned_relationship_ref_mut(),
            Function::Predicate(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Function::Itself(x) => x.owning_relationship_ref_mut(),
            Function::Predicate(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Function::Itself(x) => x.element_id_ref_mut(),
            Function::Predicate(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Function::Itself(x) => x.alias_ids_ref_mut(),
            Function::Predicate(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Function::Itself(x) => x.declared_short_name_ref_mut(),
            Function::Predicate(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Function::Itself(x) => x.declared_name_ref_mut(),
            Function::Predicate(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Function::Itself(x) => x.is_implied_included_ref_mut(),
            Function::Predicate(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Function {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Function::Itself(x) => x.owned_relationship_ref(),
            Function::Predicate(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Function::Itself(x) => x.owning_relationship_ref(),
            Function::Predicate(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Function::Itself(x) => x.element_id_ref(),
            Function::Predicate(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Function::Itself(x) => x.alias_ids_ref(),
            Function::Predicate(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Function::Itself(x) => x.declared_short_name_ref(),
            Function::Predicate(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Function::Itself(x) => x.declared_name_ref(),
            Function::Predicate(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Function::Itself(x) => x.is_implied_included_ref(),
            Function::Predicate(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FunctionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FunctionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            FunctionRefMut::Predicate(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FunctionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            FunctionRefMut::Predicate(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FunctionRefMut::Itself(x) => x.element_id_ref_mut(),
            FunctionRefMut::Predicate(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FunctionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            FunctionRefMut::Predicate(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FunctionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            FunctionRefMut::Predicate(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FunctionRefMut::Itself(x) => x.declared_name_ref_mut(),
            FunctionRefMut::Predicate(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            FunctionRefMut::Predicate(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FunctionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FunctionRefMut::Itself(x) => x.owned_relationship_ref(),
            FunctionRefMut::Predicate(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FunctionRefMut::Itself(x) => x.owning_relationship_ref(),
            FunctionRefMut::Predicate(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FunctionRefMut::Itself(x) => x.element_id_ref(),
            FunctionRefMut::Predicate(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FunctionRefMut::Itself(x) => x.alias_ids_ref(),
            FunctionRefMut::Predicate(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FunctionRefMut::Itself(x) => x.declared_short_name_ref(),
            FunctionRefMut::Predicate(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FunctionRefMut::Itself(x) => x.declared_name_ref(),
            FunctionRefMut::Predicate(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FunctionRefMut::Itself(x) => x.is_implied_included_ref(),
            FunctionRefMut::Predicate(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FunctionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FunctionRef::Itself(x) => x.owned_relationship_ref(),
            FunctionRef::Predicate(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FunctionRef::Itself(x) => x.owning_relationship_ref(),
            FunctionRef::Predicate(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FunctionRef::Itself(x) => x.element_id_ref(),
            FunctionRef::Predicate(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FunctionRef::Itself(x) => x.alias_ids_ref(),
            FunctionRef::Predicate(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FunctionRef::Itself(x) => x.declared_short_name_ref(),
            FunctionRef::Predicate(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FunctionRef::Itself(x) => x.declared_name_ref(),
            FunctionRef::Predicate(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FunctionRef::Itself(x) => x.is_implied_included_ref(),
            FunctionRef::Predicate(x) => x.is_implied_included_ref(),
        }
    }
}
impl FunctionUpcast for Function {
    fn into_function(self) -> Function {
        self
    }
}
impl<'a> FunctionUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_function_ref_mut(self) -> FunctionRefMut<'a> {
        self
    }
}
impl<'a> FunctionUpcastRef<'a> for FunctionRef<'a> {
    fn as_function_ref(self) -> FunctionRef<'a> {
        self
    }
}
impl BehaviorUpcast for Function {
    fn into_behavior(self) -> Behavior {
        Behavior::Function(self).into_behavior()
    }
}
impl<'a> BehaviorUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_behavior_ref_mut(self) -> BehaviorRefMut<'a> {
        BehaviorRefMut::Function(self).as_behavior_ref_mut()
    }
}
impl<'a> BehaviorUpcastRef<'a> for FunctionRef<'a> {
    fn as_behavior_ref(self) -> BehaviorRef<'a> {
        BehaviorRef::Function(self).as_behavior_ref()
    }
}
impl ClassUpcast for Function {
    fn into_class(self) -> Class {
        Behavior::Function(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        BehaviorRefMut::Function(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for FunctionRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        BehaviorRef::Function(self).as_class_ref()
    }
}
impl ClassifierUpcast for Function {
    fn into_classifier(self) -> Classifier {
        Behavior::Function(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        BehaviorRefMut::Function(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for FunctionRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        BehaviorRef::Function(self).as_classifier_ref()
    }
}
impl TypeUpcast for Function {
    fn into_type_(self) -> Type {
        Behavior::Function(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        BehaviorRefMut::Function(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FunctionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        BehaviorRef::Function(self).as_type_ref()
    }
}
impl NamespaceUpcast for Function {
    fn into_namespace(self) -> Namespace {
        Behavior::Function(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        BehaviorRefMut::Function(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FunctionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        BehaviorRef::Function(self).as_namespace_ref()
    }
}
impl ElementUpcast for Function {
    fn into_element(self) -> Element {
        Behavior::Function(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FunctionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        BehaviorRefMut::Function(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FunctionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        BehaviorRef::Function(self).as_element_ref()
    }
}
pub trait FunctionDowncast {
    fn try_into_predicate(self) -> Result<Predicate, String>;
}
pub trait FunctionDowncastRefMut<'a> {
    fn try_as_predicate_ref_mut(self) -> Result<PredicateRefMut<'a>, String>;
}
pub trait FunctionDowncastRef<'a> {
    fn try_as_predicate_ref(self) -> Result<PredicateRef<'a>, String>;
}
impl FunctionDowncast for Function {
    fn try_into_predicate(self) -> Result<Predicate, String> {
        match self {
            Function::Predicate(e) => Ok(e),
            _ => Err("Not a Predicate".into()),
        }
    }
}
impl<'a> FunctionDowncastRefMut<'a> for FunctionRefMut<'a> {
    fn try_as_predicate_ref_mut(self) -> Result<PredicateRefMut<'a>, String> {
        match self {
            FunctionRefMut::Predicate(e) => Ok(e),
            _ => Err("Not a Predicate".into()),
        }
    }
}
impl<'a> FunctionDowncastRef<'a> for FunctionRef<'a> {
    fn try_as_predicate_ref(self) -> Result<PredicateRef<'a>, String> {
        match self {
            FunctionRef::Predicate(e) => Ok(e),
            _ => Err("Not a Predicate".into()),
        }
    }
}
pub trait FunctionMethodsDescendants
where
    Self: DescendantOf<Function>,
    Self::Via: FunctionMethods,
    Self: Sized,
{}
pub trait FunctionMethods: Sized {}
impl<T: FunctionMethodsDescendants> FunctionMethods for T
where
    T::Via: FunctionMethods,
{}
impl DescendantOf<Behavior> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl BehaviorMethodsDescendants for Function {}
impl DescendantOf<Class> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ClassMethodsDescendants for Function {}
impl DescendantOf<Classifier> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ClassifierMethodsDescendants for Function {}
impl DescendantOf<Type> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl TypeMethodsDescendants for Function {}
impl DescendantOf<Namespace> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl NamespaceMethodsDescendants for Function {}
impl DescendantOf<Element> for Function {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ElementMethodsDescendants for Function {}
pub trait FunctionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FunctionRefMut<'a>>,
    Self::Via: FunctionRefMutMethods,
    Self: Sized,
{}
pub trait FunctionRefMutMethods: Sized {}
impl<'a, T: FunctionRefMutMethodsDescendants<'a>> FunctionRefMutMethods for T
where
    T::Via: FunctionRefMutMethods,
{}
impl<'a> DescendantOf<BehaviorRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> BehaviorRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
impl<'a> DescendantOf<ClassRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FunctionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FunctionRefMut<'a> {}
pub trait FunctionRefMethodsDescendants<'a>
where
    Self: DescendantOf<FunctionRef<'a>>,
    Self::Via: FunctionRefMethods,
    Self: Sized,
{}
pub trait FunctionRefMethods: Sized {}
impl<'a, T: FunctionRefMethodsDescendants<'a>> FunctionRefMethods for T
where
    T::Via: FunctionRefMethods,
{}
impl<'a> DescendantOf<BehaviorRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> BehaviorRefMethodsDescendants<'a> for FunctionRef<'a> {}
impl<'a> DescendantOf<ClassRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for FunctionRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for FunctionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FunctionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FunctionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FunctionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FunctionRef<'a> {}

