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
use super::cross_subsetting::{
    CrossSubsetting, CrossSubsettingRefMut, CrossSubsettingRef,
};
use super::redefinition::{Redefinition, RedefinitionRefMut, RedefinitionRef};
use super::reference_subsetting::{
    ReferenceSubsetting, ReferenceSubsettingRefMut, ReferenceSubsettingRef,
};
pub struct SubsettingInner {
    pub(super) sup_specialization: SpecializationInner,
    pub(super) subsetted_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
    pub(super) subsetting_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
}
pub trait SubsettingStruct
where
    Self: SubsettingStructRefMut,
    Self: SubsettingStructRef,
    Self: SpecializationStruct,
{
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait SubsettingStructRefMut
where
    Self: SubsettingStructRef,
    Self: SpecializationStructRefMut,
{
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait SubsettingStructRef
where
    Self: SpecializationStructRef,
{
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait SubsettingUpcast: SubsettingStruct {
    fn into_subsetting(self) -> Subsetting;
}
pub trait SubsettingUpcastRefMut<'a>: SubsettingStructRefMut {
    fn as_subsetting_ref_mut(self) -> SubsettingRefMut<'a>;
}
pub trait SubsettingUpcastRef<'a>: SubsettingStructRef {
    fn as_subsetting_ref(self) -> SubsettingRef<'a>;
}
impl SubsettingStruct for SubsettingInner {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.subsetted_feature
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.subsetting_feature
    }
}
impl SubsettingStructRefMut for SubsettingInner {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.subsetted_feature
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.subsetting_feature
    }
}
impl SubsettingStructRef for SubsettingInner {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.subsetted_feature
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.subsetting_feature
    }
}
impl SpecializationStruct for SubsettingInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific()
    }
}
impl SpecializationStructRefMut for SubsettingInner {
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
impl SpecializationStructRef for SubsettingInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific_ref()
    }
}
impl RelationshipStruct for SubsettingInner {
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
impl RelationshipStructRefMut for SubsettingInner {
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
impl RelationshipStructRef for SubsettingInner {
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
impl ElementStruct for SubsettingInner {
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
impl ElementStructRefMut for SubsettingInner {
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
impl ElementStructRef for SubsettingInner {
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
pub enum Subsetting {
    Itself(SubsettingInner),
    CrossSubsetting(CrossSubsetting),
    Redefinition(Redefinition),
    ReferenceSubsetting(ReferenceSubsetting),
}
pub enum SubsettingRefMut<'a> {
    Itself(&'a mut SubsettingInner),
    CrossSubsetting(CrossSubsettingRefMut<'a>),
    Redefinition(RedefinitionRefMut<'a>),
    ReferenceSubsetting(ReferenceSubsettingRefMut<'a>),
}
pub enum SubsettingRef<'a> {
    Itself(&'a SubsettingInner),
    CrossSubsetting(CrossSubsettingRef<'a>),
    Redefinition(RedefinitionRef<'a>),
    ReferenceSubsetting(ReferenceSubsettingRef<'a>),
}
impl Subsetting {
    pub fn as_ref(&self) -> SubsettingRef {
        match self {
            Subsetting::Itself(inner) => SubsettingRef::Itself(&inner),
            Subsetting::CrossSubsetting(inner) => {
                SubsettingRef::CrossSubsetting(inner.as_ref())
            }
            Subsetting::Redefinition(inner) => {
                SubsettingRef::Redefinition(inner.as_ref())
            }
            Subsetting::ReferenceSubsetting(inner) => {
                SubsettingRef::ReferenceSubsetting(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> SubsettingRefMut {
        match self {
            Subsetting::Itself(inner) => SubsettingRefMut::Itself(inner),
            Subsetting::CrossSubsetting(inner) => {
                SubsettingRefMut::CrossSubsetting(inner.as_ref_mut())
            }
            Subsetting::Redefinition(inner) => {
                SubsettingRefMut::Redefinition(inner.as_ref_mut())
            }
            Subsetting::ReferenceSubsetting(inner) => {
                SubsettingRefMut::ReferenceSubsetting(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> SubsettingRefMut<'a> {
    pub fn as_ref(self) -> SubsettingRef<'a> {
        match self {
            SubsettingRefMut::Itself(inner) => SubsettingRef::Itself(inner),
            SubsettingRefMut::CrossSubsetting(inner) => {
                SubsettingRef::CrossSubsetting(inner.as_ref())
            }
            SubsettingRefMut::Redefinition(inner) => {
                SubsettingRef::Redefinition(inner.as_ref())
            }
            SubsettingRefMut::ReferenceSubsetting(inner) => {
                SubsettingRef::ReferenceSubsetting(inner.as_ref())
            }
        }
    }
}
impl SubsettingStruct for Subsetting {
    fn subsetted_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetted_feature(),
            Subsetting::CrossSubsetting(x) => x.subsetted_feature(),
            Subsetting::Redefinition(x) => x.subsetted_feature(),
            Subsetting::ReferenceSubsetting(x) => x.subsetted_feature(),
        }
    }
    fn subsetting_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetting_feature(),
            Subsetting::CrossSubsetting(x) => x.subsetting_feature(),
            Subsetting::Redefinition(x) => x.subsetting_feature(),
            Subsetting::ReferenceSubsetting(x) => x.subsetting_feature(),
        }
    }
}
impl SubsettingStructRefMut for Subsetting {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetted_feature_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.subsetted_feature_ref_mut(),
            Subsetting::Redefinition(x) => x.subsetted_feature_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetting_feature_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.subsetting_feature_ref_mut(),
            Subsetting::Redefinition(x) => x.subsetting_feature_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl SubsettingStructRef for Subsetting {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetted_feature_ref(),
            Subsetting::CrossSubsetting(x) => x.subsetted_feature_ref(),
            Subsetting::Redefinition(x) => x.subsetted_feature_ref(),
            Subsetting::ReferenceSubsetting(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            Subsetting::Itself(x) => x.subsetting_feature_ref(),
            Subsetting::CrossSubsetting(x) => x.subsetting_feature_ref(),
            Subsetting::Redefinition(x) => x.subsetting_feature_ref(),
            Subsetting::ReferenceSubsetting(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRefMut for SubsettingRefMut<'a> {
    fn subsetted_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRefMut::Itself(x) => x.subsetted_feature_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.subsetted_feature_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.subsetted_feature_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.subsetted_feature_ref_mut(),
        }
    }
    fn subsetting_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRefMut::Itself(x) => x.subsetting_feature_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.subsetting_feature_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.subsetting_feature_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.subsetting_feature_ref_mut(),
        }
    }
}
impl<'a> SubsettingStructRef for SubsettingRefMut<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRefMut::Itself(x) => x.subsetted_feature_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.subsetted_feature_ref(),
            SubsettingRefMut::Redefinition(x) => x.subsetted_feature_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRefMut::Itself(x) => x.subsetting_feature_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.subsetting_feature_ref(),
            SubsettingRefMut::Redefinition(x) => x.subsetting_feature_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.subsetting_feature_ref(),
        }
    }
}
impl<'a> SubsettingStructRef for SubsettingRef<'a> {
    fn subsetted_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRef::Itself(x) => x.subsetted_feature_ref(),
            SubsettingRef::CrossSubsetting(x) => x.subsetted_feature_ref(),
            SubsettingRef::Redefinition(x) => x.subsetted_feature_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.subsetted_feature_ref(),
        }
    }
    fn subsetting_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            SubsettingRef::Itself(x) => x.subsetting_feature_ref(),
            SubsettingRef::CrossSubsetting(x) => x.subsetting_feature_ref(),
            SubsettingRef::Redefinition(x) => x.subsetting_feature_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.subsetting_feature_ref(),
        }
    }
}
impl SpecializationStruct for Subsetting {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.general(),
            Subsetting::CrossSubsetting(x) => x.general(),
            Subsetting::Redefinition(x) => x.general(),
            Subsetting::ReferenceSubsetting(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.specific(),
            Subsetting::CrossSubsetting(x) => x.specific(),
            Subsetting::Redefinition(x) => x.specific(),
            Subsetting::ReferenceSubsetting(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for Subsetting {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.general_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.general_ref_mut(),
            Subsetting::Redefinition(x) => x.general_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.specific_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.specific_ref_mut(),
            Subsetting::Redefinition(x) => x.specific_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for Subsetting {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.general_ref(),
            Subsetting::CrossSubsetting(x) => x.general_ref(),
            Subsetting::Redefinition(x) => x.general_ref(),
            Subsetting::ReferenceSubsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subsetting::Itself(x) => x.specific_ref(),
            Subsetting::CrossSubsetting(x) => x.specific_ref(),
            Subsetting::Redefinition(x) => x.specific_ref(),
            Subsetting::ReferenceSubsetting(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for SubsettingRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRefMut::Itself(x) => x.general_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.general_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.general_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRefMut::Itself(x) => x.specific_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.specific_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.specific_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for SubsettingRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRefMut::Itself(x) => x.general_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.general_ref(),
            SubsettingRefMut::Redefinition(x) => x.general_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRefMut::Itself(x) => x.specific_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.specific_ref(),
            SubsettingRefMut::Redefinition(x) => x.specific_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for SubsettingRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRef::Itself(x) => x.general_ref(),
            SubsettingRef::CrossSubsetting(x) => x.general_ref(),
            SubsettingRef::Redefinition(x) => x.general_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubsettingRef::Itself(x) => x.specific_ref(),
            SubsettingRef::CrossSubsetting(x) => x.specific_ref(),
            SubsettingRef::Redefinition(x) => x.specific_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for Subsetting {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.target(),
            Subsetting::CrossSubsetting(x) => x.target(),
            Subsetting::Redefinition(x) => x.target(),
            Subsetting::ReferenceSubsetting(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.source(),
            Subsetting::CrossSubsetting(x) => x.source(),
            Subsetting::Redefinition(x) => x.source(),
            Subsetting::ReferenceSubsetting(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owning_related_element(),
            Subsetting::CrossSubsetting(x) => x.owning_related_element(),
            Subsetting::Redefinition(x) => x.owning_related_element(),
            Subsetting::ReferenceSubsetting(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owned_related_element(),
            Subsetting::CrossSubsetting(x) => x.owned_related_element(),
            Subsetting::Redefinition(x) => x.owned_related_element(),
            Subsetting::ReferenceSubsetting(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Subsetting::Itself(x) => x.is_implied(),
            Subsetting::CrossSubsetting(x) => x.is_implied(),
            Subsetting::Redefinition(x) => x.is_implied(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Subsetting {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.target_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.target_ref_mut(),
            Subsetting::Redefinition(x) => x.target_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.source_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.source_ref_mut(),
            Subsetting::Redefinition(x) => x.source_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owning_related_element_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.owning_related_element_ref_mut(),
            Subsetting::Redefinition(x) => x.owning_related_element_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owned_related_element_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.owned_related_element_ref_mut(),
            Subsetting::Redefinition(x) => x.owned_related_element_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Subsetting::Itself(x) => x.is_implied_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.is_implied_ref_mut(),
            Subsetting::Redefinition(x) => x.is_implied_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Subsetting {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.target_ref(),
            Subsetting::CrossSubsetting(x) => x.target_ref(),
            Subsetting::Redefinition(x) => x.target_ref(),
            Subsetting::ReferenceSubsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.source_ref(),
            Subsetting::CrossSubsetting(x) => x.source_ref(),
            Subsetting::Redefinition(x) => x.source_ref(),
            Subsetting::ReferenceSubsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owning_related_element_ref(),
            Subsetting::CrossSubsetting(x) => x.owning_related_element_ref(),
            Subsetting::Redefinition(x) => x.owning_related_element_ref(),
            Subsetting::ReferenceSubsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subsetting::Itself(x) => x.owned_related_element_ref(),
            Subsetting::CrossSubsetting(x) => x.owned_related_element_ref(),
            Subsetting::Redefinition(x) => x.owned_related_element_ref(),
            Subsetting::ReferenceSubsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Subsetting::Itself(x) => x.is_implied_ref(),
            Subsetting::CrossSubsetting(x) => x.is_implied_ref(),
            Subsetting::Redefinition(x) => x.is_implied_ref(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for SubsettingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.target_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.target_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.target_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.source_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.source_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.source_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.owning_related_element_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.owning_related_element_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.owned_related_element_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.owned_related_element_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SubsettingRefMut::Itself(x) => x.is_implied_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.is_implied_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.is_implied_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for SubsettingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.target_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.target_ref(),
            SubsettingRefMut::Redefinition(x) => x.target_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.source_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.source_ref(),
            SubsettingRefMut::Redefinition(x) => x.source_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owning_related_element_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.owning_related_element_ref(),
            SubsettingRefMut::Redefinition(x) => x.owning_related_element_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owned_related_element_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.owned_related_element_ref(),
            SubsettingRefMut::Redefinition(x) => x.owned_related_element_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SubsettingRefMut::Itself(x) => x.is_implied_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.is_implied_ref(),
            SubsettingRefMut::Redefinition(x) => x.is_implied_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for SubsettingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRef::Itself(x) => x.target_ref(),
            SubsettingRef::CrossSubsetting(x) => x.target_ref(),
            SubsettingRef::Redefinition(x) => x.target_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRef::Itself(x) => x.source_ref(),
            SubsettingRef::CrossSubsetting(x) => x.source_ref(),
            SubsettingRef::Redefinition(x) => x.source_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRef::Itself(x) => x.owning_related_element_ref(),
            SubsettingRef::CrossSubsetting(x) => x.owning_related_element_ref(),
            SubsettingRef::Redefinition(x) => x.owning_related_element_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubsettingRef::Itself(x) => x.owned_related_element_ref(),
            SubsettingRef::CrossSubsetting(x) => x.owned_related_element_ref(),
            SubsettingRef::Redefinition(x) => x.owned_related_element_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SubsettingRef::Itself(x) => x.is_implied_ref(),
            SubsettingRef::CrossSubsetting(x) => x.is_implied_ref(),
            SubsettingRef::Redefinition(x) => x.is_implied_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Subsetting {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subsetting::Itself(x) => x.owned_relationship(),
            Subsetting::CrossSubsetting(x) => x.owned_relationship(),
            Subsetting::Redefinition(x) => x.owned_relationship(),
            Subsetting::ReferenceSubsetting(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subsetting::Itself(x) => x.owning_relationship(),
            Subsetting::CrossSubsetting(x) => x.owning_relationship(),
            Subsetting::Redefinition(x) => x.owning_relationship(),
            Subsetting::ReferenceSubsetting(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Subsetting::Itself(x) => x.element_id(),
            Subsetting::CrossSubsetting(x) => x.element_id(),
            Subsetting::Redefinition(x) => x.element_id(),
            Subsetting::ReferenceSubsetting(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Subsetting::Itself(x) => x.alias_ids(),
            Subsetting::CrossSubsetting(x) => x.alias_ids(),
            Subsetting::Redefinition(x) => x.alias_ids(),
            Subsetting::ReferenceSubsetting(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_short_name(),
            Subsetting::CrossSubsetting(x) => x.declared_short_name(),
            Subsetting::Redefinition(x) => x.declared_short_name(),
            Subsetting::ReferenceSubsetting(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_name(),
            Subsetting::CrossSubsetting(x) => x.declared_name(),
            Subsetting::Redefinition(x) => x.declared_name(),
            Subsetting::ReferenceSubsetting(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Subsetting::Itself(x) => x.is_implied_included(),
            Subsetting::CrossSubsetting(x) => x.is_implied_included(),
            Subsetting::Redefinition(x) => x.is_implied_included(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Subsetting {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subsetting::Itself(x) => x.owned_relationship_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.owned_relationship_ref_mut(),
            Subsetting::Redefinition(x) => x.owned_relationship_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Subsetting::Itself(x) => x.owning_relationship_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.owning_relationship_ref_mut(),
            Subsetting::Redefinition(x) => x.owning_relationship_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Subsetting::Itself(x) => x.element_id_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.element_id_ref_mut(),
            Subsetting::Redefinition(x) => x.element_id_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Subsetting::Itself(x) => x.alias_ids_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.alias_ids_ref_mut(),
            Subsetting::Redefinition(x) => x.alias_ids_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_short_name_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.declared_short_name_ref_mut(),
            Subsetting::Redefinition(x) => x.declared_short_name_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_name_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.declared_name_ref_mut(),
            Subsetting::Redefinition(x) => x.declared_name_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Subsetting::Itself(x) => x.is_implied_included_ref_mut(),
            Subsetting::CrossSubsetting(x) => x.is_implied_included_ref_mut(),
            Subsetting::Redefinition(x) => x.is_implied_included_ref_mut(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Subsetting {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subsetting::Itself(x) => x.owned_relationship_ref(),
            Subsetting::CrossSubsetting(x) => x.owned_relationship_ref(),
            Subsetting::Redefinition(x) => x.owned_relationship_ref(),
            Subsetting::ReferenceSubsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subsetting::Itself(x) => x.owning_relationship_ref(),
            Subsetting::CrossSubsetting(x) => x.owning_relationship_ref(),
            Subsetting::Redefinition(x) => x.owning_relationship_ref(),
            Subsetting::ReferenceSubsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Subsetting::Itself(x) => x.element_id_ref(),
            Subsetting::CrossSubsetting(x) => x.element_id_ref(),
            Subsetting::Redefinition(x) => x.element_id_ref(),
            Subsetting::ReferenceSubsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Subsetting::Itself(x) => x.alias_ids_ref(),
            Subsetting::CrossSubsetting(x) => x.alias_ids_ref(),
            Subsetting::Redefinition(x) => x.alias_ids_ref(),
            Subsetting::ReferenceSubsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_short_name_ref(),
            Subsetting::CrossSubsetting(x) => x.declared_short_name_ref(),
            Subsetting::Redefinition(x) => x.declared_short_name_ref(),
            Subsetting::ReferenceSubsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Subsetting::Itself(x) => x.declared_name_ref(),
            Subsetting::CrossSubsetting(x) => x.declared_name_ref(),
            Subsetting::Redefinition(x) => x.declared_name_ref(),
            Subsetting::ReferenceSubsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Subsetting::Itself(x) => x.is_implied_included_ref(),
            Subsetting::CrossSubsetting(x) => x.is_implied_included_ref(),
            Subsetting::Redefinition(x) => x.is_implied_included_ref(),
            Subsetting::ReferenceSubsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SubsettingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.owned_relationship_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.owned_relationship_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SubsettingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.owning_relationship_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.owning_relationship_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SubsettingRefMut::Itself(x) => x.element_id_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.element_id_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.element_id_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.alias_ids_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.alias_ids_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.alias_ids_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.declared_short_name_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.declared_short_name_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.declared_name_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.declared_name_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.declared_name_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SubsettingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            SubsettingRefMut::CrossSubsetting(x) => x.is_implied_included_ref_mut(),
            SubsettingRefMut::Redefinition(x) => x.is_implied_included_ref_mut(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SubsettingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owned_relationship_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.owned_relationship_ref(),
            SubsettingRefMut::Redefinition(x) => x.owned_relationship_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubsettingRefMut::Itself(x) => x.owning_relationship_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.owning_relationship_ref(),
            SubsettingRefMut::Redefinition(x) => x.owning_relationship_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SubsettingRefMut::Itself(x) => x.element_id_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.element_id_ref(),
            SubsettingRefMut::Redefinition(x) => x.element_id_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.alias_ids_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.alias_ids_ref(),
            SubsettingRefMut::Redefinition(x) => x.alias_ids_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.declared_short_name_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.declared_short_name_ref(),
            SubsettingRefMut::Redefinition(x) => x.declared_short_name_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SubsettingRefMut::Itself(x) => x.declared_name_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.declared_name_ref(),
            SubsettingRefMut::Redefinition(x) => x.declared_name_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SubsettingRefMut::Itself(x) => x.is_implied_included_ref(),
            SubsettingRefMut::CrossSubsetting(x) => x.is_implied_included_ref(),
            SubsettingRefMut::Redefinition(x) => x.is_implied_included_ref(),
            SubsettingRefMut::ReferenceSubsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SubsettingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubsettingRef::Itself(x) => x.owned_relationship_ref(),
            SubsettingRef::CrossSubsetting(x) => x.owned_relationship_ref(),
            SubsettingRef::Redefinition(x) => x.owned_relationship_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubsettingRef::Itself(x) => x.owning_relationship_ref(),
            SubsettingRef::CrossSubsetting(x) => x.owning_relationship_ref(),
            SubsettingRef::Redefinition(x) => x.owning_relationship_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SubsettingRef::Itself(x) => x.element_id_ref(),
            SubsettingRef::CrossSubsetting(x) => x.element_id_ref(),
            SubsettingRef::Redefinition(x) => x.element_id_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SubsettingRef::Itself(x) => x.alias_ids_ref(),
            SubsettingRef::CrossSubsetting(x) => x.alias_ids_ref(),
            SubsettingRef::Redefinition(x) => x.alias_ids_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SubsettingRef::Itself(x) => x.declared_short_name_ref(),
            SubsettingRef::CrossSubsetting(x) => x.declared_short_name_ref(),
            SubsettingRef::Redefinition(x) => x.declared_short_name_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SubsettingRef::Itself(x) => x.declared_name_ref(),
            SubsettingRef::CrossSubsetting(x) => x.declared_name_ref(),
            SubsettingRef::Redefinition(x) => x.declared_name_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SubsettingRef::Itself(x) => x.is_implied_included_ref(),
            SubsettingRef::CrossSubsetting(x) => x.is_implied_included_ref(),
            SubsettingRef::Redefinition(x) => x.is_implied_included_ref(),
            SubsettingRef::ReferenceSubsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl SubsettingUpcast for Subsetting {
    fn into_subsetting(self) -> Subsetting {
        self
    }
}
impl<'a> SubsettingUpcastRefMut<'a> for SubsettingRefMut<'a> {
    fn as_subsetting_ref_mut(self) -> SubsettingRefMut<'a> {
        self
    }
}
impl<'a> SubsettingUpcastRef<'a> for SubsettingRef<'a> {
    fn as_subsetting_ref(self) -> SubsettingRef<'a> {
        self
    }
}
impl SpecializationUpcast for Subsetting {
    fn into_specialization(self) -> Specialization {
        Specialization::Subsetting(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for SubsettingRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SpecializationRefMut::Subsetting(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for SubsettingRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SpecializationRef::Subsetting(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for Subsetting {
    fn into_relationship(self) -> Relationship {
        Specialization::Subsetting(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for SubsettingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SpecializationRefMut::Subsetting(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for SubsettingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SpecializationRef::Subsetting(self).as_relationship_ref()
    }
}
impl ElementUpcast for Subsetting {
    fn into_element(self) -> Element {
        Specialization::Subsetting(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SubsettingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SpecializationRefMut::Subsetting(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SubsettingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SpecializationRef::Subsetting(self).as_element_ref()
    }
}
pub trait SubsettingDowncast {
    fn try_into_cross_subsetting(self) -> Result<CrossSubsetting, String>;
    fn try_into_redefinition(self) -> Result<Redefinition, String>;
    fn try_into_reference_subsetting(self) -> Result<ReferenceSubsetting, String>;
}
pub trait SubsettingDowncastRefMut<'a> {
    fn try_as_cross_subsetting_ref_mut(
        self,
    ) -> Result<CrossSubsettingRefMut<'a>, String>;
    fn try_as_redefinition_ref_mut(self) -> Result<RedefinitionRefMut<'a>, String>;
    fn try_as_reference_subsetting_ref_mut(
        self,
    ) -> Result<ReferenceSubsettingRefMut<'a>, String>;
}
pub trait SubsettingDowncastRef<'a> {
    fn try_as_cross_subsetting_ref(self) -> Result<CrossSubsettingRef<'a>, String>;
    fn try_as_redefinition_ref(self) -> Result<RedefinitionRef<'a>, String>;
    fn try_as_reference_subsetting_ref(
        self,
    ) -> Result<ReferenceSubsettingRef<'a>, String>;
}
impl SubsettingDowncast for Subsetting {
    fn try_into_cross_subsetting(self) -> Result<CrossSubsetting, String> {
        match self {
            Subsetting::CrossSubsetting(e) => Ok(e),
            _ => Err("Not a CrossSubsetting".into()),
        }
    }
    fn try_into_redefinition(self) -> Result<Redefinition, String> {
        match self {
            Subsetting::Redefinition(e) => Ok(e),
            _ => Err("Not a Redefinition".into()),
        }
    }
    fn try_into_reference_subsetting(self) -> Result<ReferenceSubsetting, String> {
        match self {
            Subsetting::ReferenceSubsetting(e) => Ok(e),
            _ => Err("Not a ReferenceSubsetting".into()),
        }
    }
}
impl<'a> SubsettingDowncastRefMut<'a> for SubsettingRefMut<'a> {
    fn try_as_cross_subsetting_ref_mut(
        self,
    ) -> Result<CrossSubsettingRefMut<'a>, String> {
        match self {
            SubsettingRefMut::CrossSubsetting(e) => Ok(e),
            _ => Err("Not a CrossSubsetting".into()),
        }
    }
    fn try_as_redefinition_ref_mut(self) -> Result<RedefinitionRefMut<'a>, String> {
        match self {
            SubsettingRefMut::Redefinition(e) => Ok(e),
            _ => Err("Not a Redefinition".into()),
        }
    }
    fn try_as_reference_subsetting_ref_mut(
        self,
    ) -> Result<ReferenceSubsettingRefMut<'a>, String> {
        match self {
            SubsettingRefMut::ReferenceSubsetting(e) => Ok(e),
            _ => Err("Not a ReferenceSubsetting".into()),
        }
    }
}
impl<'a> SubsettingDowncastRef<'a> for SubsettingRef<'a> {
    fn try_as_cross_subsetting_ref(self) -> Result<CrossSubsettingRef<'a>, String> {
        match self {
            SubsettingRef::CrossSubsetting(e) => Ok(e),
            _ => Err("Not a CrossSubsetting".into()),
        }
    }
    fn try_as_redefinition_ref(self) -> Result<RedefinitionRef<'a>, String> {
        match self {
            SubsettingRef::Redefinition(e) => Ok(e),
            _ => Err("Not a Redefinition".into()),
        }
    }
    fn try_as_reference_subsetting_ref(
        self,
    ) -> Result<ReferenceSubsettingRef<'a>, String> {
        match self {
            SubsettingRef::ReferenceSubsetting(e) => Ok(e),
            _ => Err("Not a ReferenceSubsetting".into()),
        }
    }
}
pub trait SubsettingMethodsDescendants
where
    Self: DescendantOf<Subsetting>,
    Self::Via: SubsettingMethods,
    Self: Sized,
{}
pub trait SubsettingMethods: Sized {}
impl<T: SubsettingMethodsDescendants> SubsettingMethods for T
where
    T::Via: SubsettingMethods,
{}
impl DescendantOf<Specialization> for Subsetting {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl SpecializationMethodsDescendants for Subsetting {}
impl DescendantOf<Relationship> for Subsetting {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl RelationshipMethodsDescendants for Subsetting {}
impl DescendantOf<Element> for Subsetting {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl ElementMethodsDescendants for Subsetting {}
pub trait SubsettingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SubsettingRefMut<'a>>,
    Self::Via: SubsettingRefMutMethods,
    Self: Sized,
{}
pub trait SubsettingRefMutMethods: Sized {}
impl<'a, T: SubsettingRefMutMethodsDescendants<'a>> SubsettingRefMutMethods for T
where
    T::Via: SubsettingRefMutMethods,
{}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for SubsettingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for SubsettingRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for SubsettingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for SubsettingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SubsettingRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SubsettingRefMut<'a> {}
pub trait SubsettingRefMethodsDescendants<'a>
where
    Self: DescendantOf<SubsettingRef<'a>>,
    Self::Via: SubsettingRefMethods,
    Self: Sized,
{}
pub trait SubsettingRefMethods: Sized {}
impl<'a, T: SubsettingRefMethodsDescendants<'a>> SubsettingRefMethods for T
where
    T::Via: SubsettingRefMethods,
{}
impl<'a> DescendantOf<SpecializationRef<'a>> for SubsettingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for SubsettingRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for SubsettingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for SubsettingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SubsettingRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SubsettingRef<'a> {}

