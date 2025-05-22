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
pub struct FeatureChainingInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) chaining_feature: std::rc::Rc<
        std::cell::RefCell<super::feature::Feature>,
    >,
}
pub trait FeatureChainingStruct
where
    Self: FeatureChainingStructRefMut,
    Self: FeatureChainingStructRef,
    Self: RelationshipStruct,
{
    fn chaining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureChainingStructRefMut
where
    Self: FeatureChainingStructRef,
    Self: RelationshipStructRefMut,
{
    fn chaining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureChainingStructRef
where
    Self: RelationshipStructRef,
{
    fn chaining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>>;
}
pub trait FeatureChainingUpcast: FeatureChainingStruct {
    fn into_feature_chaining(self) -> FeatureChaining;
}
pub trait FeatureChainingUpcastRefMut<'a>: FeatureChainingStructRefMut {
    fn as_feature_chaining_ref_mut(self) -> FeatureChainingRefMut<'a>;
}
pub trait FeatureChainingUpcastRef<'a>: FeatureChainingStructRef {
    fn as_feature_chaining_ref(self) -> FeatureChainingRef<'a>;
}
impl FeatureChainingStruct for FeatureChainingInner {
    fn chaining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        self.chaining_feature
    }
}
impl FeatureChainingStructRefMut for FeatureChainingInner {
    fn chaining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &mut self.chaining_feature
    }
}
impl FeatureChainingStructRef for FeatureChainingInner {
    fn chaining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        &self.chaining_feature
    }
}
impl RelationshipStruct for FeatureChainingInner {
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
impl RelationshipStructRefMut for FeatureChainingInner {
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
impl RelationshipStructRef for FeatureChainingInner {
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
impl ElementStruct for FeatureChainingInner {
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
impl ElementStructRefMut for FeatureChainingInner {
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
impl ElementStructRef for FeatureChainingInner {
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
pub enum FeatureChaining {
    Itself(FeatureChainingInner),
}
pub enum FeatureChainingRefMut<'a> {
    Itself(&'a mut FeatureChainingInner),
}
pub enum FeatureChainingRef<'a> {
    Itself(&'a FeatureChainingInner),
}
impl FeatureChaining {
    pub fn as_ref(&self) -> FeatureChainingRef {
        match self {
            FeatureChaining::Itself(inner) => FeatureChainingRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureChainingRefMut {
        match self {
            FeatureChaining::Itself(inner) => FeatureChainingRefMut::Itself(inner),
        }
    }
}
impl<'a> FeatureChainingRefMut<'a> {
    pub fn as_ref(self) -> FeatureChainingRef<'a> {
        match self {
            FeatureChainingRefMut::Itself(inner) => FeatureChainingRef::Itself(inner),
        }
    }
}
impl FeatureChainingStruct for FeatureChaining {
    fn chaining_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChaining::Itself(x) => x.chaining_feature(),
        }
    }
}
impl FeatureChainingStructRefMut for FeatureChaining {
    fn chaining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChaining::Itself(x) => x.chaining_feature_ref_mut(),
        }
    }
}
impl FeatureChainingStructRef for FeatureChaining {
    fn chaining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChaining::Itself(x) => x.chaining_feature_ref(),
        }
    }
}
impl<'a> FeatureChainingStructRefMut for FeatureChainingRefMut<'a> {
    fn chaining_feature_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.chaining_feature_ref_mut(),
        }
    }
}
impl<'a> FeatureChainingStructRef for FeatureChainingRefMut<'a> {
    fn chaining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.chaining_feature_ref(),
        }
    }
}
impl<'a> FeatureChainingStructRef for FeatureChainingRef<'a> {
    fn chaining_feature_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::feature::Feature>> {
        match self {
            FeatureChainingRef::Itself(x) => x.chaining_feature_ref(),
        }
    }
}
impl RelationshipStruct for FeatureChaining {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for FeatureChaining {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for FeatureChaining {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FeatureChainingRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainingRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureChainingRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureChainingRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureChainingRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureChainingRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for FeatureChaining {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChaining::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureChaining::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureChaining::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureChaining {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureChaining::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChaining::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureChaining::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureChaining {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChaining::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChaining::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChaining::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChaining::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChaining::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChaining::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureChainingRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChainingRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainingRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureChainingRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChainingRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainingRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChainingRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureChainingRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainingRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChainingRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChainingRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainingRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainingRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChainingRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureChainingUpcast for FeatureChaining {
    fn into_feature_chaining(self) -> FeatureChaining {
        self
    }
}
impl<'a> FeatureChainingUpcastRefMut<'a> for FeatureChainingRefMut<'a> {
    fn as_feature_chaining_ref_mut(self) -> FeatureChainingRefMut<'a> {
        self
    }
}
impl<'a> FeatureChainingUpcastRef<'a> for FeatureChainingRef<'a> {
    fn as_feature_chaining_ref(self) -> FeatureChainingRef<'a> {
        self
    }
}
impl RelationshipUpcast for FeatureChaining {
    fn into_relationship(self) -> Relationship {
        Relationship::FeatureChaining(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FeatureChainingRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::FeatureChaining(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FeatureChainingRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::FeatureChaining(self).as_relationship_ref()
    }
}
impl ElementUpcast for FeatureChaining {
    fn into_element(self) -> Element {
        Relationship::FeatureChaining(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureChainingRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::FeatureChaining(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureChainingRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::FeatureChaining(self).as_element_ref()
    }
}
pub trait FeatureChainingDowncast {}
pub trait FeatureChainingDowncastRefMut<'a> {}
pub trait FeatureChainingDowncastRef<'a> {}
impl FeatureChainingDowncast for FeatureChaining {}
impl<'a> FeatureChainingDowncastRefMut<'a> for FeatureChainingRefMut<'a> {}
impl<'a> FeatureChainingDowncastRef<'a> for FeatureChainingRef<'a> {}
pub trait FeatureChainingMethodsDescendants
where
    Self: DescendantOf<FeatureChaining>,
    Self::Via: FeatureChainingMethods,
    Self: Sized,
{}
pub trait FeatureChainingMethods: Sized {}
impl<T: FeatureChainingMethodsDescendants> FeatureChainingMethods for T
where
    T::Via: FeatureChainingMethods,
{}
impl DescendantOf<Relationship> for FeatureChaining {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for FeatureChaining {}
impl DescendantOf<Element> for FeatureChaining {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for FeatureChaining {}
pub trait FeatureChainingRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureChainingRefMut<'a>>,
    Self::Via: FeatureChainingRefMutMethods,
    Self: Sized,
{}
pub trait FeatureChainingRefMutMethods: Sized {}
impl<'a, T: FeatureChainingRefMutMethodsDescendants<'a>> FeatureChainingRefMutMethods
for T
where
    T::Via: FeatureChainingRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FeatureChainingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FeatureChainingRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureChainingRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureChainingRefMut<'a> {}
pub trait FeatureChainingRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureChainingRef<'a>>,
    Self::Via: FeatureChainingRefMethods,
    Self: Sized,
{}
pub trait FeatureChainingRefMethods: Sized {}
impl<'a, T: FeatureChainingRefMethodsDescendants<'a>> FeatureChainingRefMethods for T
where
    T::Via: FeatureChainingRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for FeatureChainingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FeatureChainingRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureChainingRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureChainingRef<'a> {}

