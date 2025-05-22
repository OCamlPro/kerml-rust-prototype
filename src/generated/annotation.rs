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
pub struct AnnotationInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) annotated_element: std::rc::Rc<
        std::cell::RefCell<super::element::Element>,
    >,
}
pub trait AnnotationStruct
where
    Self: AnnotationStructRefMut,
    Self: AnnotationStructRef,
    Self: RelationshipStruct,
{
    fn annotated_element(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::element::Element>>;
}
pub trait AnnotationStructRefMut
where
    Self: AnnotationStructRef,
    Self: RelationshipStructRefMut,
{
    fn annotated_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>>;
}
pub trait AnnotationStructRef
where
    Self: RelationshipStructRef,
{
    fn annotated_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>>;
}
pub trait AnnotationUpcast: AnnotationStruct {
    fn into_annotation(self) -> Annotation;
}
pub trait AnnotationUpcastRefMut<'a>: AnnotationStructRefMut {
    fn as_annotation_ref_mut(self) -> AnnotationRefMut<'a>;
}
pub trait AnnotationUpcastRef<'a>: AnnotationStructRef {
    fn as_annotation_ref(self) -> AnnotationRef<'a>;
}
impl AnnotationStruct for AnnotationInner {
    fn annotated_element(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.annotated_element
    }
}
impl AnnotationStructRefMut for AnnotationInner {
    fn annotated_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        &mut self.annotated_element
    }
}
impl AnnotationStructRef for AnnotationInner {
    fn annotated_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        &self.annotated_element
    }
}
impl RelationshipStruct for AnnotationInner {
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
impl RelationshipStructRefMut for AnnotationInner {
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
impl RelationshipStructRef for AnnotationInner {
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
impl ElementStruct for AnnotationInner {
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
impl ElementStructRefMut for AnnotationInner {
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
impl ElementStructRef for AnnotationInner {
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
pub enum Annotation {
    Itself(AnnotationInner),
}
pub enum AnnotationRefMut<'a> {
    Itself(&'a mut AnnotationInner),
}
pub enum AnnotationRef<'a> {
    Itself(&'a AnnotationInner),
}
impl Annotation {
    pub fn as_ref(&self) -> AnnotationRef {
        match self {
            Annotation::Itself(inner) => AnnotationRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> AnnotationRefMut {
        match self {
            Annotation::Itself(inner) => AnnotationRefMut::Itself(inner),
        }
    }
}
impl<'a> AnnotationRefMut<'a> {
    pub fn as_ref(self) -> AnnotationRef<'a> {
        match self {
            AnnotationRefMut::Itself(inner) => AnnotationRef::Itself(inner),
        }
    }
}
impl AnnotationStruct for Annotation {
    fn annotated_element(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Annotation::Itself(x) => x.annotated_element(),
        }
    }
}
impl AnnotationStructRefMut for Annotation {
    fn annotated_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Annotation::Itself(x) => x.annotated_element_ref_mut(),
        }
    }
}
impl AnnotationStructRef for Annotation {
    fn annotated_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Annotation::Itself(x) => x.annotated_element_ref(),
        }
    }
}
impl<'a> AnnotationStructRefMut for AnnotationRefMut<'a> {
    fn annotated_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            AnnotationRefMut::Itself(x) => x.annotated_element_ref_mut(),
        }
    }
}
impl<'a> AnnotationStructRef for AnnotationRefMut<'a> {
    fn annotated_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            AnnotationRefMut::Itself(x) => x.annotated_element_ref(),
        }
    }
}
impl<'a> AnnotationStructRef for AnnotationRef<'a> {
    fn annotated_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            AnnotationRef::Itself(x) => x.annotated_element_ref(),
        }
    }
}
impl RelationshipStruct for Annotation {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Annotation::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Annotation {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Annotation::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Annotation {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Annotation::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Annotation::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for AnnotationRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            AnnotationRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for AnnotationRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AnnotationRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for AnnotationRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AnnotationRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AnnotationRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Annotation {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Annotation::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Annotation::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Annotation::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Annotation::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Annotation::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Annotation {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Annotation::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Annotation::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Annotation::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Annotation::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Annotation::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Annotation {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Annotation::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Annotation::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Annotation::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Annotation::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Annotation::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Annotation::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for AnnotationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AnnotationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AnnotationRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AnnotationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for AnnotationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotationRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AnnotationRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AnnotationRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AnnotationRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for AnnotationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotationRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotationRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AnnotationRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AnnotationRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AnnotationRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AnnotationRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AnnotationRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl AnnotationUpcast for Annotation {
    fn into_annotation(self) -> Annotation {
        self
    }
}
impl<'a> AnnotationUpcastRefMut<'a> for AnnotationRefMut<'a> {
    fn as_annotation_ref_mut(self) -> AnnotationRefMut<'a> {
        self
    }
}
impl<'a> AnnotationUpcastRef<'a> for AnnotationRef<'a> {
    fn as_annotation_ref(self) -> AnnotationRef<'a> {
        self
    }
}
impl RelationshipUpcast for Annotation {
    fn into_relationship(self) -> Relationship {
        Relationship::Annotation(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for AnnotationRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Annotation(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for AnnotationRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Annotation(self).as_relationship_ref()
    }
}
impl ElementUpcast for Annotation {
    fn into_element(self) -> Element {
        Relationship::Annotation(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for AnnotationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Annotation(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for AnnotationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Annotation(self).as_element_ref()
    }
}
pub trait AnnotationDowncast {}
pub trait AnnotationDowncastRefMut<'a> {}
pub trait AnnotationDowncastRef<'a> {}
impl AnnotationDowncast for Annotation {}
impl<'a> AnnotationDowncastRefMut<'a> for AnnotationRefMut<'a> {}
impl<'a> AnnotationDowncastRef<'a> for AnnotationRef<'a> {}
pub trait AnnotationMethodsDescendants
where
    Self: DescendantOf<Annotation>,
    Self::Via: AnnotationMethods,
    Self: Sized,
{}
pub trait AnnotationMethods: Sized {}
impl<T: AnnotationMethodsDescendants> AnnotationMethods for T
where
    T::Via: AnnotationMethods,
{}
impl DescendantOf<Relationship> for Annotation {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Annotation {}
impl DescendantOf<Element> for Annotation {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Annotation {}
pub trait AnnotationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<AnnotationRefMut<'a>>,
    Self::Via: AnnotationRefMutMethods,
    Self: Sized,
{}
pub trait AnnotationRefMutMethods: Sized {}
impl<'a, T: AnnotationRefMutMethodsDescendants<'a>> AnnotationRefMutMethods for T
where
    T::Via: AnnotationRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for AnnotationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for AnnotationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for AnnotationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for AnnotationRefMut<'a> {}
pub trait AnnotationRefMethodsDescendants<'a>
where
    Self: DescendantOf<AnnotationRef<'a>>,
    Self::Via: AnnotationRefMethods,
    Self: Sized,
{}
pub trait AnnotationRefMethods: Sized {}
impl<'a, T: AnnotationRefMethodsDescendants<'a>> AnnotationRefMethods for T
where
    T::Via: AnnotationRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for AnnotationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for AnnotationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for AnnotationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for AnnotationRef<'a> {}

