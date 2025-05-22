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
pub struct IntersectingInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) intersecting_type: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait IntersectingStruct
where
    Self: IntersectingStructRefMut,
    Self: IntersectingStructRef,
    Self: RelationshipStruct,
{
    fn intersecting_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait IntersectingStructRefMut
where
    Self: IntersectingStructRef,
    Self: RelationshipStructRefMut,
{
    fn intersecting_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait IntersectingStructRef
where
    Self: RelationshipStructRef,
{
    fn intersecting_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait IntersectingUpcast: IntersectingStruct {
    fn into_intersecting(self) -> Intersecting;
}
pub trait IntersectingUpcastRefMut<'a>: IntersectingStructRefMut {
    fn as_intersecting_ref_mut(self) -> IntersectingRefMut<'a>;
}
pub trait IntersectingUpcastRef<'a>: IntersectingStructRef {
    fn as_intersecting_ref(self) -> IntersectingRef<'a>;
}
impl IntersectingStruct for IntersectingInner {
    fn intersecting_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.intersecting_type
    }
}
impl IntersectingStructRefMut for IntersectingInner {
    fn intersecting_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.intersecting_type
    }
}
impl IntersectingStructRef for IntersectingInner {
    fn intersecting_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.intersecting_type
    }
}
impl RelationshipStruct for IntersectingInner {
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
impl RelationshipStructRefMut for IntersectingInner {
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
impl RelationshipStructRef for IntersectingInner {
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
impl ElementStruct for IntersectingInner {
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
impl ElementStructRefMut for IntersectingInner {
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
impl ElementStructRef for IntersectingInner {
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
pub enum Intersecting {
    Itself(IntersectingInner),
}
pub enum IntersectingRefMut<'a> {
    Itself(&'a mut IntersectingInner),
}
pub enum IntersectingRef<'a> {
    Itself(&'a IntersectingInner),
}
impl Intersecting {
    pub fn as_ref(&self) -> IntersectingRef {
        match self {
            Intersecting::Itself(inner) => IntersectingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> IntersectingRefMut {
        match self {
            Intersecting::Itself(inner) => IntersectingRefMut::Itself(inner),
        }
    }
}
impl<'a> IntersectingRefMut<'a> {
    pub fn as_ref(self) -> IntersectingRef<'a> {
        match self {
            IntersectingRefMut::Itself(inner) => IntersectingRef::Itself(inner),
        }
    }
}
impl IntersectingStruct for Intersecting {
    fn intersecting_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Intersecting::Itself(x) => x.intersecting_type(),
        }
    }
}
impl IntersectingStructRefMut for Intersecting {
    fn intersecting_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Intersecting::Itself(x) => x.intersecting_type_ref_mut(),
        }
    }
}
impl IntersectingStructRef for Intersecting {
    fn intersecting_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Intersecting::Itself(x) => x.intersecting_type_ref(),
        }
    }
}
impl<'a> IntersectingStructRefMut for IntersectingRefMut<'a> {
    fn intersecting_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            IntersectingRefMut::Itself(x) => x.intersecting_type_ref_mut(),
        }
    }
}
impl<'a> IntersectingStructRef for IntersectingRefMut<'a> {
    fn intersecting_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            IntersectingRefMut::Itself(x) => x.intersecting_type_ref(),
        }
    }
}
impl<'a> IntersectingStructRef for IntersectingRef<'a> {
    fn intersecting_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            IntersectingRef::Itself(x) => x.intersecting_type_ref(),
        }
    }
}
impl RelationshipStruct for Intersecting {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Intersecting::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Intersecting {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Intersecting::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Intersecting {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Intersecting::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Intersecting::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for IntersectingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            IntersectingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for IntersectingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            IntersectingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for IntersectingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            IntersectingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            IntersectingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Intersecting {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Intersecting::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Intersecting::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Intersecting::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Intersecting::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Intersecting::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Intersecting {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Intersecting::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Intersecting::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Intersecting::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Intersecting::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Intersecting::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Intersecting {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Intersecting::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Intersecting::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Intersecting::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Intersecting::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Intersecting::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Intersecting::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for IntersectingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            IntersectingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            IntersectingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            IntersectingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for IntersectingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IntersectingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            IntersectingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            IntersectingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            IntersectingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for IntersectingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IntersectingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IntersectingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            IntersectingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            IntersectingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            IntersectingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            IntersectingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            IntersectingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl IntersectingUpcast for Intersecting {
    fn into_intersecting(self) -> Intersecting {
        self
    }
}
impl<'a> IntersectingUpcastRefMut<'a> for IntersectingRefMut<'a> {
    fn as_intersecting_ref_mut(self) -> IntersectingRefMut<'a> {
        self
    }
}
impl<'a> IntersectingUpcastRef<'a> for IntersectingRef<'a> {
    fn as_intersecting_ref(self) -> IntersectingRef<'a> {
        self
    }
}
impl RelationshipUpcast for Intersecting {
    fn into_relationship(self) -> Relationship {
        Relationship::Intersecting(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for IntersectingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Intersecting(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for IntersectingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Intersecting(self).as_relationship_ref()
    }
}
impl ElementUpcast for Intersecting {
    fn into_element(self) -> Element {
        Relationship::Intersecting(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for IntersectingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Intersecting(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for IntersectingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Intersecting(self).as_element_ref()
    }
}
pub trait IntersectingDowncast {}
pub trait IntersectingDowncastRefMut<'a> {}
pub trait IntersectingDowncastRef<'a> {}
impl IntersectingDowncast for Intersecting {}
impl<'a> IntersectingDowncastRefMut<'a> for IntersectingRefMut<'a> {}
impl<'a> IntersectingDowncastRef<'a> for IntersectingRef<'a> {}
pub trait IntersectingMethodsDescendants
where
    Self: DescendantOf<Intersecting>,
    Self::Via: IntersectingMethods,
    Self: Sized,
{}
pub trait IntersectingMethods: Sized {}
impl<T: IntersectingMethodsDescendants> IntersectingMethods for T
where
    T::Via: IntersectingMethods,
{}
impl DescendantOf<Relationship> for Intersecting {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Intersecting {}
impl DescendantOf<Element> for Intersecting {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Intersecting {}
pub trait IntersectingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<IntersectingRefMut<'a>>,
    Self::Via: IntersectingRefMutMethods,
    Self: Sized,
{}
pub trait IntersectingRefMutMethods: Sized {}
impl<'a, T: IntersectingRefMutMethodsDescendants<'a>> IntersectingRefMutMethods for T
where
    T::Via: IntersectingRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for IntersectingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for IntersectingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for IntersectingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for IntersectingRefMut<'a> {}
pub trait IntersectingRefMethodsDescendants<'a>
where
    Self: DescendantOf<IntersectingRef<'a>>,
    Self::Via: IntersectingRefMethods,
    Self: Sized,
{}
pub trait IntersectingRefMethods: Sized {}
impl<'a, T: IntersectingRefMethodsDescendants<'a>> IntersectingRefMethods for T
where
    T::Via: IntersectingRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for IntersectingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for IntersectingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for IntersectingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for IntersectingRef<'a> {}

