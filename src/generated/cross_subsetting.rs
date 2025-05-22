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
pub struct CrossSubsettingInner {
    pub(super) sup_subsetting: SubsettingInner,
    pub(super) crossed_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
}
pub trait CrossSubsettingStruct
where
    Self: CrossSubsettingStructRefMut,
    Self: CrossSubsettingStructRef,
    Self: SubsettingStruct,
{
    fn crossed_feature(self) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait CrossSubsettingStructRefMut
where
    Self: CrossSubsettingStructRef,
    Self: SubsettingStructRefMut,
{
    fn crossed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait CrossSubsettingStructRef
where
    Self: SubsettingStructRef,
{
    fn crossed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait CrossSubsettingUpcast: CrossSubsettingStruct {
    fn into_cross_subsetting(self) -> CrossSubsetting;
}
pub trait CrossSubsettingUpcastRefMut<'a>: CrossSubsettingStructRefMut {
    fn as_cross_subsetting_ref_mut(self) -> CrossSubsettingRefMut<'a>;
}
pub trait CrossSubsettingUpcastRef<'a>: CrossSubsettingStructRef {
    fn as_cross_subsetting_ref(self) -> CrossSubsettingRef<'a>;
}
impl CrossSubsettingStruct for CrossSubsettingInner {
    fn crossed_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.crossed_feature
    }
}
impl CrossSubsettingStructRefMut for CrossSubsettingInner {
    fn crossed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.crossed_feature
    }
}
impl CrossSubsettingStructRef for CrossSubsettingInner {
    fn crossed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.crossed_feature
    }
}
impl SubsettingStruct for CrossSubsettingInner {
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
impl SubsettingStructRefMut for CrossSubsettingInner {
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
impl SubsettingStructRef for CrossSubsettingInner {
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
impl SpecializationStruct for CrossSubsettingInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific()
    }
}
impl SpecializationStructRefMut for CrossSubsettingInner {
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
impl SpecializationStructRef for CrossSubsettingInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific_ref()
    }
}
impl RelationshipStruct for CrossSubsettingInner {
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
impl RelationshipStructRefMut for CrossSubsettingInner {
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
impl RelationshipStructRef for CrossSubsettingInner {
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
impl ElementStruct for CrossSubsettingInner {
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
impl ElementStructRefMut for CrossSubsettingInner {
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
impl ElementStructRef for CrossSubsettingInner {
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
pub enum CrossSubsetting {
    Itself(CrossSubsettingInner),
}
pub enum CrossSubsettingRefMut<'a> {
    Itself(&'a mut CrossSubsettingInner),
}
pub enum CrossSubsettingRef<'a> {
    Itself(&'a CrossSubsettingInner),
}
impl CrossSubsetting {
    pub fn as_ref(&self) -> CrossSubsettingRef {
        match self {
            CrossSubsetting::Itself(inner) => CrossSubsettingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> CrossSubsettingRefMut {
        match self {
            CrossSubsetting::Itself(inner) => CrossSubsettingRefMut::Itself(inner),
        }
    }
}
impl<'a> CrossSubsettingRefMut<'a> {
    pub fn as_ref(self) -> CrossSubsettingRef<'a> {
        match self {
            CrossSubsettingRefMut::Itself(inner) => CrossSubsettingRef::Itself(inner),
        }
    }
}
impl CrossSubsettingStruct for CrossSubsetting {
    fn crossed_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.crossed_feature(),
        }
    }
}
impl CrossSubsettingStructRefMut for CrossSubsetting {
    fn crossed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.crossed_feature_ref_mut(),
        }
    }
}
impl CrossSubsettingStructRef for CrossSubsetting {
    fn crossed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.crossed_feature_ref(),
        }
    }
}
impl<'a> CrossSubsettingStructRefMut for CrossSubsettingRefMut<'a> {
    fn crossed_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.crossed_feature_ref_mut(),
        }
    }
}
impl<'a> CrossSubsettingStructRef for CrossSubsettingRefMut<'a> {
    fn crossed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.crossed_feature_ref(),
        }
    }
}
impl<'a> CrossSubsettingStructRef for CrossSubsettingRef<'a> {
    fn crossed_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.crossed_feature_ref(),
        }
    }
}
impl SubsettingStruct for CrossSubsetting {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetted_feature(),
        }
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetting_feature(),
        }
    }
}
impl SubsettingStructRefMut for CrossSubsetting {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl SubsettingStructRef for CrossSubsetting {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsetting::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRefMut for CrossSubsettingRefMut<'a> {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl<'a> SubsettingStructRef for CrossSubsettingRefMut<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRef for CrossSubsettingRef<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl SpecializationStruct for CrossSubsetting {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for CrossSubsetting {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for CrossSubsetting {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsetting::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for CrossSubsettingRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for CrossSubsettingRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for CrossSubsettingRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for CrossSubsetting {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for CrossSubsetting {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for CrossSubsetting {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for CrossSubsettingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for CrossSubsettingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for CrossSubsettingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            CrossSubsettingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for CrossSubsetting {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            CrossSubsetting::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            CrossSubsetting::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for CrossSubsetting {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            CrossSubsetting::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            CrossSubsetting::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            CrossSubsetting::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for CrossSubsetting {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsetting::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CrossSubsetting::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CrossSubsetting::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsetting::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CrossSubsetting::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for CrossSubsettingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for CrossSubsettingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CrossSubsettingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for CrossSubsettingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CrossSubsettingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CrossSubsettingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CrossSubsettingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsettingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CrossSubsettingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CrossSubsettingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl CrossSubsettingUpcast for CrossSubsetting {
    fn into_cross_subsetting(self) -> CrossSubsetting {
        self
    }
}
impl<'a> CrossSubsettingUpcastRefMut<'a> for CrossSubsettingRefMut<'a> {
    fn as_cross_subsetting_ref_mut(self) -> CrossSubsettingRefMut<'a> {
        self
    }
}
impl<'a> CrossSubsettingUpcastRef<'a> for CrossSubsettingRef<'a> {
    fn as_cross_subsetting_ref(self) -> CrossSubsettingRef<'a> {
        self
    }
}
impl SubsettingUpcast for CrossSubsetting {
    fn into_subsetting(self) -> Subsetting {
        Subsetting::CrossSubsetting(self).into_subsetting()
    }
}
impl<'a> SubsettingUpcastRefMut<'a> for CrossSubsettingRefMut<'a> {
    fn as_subsetting_ref_mut(self) -> SubsettingRefMut<'a> {
        SubsettingRefMut::CrossSubsetting(self).as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingUpcastRef<'a> for CrossSubsettingRef<'a> {
    fn as_subsetting_ref(self) -> SubsettingRef<'a> {
        SubsettingRef::CrossSubsetting(self).as_subsetting_ref()
    }
}
impl SpecializationUpcast for CrossSubsetting {
    fn into_specialization(self) -> Specialization {
        Subsetting::CrossSubsetting(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for CrossSubsettingRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SubsettingRefMut::CrossSubsetting(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for CrossSubsettingRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SubsettingRef::CrossSubsetting(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for CrossSubsetting {
    fn into_relationship(self) -> Relationship {
        Subsetting::CrossSubsetting(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for CrossSubsettingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SubsettingRefMut::CrossSubsetting(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for CrossSubsettingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SubsettingRef::CrossSubsetting(self).as_relationship_ref()
    }
}
impl ElementUpcast for CrossSubsetting {
    fn into_element(self) -> Element {
        Subsetting::CrossSubsetting(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for CrossSubsettingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SubsettingRefMut::CrossSubsetting(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for CrossSubsettingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SubsettingRef::CrossSubsetting(self).as_element_ref()
    }
}
pub trait CrossSubsettingDowncast {}
pub trait CrossSubsettingDowncastRefMut<'a> {}
pub trait CrossSubsettingDowncastRef<'a> {}
impl CrossSubsettingDowncast for CrossSubsetting {}
impl<'a> CrossSubsettingDowncastRefMut<'a> for CrossSubsettingRefMut<'a> {}
impl<'a> CrossSubsettingDowncastRef<'a> for CrossSubsettingRef<'a> {}
pub trait CrossSubsettingMethodsDescendants
where
    Self: DescendantOf<CrossSubsetting>,
    Self::Via: CrossSubsettingMethods,
    Self: Sized,
{}
pub trait CrossSubsettingMethods: Sized {}
impl<T: CrossSubsettingMethodsDescendants> CrossSubsettingMethods for T
where
    T::Via: CrossSubsettingMethods,
{}
impl DescendantOf<Subsetting> for CrossSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SubsettingMethodsDescendants for CrossSubsetting {}
impl DescendantOf<Specialization> for CrossSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SpecializationMethodsDescendants for CrossSubsetting {}
impl DescendantOf<Relationship> for CrossSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl RelationshipMethodsDescendants for CrossSubsetting {}
impl DescendantOf<Element> for CrossSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl ElementMethodsDescendants for CrossSubsetting {}
pub trait CrossSubsettingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<CrossSubsettingRefMut<'a>>,
    Self::Via: CrossSubsettingRefMutMethods,
    Self: Sized,
{}
pub trait CrossSubsettingRefMutMethods: Sized {}
impl<'a, T: CrossSubsettingRefMutMethodsDescendants<'a>> CrossSubsettingRefMutMethods
for T
where
    T::Via: CrossSubsettingRefMutMethods,
{}
impl<'a> DescendantOf<SubsettingRefMut<'a>> for CrossSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingRefMutMethodsDescendants<'a> for CrossSubsettingRefMut<'a> {}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for CrossSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for CrossSubsettingRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for CrossSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for CrossSubsettingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for CrossSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for CrossSubsettingRefMut<'a> {}
pub trait CrossSubsettingRefMethodsDescendants<'a>
where
    Self: DescendantOf<CrossSubsettingRef<'a>>,
    Self::Via: CrossSubsettingRefMethods,
    Self: Sized,
{}
pub trait CrossSubsettingRefMethods: Sized {}
impl<'a, T: CrossSubsettingRefMethodsDescendants<'a>> CrossSubsettingRefMethods for T
where
    T::Via: CrossSubsettingRefMethods,
{}
impl<'a> DescendantOf<SubsettingRef<'a>> for CrossSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SubsettingRefMethodsDescendants<'a> for CrossSubsettingRef<'a> {}
impl<'a> DescendantOf<SpecializationRef<'a>> for CrossSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for CrossSubsettingRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for CrossSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for CrossSubsettingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for CrossSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for CrossSubsettingRef<'a> {}

