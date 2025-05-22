#![allow(unused)]
use super::utils::DescendantOf;
use super::association::{
    Association, AssociationRefMut, AssociationRef, AssociationStruct,
    AssociationStructRefMut, AssociationStructRef, AssociationInner, AssociationUpcast,
    AssociationUpcastRefMut, AssociationUpcastRef, AssociationMethodsDescendants,
    AssociationRefMutMethodsDescendants, AssociationRefMethodsDescendants,
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
use super::relationship::{
    Relationship, RelationshipRefMut, RelationshipRef, RelationshipStruct,
    RelationshipStructRefMut, RelationshipStructRef, RelationshipInner,
    RelationshipUpcast, RelationshipUpcastRefMut, RelationshipUpcastRef,
    RelationshipMethodsDescendants, RelationshipRefMutMethodsDescendants,
    RelationshipRefMethodsDescendants,
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
pub struct InteractionInner {
    pub(super) sup_behavior: BehaviorInner,
    pub(super) sup_association: AssociationInner,
}
pub trait InteractionStruct
where
    Self: InteractionStructRefMut,
    Self: InteractionStructRef,
    Self: BehaviorStruct,
    Self: AssociationStruct,
{}
pub trait InteractionStructRefMut
where
    Self: InteractionStructRef,
    Self: BehaviorStructRefMut,
    Self: AssociationStructRefMut,
{}
pub trait InteractionStructRef
where
    Self: BehaviorStructRef,
    Self: AssociationStructRef,
{}
pub trait InteractionUpcast: InteractionStruct {
    fn into_interaction(self) -> Interaction;
}
pub trait InteractionUpcastRefMut<'a>: InteractionStructRefMut {
    fn as_interaction_ref_mut(self) -> InteractionRefMut<'a>;
}
pub trait InteractionUpcastRef<'a>: InteractionStructRef {
    fn as_interaction_ref(self) -> InteractionRef<'a>;
}
impl InteractionStruct for InteractionInner {}
impl InteractionStructRefMut for InteractionInner {}
impl InteractionStructRef for InteractionInner {}
impl AssociationStruct for InteractionInner {}
impl AssociationStructRefMut for InteractionInner {}
impl AssociationStructRef for InteractionInner {}
impl ClassifierStruct for InteractionInner {}
impl ClassifierStructRefMut for InteractionInner {}
impl ClassifierStructRef for InteractionInner {}
impl TypeStruct for InteractionInner {
    fn is_abstract(self) -> bool {
        self.sup_association.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_association.is_sufficient()
    }
}
impl TypeStructRefMut for InteractionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for InteractionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_association.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_association.is_sufficient_ref()
    }
}
impl NamespaceStruct for InteractionInner {}
impl NamespaceStructRefMut for InteractionInner {}
impl NamespaceStructRef for InteractionInner {}
impl ElementStruct for InteractionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_association.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_association.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_association.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_association.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_association.is_implied_included()
    }
}
impl ElementStructRefMut for InteractionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_association.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_association.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_association.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_association.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_association.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for InteractionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_association.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_association.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_association.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_association.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_association.is_implied_included_ref()
    }
}
impl RelationshipStruct for InteractionInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_association.is_implied()
    }
}
impl RelationshipStructRefMut for InteractionInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for InteractionInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_association.is_implied_ref()
    }
}
impl BehaviorStruct for InteractionInner {}
impl BehaviorStructRefMut for InteractionInner {}
impl BehaviorStructRef for InteractionInner {}
impl ClassStruct for InteractionInner {}
impl ClassStructRefMut for InteractionInner {}
impl ClassStructRef for InteractionInner {}
pub enum Interaction {
    Itself(InteractionInner),
}
pub enum InteractionRefMut<'a> {
    Itself(&'a mut InteractionInner),
}
pub enum InteractionRef<'a> {
    Itself(&'a InteractionInner),
}
impl Interaction {
    pub fn as_ref(&self) -> InteractionRef {
        match self {
            Interaction::Itself(inner) => InteractionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> InteractionRefMut {
        match self {
            Interaction::Itself(inner) => InteractionRefMut::Itself(inner),
        }
    }
}
impl<'a> InteractionRefMut<'a> {
    pub fn as_ref(self) -> InteractionRef<'a> {
        match self {
            InteractionRefMut::Itself(inner) => InteractionRef::Itself(inner),
        }
    }
}
impl InteractionStruct for Interaction {}
impl InteractionStructRefMut for Interaction {}
impl InteractionStructRef for Interaction {}
impl<'a> InteractionStructRefMut for InteractionRefMut<'a> {}
impl<'a> InteractionStructRef for InteractionRefMut<'a> {}
impl<'a> InteractionStructRef for InteractionRef<'a> {}
impl AssociationStruct for Interaction {}
impl AssociationStructRefMut for Interaction {}
impl AssociationStructRef for Interaction {}
impl<'a> AssociationStructRefMut for InteractionRefMut<'a> {}
impl<'a> AssociationStructRef for InteractionRefMut<'a> {}
impl<'a> AssociationStructRef for InteractionRef<'a> {}
impl ClassifierStruct for Interaction {}
impl ClassifierStructRefMut for Interaction {}
impl ClassifierStructRef for Interaction {}
impl<'a> ClassifierStructRefMut for InteractionRefMut<'a> {}
impl<'a> ClassifierStructRef for InteractionRefMut<'a> {}
impl<'a> ClassifierStructRef for InteractionRef<'a> {}
impl TypeStruct for Interaction {
    fn is_abstract(self) -> bool {
        match self {
            Interaction::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Interaction::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Interaction {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Interaction::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Interaction::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Interaction {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Interaction::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Interaction::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for InteractionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for InteractionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for InteractionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InteractionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InteractionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Interaction {}
impl NamespaceStructRefMut for Interaction {}
impl NamespaceStructRef for Interaction {}
impl<'a> NamespaceStructRefMut for InteractionRefMut<'a> {}
impl<'a> NamespaceStructRef for InteractionRefMut<'a> {}
impl<'a> NamespaceStructRef for InteractionRef<'a> {}
impl ElementStruct for Interaction {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Interaction::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Interaction::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Interaction::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Interaction::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Interaction::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Interaction {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Interaction::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Interaction::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Interaction::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Interaction::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Interaction::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Interaction {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Interaction::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Interaction::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Interaction::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Interaction::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Interaction::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Interaction::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for InteractionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            InteractionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InteractionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InteractionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InteractionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InteractionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for InteractionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InteractionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InteractionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InteractionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InteractionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for InteractionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InteractionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InteractionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InteractionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InteractionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InteractionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InteractionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InteractionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for Interaction {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Interaction::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Interaction {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Interaction::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Interaction {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Interaction::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Interaction::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for InteractionRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for InteractionRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            InteractionRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for InteractionRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            InteractionRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            InteractionRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl BehaviorStruct for Interaction {}
impl BehaviorStructRefMut for Interaction {}
impl BehaviorStructRef for Interaction {}
impl<'a> BehaviorStructRefMut for InteractionRefMut<'a> {}
impl<'a> BehaviorStructRef for InteractionRefMut<'a> {}
impl<'a> BehaviorStructRef for InteractionRef<'a> {}
impl ClassStruct for Interaction {}
impl ClassStructRefMut for Interaction {}
impl ClassStructRef for Interaction {}
impl<'a> ClassStructRefMut for InteractionRefMut<'a> {}
impl<'a> ClassStructRef for InteractionRefMut<'a> {}
impl<'a> ClassStructRef for InteractionRef<'a> {}
impl InteractionUpcast for Interaction {
    fn into_interaction(self) -> Interaction {
        self
    }
}
impl<'a> InteractionUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_interaction_ref_mut(self) -> InteractionRefMut<'a> {
        self
    }
}
impl<'a> InteractionUpcastRef<'a> for InteractionRef<'a> {
    fn as_interaction_ref(self) -> InteractionRef<'a> {
        self
    }
}
impl AssociationUpcast for Interaction {
    fn into_association(self) -> Association {
        Association::Interaction(self).into_association()
    }
}
impl<'a> AssociationUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_association_ref_mut(self) -> AssociationRefMut<'a> {
        AssociationRefMut::Interaction(self).as_association_ref_mut()
    }
}
impl<'a> AssociationUpcastRef<'a> for InteractionRef<'a> {
    fn as_association_ref(self) -> AssociationRef<'a> {
        AssociationRef::Interaction(self).as_association_ref()
    }
}
impl ClassifierUpcast for Interaction {
    fn into_classifier(self) -> Classifier {
        Association::Interaction(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        AssociationRefMut::Interaction(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for InteractionRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        AssociationRef::Interaction(self).as_classifier_ref()
    }
}
impl TypeUpcast for Interaction {
    fn into_type_(self) -> Type {
        Association::Interaction(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        AssociationRefMut::Interaction(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for InteractionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        AssociationRef::Interaction(self).as_type_ref()
    }
}
impl NamespaceUpcast for Interaction {
    fn into_namespace(self) -> Namespace {
        Association::Interaction(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        AssociationRefMut::Interaction(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for InteractionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        AssociationRef::Interaction(self).as_namespace_ref()
    }
}
impl ElementUpcast for Interaction {
    fn into_element(self) -> Element {
        Association::Interaction(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        AssociationRefMut::Interaction(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for InteractionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        AssociationRef::Interaction(self).as_element_ref()
    }
}
impl RelationshipUpcast for Interaction {
    fn into_relationship(self) -> Relationship {
        Association::Interaction(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        AssociationRefMut::Interaction(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for InteractionRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        AssociationRef::Interaction(self).as_relationship_ref()
    }
}
impl BehaviorUpcast for Interaction {
    fn into_behavior(self) -> Behavior {
        Behavior::Interaction(self).into_behavior()
    }
}
impl<'a> BehaviorUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_behavior_ref_mut(self) -> BehaviorRefMut<'a> {
        BehaviorRefMut::Interaction(self).as_behavior_ref_mut()
    }
}
impl<'a> BehaviorUpcastRef<'a> for InteractionRef<'a> {
    fn as_behavior_ref(self) -> BehaviorRef<'a> {
        BehaviorRef::Interaction(self).as_behavior_ref()
    }
}
impl ClassUpcast for Interaction {
    fn into_class(self) -> Class {
        Behavior::Interaction(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for InteractionRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        BehaviorRefMut::Interaction(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for InteractionRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        BehaviorRef::Interaction(self).as_class_ref()
    }
}
pub trait InteractionDowncast {}
pub trait InteractionDowncastRefMut<'a> {}
pub trait InteractionDowncastRef<'a> {}
impl InteractionDowncast for Interaction {}
impl<'a> InteractionDowncastRefMut<'a> for InteractionRefMut<'a> {}
impl<'a> InteractionDowncastRef<'a> for InteractionRef<'a> {}
pub trait InteractionMethodsDescendants
where
    Self: DescendantOf<Interaction>,
    Self::Via: InteractionMethods,
    Self: Sized,
{}
pub trait InteractionMethods: Sized {}
impl<T: InteractionMethodsDescendants> InteractionMethods for T
where
    T::Via: InteractionMethods,
{}
impl DescendantOf<Behavior> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl BehaviorMethodsDescendants for Interaction {}
impl DescendantOf<Class> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ClassMethodsDescendants for Interaction {}
impl DescendantOf<Classifier> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ClassifierMethodsDescendants for Interaction {}
impl DescendantOf<Type> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl TypeMethodsDescendants for Interaction {}
impl DescendantOf<Namespace> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl NamespaceMethodsDescendants for Interaction {}
impl DescendantOf<Element> for Interaction {
    type Via = Behavior;
    fn into_via(self) -> Self::Via {
        self.into_behavior()
    }
}
impl ElementMethodsDescendants for Interaction {}
impl DescendantOf<Association> for Interaction {
    type Via = Association;
    fn into_via(self) -> Self::Via {
        self.into_association()
    }
}
impl AssociationMethodsDescendants for Interaction {}
impl DescendantOf<Relationship> for Interaction {
    type Via = Association;
    fn into_via(self) -> Self::Via {
        self.into_association()
    }
}
impl RelationshipMethodsDescendants for Interaction {}
pub trait InteractionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<InteractionRefMut<'a>>,
    Self::Via: InteractionRefMutMethods,
    Self: Sized,
{}
pub trait InteractionRefMutMethods: Sized {}
impl<'a, T: InteractionRefMutMethodsDescendants<'a>> InteractionRefMutMethods for T
where
    T::Via: InteractionRefMutMethods,
{}
impl<'a> DescendantOf<BehaviorRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> BehaviorRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<ClassRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for InteractionRefMut<'a> {
    type Via = BehaviorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<AssociationRefMut<'a>> for InteractionRefMut<'a> {
    type Via = AssociationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref_mut()
    }
}
impl<'a> AssociationRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for InteractionRefMut<'a> {
    type Via = AssociationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for InteractionRefMut<'a> {}
pub trait InteractionRefMethodsDescendants<'a>
where
    Self: DescendantOf<InteractionRef<'a>>,
    Self::Via: InteractionRefMethods,
    Self: Sized,
{}
pub trait InteractionRefMethods: Sized {}
impl<'a, T: InteractionRefMethodsDescendants<'a>> InteractionRefMethods for T
where
    T::Via: InteractionRefMethods,
{}
impl<'a> DescendantOf<BehaviorRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> BehaviorRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<ClassRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for InteractionRef<'a> {
    type Via = BehaviorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_behavior_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<AssociationRef<'a>> for InteractionRef<'a> {
    type Via = AssociationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref()
    }
}
impl<'a> AssociationRefMethodsDescendants<'a> for InteractionRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for InteractionRef<'a> {
    type Via = AssociationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for InteractionRef<'a> {}

