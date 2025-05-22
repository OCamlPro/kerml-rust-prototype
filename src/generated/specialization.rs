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
use super::feature_typing::{FeatureTyping, FeatureTypingRefMut, FeatureTypingRef};
use super::subclassification::{
    Subclassification, SubclassificationRefMut, SubclassificationRef,
};
use super::subsetting::{Subsetting, SubsettingRefMut, SubsettingRef};
pub struct SpecializationInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) general: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    pub(super) specific: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait SpecializationStruct
where
    Self: SpecializationStructRefMut,
    Self: SpecializationStructRef,
    Self: RelationshipStruct,
{
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait SpecializationStructRefMut
where
    Self: SpecializationStructRef,
    Self: RelationshipStructRefMut,
{
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait SpecializationStructRef
where
    Self: RelationshipStructRef,
{
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait SpecializationUpcast: SpecializationStruct {
    fn into_specialization(self) -> Specialization;
}
pub trait SpecializationUpcastRefMut<'a>: SpecializationStructRefMut {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a>;
}
pub trait SpecializationUpcastRef<'a>: SpecializationStructRef {
    fn as_specialization_ref(self) -> SpecializationRef<'a>;
}
impl SpecializationStruct for SpecializationInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.general
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.specific
    }
}
impl SpecializationStructRefMut for SpecializationInner {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.general
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.specific
    }
}
impl SpecializationStructRef for SpecializationInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.general
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.specific
    }
}
impl RelationshipStruct for SpecializationInner {
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
impl RelationshipStructRefMut for SpecializationInner {
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
impl RelationshipStructRef for SpecializationInner {
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
impl ElementStruct for SpecializationInner {
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
impl ElementStructRefMut for SpecializationInner {
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
impl ElementStructRef for SpecializationInner {
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
pub enum Specialization {
    Itself(SpecializationInner),
    FeatureTyping(FeatureTyping),
    Subclassification(Subclassification),
    Subsetting(Subsetting),
}
pub enum SpecializationRefMut<'a> {
    Itself(&'a mut SpecializationInner),
    FeatureTyping(FeatureTypingRefMut<'a>),
    Subclassification(SubclassificationRefMut<'a>),
    Subsetting(SubsettingRefMut<'a>),
}
pub enum SpecializationRef<'a> {
    Itself(&'a SpecializationInner),
    FeatureTyping(FeatureTypingRef<'a>),
    Subclassification(SubclassificationRef<'a>),
    Subsetting(SubsettingRef<'a>),
}
impl Specialization {
    pub fn as_ref(&self) -> SpecializationRef {
        match self {
            Specialization::Itself(inner) => SpecializationRef::Itself(&inner),
            Specialization::FeatureTyping(inner) => {
                SpecializationRef::FeatureTyping(inner.as_ref())
            }
            Specialization::Subclassification(inner) => {
                SpecializationRef::Subclassification(inner.as_ref())
            }
            Specialization::Subsetting(inner) => {
                SpecializationRef::Subsetting(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> SpecializationRefMut {
        match self {
            Specialization::Itself(inner) => SpecializationRefMut::Itself(inner),
            Specialization::FeatureTyping(inner) => {
                SpecializationRefMut::FeatureTyping(inner.as_ref_mut())
            }
            Specialization::Subclassification(inner) => {
                SpecializationRefMut::Subclassification(inner.as_ref_mut())
            }
            Specialization::Subsetting(inner) => {
                SpecializationRefMut::Subsetting(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> SpecializationRefMut<'a> {
    pub fn as_ref(self) -> SpecializationRef<'a> {
        match self {
            SpecializationRefMut::Itself(inner) => SpecializationRef::Itself(inner),
            SpecializationRefMut::FeatureTyping(inner) => {
                SpecializationRef::FeatureTyping(inner.as_ref())
            }
            SpecializationRefMut::Subclassification(inner) => {
                SpecializationRef::Subclassification(inner.as_ref())
            }
            SpecializationRefMut::Subsetting(inner) => {
                SpecializationRef::Subsetting(inner.as_ref())
            }
        }
    }
}
impl SpecializationStruct for Specialization {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.general(),
            Specialization::FeatureTyping(x) => x.general(),
            Specialization::Subclassification(x) => x.general(),
            Specialization::Subsetting(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.specific(),
            Specialization::FeatureTyping(x) => x.specific(),
            Specialization::Subclassification(x) => x.specific(),
            Specialization::Subsetting(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for Specialization {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.general_ref_mut(),
            Specialization::FeatureTyping(x) => x.general_ref_mut(),
            Specialization::Subclassification(x) => x.general_ref_mut(),
            Specialization::Subsetting(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.specific_ref_mut(),
            Specialization::FeatureTyping(x) => x.specific_ref_mut(),
            Specialization::Subclassification(x) => x.specific_ref_mut(),
            Specialization::Subsetting(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for Specialization {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.general_ref(),
            Specialization::FeatureTyping(x) => x.general_ref(),
            Specialization::Subclassification(x) => x.general_ref(),
            Specialization::Subsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Specialization::Itself(x) => x.specific_ref(),
            Specialization::FeatureTyping(x) => x.specific_ref(),
            Specialization::Subclassification(x) => x.specific_ref(),
            Specialization::Subsetting(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for SpecializationRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRefMut::Itself(x) => x.general_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.general_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.general_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRefMut::Itself(x) => x.specific_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.specific_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.specific_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for SpecializationRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRefMut::Itself(x) => x.general_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.general_ref(),
            SpecializationRefMut::Subclassification(x) => x.general_ref(),
            SpecializationRefMut::Subsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRefMut::Itself(x) => x.specific_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.specific_ref(),
            SpecializationRefMut::Subclassification(x) => x.specific_ref(),
            SpecializationRefMut::Subsetting(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for SpecializationRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRef::Itself(x) => x.general_ref(),
            SpecializationRef::FeatureTyping(x) => x.general_ref(),
            SpecializationRef::Subclassification(x) => x.general_ref(),
            SpecializationRef::Subsetting(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SpecializationRef::Itself(x) => x.specific_ref(),
            SpecializationRef::FeatureTyping(x) => x.specific_ref(),
            SpecializationRef::Subclassification(x) => x.specific_ref(),
            SpecializationRef::Subsetting(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for Specialization {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.target(),
            Specialization::FeatureTyping(x) => x.target(),
            Specialization::Subclassification(x) => x.target(),
            Specialization::Subsetting(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.source(),
            Specialization::FeatureTyping(x) => x.source(),
            Specialization::Subclassification(x) => x.source(),
            Specialization::Subsetting(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owning_related_element(),
            Specialization::FeatureTyping(x) => x.owning_related_element(),
            Specialization::Subclassification(x) => x.owning_related_element(),
            Specialization::Subsetting(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owned_related_element(),
            Specialization::FeatureTyping(x) => x.owned_related_element(),
            Specialization::Subclassification(x) => x.owned_related_element(),
            Specialization::Subsetting(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Specialization::Itself(x) => x.is_implied(),
            Specialization::FeatureTyping(x) => x.is_implied(),
            Specialization::Subclassification(x) => x.is_implied(),
            Specialization::Subsetting(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Specialization {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.target_ref_mut(),
            Specialization::FeatureTyping(x) => x.target_ref_mut(),
            Specialization::Subclassification(x) => x.target_ref_mut(),
            Specialization::Subsetting(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.source_ref_mut(),
            Specialization::FeatureTyping(x) => x.source_ref_mut(),
            Specialization::Subclassification(x) => x.source_ref_mut(),
            Specialization::Subsetting(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owning_related_element_ref_mut(),
            Specialization::FeatureTyping(x) => x.owning_related_element_ref_mut(),
            Specialization::Subclassification(x) => x.owning_related_element_ref_mut(),
            Specialization::Subsetting(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owned_related_element_ref_mut(),
            Specialization::FeatureTyping(x) => x.owned_related_element_ref_mut(),
            Specialization::Subclassification(x) => x.owned_related_element_ref_mut(),
            Specialization::Subsetting(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Specialization::Itself(x) => x.is_implied_ref_mut(),
            Specialization::FeatureTyping(x) => x.is_implied_ref_mut(),
            Specialization::Subclassification(x) => x.is_implied_ref_mut(),
            Specialization::Subsetting(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Specialization {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.target_ref(),
            Specialization::FeatureTyping(x) => x.target_ref(),
            Specialization::Subclassification(x) => x.target_ref(),
            Specialization::Subsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.source_ref(),
            Specialization::FeatureTyping(x) => x.source_ref(),
            Specialization::Subclassification(x) => x.source_ref(),
            Specialization::Subsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owning_related_element_ref(),
            Specialization::FeatureTyping(x) => x.owning_related_element_ref(),
            Specialization::Subclassification(x) => x.owning_related_element_ref(),
            Specialization::Subsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Specialization::Itself(x) => x.owned_related_element_ref(),
            Specialization::FeatureTyping(x) => x.owned_related_element_ref(),
            Specialization::Subclassification(x) => x.owned_related_element_ref(),
            Specialization::Subsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Specialization::Itself(x) => x.is_implied_ref(),
            Specialization::FeatureTyping(x) => x.is_implied_ref(),
            Specialization::Subclassification(x) => x.is_implied_ref(),
            Specialization::Subsetting(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for SpecializationRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.target_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.target_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.target_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.source_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.source_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.source_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.owning_related_element_ref_mut(),
            SpecializationRefMut::Subclassification(x) => {
                x.owning_related_element_ref_mut()
            }
            SpecializationRefMut::Subsetting(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.owned_related_element_ref_mut(),
            SpecializationRefMut::Subclassification(x) => {
                x.owned_related_element_ref_mut()
            }
            SpecializationRefMut::Subsetting(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SpecializationRefMut::Itself(x) => x.is_implied_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.is_implied_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.is_implied_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for SpecializationRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.target_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.target_ref(),
            SpecializationRefMut::Subclassification(x) => x.target_ref(),
            SpecializationRefMut::Subsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.source_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.source_ref(),
            SpecializationRefMut::Subclassification(x) => x.source_ref(),
            SpecializationRefMut::Subsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owning_related_element_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.owning_related_element_ref(),
            SpecializationRefMut::Subclassification(x) => x.owning_related_element_ref(),
            SpecializationRefMut::Subsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owned_related_element_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.owned_related_element_ref(),
            SpecializationRefMut::Subclassification(x) => x.owned_related_element_ref(),
            SpecializationRefMut::Subsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SpecializationRefMut::Itself(x) => x.is_implied_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.is_implied_ref(),
            SpecializationRefMut::Subclassification(x) => x.is_implied_ref(),
            SpecializationRefMut::Subsetting(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for SpecializationRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRef::Itself(x) => x.target_ref(),
            SpecializationRef::FeatureTyping(x) => x.target_ref(),
            SpecializationRef::Subclassification(x) => x.target_ref(),
            SpecializationRef::Subsetting(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRef::Itself(x) => x.source_ref(),
            SpecializationRef::FeatureTyping(x) => x.source_ref(),
            SpecializationRef::Subclassification(x) => x.source_ref(),
            SpecializationRef::Subsetting(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRef::Itself(x) => x.owning_related_element_ref(),
            SpecializationRef::FeatureTyping(x) => x.owning_related_element_ref(),
            SpecializationRef::Subclassification(x) => x.owning_related_element_ref(),
            SpecializationRef::Subsetting(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SpecializationRef::Itself(x) => x.owned_related_element_ref(),
            SpecializationRef::FeatureTyping(x) => x.owned_related_element_ref(),
            SpecializationRef::Subclassification(x) => x.owned_related_element_ref(),
            SpecializationRef::Subsetting(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SpecializationRef::Itself(x) => x.is_implied_ref(),
            SpecializationRef::FeatureTyping(x) => x.is_implied_ref(),
            SpecializationRef::Subclassification(x) => x.is_implied_ref(),
            SpecializationRef::Subsetting(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Specialization {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Specialization::Itself(x) => x.owned_relationship(),
            Specialization::FeatureTyping(x) => x.owned_relationship(),
            Specialization::Subclassification(x) => x.owned_relationship(),
            Specialization::Subsetting(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Specialization::Itself(x) => x.owning_relationship(),
            Specialization::FeatureTyping(x) => x.owning_relationship(),
            Specialization::Subclassification(x) => x.owning_relationship(),
            Specialization::Subsetting(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Specialization::Itself(x) => x.element_id(),
            Specialization::FeatureTyping(x) => x.element_id(),
            Specialization::Subclassification(x) => x.element_id(),
            Specialization::Subsetting(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Specialization::Itself(x) => x.alias_ids(),
            Specialization::FeatureTyping(x) => x.alias_ids(),
            Specialization::Subclassification(x) => x.alias_ids(),
            Specialization::Subsetting(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_short_name(),
            Specialization::FeatureTyping(x) => x.declared_short_name(),
            Specialization::Subclassification(x) => x.declared_short_name(),
            Specialization::Subsetting(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_name(),
            Specialization::FeatureTyping(x) => x.declared_name(),
            Specialization::Subclassification(x) => x.declared_name(),
            Specialization::Subsetting(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Specialization::Itself(x) => x.is_implied_included(),
            Specialization::FeatureTyping(x) => x.is_implied_included(),
            Specialization::Subclassification(x) => x.is_implied_included(),
            Specialization::Subsetting(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Specialization {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Specialization::Itself(x) => x.owned_relationship_ref_mut(),
            Specialization::FeatureTyping(x) => x.owned_relationship_ref_mut(),
            Specialization::Subclassification(x) => x.owned_relationship_ref_mut(),
            Specialization::Subsetting(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Specialization::Itself(x) => x.owning_relationship_ref_mut(),
            Specialization::FeatureTyping(x) => x.owning_relationship_ref_mut(),
            Specialization::Subclassification(x) => x.owning_relationship_ref_mut(),
            Specialization::Subsetting(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Specialization::Itself(x) => x.element_id_ref_mut(),
            Specialization::FeatureTyping(x) => x.element_id_ref_mut(),
            Specialization::Subclassification(x) => x.element_id_ref_mut(),
            Specialization::Subsetting(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Specialization::Itself(x) => x.alias_ids_ref_mut(),
            Specialization::FeatureTyping(x) => x.alias_ids_ref_mut(),
            Specialization::Subclassification(x) => x.alias_ids_ref_mut(),
            Specialization::Subsetting(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_short_name_ref_mut(),
            Specialization::FeatureTyping(x) => x.declared_short_name_ref_mut(),
            Specialization::Subclassification(x) => x.declared_short_name_ref_mut(),
            Specialization::Subsetting(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_name_ref_mut(),
            Specialization::FeatureTyping(x) => x.declared_name_ref_mut(),
            Specialization::Subclassification(x) => x.declared_name_ref_mut(),
            Specialization::Subsetting(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Specialization::Itself(x) => x.is_implied_included_ref_mut(),
            Specialization::FeatureTyping(x) => x.is_implied_included_ref_mut(),
            Specialization::Subclassification(x) => x.is_implied_included_ref_mut(),
            Specialization::Subsetting(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Specialization {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Specialization::Itself(x) => x.owned_relationship_ref(),
            Specialization::FeatureTyping(x) => x.owned_relationship_ref(),
            Specialization::Subclassification(x) => x.owned_relationship_ref(),
            Specialization::Subsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Specialization::Itself(x) => x.owning_relationship_ref(),
            Specialization::FeatureTyping(x) => x.owning_relationship_ref(),
            Specialization::Subclassification(x) => x.owning_relationship_ref(),
            Specialization::Subsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Specialization::Itself(x) => x.element_id_ref(),
            Specialization::FeatureTyping(x) => x.element_id_ref(),
            Specialization::Subclassification(x) => x.element_id_ref(),
            Specialization::Subsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Specialization::Itself(x) => x.alias_ids_ref(),
            Specialization::FeatureTyping(x) => x.alias_ids_ref(),
            Specialization::Subclassification(x) => x.alias_ids_ref(),
            Specialization::Subsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_short_name_ref(),
            Specialization::FeatureTyping(x) => x.declared_short_name_ref(),
            Specialization::Subclassification(x) => x.declared_short_name_ref(),
            Specialization::Subsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Specialization::Itself(x) => x.declared_name_ref(),
            Specialization::FeatureTyping(x) => x.declared_name_ref(),
            Specialization::Subclassification(x) => x.declared_name_ref(),
            Specialization::Subsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Specialization::Itself(x) => x.is_implied_included_ref(),
            Specialization::FeatureTyping(x) => x.is_implied_included_ref(),
            Specialization::Subclassification(x) => x.is_implied_included_ref(),
            Specialization::Subsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SpecializationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.owned_relationship_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.owned_relationship_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SpecializationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.owning_relationship_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.owning_relationship_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SpecializationRefMut::Itself(x) => x.element_id_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.element_id_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.element_id_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.alias_ids_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.alias_ids_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.alias_ids_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.declared_short_name_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.declared_short_name_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.declared_name_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.declared_name_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.declared_name_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SpecializationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            SpecializationRefMut::FeatureTyping(x) => x.is_implied_included_ref_mut(),
            SpecializationRefMut::Subclassification(x) => x.is_implied_included_ref_mut(),
            SpecializationRefMut::Subsetting(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SpecializationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owned_relationship_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.owned_relationship_ref(),
            SpecializationRefMut::Subclassification(x) => x.owned_relationship_ref(),
            SpecializationRefMut::Subsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SpecializationRefMut::Itself(x) => x.owning_relationship_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.owning_relationship_ref(),
            SpecializationRefMut::Subclassification(x) => x.owning_relationship_ref(),
            SpecializationRefMut::Subsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SpecializationRefMut::Itself(x) => x.element_id_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.element_id_ref(),
            SpecializationRefMut::Subclassification(x) => x.element_id_ref(),
            SpecializationRefMut::Subsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.alias_ids_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.alias_ids_ref(),
            SpecializationRefMut::Subclassification(x) => x.alias_ids_ref(),
            SpecializationRefMut::Subsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.declared_short_name_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.declared_short_name_ref(),
            SpecializationRefMut::Subclassification(x) => x.declared_short_name_ref(),
            SpecializationRefMut::Subsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SpecializationRefMut::Itself(x) => x.declared_name_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.declared_name_ref(),
            SpecializationRefMut::Subclassification(x) => x.declared_name_ref(),
            SpecializationRefMut::Subsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SpecializationRefMut::Itself(x) => x.is_implied_included_ref(),
            SpecializationRefMut::FeatureTyping(x) => x.is_implied_included_ref(),
            SpecializationRefMut::Subclassification(x) => x.is_implied_included_ref(),
            SpecializationRefMut::Subsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SpecializationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SpecializationRef::Itself(x) => x.owned_relationship_ref(),
            SpecializationRef::FeatureTyping(x) => x.owned_relationship_ref(),
            SpecializationRef::Subclassification(x) => x.owned_relationship_ref(),
            SpecializationRef::Subsetting(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SpecializationRef::Itself(x) => x.owning_relationship_ref(),
            SpecializationRef::FeatureTyping(x) => x.owning_relationship_ref(),
            SpecializationRef::Subclassification(x) => x.owning_relationship_ref(),
            SpecializationRef::Subsetting(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SpecializationRef::Itself(x) => x.element_id_ref(),
            SpecializationRef::FeatureTyping(x) => x.element_id_ref(),
            SpecializationRef::Subclassification(x) => x.element_id_ref(),
            SpecializationRef::Subsetting(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SpecializationRef::Itself(x) => x.alias_ids_ref(),
            SpecializationRef::FeatureTyping(x) => x.alias_ids_ref(),
            SpecializationRef::Subclassification(x) => x.alias_ids_ref(),
            SpecializationRef::Subsetting(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SpecializationRef::Itself(x) => x.declared_short_name_ref(),
            SpecializationRef::FeatureTyping(x) => x.declared_short_name_ref(),
            SpecializationRef::Subclassification(x) => x.declared_short_name_ref(),
            SpecializationRef::Subsetting(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SpecializationRef::Itself(x) => x.declared_name_ref(),
            SpecializationRef::FeatureTyping(x) => x.declared_name_ref(),
            SpecializationRef::Subclassification(x) => x.declared_name_ref(),
            SpecializationRef::Subsetting(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SpecializationRef::Itself(x) => x.is_implied_included_ref(),
            SpecializationRef::FeatureTyping(x) => x.is_implied_included_ref(),
            SpecializationRef::Subclassification(x) => x.is_implied_included_ref(),
            SpecializationRef::Subsetting(x) => x.is_implied_included_ref(),
        }
    }
}
impl SpecializationUpcast for Specialization {
    fn into_specialization(self) -> Specialization {
        self
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for SpecializationRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        self
    }
}
impl<'a> SpecializationUpcastRef<'a> for SpecializationRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        self
    }
}
impl RelationshipUpcast for Specialization {
    fn into_relationship(self) -> Relationship {
        Relationship::Specialization(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for SpecializationRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Specialization(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for SpecializationRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Specialization(self).as_relationship_ref()
    }
}
impl ElementUpcast for Specialization {
    fn into_element(self) -> Element {
        Relationship::Specialization(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SpecializationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Specialization(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SpecializationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Specialization(self).as_element_ref()
    }
}
pub trait SpecializationDowncast {
    fn try_into_feature_typing(self) -> Result<FeatureTyping, String>;
    fn try_into_subclassification(self) -> Result<Subclassification, String>;
    fn try_into_subsetting(self) -> Result<Subsetting, String>;
}
pub trait SpecializationDowncastRefMut<'a> {
    fn try_as_feature_typing_ref_mut(self) -> Result<FeatureTypingRefMut<'a>, String>;
    fn try_as_subclassification_ref_mut(
        self,
    ) -> Result<SubclassificationRefMut<'a>, String>;
    fn try_as_subsetting_ref_mut(self) -> Result<SubsettingRefMut<'a>, String>;
}
pub trait SpecializationDowncastRef<'a> {
    fn try_as_feature_typing_ref(self) -> Result<FeatureTypingRef<'a>, String>;
    fn try_as_subclassification_ref(self) -> Result<SubclassificationRef<'a>, String>;
    fn try_as_subsetting_ref(self) -> Result<SubsettingRef<'a>, String>;
}
impl SpecializationDowncast for Specialization {
    fn try_into_feature_typing(self) -> Result<FeatureTyping, String> {
        match self {
            Specialization::FeatureTyping(e) => Ok(e),
            _ => Err("Not a FeatureTyping".into()),
        }
    }
    fn try_into_subclassification(self) -> Result<Subclassification, String> {
        match self {
            Specialization::Subclassification(e) => Ok(e),
            _ => Err("Not a Subclassification".into()),
        }
    }
    fn try_into_subsetting(self) -> Result<Subsetting, String> {
        match self {
            Specialization::Subsetting(e) => Ok(e),
            _ => Err("Not a Subsetting".into()),
        }
    }
}
impl<'a> SpecializationDowncastRefMut<'a> for SpecializationRefMut<'a> {
    fn try_as_feature_typing_ref_mut(self) -> Result<FeatureTypingRefMut<'a>, String> {
        match self {
            SpecializationRefMut::FeatureTyping(e) => Ok(e),
            _ => Err("Not a FeatureTyping".into()),
        }
    }
    fn try_as_subclassification_ref_mut(
        self,
    ) -> Result<SubclassificationRefMut<'a>, String> {
        match self {
            SpecializationRefMut::Subclassification(e) => Ok(e),
            _ => Err("Not a Subclassification".into()),
        }
    }
    fn try_as_subsetting_ref_mut(self) -> Result<SubsettingRefMut<'a>, String> {
        match self {
            SpecializationRefMut::Subsetting(e) => Ok(e),
            _ => Err("Not a Subsetting".into()),
        }
    }
}
impl<'a> SpecializationDowncastRef<'a> for SpecializationRef<'a> {
    fn try_as_feature_typing_ref(self) -> Result<FeatureTypingRef<'a>, String> {
        match self {
            SpecializationRef::FeatureTyping(e) => Ok(e),
            _ => Err("Not a FeatureTyping".into()),
        }
    }
    fn try_as_subclassification_ref(self) -> Result<SubclassificationRef<'a>, String> {
        match self {
            SpecializationRef::Subclassification(e) => Ok(e),
            _ => Err("Not a Subclassification".into()),
        }
    }
    fn try_as_subsetting_ref(self) -> Result<SubsettingRef<'a>, String> {
        match self {
            SpecializationRef::Subsetting(e) => Ok(e),
            _ => Err("Not a Subsetting".into()),
        }
    }
}
pub trait SpecializationMethodsDescendants
where
    Self: DescendantOf<Specialization>,
    Self::Via: SpecializationMethods,
    Self: Sized,
{}
pub trait SpecializationMethods: Sized {}
impl<T: SpecializationMethodsDescendants> SpecializationMethods for T
where
    T::Via: SpecializationMethods,
{}
impl DescendantOf<Relationship> for Specialization {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Specialization {}
impl DescendantOf<Element> for Specialization {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Specialization {}
pub trait SpecializationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SpecializationRefMut<'a>>,
    Self::Via: SpecializationRefMutMethods,
    Self: Sized,
{}
pub trait SpecializationRefMutMethods: Sized {}
impl<'a, T: SpecializationRefMutMethodsDescendants<'a>> SpecializationRefMutMethods for T
where
    T::Via: SpecializationRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for SpecializationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for SpecializationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SpecializationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SpecializationRefMut<'a> {}
pub trait SpecializationRefMethodsDescendants<'a>
where
    Self: DescendantOf<SpecializationRef<'a>>,
    Self::Via: SpecializationRefMethods,
    Self: Sized,
{}
pub trait SpecializationRefMethods: Sized {}
impl<'a, T: SpecializationRefMethodsDescendants<'a>> SpecializationRefMethods for T
where
    T::Via: SpecializationRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for SpecializationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for SpecializationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SpecializationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SpecializationRef<'a> {}

