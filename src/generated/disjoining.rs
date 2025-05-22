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
pub struct DisjoiningInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) type_disjoined: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    pub(super) disjoining_type: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait DisjoiningStruct
where
    Self: DisjoiningStructRefMut,
    Self: DisjoiningStructRef,
    Self: RelationshipStruct,
{
    fn type_disjoined(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn disjoining_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait DisjoiningStructRefMut
where
    Self: DisjoiningStructRef,
    Self: RelationshipStructRefMut,
{
    fn type_disjoined_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn disjoining_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait DisjoiningStructRef
where
    Self: RelationshipStructRef,
{
    fn type_disjoined_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
    fn disjoining_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait DisjoiningUpcast: DisjoiningStruct {
    fn into_disjoining(self) -> Disjoining;
}
pub trait DisjoiningUpcastRefMut<'a>: DisjoiningStructRefMut {
    fn as_disjoining_ref_mut(self) -> DisjoiningRefMut<'a>;
}
pub trait DisjoiningUpcastRef<'a>: DisjoiningStructRef {
    fn as_disjoining_ref(self) -> DisjoiningRef<'a>;
}
impl DisjoiningStruct for DisjoiningInner {
    fn type_disjoined(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.type_disjoined
    }
    fn disjoining_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.disjoining_type
    }
}
impl DisjoiningStructRefMut for DisjoiningInner {
    fn type_disjoined_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.type_disjoined
    }
    fn disjoining_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.disjoining_type
    }
}
impl DisjoiningStructRef for DisjoiningInner {
    fn type_disjoined_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.type_disjoined
    }
    fn disjoining_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.disjoining_type
    }
}
impl RelationshipStruct for DisjoiningInner {
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
impl RelationshipStructRefMut for DisjoiningInner {
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
impl RelationshipStructRef for DisjoiningInner {
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
impl ElementStruct for DisjoiningInner {
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
impl ElementStructRefMut for DisjoiningInner {
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
impl ElementStructRef for DisjoiningInner {
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
pub enum Disjoining {
    Itself(DisjoiningInner),
}
pub enum DisjoiningRefMut<'a> {
    Itself(&'a mut DisjoiningInner),
}
pub enum DisjoiningRef<'a> {
    Itself(&'a DisjoiningInner),
}
impl Disjoining {
    pub fn as_ref(&self) -> DisjoiningRef {
        match self {
            Disjoining::Itself(inner) => DisjoiningRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> DisjoiningRefMut {
        match self {
            Disjoining::Itself(inner) => DisjoiningRefMut::Itself(inner),
        }
    }
}
impl<'a> DisjoiningRefMut<'a> {
    pub fn as_ref(self) -> DisjoiningRef<'a> {
        match self {
            DisjoiningRefMut::Itself(inner) => DisjoiningRef::Itself(inner),
        }
    }
}
impl DisjoiningStruct for Disjoining {
    fn type_disjoined(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.type_disjoined(),
        }
    }
    fn disjoining_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.disjoining_type(),
        }
    }
}
impl DisjoiningStructRefMut for Disjoining {
    fn type_disjoined_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.type_disjoined_ref_mut(),
        }
    }
    fn disjoining_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.disjoining_type_ref_mut(),
        }
    }
}
impl DisjoiningStructRef for Disjoining {
    fn type_disjoined_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.type_disjoined_ref(),
        }
    }
    fn disjoining_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            Disjoining::Itself(x) => x.disjoining_type_ref(),
        }
    }
}
impl<'a> DisjoiningStructRefMut for DisjoiningRefMut<'a> {
    fn type_disjoined_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.type_disjoined_ref_mut(),
        }
    }
    fn disjoining_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.disjoining_type_ref_mut(),
        }
    }
}
impl<'a> DisjoiningStructRef for DisjoiningRefMut<'a> {
    fn type_disjoined_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.type_disjoined_ref(),
        }
    }
    fn disjoining_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.disjoining_type_ref(),
        }
    }
}
impl<'a> DisjoiningStructRef for DisjoiningRef<'a> {
    fn type_disjoined_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRef::Itself(x) => x.type_disjoined_ref(),
        }
    }
    fn disjoining_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            DisjoiningRef::Itself(x) => x.disjoining_type_ref(),
        }
    }
}
impl RelationshipStruct for Disjoining {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Disjoining::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Disjoining {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Disjoining::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Disjoining {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Disjoining::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Disjoining::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for DisjoiningRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            DisjoiningRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for DisjoiningRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            DisjoiningRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for DisjoiningRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DisjoiningRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            DisjoiningRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Disjoining {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Disjoining::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Disjoining::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Disjoining::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Disjoining::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Disjoining::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Disjoining {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Disjoining::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Disjoining::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Disjoining::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Disjoining::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Disjoining::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Disjoining {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Disjoining::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Disjoining::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Disjoining::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Disjoining::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Disjoining::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Disjoining::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for DisjoiningRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            DisjoiningRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            DisjoiningRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            DisjoiningRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for DisjoiningRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DisjoiningRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DisjoiningRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DisjoiningRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DisjoiningRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for DisjoiningRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DisjoiningRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DisjoiningRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DisjoiningRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DisjoiningRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DisjoiningRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DisjoiningRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DisjoiningRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl DisjoiningUpcast for Disjoining {
    fn into_disjoining(self) -> Disjoining {
        self
    }
}
impl<'a> DisjoiningUpcastRefMut<'a> for DisjoiningRefMut<'a> {
    fn as_disjoining_ref_mut(self) -> DisjoiningRefMut<'a> {
        self
    }
}
impl<'a> DisjoiningUpcastRef<'a> for DisjoiningRef<'a> {
    fn as_disjoining_ref(self) -> DisjoiningRef<'a> {
        self
    }
}
impl RelationshipUpcast for Disjoining {
    fn into_relationship(self) -> Relationship {
        Relationship::Disjoining(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for DisjoiningRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Disjoining(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for DisjoiningRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Disjoining(self).as_relationship_ref()
    }
}
impl ElementUpcast for Disjoining {
    fn into_element(self) -> Element {
        Relationship::Disjoining(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for DisjoiningRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Disjoining(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for DisjoiningRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Disjoining(self).as_element_ref()
    }
}
pub trait DisjoiningDowncast {}
pub trait DisjoiningDowncastRefMut<'a> {}
pub trait DisjoiningDowncastRef<'a> {}
impl DisjoiningDowncast for Disjoining {}
impl<'a> DisjoiningDowncastRefMut<'a> for DisjoiningRefMut<'a> {}
impl<'a> DisjoiningDowncastRef<'a> for DisjoiningRef<'a> {}
pub trait DisjoiningMethodsDescendants
where
    Self: DescendantOf<Disjoining>,
    Self::Via: DisjoiningMethods,
    Self: Sized,
{}
pub trait DisjoiningMethods: Sized {}
impl<T: DisjoiningMethodsDescendants> DisjoiningMethods for T
where
    T::Via: DisjoiningMethods,
{}
impl DescendantOf<Relationship> for Disjoining {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Disjoining {}
impl DescendantOf<Element> for Disjoining {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Disjoining {}
pub trait DisjoiningRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<DisjoiningRefMut<'a>>,
    Self::Via: DisjoiningRefMutMethods,
    Self: Sized,
{}
pub trait DisjoiningRefMutMethods: Sized {}
impl<'a, T: DisjoiningRefMutMethodsDescendants<'a>> DisjoiningRefMutMethods for T
where
    T::Via: DisjoiningRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for DisjoiningRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for DisjoiningRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for DisjoiningRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for DisjoiningRefMut<'a> {}
pub trait DisjoiningRefMethodsDescendants<'a>
where
    Self: DescendantOf<DisjoiningRef<'a>>,
    Self::Via: DisjoiningRefMethods,
    Self: Sized,
{}
pub trait DisjoiningRefMethods: Sized {}
impl<'a, T: DisjoiningRefMethodsDescendants<'a>> DisjoiningRefMethods for T
where
    T::Via: DisjoiningRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for DisjoiningRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for DisjoiningRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for DisjoiningRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for DisjoiningRef<'a> {}

