#![allow(unused)]
use super::utils::DescendantOf;
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
use super::function::{Function, FunctionRefMut, FunctionRef};
use super::interaction::{Interaction, InteractionRefMut, InteractionRef};
pub struct BehaviorInner {
    pub(super) sup_class: ClassInner,
}
pub trait BehaviorStruct
where
    Self: BehaviorStructRefMut,
    Self: BehaviorStructRef,
    Self: ClassStruct,
{}
pub trait BehaviorStructRefMut
where
    Self: BehaviorStructRef,
    Self: ClassStructRefMut,
{}
pub trait BehaviorStructRef
where
    Self: ClassStructRef,
{}
pub trait BehaviorUpcast: BehaviorStruct {
    fn into_behavior(self) -> Behavior;
}
pub trait BehaviorUpcastRefMut<'a>: BehaviorStructRefMut {
    fn as_behavior_ref_mut(self) -> BehaviorRefMut<'a>;
}
pub trait BehaviorUpcastRef<'a>: BehaviorStructRef {
    fn as_behavior_ref(self) -> BehaviorRef<'a>;
}
impl BehaviorStruct for BehaviorInner {}
impl BehaviorStructRefMut for BehaviorInner {}
impl BehaviorStructRef for BehaviorInner {}
impl ClassStruct for BehaviorInner {}
impl ClassStructRefMut for BehaviorInner {}
impl ClassStructRef for BehaviorInner {}
impl ClassifierStruct for BehaviorInner {}
impl ClassifierStructRefMut for BehaviorInner {}
impl ClassifierStructRef for BehaviorInner {}
impl TypeStruct for BehaviorInner {
    fn is_abstract(self) -> bool {
        self.sup_class.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_class.is_sufficient()
    }
}
impl TypeStructRefMut for BehaviorInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for BehaviorInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_class.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_class.is_sufficient_ref()
    }
}
impl NamespaceStruct for BehaviorInner {}
impl NamespaceStructRefMut for BehaviorInner {}
impl NamespaceStructRef for BehaviorInner {}
impl ElementStruct for BehaviorInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_class.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_class.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_class.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_class.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_class.is_implied_included()
    }
}
impl ElementStructRefMut for BehaviorInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_class.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_class.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_class.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_class.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_class.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for BehaviorInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_class.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_class.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_class.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_class.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_class.is_implied_included_ref()
    }
}
pub enum Behavior {
    Itself(BehaviorInner),
    Function(Function),
    Interaction(Interaction),
}
pub enum BehaviorRefMut<'a> {
    Itself(&'a mut BehaviorInner),
    Function(FunctionRefMut<'a>),
    Interaction(InteractionRefMut<'a>),
}
pub enum BehaviorRef<'a> {
    Itself(&'a BehaviorInner),
    Function(FunctionRef<'a>),
    Interaction(InteractionRef<'a>),
}
impl Behavior {
    pub fn as_ref(&self) -> BehaviorRef {
        match self {
            Behavior::Itself(inner) => BehaviorRef::Itself(&inner),
            Behavior::Function(inner) => BehaviorRef::Function(inner.as_ref()),
            Behavior::Interaction(inner) => BehaviorRef::Interaction(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> BehaviorRefMut {
        match self {
            Behavior::Itself(inner) => BehaviorRefMut::Itself(inner),
            Behavior::Function(inner) => BehaviorRefMut::Function(inner.as_ref_mut()),
            Behavior::Interaction(inner) => {
                BehaviorRefMut::Interaction(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> BehaviorRefMut<'a> {
    pub fn as_ref(self) -> BehaviorRef<'a> {
        match self {
            BehaviorRefMut::Itself(inner) => BehaviorRef::Itself(inner),
            BehaviorRefMut::Function(inner) => BehaviorRef::Function(inner.as_ref()),
            BehaviorRefMut::Interaction(inner) => {
                BehaviorRef::Interaction(inner.as_ref())
            }
        }
    }
}
impl BehaviorStruct for Behavior {}
impl BehaviorStructRefMut for Behavior {}
impl BehaviorStructRef for Behavior {}
impl<'a> BehaviorStructRefMut for BehaviorRefMut<'a> {}
impl<'a> BehaviorStructRef for BehaviorRefMut<'a> {}
impl<'a> BehaviorStructRef for BehaviorRef<'a> {}
impl ClassStruct for Behavior {}
impl ClassStructRefMut for Behavior {}
impl ClassStructRef for Behavior {}
impl<'a> ClassStructRefMut for BehaviorRefMut<'a> {}
impl<'a> ClassStructRef for BehaviorRefMut<'a> {}
impl<'a> ClassStructRef for BehaviorRef<'a> {}
impl ClassifierStruct for Behavior {}
impl ClassifierStructRefMut for Behavior {}
impl ClassifierStructRef for Behavior {}
impl<'a> ClassifierStructRefMut for BehaviorRefMut<'a> {}
impl<'a> ClassifierStructRef for BehaviorRefMut<'a> {}
impl<'a> ClassifierStructRef for BehaviorRef<'a> {}
impl TypeStruct for Behavior {
    fn is_abstract(self) -> bool {
        match self {
            Behavior::Itself(x) => x.is_abstract(),
            Behavior::Function(x) => x.is_abstract(),
            Behavior::Interaction(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Behavior::Itself(x) => x.is_sufficient(),
            Behavior::Function(x) => x.is_sufficient(),
            Behavior::Interaction(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Behavior {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Behavior::Itself(x) => x.is_abstract_ref_mut(),
            Behavior::Function(x) => x.is_abstract_ref_mut(),
            Behavior::Interaction(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Behavior::Itself(x) => x.is_sufficient_ref_mut(),
            Behavior::Function(x) => x.is_sufficient_ref_mut(),
            Behavior::Interaction(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Behavior {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Behavior::Itself(x) => x.is_abstract_ref(),
            Behavior::Function(x) => x.is_abstract_ref(),
            Behavior::Interaction(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Behavior::Itself(x) => x.is_sufficient_ref(),
            Behavior::Function(x) => x.is_sufficient_ref(),
            Behavior::Interaction(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for BehaviorRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_abstract_ref_mut(),
            BehaviorRefMut::Function(x) => x.is_abstract_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            BehaviorRefMut::Function(x) => x.is_sufficient_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for BehaviorRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_abstract_ref(),
            BehaviorRefMut::Function(x) => x.is_abstract_ref(),
            BehaviorRefMut::Interaction(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_sufficient_ref(),
            BehaviorRefMut::Function(x) => x.is_sufficient_ref(),
            BehaviorRefMut::Interaction(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for BehaviorRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BehaviorRef::Itself(x) => x.is_abstract_ref(),
            BehaviorRef::Function(x) => x.is_abstract_ref(),
            BehaviorRef::Interaction(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BehaviorRef::Itself(x) => x.is_sufficient_ref(),
            BehaviorRef::Function(x) => x.is_sufficient_ref(),
            BehaviorRef::Interaction(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Behavior {}
impl NamespaceStructRefMut for Behavior {}
impl NamespaceStructRef for Behavior {}
impl<'a> NamespaceStructRefMut for BehaviorRefMut<'a> {}
impl<'a> NamespaceStructRef for BehaviorRefMut<'a> {}
impl<'a> NamespaceStructRef for BehaviorRef<'a> {}
impl ElementStruct for Behavior {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Behavior::Itself(x) => x.owned_relationship(),
            Behavior::Function(x) => x.owned_relationship(),
            Behavior::Interaction(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Behavior::Itself(x) => x.owning_relationship(),
            Behavior::Function(x) => x.owning_relationship(),
            Behavior::Interaction(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Behavior::Itself(x) => x.element_id(),
            Behavior::Function(x) => x.element_id(),
            Behavior::Interaction(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Behavior::Itself(x) => x.alias_ids(),
            Behavior::Function(x) => x.alias_ids(),
            Behavior::Interaction(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_short_name(),
            Behavior::Function(x) => x.declared_short_name(),
            Behavior::Interaction(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_name(),
            Behavior::Function(x) => x.declared_name(),
            Behavior::Interaction(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Behavior::Itself(x) => x.is_implied_included(),
            Behavior::Function(x) => x.is_implied_included(),
            Behavior::Interaction(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Behavior {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Behavior::Itself(x) => x.owned_relationship_ref_mut(),
            Behavior::Function(x) => x.owned_relationship_ref_mut(),
            Behavior::Interaction(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Behavior::Itself(x) => x.owning_relationship_ref_mut(),
            Behavior::Function(x) => x.owning_relationship_ref_mut(),
            Behavior::Interaction(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Behavior::Itself(x) => x.element_id_ref_mut(),
            Behavior::Function(x) => x.element_id_ref_mut(),
            Behavior::Interaction(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Behavior::Itself(x) => x.alias_ids_ref_mut(),
            Behavior::Function(x) => x.alias_ids_ref_mut(),
            Behavior::Interaction(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_short_name_ref_mut(),
            Behavior::Function(x) => x.declared_short_name_ref_mut(),
            Behavior::Interaction(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_name_ref_mut(),
            Behavior::Function(x) => x.declared_name_ref_mut(),
            Behavior::Interaction(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Behavior::Itself(x) => x.is_implied_included_ref_mut(),
            Behavior::Function(x) => x.is_implied_included_ref_mut(),
            Behavior::Interaction(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Behavior {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Behavior::Itself(x) => x.owned_relationship_ref(),
            Behavior::Function(x) => x.owned_relationship_ref(),
            Behavior::Interaction(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Behavior::Itself(x) => x.owning_relationship_ref(),
            Behavior::Function(x) => x.owning_relationship_ref(),
            Behavior::Interaction(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Behavior::Itself(x) => x.element_id_ref(),
            Behavior::Function(x) => x.element_id_ref(),
            Behavior::Interaction(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Behavior::Itself(x) => x.alias_ids_ref(),
            Behavior::Function(x) => x.alias_ids_ref(),
            Behavior::Interaction(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_short_name_ref(),
            Behavior::Function(x) => x.declared_short_name_ref(),
            Behavior::Interaction(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Behavior::Itself(x) => x.declared_name_ref(),
            Behavior::Function(x) => x.declared_name_ref(),
            Behavior::Interaction(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Behavior::Itself(x) => x.is_implied_included_ref(),
            Behavior::Function(x) => x.is_implied_included_ref(),
            Behavior::Interaction(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for BehaviorRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BehaviorRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            BehaviorRefMut::Function(x) => x.owned_relationship_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            BehaviorRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            BehaviorRefMut::Function(x) => x.owning_relationship_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            BehaviorRefMut::Itself(x) => x.element_id_ref_mut(),
            BehaviorRefMut::Function(x) => x.element_id_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.alias_ids_ref_mut(),
            BehaviorRefMut::Function(x) => x.alias_ids_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            BehaviorRefMut::Function(x) => x.declared_short_name_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.declared_name_ref_mut(),
            BehaviorRefMut::Function(x) => x.declared_name_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            BehaviorRefMut::Function(x) => x.is_implied_included_ref_mut(),
            BehaviorRefMut::Interaction(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for BehaviorRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BehaviorRefMut::Itself(x) => x.owned_relationship_ref(),
            BehaviorRefMut::Function(x) => x.owned_relationship_ref(),
            BehaviorRefMut::Interaction(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BehaviorRefMut::Itself(x) => x.owning_relationship_ref(),
            BehaviorRefMut::Function(x) => x.owning_relationship_ref(),
            BehaviorRefMut::Interaction(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BehaviorRefMut::Itself(x) => x.element_id_ref(),
            BehaviorRefMut::Function(x) => x.element_id_ref(),
            BehaviorRefMut::Interaction(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.alias_ids_ref(),
            BehaviorRefMut::Function(x) => x.alias_ids_ref(),
            BehaviorRefMut::Interaction(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.declared_short_name_ref(),
            BehaviorRefMut::Function(x) => x.declared_short_name_ref(),
            BehaviorRefMut::Interaction(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BehaviorRefMut::Itself(x) => x.declared_name_ref(),
            BehaviorRefMut::Function(x) => x.declared_name_ref(),
            BehaviorRefMut::Interaction(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BehaviorRefMut::Itself(x) => x.is_implied_included_ref(),
            BehaviorRefMut::Function(x) => x.is_implied_included_ref(),
            BehaviorRefMut::Interaction(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for BehaviorRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BehaviorRef::Itself(x) => x.owned_relationship_ref(),
            BehaviorRef::Function(x) => x.owned_relationship_ref(),
            BehaviorRef::Interaction(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BehaviorRef::Itself(x) => x.owning_relationship_ref(),
            BehaviorRef::Function(x) => x.owning_relationship_ref(),
            BehaviorRef::Interaction(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BehaviorRef::Itself(x) => x.element_id_ref(),
            BehaviorRef::Function(x) => x.element_id_ref(),
            BehaviorRef::Interaction(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BehaviorRef::Itself(x) => x.alias_ids_ref(),
            BehaviorRef::Function(x) => x.alias_ids_ref(),
            BehaviorRef::Interaction(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BehaviorRef::Itself(x) => x.declared_short_name_ref(),
            BehaviorRef::Function(x) => x.declared_short_name_ref(),
            BehaviorRef::Interaction(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BehaviorRef::Itself(x) => x.declared_name_ref(),
            BehaviorRef::Function(x) => x.declared_name_ref(),
            BehaviorRef::Interaction(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BehaviorRef::Itself(x) => x.is_implied_included_ref(),
            BehaviorRef::Function(x) => x.is_implied_included_ref(),
            BehaviorRef::Interaction(x) => x.is_implied_included_ref(),
        }
    }
}
impl BehaviorUpcast for Behavior {
    fn into_behavior(self) -> Behavior {
        self
    }
}
impl<'a> BehaviorUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_behavior_ref_mut(self) -> BehaviorRefMut<'a> {
        self
    }
}
impl<'a> BehaviorUpcastRef<'a> for BehaviorRef<'a> {
    fn as_behavior_ref(self) -> BehaviorRef<'a> {
        self
    }
}
impl ClassUpcast for Behavior {
    fn into_class(self) -> Class {
        Class::Behavior(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        ClassRefMut::Behavior(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for BehaviorRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        ClassRef::Behavior(self).as_class_ref()
    }
}
impl ClassifierUpcast for Behavior {
    fn into_classifier(self) -> Classifier {
        Class::Behavior(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        ClassRefMut::Behavior(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for BehaviorRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        ClassRef::Behavior(self).as_classifier_ref()
    }
}
impl TypeUpcast for Behavior {
    fn into_type_(self) -> Type {
        Class::Behavior(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ClassRefMut::Behavior(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for BehaviorRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ClassRef::Behavior(self).as_type_ref()
    }
}
impl NamespaceUpcast for Behavior {
    fn into_namespace(self) -> Namespace {
        Class::Behavior(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ClassRefMut::Behavior(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for BehaviorRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ClassRef::Behavior(self).as_namespace_ref()
    }
}
impl ElementUpcast for Behavior {
    fn into_element(self) -> Element {
        Class::Behavior(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for BehaviorRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ClassRefMut::Behavior(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for BehaviorRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ClassRef::Behavior(self).as_element_ref()
    }
}
pub trait BehaviorDowncast {
    fn try_into_function(self) -> Result<Function, String>;
    fn try_into_interaction(self) -> Result<Interaction, String>;
}
pub trait BehaviorDowncastRefMut<'a> {
    fn try_as_function_ref_mut(self) -> Result<FunctionRefMut<'a>, String>;
    fn try_as_interaction_ref_mut(self) -> Result<InteractionRefMut<'a>, String>;
}
pub trait BehaviorDowncastRef<'a> {
    fn try_as_function_ref(self) -> Result<FunctionRef<'a>, String>;
    fn try_as_interaction_ref(self) -> Result<InteractionRef<'a>, String>;
}
impl BehaviorDowncast for Behavior {
    fn try_into_function(self) -> Result<Function, String> {
        match self {
            Behavior::Function(e) => Ok(e),
            _ => Err("Not a Function".into()),
        }
    }
    fn try_into_interaction(self) -> Result<Interaction, String> {
        match self {
            Behavior::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
}
impl<'a> BehaviorDowncastRefMut<'a> for BehaviorRefMut<'a> {
    fn try_as_function_ref_mut(self) -> Result<FunctionRefMut<'a>, String> {
        match self {
            BehaviorRefMut::Function(e) => Ok(e),
            _ => Err("Not a Function".into()),
        }
    }
    fn try_as_interaction_ref_mut(self) -> Result<InteractionRefMut<'a>, String> {
        match self {
            BehaviorRefMut::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
}
impl<'a> BehaviorDowncastRef<'a> for BehaviorRef<'a> {
    fn try_as_function_ref(self) -> Result<FunctionRef<'a>, String> {
        match self {
            BehaviorRef::Function(e) => Ok(e),
            _ => Err("Not a Function".into()),
        }
    }
    fn try_as_interaction_ref(self) -> Result<InteractionRef<'a>, String> {
        match self {
            BehaviorRef::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
}
pub trait BehaviorMethodsDescendants
where
    Self: DescendantOf<Behavior>,
    Self::Via: BehaviorMethods,
    Self: Sized,
{}
pub trait BehaviorMethods: Sized {}
impl<T: BehaviorMethodsDescendants> BehaviorMethods for T
where
    T::Via: BehaviorMethods,
{}
impl DescendantOf<Class> for Behavior {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ClassMethodsDescendants for Behavior {}
impl DescendantOf<Classifier> for Behavior {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ClassifierMethodsDescendants for Behavior {}
impl DescendantOf<Type> for Behavior {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl TypeMethodsDescendants for Behavior {}
impl DescendantOf<Namespace> for Behavior {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl NamespaceMethodsDescendants for Behavior {}
impl DescendantOf<Element> for Behavior {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ElementMethodsDescendants for Behavior {}
pub trait BehaviorRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<BehaviorRefMut<'a>>,
    Self::Via: BehaviorRefMutMethods,
    Self: Sized,
{}
pub trait BehaviorRefMutMethods: Sized {}
impl<'a, T: BehaviorRefMutMethodsDescendants<'a>> BehaviorRefMutMethods for T
where
    T::Via: BehaviorRefMutMethods,
{}
impl<'a> DescendantOf<ClassRefMut<'a>> for BehaviorRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for BehaviorRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for BehaviorRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for BehaviorRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for BehaviorRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for BehaviorRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for BehaviorRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for BehaviorRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for BehaviorRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for BehaviorRefMut<'a> {}
pub trait BehaviorRefMethodsDescendants<'a>
where
    Self: DescendantOf<BehaviorRef<'a>>,
    Self::Via: BehaviorRefMethods,
    Self: Sized,
{}
pub trait BehaviorRefMethods: Sized {}
impl<'a, T: BehaviorRefMethodsDescendants<'a>> BehaviorRefMethods for T
where
    T::Via: BehaviorRefMethods,
{}
impl<'a> DescendantOf<ClassRef<'a>> for BehaviorRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for BehaviorRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for BehaviorRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for BehaviorRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for BehaviorRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for BehaviorRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for BehaviorRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for BehaviorRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for BehaviorRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for BehaviorRef<'a> {}

