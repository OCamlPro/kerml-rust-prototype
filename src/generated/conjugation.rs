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
pub struct ConjugationInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) original_type: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    pub(super) conjugated_type: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait ConjugationStruct
where
    Self: ConjugationStructRefMut,
    Self: ConjugationStructRef,
    Self: RelationshipStruct,
{
    fn original_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn conjugated_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait ConjugationStructRefMut
where
    Self: ConjugationStructRef,
    Self: RelationshipStructRefMut,
{
    fn original_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn conjugated_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait ConjugationStructRef
where
    Self: RelationshipStructRef,
{
    fn original_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn conjugated_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait ConjugationUpcast: ConjugationStruct {
    fn into_conjugation(self) -> Conjugation;
}
pub trait ConjugationUpcastRefMut<'a>: ConjugationStructRefMut {
    fn as_conjugation_ref_mut(self) -> ConjugationRefMut<'a>;
}
pub trait ConjugationUpcastRef<'a>: ConjugationStructRef {
    fn as_conjugation_ref(self) -> ConjugationRef<'a>;
}
impl ConjugationStruct for ConjugationInner {
    fn original_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.original_type
    }
    fn conjugated_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.conjugated_type
    }
}
impl ConjugationStructRefMut for ConjugationInner {
    fn original_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.original_type
    }
    fn conjugated_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.conjugated_type
    }
}
impl ConjugationStructRef for ConjugationInner {
    fn original_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.original_type
    }
    fn conjugated_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.conjugated_type
    }
}
impl RelationshipStruct for ConjugationInner {
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
impl RelationshipStructRefMut for ConjugationInner {
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
impl RelationshipStructRef for ConjugationInner {
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
impl ElementStruct for ConjugationInner {
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
impl ElementStructRefMut for ConjugationInner {
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
impl ElementStructRef for ConjugationInner {
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
pub enum Conjugation {
    Itself(ConjugationInner),
}
pub enum ConjugationRefMut<'a> {
    Itself(&'a mut ConjugationInner),
}
pub enum ConjugationRef<'a> {
    Itself(&'a ConjugationInner),
}
impl Conjugation {
    pub fn as_ref(&self) -> ConjugationRef {
        match self {
            Conjugation::Itself(inner) => ConjugationRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> ConjugationRefMut {
        match self {
            Conjugation::Itself(inner) => ConjugationRefMut::Itself(inner),
        }
    }
}
impl<'a> ConjugationRefMut<'a> {
    pub fn as_ref(self) -> ConjugationRef<'a> {
        match self {
            ConjugationRefMut::Itself(inner) => ConjugationRef::Itself(inner),
        }
    }
}
impl ConjugationStruct for Conjugation {
    fn original_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.original_type(),
        }
    }
    fn conjugated_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.conjugated_type(),
        }
    }
}
impl ConjugationStructRefMut for Conjugation {
    fn original_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.original_type_ref_mut(),
        }
    }
    fn conjugated_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.conjugated_type_ref_mut(),
        }
    }
}
impl ConjugationStructRef for Conjugation {
    fn original_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.original_type_ref(),
        }
    }
    fn conjugated_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Conjugation::Itself(x) => x.conjugated_type_ref(),
        }
    }
}
impl<'a> ConjugationStructRefMut for ConjugationRefMut<'a> {
    fn original_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRefMut::Itself(x) => x.original_type_ref_mut(),
        }
    }
    fn conjugated_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRefMut::Itself(x) => x.conjugated_type_ref_mut(),
        }
    }
}
impl<'a> ConjugationStructRef for ConjugationRefMut<'a> {
    fn original_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRefMut::Itself(x) => x.original_type_ref(),
        }
    }
    fn conjugated_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRefMut::Itself(x) => x.conjugated_type_ref(),
        }
    }
}
impl<'a> ConjugationStructRef for ConjugationRef<'a> {
    fn original_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRef::Itself(x) => x.original_type_ref(),
        }
    }
    fn conjugated_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            ConjugationRef::Itself(x) => x.conjugated_type_ref(),
        }
    }
}
impl RelationshipStruct for Conjugation {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Conjugation::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Conjugation {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Conjugation::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Conjugation {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Conjugation::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Conjugation::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ConjugationRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ConjugationRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ConjugationRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ConjugationRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ConjugationRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConjugationRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ConjugationRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Conjugation {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Conjugation::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Conjugation::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Conjugation::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Conjugation::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Conjugation::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Conjugation {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Conjugation::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Conjugation::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Conjugation::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Conjugation::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Conjugation::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Conjugation {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Conjugation::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Conjugation::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Conjugation::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Conjugation::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Conjugation::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Conjugation::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ConjugationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ConjugationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ConjugationRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ConjugationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ConjugationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConjugationRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConjugationRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConjugationRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConjugationRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ConjugationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConjugationRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConjugationRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConjugationRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConjugationRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConjugationRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConjugationRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConjugationRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ConjugationUpcast for Conjugation {
    fn into_conjugation(self) -> Conjugation {
        self
    }
}
impl<'a> ConjugationUpcastRefMut<'a> for ConjugationRefMut<'a> {
    fn as_conjugation_ref_mut(self) -> ConjugationRefMut<'a> {
        self
    }
}
impl<'a> ConjugationUpcastRef<'a> for ConjugationRef<'a> {
    fn as_conjugation_ref(self) -> ConjugationRef<'a> {
        self
    }
}
impl RelationshipUpcast for Conjugation {
    fn into_relationship(self) -> Relationship {
        Relationship::Conjugation(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ConjugationRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Conjugation(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ConjugationRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Conjugation(self).as_relationship_ref()
    }
}
impl ElementUpcast for Conjugation {
    fn into_element(self) -> Element {
        Relationship::Conjugation(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ConjugationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Conjugation(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ConjugationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Conjugation(self).as_element_ref()
    }
}
pub trait ConjugationDowncast {}
pub trait ConjugationDowncastRefMut<'a> {}
pub trait ConjugationDowncastRef<'a> {}
impl ConjugationDowncast for Conjugation {}
impl<'a> ConjugationDowncastRefMut<'a> for ConjugationRefMut<'a> {}
impl<'a> ConjugationDowncastRef<'a> for ConjugationRef<'a> {}
pub trait ConjugationMethodsDescendants
where
    Self: DescendantOf<Conjugation>,
    Self::Via: ConjugationMethods,
    Self: Sized,
{}
pub trait ConjugationMethods: Sized {}
impl<T: ConjugationMethodsDescendants> ConjugationMethods for T
where
    T::Via: ConjugationMethods,
{}
impl DescendantOf<Relationship> for Conjugation {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Conjugation {}
impl DescendantOf<Element> for Conjugation {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Conjugation {}
pub trait ConjugationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ConjugationRefMut<'a>>,
    Self::Via: ConjugationRefMutMethods,
    Self: Sized,
{}
pub trait ConjugationRefMutMethods: Sized {}
impl<'a, T: ConjugationRefMutMethodsDescendants<'a>> ConjugationRefMutMethods for T
where
    T::Via: ConjugationRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ConjugationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ConjugationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ConjugationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ConjugationRefMut<'a> {}
pub trait ConjugationRefMethodsDescendants<'a>
where
    Self: DescendantOf<ConjugationRef<'a>>,
    Self::Via: ConjugationRefMethods,
    Self: Sized,
{}
pub trait ConjugationRefMethods: Sized {}
impl<'a, T: ConjugationRefMethodsDescendants<'a>> ConjugationRefMethods for T
where
    T::Via: ConjugationRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for ConjugationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ConjugationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ConjugationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ConjugationRef<'a> {}

