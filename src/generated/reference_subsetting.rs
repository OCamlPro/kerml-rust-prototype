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
pub struct ReferenceSubsettingInner {
    pub(super) sup_subsetting: SubsettingInner,
    pub(super) referenced_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
}
pub trait ReferenceSubsettingStruct
where
    Self: ReferenceSubsettingStructRefMut,
    Self: ReferenceSubsettingStructRef,
    Self: SubsettingStruct,
{
    fn referenced_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait ReferenceSubsettingStructRefMut
where
    Self: ReferenceSubsettingStructRef,
    Self: SubsettingStructRefMut,
{
    fn referenced_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait ReferenceSubsettingStructRef
where
    Self: SubsettingStructRef,
{
    fn referenced_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait ReferenceSubsettingUpcast: ReferenceSubsettingStruct {
    fn into_reference_subsetting(self) -> ReferenceSubsetting;
}
pub trait ReferenceSubsettingUpcastRefMut<'a>: ReferenceSubsettingStructRefMut {
    fn as_reference_subsetting_ref_mut(self) -> ReferenceSubsettingRefMut<'a>;
}
pub trait ReferenceSubsettingUpcastRef<'a>: ReferenceSubsettingStructRef {
    fn as_reference_subsetting_ref(self) -> ReferenceSubsettingRef<'a>;
}
impl ReferenceSubsettingStruct for ReferenceSubsettingInner {
    fn referenced_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.referenced_feature
    }
}
impl ReferenceSubsettingStructRefMut for ReferenceSubsettingInner {
    fn referenced_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.referenced_feature
    }
}
impl ReferenceSubsettingStructRef for ReferenceSubsettingInner {
    fn referenced_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.referenced_feature
    }
}
impl SubsettingStruct for ReferenceSubsettingInner {
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
impl SubsettingStructRefMut for ReferenceSubsettingInner {
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
impl SubsettingStructRef for ReferenceSubsettingInner {
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
impl SpecializationStruct for ReferenceSubsettingInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific()
    }
}
impl SpecializationStructRefMut for ReferenceSubsettingInner {
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
impl SpecializationStructRef for ReferenceSubsettingInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_subsetting.specific_ref()
    }
}
impl RelationshipStruct for ReferenceSubsettingInner {
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
impl RelationshipStructRefMut for ReferenceSubsettingInner {
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
impl RelationshipStructRef for ReferenceSubsettingInner {
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
impl ElementStruct for ReferenceSubsettingInner {
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
impl ElementStructRefMut for ReferenceSubsettingInner {
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
impl ElementStructRef for ReferenceSubsettingInner {
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
pub enum ReferenceSubsetting {
    Itself(ReferenceSubsettingInner),
}
pub enum ReferenceSubsettingRefMut<'a> {
    Itself(&'a mut ReferenceSubsettingInner),
}
pub enum ReferenceSubsettingRef<'a> {
    Itself(&'a ReferenceSubsettingInner),
}
impl ReferenceSubsetting {
    pub fn as_ref(&self) -> ReferenceSubsettingRef {
        match self {
            ReferenceSubsetting::Itself(inner) => ReferenceSubsettingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> ReferenceSubsettingRefMut {
        match self {
            ReferenceSubsetting::Itself(inner) => {
                ReferenceSubsettingRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> ReferenceSubsettingRefMut<'a> {
    pub fn as_ref(self) -> ReferenceSubsettingRef<'a> {
        match self {
            ReferenceSubsettingRefMut::Itself(inner) => {
                ReferenceSubsettingRef::Itself(inner)
            }
        }
    }
}
impl ReferenceSubsettingStruct for ReferenceSubsetting {
    fn referenced_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.referenced_feature(),
        }
    }
}
impl ReferenceSubsettingStructRefMut for ReferenceSubsetting {
    fn referenced_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.referenced_feature_ref_mut(),
        }
    }
}
impl ReferenceSubsettingStructRef for ReferenceSubsetting {
    fn referenced_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.referenced_feature_ref(),
        }
    }
}
impl<'a> ReferenceSubsettingStructRefMut for ReferenceSubsettingRefMut<'a> {
    fn referenced_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.referenced_feature_ref_mut(),
        }
    }
}
impl<'a> ReferenceSubsettingStructRef for ReferenceSubsettingRefMut<'a> {
    fn referenced_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.referenced_feature_ref(),
        }
    }
}
impl<'a> ReferenceSubsettingStructRef for ReferenceSubsettingRef<'a> {
    fn referenced_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.referenced_feature_ref(),
        }
    }
}
impl SubsettingStruct for ReferenceSubsetting {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetted_feature(),
        }
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetting_feature(),
        }
    }
}
impl SubsettingStructRefMut for ReferenceSubsetting {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl SubsettingStructRef for ReferenceSubsetting {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRefMut for ReferenceSubsettingRefMut<'a> {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl<'a> SubsettingStructRef for ReferenceSubsettingRefMut<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRef for ReferenceSubsettingRef<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.subsetting_feature_ref(),
        }
    }
}
impl SpecializationStruct for ReferenceSubsetting {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for ReferenceSubsetting {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for ReferenceSubsetting {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for ReferenceSubsettingRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for ReferenceSubsettingRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for ReferenceSubsettingRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for ReferenceSubsetting {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for ReferenceSubsetting {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for ReferenceSubsetting {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ReferenceSubsettingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ReferenceSubsettingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ReferenceSubsettingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for ReferenceSubsetting {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ReferenceSubsetting::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ReferenceSubsetting {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ReferenceSubsetting::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for ReferenceSubsetting {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsetting::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReferenceSubsetting::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsetting::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReferenceSubsetting::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ReferenceSubsettingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ReferenceSubsettingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReferenceSubsettingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ReferenceSubsettingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReferenceSubsettingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ReferenceSubsettingUpcast for ReferenceSubsetting {
    fn into_reference_subsetting(self) -> ReferenceSubsetting {
        self
    }
}
impl<'a> ReferenceSubsettingUpcastRefMut<'a> for ReferenceSubsettingRefMut<'a> {
    fn as_reference_subsetting_ref_mut(self) -> ReferenceSubsettingRefMut<'a> {
        self
    }
}
impl<'a> ReferenceSubsettingUpcastRef<'a> for ReferenceSubsettingRef<'a> {
    fn as_reference_subsetting_ref(self) -> ReferenceSubsettingRef<'a> {
        self
    }
}
impl SubsettingUpcast for ReferenceSubsetting {
    fn into_subsetting(self) -> Subsetting {
        Subsetting::ReferenceSubsetting(self).into_subsetting()
    }
}
impl<'a> SubsettingUpcastRefMut<'a> for ReferenceSubsettingRefMut<'a> {
    fn as_subsetting_ref_mut(self) -> SubsettingRefMut<'a> {
        SubsettingRefMut::ReferenceSubsetting(self).as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingUpcastRef<'a> for ReferenceSubsettingRef<'a> {
    fn as_subsetting_ref(self) -> SubsettingRef<'a> {
        SubsettingRef::ReferenceSubsetting(self).as_subsetting_ref()
    }
}
impl SpecializationUpcast for ReferenceSubsetting {
    fn into_specialization(self) -> Specialization {
        Subsetting::ReferenceSubsetting(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for ReferenceSubsettingRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SubsettingRefMut::ReferenceSubsetting(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for ReferenceSubsettingRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SubsettingRef::ReferenceSubsetting(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for ReferenceSubsetting {
    fn into_relationship(self) -> Relationship {
        Subsetting::ReferenceSubsetting(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ReferenceSubsettingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SubsettingRefMut::ReferenceSubsetting(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ReferenceSubsettingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SubsettingRef::ReferenceSubsetting(self).as_relationship_ref()
    }
}
impl ElementUpcast for ReferenceSubsetting {
    fn into_element(self) -> Element {
        Subsetting::ReferenceSubsetting(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ReferenceSubsettingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SubsettingRefMut::ReferenceSubsetting(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ReferenceSubsettingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SubsettingRef::ReferenceSubsetting(self).as_element_ref()
    }
}
pub trait ReferenceSubsettingDowncast {}
pub trait ReferenceSubsettingDowncastRefMut<'a> {}
pub trait ReferenceSubsettingDowncastRef<'a> {}
impl ReferenceSubsettingDowncast for ReferenceSubsetting {}
impl<'a> ReferenceSubsettingDowncastRefMut<'a> for ReferenceSubsettingRefMut<'a> {}
impl<'a> ReferenceSubsettingDowncastRef<'a> for ReferenceSubsettingRef<'a> {}
pub trait ReferenceSubsettingMethodsDescendants
where
    Self: DescendantOf<ReferenceSubsetting>,
    Self::Via: ReferenceSubsettingMethods,
    Self: Sized,
{}
pub trait ReferenceSubsettingMethods: Sized {}
impl<T: ReferenceSubsettingMethodsDescendants> ReferenceSubsettingMethods for T
where
    T::Via: ReferenceSubsettingMethods,
{}
impl DescendantOf<Subsetting> for ReferenceSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SubsettingMethodsDescendants for ReferenceSubsetting {}
impl DescendantOf<Specialization> for ReferenceSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl SpecializationMethodsDescendants for ReferenceSubsetting {}
impl DescendantOf<Relationship> for ReferenceSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl RelationshipMethodsDescendants for ReferenceSubsetting {}
impl DescendantOf<Element> for ReferenceSubsetting {
    type Via = Subsetting;
    fn into_via(self) -> Self::Via {
        self.into_subsetting()
    }
}
impl ElementMethodsDescendants for ReferenceSubsetting {}
pub trait ReferenceSubsettingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ReferenceSubsettingRefMut<'a>>,
    Self::Via: ReferenceSubsettingRefMutMethods,
    Self: Sized,
{}
pub trait ReferenceSubsettingRefMutMethods: Sized {}
impl<
    'a,
    T: ReferenceSubsettingRefMutMethodsDescendants<'a>,
> ReferenceSubsettingRefMutMethods for T
where
    T::Via: ReferenceSubsettingRefMutMethods,
{}
impl<'a> DescendantOf<SubsettingRefMut<'a>> for ReferenceSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SubsettingRefMutMethodsDescendants<'a> for ReferenceSubsettingRefMut<'a> {}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for ReferenceSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for ReferenceSubsettingRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ReferenceSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ReferenceSubsettingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ReferenceSubsettingRefMut<'a> {
    type Via = SubsettingRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ReferenceSubsettingRefMut<'a> {}
pub trait ReferenceSubsettingRefMethodsDescendants<'a>
where
    Self: DescendantOf<ReferenceSubsettingRef<'a>>,
    Self::Via: ReferenceSubsettingRefMethods,
    Self: Sized,
{}
pub trait ReferenceSubsettingRefMethods: Sized {}
impl<'a, T: ReferenceSubsettingRefMethodsDescendants<'a>> ReferenceSubsettingRefMethods
for T
where
    T::Via: ReferenceSubsettingRefMethods,
{}
impl<'a> DescendantOf<SubsettingRef<'a>> for ReferenceSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SubsettingRefMethodsDescendants<'a> for ReferenceSubsettingRef<'a> {}
impl<'a> DescendantOf<SpecializationRef<'a>> for ReferenceSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for ReferenceSubsettingRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for ReferenceSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ReferenceSubsettingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ReferenceSubsettingRef<'a> {
    type Via = SubsettingRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_subsetting_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ReferenceSubsettingRef<'a> {}

