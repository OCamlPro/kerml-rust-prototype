#![allow(unused)]
use super::utils::DescendantOf;
use super::specialization::{
    Specialization, SpecializationRefMut, SpecializationRef, SpecializationStruct,
    SpecializationStructRefMut, SpecializationStructRef, SpecializationInner,
    SpecializationUpcast, SpecializationUpcastRefMut, SpecializationUpcastRef,
    SpecializationMethodsDescendants, SpecializationRefMutMethodsDescendants,
    SpecializationRefMethodsDescendants,
};
use super::relationship::{
    Relationship, RelationshipRefMut, RelationshipRef, RelationshipStruct,
    RelationshipStructRefMut, RelationshipStructRef, RelationshipInner,
    RelationshipUpcast, RelationshipUpcastRefMut, RelationshipUpcastRef,
    RelationshipMethodsDescendants, RelationshipRefMutMethodsDescendants,
    RelationshipRefMethodsDescendants,
};
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
pub struct FeatureTypingInner {
    pub(super) sup_specialization: SpecializationInner,
    pub(super) typed_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    pub(super) type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait FeatureTypingStruct
where
    Self: FeatureTypingStructRefMut,
    Self: FeatureTypingStructRef,
    Self: SpecializationStruct,
{
    fn typed_feature(self) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn type_(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait FeatureTypingStructRefMut
where
    Self: FeatureTypingStructRef,
    Self: SpecializationStructRefMut,
{
    fn typed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait FeatureTypingStructRef
where
    Self: SpecializationStructRef,
{
    fn typed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait FeatureTypingUpcast: FeatureTypingStruct {
    fn into_feature_typing(self) -> FeatureTyping;
}
pub trait FeatureTypingUpcastRefMut<'a>: FeatureTypingStructRefMut {
    fn as_feature_typing_ref_mut(self) -> FeatureTypingRefMut<'a>;
}
pub trait FeatureTypingUpcastRef<'a>: FeatureTypingStructRef {
    fn as_feature_typing_ref(self) -> FeatureTypingRef<'a>;
}
impl FeatureTypingStruct for FeatureTypingInner {
    fn typed_feature(self) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.typed_feature
    }
    fn type_(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.type_
    }
}
impl FeatureTypingStructRefMut for FeatureTypingInner {
    fn typed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.typed_feature
    }
    fn type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.type_
    }
}
impl FeatureTypingStructRef for FeatureTypingInner {
    fn typed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.typed_feature
    }
    fn type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.type_
    }
}
impl SpecializationStruct for FeatureTypingInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific()
    }
}
impl SpecializationStructRefMut for FeatureTypingInner {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general_ref_mut()
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific_ref_mut()
    }
}
impl SpecializationStructRef for FeatureTypingInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific_ref()
    }
}
impl RelationshipStruct for FeatureTypingInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_specialization.is_implied()
    }
}
impl RelationshipStructRefMut for FeatureTypingInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_specialization.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for FeatureTypingInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_specialization.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_specialization.is_implied_ref()
    }
}
impl ElementStruct for FeatureTypingInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_specialization.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_specialization.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_specialization.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_specialization.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_specialization.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_specialization.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_specialization.is_implied_included()
    }
}
impl ElementStructRefMut for FeatureTypingInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_specialization.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_specialization.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_specialization.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_specialization.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_specialization.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_specialization.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_specialization.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FeatureTypingInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_specialization.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_specialization.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_specialization.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_specialization.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_specialization.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_specialization.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_specialization.is_implied_included_ref()
    }
}
pub enum FeatureTyping {
    Itself(FeatureTypingInner),
}
pub enum FeatureTypingRefMut<'a> {
    Itself(&'a mut FeatureTypingInner),
}
pub enum FeatureTypingRef<'a> {
    Itself(&'a FeatureTypingInner),
}
impl FeatureTyping {
    pub fn as_ref(&self) -> FeatureTypingRef {
        match self {
            FeatureTyping::Itself(inner) => FeatureTypingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureTypingRefMut {
        match self {
            FeatureTyping::Itself(inner) => FeatureTypingRefMut::Itself(inner),
        }
    }
}
impl<'a> FeatureTypingRefMut<'a> {
    pub fn as_ref(self) -> FeatureTypingRef<'a> {
        match self {
            FeatureTypingRefMut::Itself(inner) => FeatureTypingRef::Itself(inner),
        }
    }
}
impl FeatureTypingStruct for FeatureTyping {
    fn typed_feature(self) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTyping::Itself(x) => x.typed_feature(),
        }
    }
    fn type_(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.type_(),
        }
    }
}
impl FeatureTypingStructRefMut for FeatureTyping {
    fn typed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTyping::Itself(x) => x.typed_feature_ref_mut(),
        }
    }
    fn type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.type_ref_mut(),
        }
    }
}
impl FeatureTypingStructRef for FeatureTyping {
    fn typed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTyping::Itself(x) => x.typed_feature_ref(),
        }
    }
    fn type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.type_ref(),
        }
    }
}
impl<'a> FeatureTypingStructRefMut for FeatureTypingRefMut<'a> {
    fn typed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.typed_feature_ref_mut(),
        }
    }
    fn type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.type_ref_mut(),
        }
    }
}
impl<'a> FeatureTypingStructRef for FeatureTypingRefMut<'a> {
    fn typed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.typed_feature_ref(),
        }
    }
    fn type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.type_ref(),
        }
    }
}
impl<'a> FeatureTypingStructRef for FeatureTypingRef<'a> {
    fn typed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureTypingRef::Itself(x) => x.typed_feature_ref(),
        }
    }
    fn type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRef::Itself(x) => x.type_ref(),
        }
    }
}
impl SpecializationStruct for FeatureTyping {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for FeatureTyping {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for FeatureTyping {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTyping::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for FeatureTypingRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for FeatureTypingRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for FeatureTypingRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRef::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            FeatureTypingRef::Itself(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for FeatureTyping {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for FeatureTyping {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for FeatureTyping {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FeatureTypingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureTypingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureTypingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureTypingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureTypingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureTypingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for FeatureTyping {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTyping::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureTyping::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureTyping::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureTyping {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureTyping::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureTyping::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureTyping::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureTyping {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTyping::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTyping::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureTyping::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureTyping::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTyping::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureTyping::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureTypingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureTypingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureTypingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureTypingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureTypingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTypingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureTypingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureTypingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureTypingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureTypingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureTypingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTypingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureTypingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureTypingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureTypingUpcast for FeatureTyping {
    fn into_feature_typing(self) -> FeatureTyping {
        self
    }
}
impl<'a> FeatureTypingUpcastRefMut<'a> for FeatureTypingRefMut<'a> {
    fn as_feature_typing_ref_mut(self) -> FeatureTypingRefMut<'a> {
        self
    }
}
impl<'a> FeatureTypingUpcastRef<'a> for FeatureTypingRef<'a> {
    fn as_feature_typing_ref(self) -> FeatureTypingRef<'a> {
        self
    }
}
impl SpecializationUpcast for FeatureTyping {
    fn into_specialization(self) -> Specialization {
        Specialization::FeatureTyping(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for FeatureTypingRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SpecializationRefMut::FeatureTyping(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for FeatureTypingRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SpecializationRef::FeatureTyping(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for FeatureTyping {
    fn into_relationship(self) -> Relationship {
        Specialization::FeatureTyping(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FeatureTypingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SpecializationRefMut::FeatureTyping(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FeatureTypingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SpecializationRef::FeatureTyping(self).as_relationship_ref()
    }
}
impl ElementUpcast for FeatureTyping {
    fn into_element(self) -> Element {
        Specialization::FeatureTyping(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureTypingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SpecializationRefMut::FeatureTyping(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureTypingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SpecializationRef::FeatureTyping(self).as_element_ref()
    }
}
pub trait FeatureTypingDowncast {}
pub trait FeatureTypingDowncastRefMut<'a> {}
pub trait FeatureTypingDowncastRef<'a> {}
impl FeatureTypingDowncast for FeatureTyping {}
impl<'a> FeatureTypingDowncastRefMut<'a> for FeatureTypingRefMut<'a> {}
impl<'a> FeatureTypingDowncastRef<'a> for FeatureTypingRef<'a> {}
pub trait FeatureTypingMethodsDescendants
where
    Self: DescendantOf<FeatureTyping>,
    Self::Via: FeatureTypingMethods,
    Self: Sized,
{}
pub trait FeatureTypingMethods: Sized {}
impl<T: FeatureTypingMethodsDescendants> FeatureTypingMethods for T
where
    T::Via: FeatureTypingMethods,
{}
impl DescendantOf<Specialization> for FeatureTyping {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl SpecializationMethodsDescendants for FeatureTyping {}
impl DescendantOf<Relationship> for FeatureTyping {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl RelationshipMethodsDescendants for FeatureTyping {}
impl DescendantOf<Element> for FeatureTyping {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl ElementMethodsDescendants for FeatureTyping {}
pub trait FeatureTypingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureTypingRefMut<'a>>,
    Self::Via: FeatureTypingRefMutMethods,
    Self: Sized,
{}
pub trait FeatureTypingRefMutMethods: Sized {}
impl<'a, T: FeatureTypingRefMutMethodsDescendants<'a>> FeatureTypingRefMutMethods for T
where
    T::Via: FeatureTypingRefMutMethods,
{}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for FeatureTypingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for FeatureTypingRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FeatureTypingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FeatureTypingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureTypingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureTypingRefMut<'a> {}
pub trait FeatureTypingRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureTypingRef<'a>>,
    Self::Via: FeatureTypingRefMethods,
    Self: Sized,
{}
pub trait FeatureTypingRefMethods: Sized {}
impl<'a, T: FeatureTypingRefMethodsDescendants<'a>> FeatureTypingRefMethods for T
where
    T::Via: FeatureTypingRefMethods,
{}
impl<'a> DescendantOf<SpecializationRef<'a>> for FeatureTypingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for FeatureTypingRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for FeatureTypingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FeatureTypingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureTypingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureTypingRef<'a> {}

