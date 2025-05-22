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
pub struct TypeFeaturingInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) feature_of_type: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    pub(super) featuring_type: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
}
pub trait TypeFeaturingStruct
where
    Self: TypeFeaturingStructRefMut,
    Self: TypeFeaturingStructRef,
    Self: RelationshipStruct,
{
    fn feature_of_type(self) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn featuring_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait TypeFeaturingStructRefMut
where
    Self: TypeFeaturingStructRef,
    Self: RelationshipStructRefMut,
{
    fn feature_of_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn featuring_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait TypeFeaturingStructRef
where
    Self: RelationshipStructRef,
{
    fn feature_of_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
    fn featuring_type_ref(&self) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>>;
}
pub trait TypeFeaturingUpcast: TypeFeaturingStruct {
    fn into_type_featuring(self) -> TypeFeaturing;
}
pub trait TypeFeaturingUpcastRefMut<'a>: TypeFeaturingStructRefMut {
    fn as_type_featuring_ref_mut(self) -> TypeFeaturingRefMut<'a>;
}
pub trait TypeFeaturingUpcastRef<'a>: TypeFeaturingStructRef {
    fn as_type_featuring_ref(self) -> TypeFeaturingRef<'a>;
}
impl TypeFeaturingStruct for TypeFeaturingInner {
    fn feature_of_type(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.feature_of_type
    }
    fn featuring_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        self.featuring_type
    }
}
impl TypeFeaturingStructRefMut for TypeFeaturingInner {
    fn feature_of_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.feature_of_type
    }
    fn featuring_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &mut self.featuring_type
    }
}
impl TypeFeaturingStructRef for TypeFeaturingInner {
    fn feature_of_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.feature_of_type
    }
    fn featuring_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        &self.featuring_type
    }
}
impl RelationshipStruct for TypeFeaturingInner {
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
impl RelationshipStructRefMut for TypeFeaturingInner {
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
impl RelationshipStructRef for TypeFeaturingInner {
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
impl ElementStruct for TypeFeaturingInner {
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
impl ElementStructRefMut for TypeFeaturingInner {
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
impl ElementStructRef for TypeFeaturingInner {
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
pub enum TypeFeaturing {
    Itself(TypeFeaturingInner),
}
pub enum TypeFeaturingRefMut<'a> {
    Itself(&'a mut TypeFeaturingInner),
}
pub enum TypeFeaturingRef<'a> {
    Itself(&'a TypeFeaturingInner),
}
impl TypeFeaturing {
    pub fn as_ref(&self) -> TypeFeaturingRef {
        match self {
            TypeFeaturing::Itself(inner) => TypeFeaturingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> TypeFeaturingRefMut {
        match self {
            TypeFeaturing::Itself(inner) => TypeFeaturingRefMut::Itself(inner),
        }
    }
}
impl<'a> TypeFeaturingRefMut<'a> {
    pub fn as_ref(self) -> TypeFeaturingRef<'a> {
        match self {
            TypeFeaturingRefMut::Itself(inner) => TypeFeaturingRef::Itself(inner),
        }
    }
}
impl TypeFeaturingStruct for TypeFeaturing {
    fn feature_of_type(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturing::Itself(x) => x.feature_of_type(),
        }
    }
    fn featuring_type(self) -> std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturing::Itself(x) => x.featuring_type(),
        }
    }
}
impl TypeFeaturingStructRefMut for TypeFeaturing {
    fn feature_of_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturing::Itself(x) => x.feature_of_type_ref_mut(),
        }
    }
    fn featuring_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturing::Itself(x) => x.featuring_type_ref_mut(),
        }
    }
}
impl TypeFeaturingStructRef for TypeFeaturing {
    fn feature_of_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturing::Itself(x) => x.feature_of_type_ref(),
        }
    }
    fn featuring_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturing::Itself(x) => x.featuring_type_ref(),
        }
    }
}
impl<'a> TypeFeaturingStructRefMut for TypeFeaturingRefMut<'a> {
    fn feature_of_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.feature_of_type_ref_mut(),
        }
    }
    fn featuring_type_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.featuring_type_ref_mut(),
        }
    }
}
impl<'a> TypeFeaturingStructRef for TypeFeaturingRefMut<'a> {
    fn feature_of_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.feature_of_type_ref(),
        }
    }
    fn featuring_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.featuring_type_ref(),
        }
    }
}
impl<'a> TypeFeaturingStructRef for TypeFeaturingRef<'a> {
    fn feature_of_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.feature_of_type_ref(),
        }
    }
    fn featuring_type_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::type_::Type>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.featuring_type_ref(),
        }
    }
}
impl RelationshipStruct for TypeFeaturing {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for TypeFeaturing {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for TypeFeaturing {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for TypeFeaturingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for TypeFeaturingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for TypeFeaturingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            TypeFeaturingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for TypeFeaturing {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            TypeFeaturing::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            TypeFeaturing::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for TypeFeaturing {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            TypeFeaturing::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            TypeFeaturing::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            TypeFeaturing::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for TypeFeaturing {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturing::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TypeFeaturing::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TypeFeaturing::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturing::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TypeFeaturing::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for TypeFeaturingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for TypeFeaturingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TypeFeaturingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for TypeFeaturingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TypeFeaturingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TypeFeaturingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TypeFeaturingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TypeFeaturingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TypeFeaturingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl TypeFeaturingUpcast for TypeFeaturing {
    fn into_type_featuring(self) -> TypeFeaturing {
        self
    }
}
impl<'a> TypeFeaturingUpcastRefMut<'a> for TypeFeaturingRefMut<'a> {
    fn as_type_featuring_ref_mut(self) -> TypeFeaturingRefMut<'a> {
        self
    }
}
impl<'a> TypeFeaturingUpcastRef<'a> for TypeFeaturingRef<'a> {
    fn as_type_featuring_ref(self) -> TypeFeaturingRef<'a> {
        self
    }
}
impl RelationshipUpcast for TypeFeaturing {
    fn into_relationship(self) -> Relationship {
        Relationship::TypeFeaturing(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for TypeFeaturingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::TypeFeaturing(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for TypeFeaturingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::TypeFeaturing(self).as_relationship_ref()
    }
}
impl ElementUpcast for TypeFeaturing {
    fn into_element(self) -> Element {
        Relationship::TypeFeaturing(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for TypeFeaturingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::TypeFeaturing(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for TypeFeaturingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::TypeFeaturing(self).as_element_ref()
    }
}
pub trait TypeFeaturingDowncast {}
pub trait TypeFeaturingDowncastRefMut<'a> {}
pub trait TypeFeaturingDowncastRef<'a> {}
impl TypeFeaturingDowncast for TypeFeaturing {}
impl<'a> TypeFeaturingDowncastRefMut<'a> for TypeFeaturingRefMut<'a> {}
impl<'a> TypeFeaturingDowncastRef<'a> for TypeFeaturingRef<'a> {}
pub trait TypeFeaturingMethodsDescendants
where
    Self: DescendantOf<TypeFeaturing>,
    Self::Via: TypeFeaturingMethods,
    Self: Sized,
{}
pub trait TypeFeaturingMethods: Sized {}
impl<T: TypeFeaturingMethodsDescendants> TypeFeaturingMethods for T
where
    T::Via: TypeFeaturingMethods,
{}
impl DescendantOf<Relationship> for TypeFeaturing {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for TypeFeaturing {}
impl DescendantOf<Element> for TypeFeaturing {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for TypeFeaturing {}
pub trait TypeFeaturingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<TypeFeaturingRefMut<'a>>,
    Self::Via: TypeFeaturingRefMutMethods,
    Self: Sized,
{}
pub trait TypeFeaturingRefMutMethods: Sized {}
impl<'a, T: TypeFeaturingRefMutMethodsDescendants<'a>> TypeFeaturingRefMutMethods for T
where
    T::Via: TypeFeaturingRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for TypeFeaturingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for TypeFeaturingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for TypeFeaturingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for TypeFeaturingRefMut<'a> {}
pub trait TypeFeaturingRefMethodsDescendants<'a>
where
    Self: DescendantOf<TypeFeaturingRef<'a>>,
    Self::Via: TypeFeaturingRefMethods,
    Self: Sized,
{}
pub trait TypeFeaturingRefMethods: Sized {}
impl<'a, T: TypeFeaturingRefMethodsDescendants<'a>> TypeFeaturingRefMethods for T
where
    T::Via: TypeFeaturingRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for TypeFeaturingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for TypeFeaturingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for TypeFeaturingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for TypeFeaturingRef<'a> {}

