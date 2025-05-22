#![allow(unused)]
use super::utils::DescendantOf;
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
pub struct FeatureInvertingInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) feature_inverted: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
    pub(super) inverting_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
}
pub trait FeatureInvertingStruct
where
    Self: FeatureInvertingStructRefMut,
    Self: FeatureInvertingStructRef,
    Self: RelationshipStruct,
{
    fn feature_inverted(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn inverting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureInvertingStructRefMut
where
    Self: FeatureInvertingStructRef,
    Self: RelationshipStructRefMut,
{
    fn feature_inverted_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn inverting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureInvertingStructRef
where
    Self: RelationshipStructRef,
{
    fn feature_inverted_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn inverting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureInvertingUpcast: FeatureInvertingStruct {
    fn into_feature_inverting(self) -> FeatureInverting;
}
pub trait FeatureInvertingUpcastRefMut<'a>: FeatureInvertingStructRefMut {
    fn as_feature_inverting_ref_mut(self) -> FeatureInvertingRefMut<'a>;
}
pub trait FeatureInvertingUpcastRef<'a>: FeatureInvertingStructRef {
    fn as_feature_inverting_ref(self) -> FeatureInvertingRef<'a>;
}
impl FeatureInvertingStruct for FeatureInvertingInner {
    fn feature_inverted(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.feature_inverted
    }
    fn inverting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.inverting_feature
    }
}
impl FeatureInvertingStructRefMut for FeatureInvertingInner {
    fn feature_inverted_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.feature_inverted
    }
    fn inverting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.inverting_feature
    }
}
impl FeatureInvertingStructRef for FeatureInvertingInner {
    fn feature_inverted_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.feature_inverted
    }
    fn inverting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.inverting_feature
    }
}
impl RelationshipStruct for FeatureInvertingInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_relationship.is_implied()
    }
}
impl RelationshipStructRefMut for FeatureInvertingInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_relationship.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for FeatureInvertingInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_relationship.is_implied_ref()
    }
}
impl ElementStruct for FeatureInvertingInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_relationship.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_relationship.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_relationship.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_relationship.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_relationship.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_relationship.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_relationship.is_implied_included()
    }
}
impl ElementStructRefMut for FeatureInvertingInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_relationship.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_relationship.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_relationship.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_relationship.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_relationship.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_relationship.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_relationship.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FeatureInvertingInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_relationship.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_relationship.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_relationship.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_relationship.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_relationship.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_relationship.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_relationship.is_implied_included_ref()
    }
}
pub enum FeatureInverting {
    Itself(FeatureInvertingInner),
}
pub enum FeatureInvertingRefMut<'a> {
    Itself(&'a mut FeatureInvertingInner),
}
pub enum FeatureInvertingRef<'a> {
    Itself(&'a FeatureInvertingInner),
}
impl FeatureInverting {
    pub fn as_ref(&self) -> FeatureInvertingRef {
        match self {
            FeatureInverting::Itself(inner) => FeatureInvertingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureInvertingRefMut {
        match self {
            FeatureInverting::Itself(inner) => FeatureInvertingRefMut::Itself(inner),
        }
    }
}
impl<'a> FeatureInvertingRefMut<'a> {
    pub fn as_ref(self) -> FeatureInvertingRef<'a> {
        match self {
            FeatureInvertingRefMut::Itself(inner) => FeatureInvertingRef::Itself(inner),
        }
    }
}
impl FeatureInvertingStruct for FeatureInverting {
    fn feature_inverted(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.feature_inverted(),
        }
    }
    fn inverting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.inverting_feature(),
        }
    }
}
impl FeatureInvertingStructRefMut for FeatureInverting {
    fn feature_inverted_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.feature_inverted_ref_mut(),
        }
    }
    fn inverting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.inverting_feature_ref_mut(),
        }
    }
}
impl FeatureInvertingStructRef for FeatureInverting {
    fn feature_inverted_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.feature_inverted_ref(),
        }
    }
    fn inverting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInverting::Itself(x) => x.inverting_feature_ref(),
        }
    }
}
impl<'a> FeatureInvertingStructRefMut for FeatureInvertingRefMut<'a> {
    fn feature_inverted_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.feature_inverted_ref_mut(),
        }
    }
    fn inverting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.inverting_feature_ref_mut(),
        }
    }
}
impl<'a> FeatureInvertingStructRef for FeatureInvertingRefMut<'a> {
    fn feature_inverted_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.feature_inverted_ref(),
        }
    }
    fn inverting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.inverting_feature_ref(),
        }
    }
}
impl<'a> FeatureInvertingStructRef for FeatureInvertingRef<'a> {
    fn feature_inverted_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.feature_inverted_ref(),
        }
    }
    fn inverting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.inverting_feature_ref(),
        }
    }
}
impl RelationshipStruct for FeatureInverting {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for FeatureInverting {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for FeatureInverting {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FeatureInvertingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureInvertingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureInvertingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureInvertingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for FeatureInverting {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInverting::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureInverting::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureInverting::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureInverting {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureInverting::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureInverting::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureInverting::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureInverting {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInverting::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInverting::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureInverting::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureInverting::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInverting::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureInverting::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureInvertingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureInvertingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureInvertingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureInvertingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureInvertingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureInvertingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureInvertingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInvertingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureInvertingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureInvertingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureInvertingUpcast for FeatureInverting {
    fn into_feature_inverting(self) -> FeatureInverting {
        self
    }
}
impl<'a> FeatureInvertingUpcastRefMut<'a> for FeatureInvertingRefMut<'a> {
    fn as_feature_inverting_ref_mut(self) -> FeatureInvertingRefMut<'a> {
        self
    }
}
impl<'a> FeatureInvertingUpcastRef<'a> for FeatureInvertingRef<'a> {
    fn as_feature_inverting_ref(self) -> FeatureInvertingRef<'a> {
        self
    }
}
impl RelationshipUpcast for FeatureInverting {
    fn into_relationship(self) -> Relationship {
        Relationship::FeatureInverting(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FeatureInvertingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::FeatureInverting(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FeatureInvertingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::FeatureInverting(self).as_relationship_ref()
    }
}
impl ElementUpcast for FeatureInverting {
    fn into_element(self) -> Element {
        Relationship::FeatureInverting(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureInvertingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::FeatureInverting(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureInvertingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::FeatureInverting(self).as_element_ref()
    }
}
pub trait FeatureInvertingDowncast {}
pub trait FeatureInvertingDowncastRefMut<'a> {}
pub trait FeatureInvertingDowncastRef<'a> {}
impl FeatureInvertingDowncast for FeatureInverting {}
impl<'a> FeatureInvertingDowncastRefMut<'a> for FeatureInvertingRefMut<'a> {}
impl<'a> FeatureInvertingDowncastRef<'a> for FeatureInvertingRef<'a> {}
pub trait FeatureInvertingMethodsDescendants
where
    Self: DescendantOf<FeatureInverting>,
    Self::Via: FeatureInvertingMethods,
    Self: Sized,
{}
pub trait FeatureInvertingMethods: Sized {}
impl<T: FeatureInvertingMethodsDescendants> FeatureInvertingMethods for T
where
    T::Via: FeatureInvertingMethods,
{}
impl DescendantOf<Relationship> for FeatureInverting {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for FeatureInverting {}
impl DescendantOf<Element> for FeatureInverting {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for FeatureInverting {}
pub trait FeatureInvertingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureInvertingRefMut<'a>>,
    Self::Via: FeatureInvertingRefMutMethods,
    Self: Sized,
{}
pub trait FeatureInvertingRefMutMethods: Sized {}
impl<'a, T: FeatureInvertingRefMutMethodsDescendants<'a>> FeatureInvertingRefMutMethods
for T
where
    T::Via: FeatureInvertingRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FeatureInvertingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FeatureInvertingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureInvertingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureInvertingRefMut<'a> {}
pub trait FeatureInvertingRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureInvertingRef<'a>>,
    Self::Via: FeatureInvertingRefMethods,
    Self: Sized,
{}
pub trait FeatureInvertingRefMethods: Sized {}
impl<'a, T: FeatureInvertingRefMethodsDescendants<'a>> FeatureInvertingRefMethods for T
where
    T::Via: FeatureInvertingRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for FeatureInvertingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FeatureInvertingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureInvertingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureInvertingRef<'a> {}

