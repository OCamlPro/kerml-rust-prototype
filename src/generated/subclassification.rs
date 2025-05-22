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
pub struct SubclassificationInner {
    pub(super) sup_specialization: SpecializationInner,
    pub(super) superclassifier: std::rc::Rc<
        std::cell::RefCell<super::classifier::Classifier>,
    >,
    pub(super) subclassifier: std::rc::Rc<
        std::cell::RefCell<super::classifier::Classifier>,
    >,
}
pub trait SubclassificationStruct
where
    Self: SubclassificationStructRefMut,
    Self: SubclassificationStructRef,
    Self: SpecializationStruct,
{
    fn superclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
    fn subclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
}
pub trait SubclassificationStructRefMut
where
    Self: SubclassificationStructRef,
    Self: SpecializationStructRefMut,
{
    fn superclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
    fn subclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
}
pub trait SubclassificationStructRef
where
    Self: SpecializationStructRef,
{
    fn superclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
    fn subclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>>;
}
pub trait SubclassificationUpcast: SubclassificationStruct {
    fn into_subclassification(self) -> Subclassification;
}
pub trait SubclassificationUpcastRefMut<'a>: SubclassificationStructRefMut {
    fn as_subclassification_ref_mut(self) -> SubclassificationRefMut<'a>;
}
pub trait SubclassificationUpcastRef<'a>: SubclassificationStructRef {
    fn as_subclassification_ref(self) -> SubclassificationRef<'a>;
}
impl SubclassificationStruct for SubclassificationInner {
    fn superclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        self.superclassifier
    }
    fn subclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        self.subclassifier
    }
}
impl SubclassificationStructRefMut for SubclassificationInner {
    fn superclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        &mut self.superclassifier
    }
    fn subclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        &mut self.subclassifier
    }
}
impl SubclassificationStructRef for SubclassificationInner {
    fn superclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        &self.superclassifier
    }
    fn subclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        &self.subclassifier
    }
}
impl SpecializationStruct for SubclassificationInner {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general()
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific()
    }
}
impl SpecializationStructRefMut for SubclassificationInner {
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
impl SpecializationStructRef for SubclassificationInner {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.general_ref()
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.sup_specialization.specific_ref()
    }
}
impl RelationshipStruct for SubclassificationInner {
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
impl RelationshipStructRefMut for SubclassificationInner {
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
impl RelationshipStructRef for SubclassificationInner {
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
impl ElementStruct for SubclassificationInner {
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
impl ElementStructRefMut for SubclassificationInner {
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
impl ElementStructRef for SubclassificationInner {
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
pub enum Subclassification {
    Itself(SubclassificationInner),
}
pub enum SubclassificationRefMut<'a> {
    Itself(&'a mut SubclassificationInner),
}
pub enum SubclassificationRef<'a> {
    Itself(&'a SubclassificationInner),
}
impl Subclassification {
    pub fn as_ref(&self) -> SubclassificationRef {
        match self {
            Subclassification::Itself(inner) => SubclassificationRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> SubclassificationRefMut {
        match self {
            Subclassification::Itself(inner) => SubclassificationRefMut::Itself(inner),
        }
    }
}
impl<'a> SubclassificationRefMut<'a> {
    pub fn as_ref(self) -> SubclassificationRef<'a> {
        match self {
            SubclassificationRefMut::Itself(inner) => SubclassificationRef::Itself(inner),
        }
    }
}
impl SubclassificationStruct for Subclassification {
    fn superclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.superclassifier(),
        }
    }
    fn subclassifier(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.subclassifier(),
        }
    }
}
impl SubclassificationStructRefMut for Subclassification {
    fn superclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.superclassifier_ref_mut(),
        }
    }
    fn subclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.subclassifier_ref_mut(),
        }
    }
}
impl SubclassificationStructRef for Subclassification {
    fn superclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.superclassifier_ref(),
        }
    }
    fn subclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            Subclassification::Itself(x) => x.subclassifier_ref(),
        }
    }
}
impl<'a> SubclassificationStructRefMut for SubclassificationRefMut<'a> {
    fn superclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.superclassifier_ref_mut(),
        }
    }
    fn subclassifier_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.subclassifier_ref_mut(),
        }
    }
}
impl<'a> SubclassificationStructRef for SubclassificationRefMut<'a> {
    fn superclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.superclassifier_ref(),
        }
    }
    fn subclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.subclassifier_ref(),
        }
    }
}
impl<'a> SubclassificationStructRef for SubclassificationRef<'a> {
    fn superclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRef::Itself(x) => x.superclassifier_ref(),
        }
    }
    fn subclassifier_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::classifier::Classifier>> {
        match self {
            SubclassificationRef::Itself(x) => x.subclassifier_ref(),
        }
    }
}
impl SpecializationStruct for Subclassification {
    fn general(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.general(),
        }
    }
    fn specific(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.specific(),
        }
    }
}
impl SpecializationStructRefMut for Subclassification {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl SpecializationStructRef for Subclassification {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Subclassification::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRefMut for SubclassificationRefMut<'a> {
    fn general_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.general_ref_mut(),
        }
    }
    fn specific_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.specific_ref_mut(),
        }
    }
}
impl<'a> SpecializationStructRef for SubclassificationRefMut<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.specific_ref(),
        }
    }
}
impl<'a> SpecializationStructRef for SubclassificationRef<'a> {
    fn general_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRef::Itself(x) => x.general_ref(),
        }
    }
    fn specific_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            SubclassificationRef::Itself(x) => x.specific_ref(),
        }
    }
}
impl RelationshipStruct for Subclassification {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Subclassification::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Subclassification {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Subclassification::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Subclassification {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Subclassification::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Subclassification::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for SubclassificationRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SubclassificationRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for SubclassificationRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SubclassificationRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for SubclassificationRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SubclassificationRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SubclassificationRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Subclassification {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subclassification::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subclassification::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Subclassification::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Subclassification::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Subclassification::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Subclassification {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subclassification::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Subclassification::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Subclassification::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Subclassification::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Subclassification::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Subclassification {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subclassification::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Subclassification::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Subclassification::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Subclassification::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Subclassification::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Subclassification::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SubclassificationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SubclassificationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SubclassificationRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SubclassificationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SubclassificationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubclassificationRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SubclassificationRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SubclassificationRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SubclassificationRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SubclassificationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubclassificationRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SubclassificationRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SubclassificationRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SubclassificationRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SubclassificationRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SubclassificationRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SubclassificationRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl SubclassificationUpcast for Subclassification {
    fn into_subclassification(self) -> Subclassification {
        self
    }
}
impl<'a> SubclassificationUpcastRefMut<'a> for SubclassificationRefMut<'a> {
    fn as_subclassification_ref_mut(self) -> SubclassificationRefMut<'a> {
        self
    }
}
impl<'a> SubclassificationUpcastRef<'a> for SubclassificationRef<'a> {
    fn as_subclassification_ref(self) -> SubclassificationRef<'a> {
        self
    }
}
impl SpecializationUpcast for Subclassification {
    fn into_specialization(self) -> Specialization {
        Specialization::Subclassification(self).into_specialization()
    }
}
impl<'a> SpecializationUpcastRefMut<'a> for SubclassificationRefMut<'a> {
    fn as_specialization_ref_mut(self) -> SpecializationRefMut<'a> {
        SpecializationRefMut::Subclassification(self).as_specialization_ref_mut()
    }
}
impl<'a> SpecializationUpcastRef<'a> for SubclassificationRef<'a> {
    fn as_specialization_ref(self) -> SpecializationRef<'a> {
        SpecializationRef::Subclassification(self).as_specialization_ref()
    }
}
impl RelationshipUpcast for Subclassification {
    fn into_relationship(self) -> Relationship {
        Specialization::Subclassification(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for SubclassificationRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        SpecializationRefMut::Subclassification(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for SubclassificationRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        SpecializationRef::Subclassification(self).as_relationship_ref()
    }
}
impl ElementUpcast for Subclassification {
    fn into_element(self) -> Element {
        Specialization::Subclassification(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SubclassificationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        SpecializationRefMut::Subclassification(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SubclassificationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        SpecializationRef::Subclassification(self).as_element_ref()
    }
}
pub trait SubclassificationDowncast {}
pub trait SubclassificationDowncastRefMut<'a> {}
pub trait SubclassificationDowncastRef<'a> {}
impl SubclassificationDowncast for Subclassification {}
impl<'a> SubclassificationDowncastRefMut<'a> for SubclassificationRefMut<'a> {}
impl<'a> SubclassificationDowncastRef<'a> for SubclassificationRef<'a> {}
pub trait SubclassificationMethodsDescendants
where
    Self: DescendantOf<Subclassification>,
    Self::Via: SubclassificationMethods,
    Self: Sized,
{}
pub trait SubclassificationMethods: Sized {}
impl<T: SubclassificationMethodsDescendants> SubclassificationMethods for T
where
    T::Via: SubclassificationMethods,
{}
impl DescendantOf<Specialization> for Subclassification {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl SpecializationMethodsDescendants for Subclassification {}
impl DescendantOf<Relationship> for Subclassification {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl RelationshipMethodsDescendants for Subclassification {}
impl DescendantOf<Element> for Subclassification {
    type Via = Specialization;
    fn into_via(self) -> Self::Via {
        self.into_specialization()
    }
}
impl ElementMethodsDescendants for Subclassification {}
pub trait SubclassificationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SubclassificationRefMut<'a>>,
    Self::Via: SubclassificationRefMutMethods,
    Self: Sized,
{}
pub trait SubclassificationRefMutMethods: Sized {}
impl<'a, T: SubclassificationRefMutMethodsDescendants<'a>> SubclassificationRefMutMethods
for T
where
    T::Via: SubclassificationRefMutMethods,
{}
impl<'a> DescendantOf<SpecializationRefMut<'a>> for SubclassificationRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> SpecializationRefMutMethodsDescendants<'a> for SubclassificationRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for SubclassificationRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for SubclassificationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SubclassificationRefMut<'a> {
    type Via = SpecializationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SubclassificationRefMut<'a> {}
pub trait SubclassificationRefMethodsDescendants<'a>
where
    Self: DescendantOf<SubclassificationRef<'a>>,
    Self::Via: SubclassificationRefMethods,
    Self: Sized,
{}
pub trait SubclassificationRefMethods: Sized {}
impl<'a, T: SubclassificationRefMethodsDescendants<'a>> SubclassificationRefMethods for T
where
    T::Via: SubclassificationRefMethods,
{}
impl<'a> DescendantOf<SpecializationRef<'a>> for SubclassificationRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> SpecializationRefMethodsDescendants<'a> for SubclassificationRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for SubclassificationRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for SubclassificationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SubclassificationRef<'a> {
    type Via = SpecializationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_specialization_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SubclassificationRef<'a> {}

