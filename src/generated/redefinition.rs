#![allow(unused)]
use super::utils::DescendantOf;
use super::subsetting::{
    Subsetting, SubsettingRefMut, SubsettingRef, SubsettingStruct,
    SubsettingStructRefMut, SubsettingStructRef, SubsettingInner, SubsettingUpcast,
    SubsettingUpcastRefMut, SubsettingUpcastRef, SubsettingMethodsDescendants,
    SubsettingRefMutMethodsDescendants, SubsettingRefMethodsDescendants,
};
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
pub struct RedefinitionInner {
    pub(super) sup_subsetting: SubsettingInner,
    pub(super) redefining_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
    pub(super) redefined_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
}
pub trait RedefinitionStruct
where
    Self: RedefinitionStructRefMut,
    Self: RedefinitionStructRef,
    Self: SubsettingStruct,
{
    fn redefining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn redefined_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait RedefinitionStructRefMut
where
    Self: RedefinitionStructRef,
    Self: SubsettingStructRefMut,
{
    fn redefining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn redefined_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait RedefinitionStructRef
where
    Self: SubsettingStructRef,
{
    fn redefining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn redefined_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait RedefinitionUpcast: RedefinitionStruct {
    fn into_redefinition(self) -> Redefinition;
}
pub trait RedefinitionUpcastRefMut<'a>: RedefinitionStructRefMut {
    fn as_redefinition_ref_mut(self) -> RedefinitionRefMut<'a>;
}
pub trait RedefinitionUpcastRef<'a>: RedefinitionStructRef {
    fn as_redefinition_ref(self) -> RedefinitionRef<'a>;
}
impl RedefinitionStruct for RedefinitionInner {
    fn redefining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.redefining_feature
    }
    fn redefined_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.redefined_feature
    }
}
impl RedefinitionStructRefMut for RedefinitionInner {
    fn redefining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.redefining_feature
    }
    fn redefined_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.redefined_feature
    }
}
impl RedefinitionStructRef for RedefinitionInner {
    fn redefining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.redefining_feature
    }
    fn redefined_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.redefined_feature
    }
}
impl SubsettingStruct for RedefinitionInner {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetted_feature()
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetting_feature()
    }
}
impl SubsettingStructRefMut for RedefinitionInner {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetted_feature_ref_mut()
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetting_feature_ref_mut()
    }
}
impl SubsettingStructRef for RedefinitionInner {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetted_feature_ref()
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.sup_subsetting.subsetting_feature_ref()
    }
}
impl SpecializationStruct for RedefinitionInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific()
    }
}
impl SpecializationStructRefMut for RedefinitionInner {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general_ref_mut()
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific_ref_mut()
    }
}
impl SpecializationStructRef for RedefinitionInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific_ref()
    }
}
impl RelationshipStruct for RedefinitionInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_subsetting.is_implied()
    }
}
impl RelationshipStructRefMut for RedefinitionInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_subsetting.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for RedefinitionInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_subsetting.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_subsetting.is_implied_ref()
    }
}
impl ElementStruct for RedefinitionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_subsetting.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_subsetting.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_subsetting.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_subsetting.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_subsetting.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_subsetting.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_subsetting.is_implied_included()
    }
}
impl ElementStructRefMut for RedefinitionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_subsetting.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_subsetting.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_subsetting.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_subsetting.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_subsetting.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_subsetting.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_subsetting.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for RedefinitionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_subsetting.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_subsetting.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_subsetting.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_subsetting.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_subsetting.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_subsetting.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_subsetting.is_implied_included_ref()
    }
}
pub enum Redefinition {
    Itself(RedefinitionInner),
}
pub enum RedefinitionRefMut<'a> {
    Itself(&'a mut RedefinitionInner),
}
pub enum RedefinitionRef<'a> {
    Itself(&'a RedefinitionInner),
}
impl Redefinition {
    pub fn as_ref(&self) -> RedefinitionRef {
        match self {
            Redefinition::Itself(inner) => RedefinitionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> RedefinitionRefMut {
        match self {
            Redefinition::Itself(inner) => RedefinitionRefMut::Itself(inner),
        }
    }
}
impl<'a> RedefinitionRefMut<'a> {
    pub fn as_ref(self) -> RedefinitionRef<'a> {
        match self {
            RedefinitionRefMut::Itself(inner) => RedefinitionRef::Itself(inner),
        }
    }
}
impl RedefinitionStruct for Redefinition {
    fn redefining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefining_feature(),
        }
    }
    fn redefined_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefined_feature(),
        }
    }
}
impl RedefinitionStructRefMut for Redefinition {
    fn redefining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefining_feature_ref_mut(),
        }
    }
    fn redefined_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefined_feature_ref_mut(),
        }
    }
}
impl RedefinitionStructRef for Redefinition {
    fn redefining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefining_feature_ref(),
        }
    }
    fn redefined_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.redefined_feature_ref(),
        }
    }
}
impl<'a> RedefinitionStructRefMut for RedefinitionRefMut<'a> {
    fn redefining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.redefining_feature_ref_mut(),
        }
    }
    fn redefined_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.redefined_feature_ref_mut(),
        }
    }
}
impl<'a> RedefinitionStructRef for RedefinitionRefMut<'a> {
    fn redefining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.redefining_feature_ref(),
        }
    }
    fn redefined_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.redefined_feature_ref(),
        }
    }
}
impl<'a> RedefinitionStructRef for RedefinitionRef<'a> {
    fn redefining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRef::Itself(x) => x.redefining_feature_ref(),
        }
    }
    fn redefined_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRef::Itself(x) => x.redefined_feature_ref(),
        }
    }
}
impl SubsettingStruct for Redefinition {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetted_feature(),
        }
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetting_feature(),
        }
    }
}
impl SubsettingStructRefMut for Redefinition {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl SubsettingStructRef for Redefinition {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Redefinition::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRefMut for RedefinitionRefMut<'a> {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl<'a> SubsettingStructRef for RedefinitionRefMut<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRef for RedefinitionRef<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRef::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            RedefinitionRef::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl SpecializationStruct for Redefinition {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for Redefinition {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for Redefinition {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Redefinition::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for RedefinitionRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for RedefinitionRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for RedefinitionRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRef::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            RedefinitionRef::Itself(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for Redefinition {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Redefinition::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Redefinition {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Redefinition::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Redefinition {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Redefinition::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Redefinition::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for RedefinitionRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            RedefinitionRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for RedefinitionRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            RedefinitionRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for RedefinitionRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RedefinitionRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            RedefinitionRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Redefinition {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Redefinition::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Redefinition::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Redefinition::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Redefinition::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Redefinition::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Redefinition {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Redefinition::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Redefinition::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Redefinition::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Redefinition::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Redefinition::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Redefinition {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Redefinition::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Redefinition::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Redefinition::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Redefinition::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Redefinition::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Redefinition::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for RedefinitionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            RedefinitionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            RedefinitionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            RedefinitionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for RedefinitionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RedefinitionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            RedefinitionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            RedefinitionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            RedefinitionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for RedefinitionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RedefinitionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RedefinitionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            RedefinitionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            RedefinitionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            RedefinitionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            RedefinitionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            RedefinitionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl RedefinitionUpcast for Redefinition {
    fn into_redefinition(self) -> Redefinition {
        self
    }
}
impl<'a> RedefinitionUpcastRefMut<'a> for RedefinitionRefMut<'a> {
    fn as_redefinition_ref_mut(self) -> RedefinitionRefMut<'a> {
        self
    }
}
impl<'a> RedefinitionUpcastRef<'a> for RedefinitionRef<'a> {
    fn as_redefinition_ref(self) -> RedefinitionRef<'a> {
        self
    }
}
impl SubsettingUpcast for Redefinition {
    fn into_subsetting(self) -> Subsetting {
        Subsetting::Redefinition(self).into_subsetting()
    }
}
impl<'a> SubsettingUpcastRefMut<'a> for RedefinitionRefMut<'a> {
    fn as_subsetting_ref_mut(self) -> SubsettingRefMut<'a> {
        SubsettingRefMut::Redefinition(self).as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingUpcastRef<'a> for RedefinitionRef<'a> {
    fn as_subsetting_ref(self) -> SubsettingRef<'a> {
        SubsettingRef::Redefinition(self).as_subsetting_ref()
    }
}
impl SpecializationUpcast for Redefinition {
    fn into_specialization(self) -> Specialization {
        Subsetting::Redefinition(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for RedefinitionRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SubsettingRefMut::Redefinition(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for RedefinitionRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SubsettingRef::Redefinition(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for Redefinition {
    fn into_relationship(self) -> Relationship {
        Subsetting::Redefinition(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for RedefinitionRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SubsettingRefMut::Redefinition(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for RedefinitionRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SubsettingRef::Redefinition(self).as_relationship_ref()
    }
}
impl ElementUpcast for Redefinition {
    fn into_element(self) -> Element {
        Subsetting::Redefinition(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for RedefinitionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SubsettingRefMut::Redefinition(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for RedefinitionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SubsettingRef::Redefinition(self).as_element_ref()
    }
}
pub trait RedefinitionDowncast {}
pub trait RedefinitionDowncastRefMut<'a> {}
pub trait RedefinitionDowncastRef<'a> {}
impl RedefinitionDowncast for Redefinition {}
impl<'a> RedefinitionDowncastRefMut<'a> for RedefinitionRefMut<'a> {}
impl<'a> RedefinitionDowncastRef<'a> for RedefinitionRef<'a> {}
pub trait RedefinitionMethodsDescendants
where
    Self: DescendantOf<Redefinition>,
    Self::Via: RedefinitionMethods,
    Self: Sized,
{}
pub trait RedefinitionMethods: Sized {}
impl<T: RedefinitionMethodsDescendants> RedefinitionMethods for T
where
    T::Via: RedefinitionMethods,
{}
impl DescendantOf<Subsetting> for Redefinition {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SubsettingMethodsDescendants for Redefinition {}
impl DescendantOf<Specialization> for Redefinition {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SpecializationMethodsDescendants for Redefinition {}
impl DescendantOf<Relationship> for Redefinition {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl RelationshipMethodsDescendants for Redefinition {}
impl DescendantOf<Element> for Redefinition {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl ElementMethodsDescendants for Redefinition {}
pub trait RedefinitionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<RedefinitionRefMut<'a>>,
    Self::Via: RedefinitionRefMutMethods,
    Self: Sized,
{}
pub trait RedefinitionRefMutMethods: Sized {}
impl<'a, T: RedefinitionRefMutMethodsDescendants<'a>> RedefinitionRefMutMethods for T
where
    T::Via: RedefinitionRefMutMethods,
{}
impl<'a> DescendantOf<SubsettingRefMut<'a>> for RedefinitionRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingRefMutMethodsDescendants<'a> for RedefinitionRefMut<'a> {}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for RedefinitionRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for RedefinitionRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for RedefinitionRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for RedefinitionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for RedefinitionRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for RedefinitionRefMut<'a> {}
pub trait RedefinitionRefMethodsDescendants<'a>
where
    Self: DescendantOf<RedefinitionRef<'a>>,
    Self::Via: RedefinitionRefMethods,
    Self: Sized,
{}
pub trait RedefinitionRefMethods: Sized {}
impl<'a, T: RedefinitionRefMethodsDescendants<'a>> RedefinitionRefMethods for T
where
    T::Via: RedefinitionRefMethods,
{}
impl<'a> DescendantOf<SubsettingRef<'a>> for RedefinitionRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SubsettingRefMethodsDescendants<'a> for RedefinitionRef<'a> {}
impl<'a> DescendantOf<SpecializationRef<'a>> for RedefinitionRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for RedefinitionRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for RedefinitionRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for RedefinitionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for RedefinitionRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for RedefinitionRef<'a> {}

