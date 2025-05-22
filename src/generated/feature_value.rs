#![allow(unused)]
use super::utils::DescendantOf;
use super::owning_membership::{
    OwningMembership, OwningMembershipRefMut, OwningMembershipRef,
    OwningMembershipStruct, OwningMembershipStructRefMut, OwningMembershipStructRef,
    OwningMembershipInner, OwningMembershipUpcast, OwningMembershipUpcastRefMut,
    OwningMembershipUpcastRef, OwningMembershipMethodsDescendants,
    OwningMembershipRefMutMethodsDescendants, OwningMembershipRefMethodsDescendants,
};
use super::membership::{
    Membership, MembershipRefMut, MembershipRef, MembershipStruct,
    MembershipStructRefMut, MembershipStructRef, MembershipInner, MembershipUpcast,
    MembershipUpcastRefMut, MembershipUpcastRef, MembershipMethodsDescendants,
    MembershipRefMutMethodsDescendants, MembershipRefMethodsDescendants,
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
pub struct FeatureValueInner {
    pub(super) sup_owning_membership: OwningMembershipInner,
    pub(super) is_initial: bool,
    pub(super) is_default: bool,
}
pub trait FeatureValueStruct
where
    Self: FeatureValueStructRefMut,
    Self: FeatureValueStructRef,
    Self: OwningMembershipStruct,
{
    fn is_initial(self) -> bool;
    fn is_default(self) -> bool;
}
pub trait FeatureValueStructRefMut
where
    Self: FeatureValueStructRef,
    Self: OwningMembershipStructRefMut,
{
    fn is_initial_ref_mut(&mut self) -> &mut bool;
    fn is_default_ref_mut(&mut self) -> &mut bool;
}
pub trait FeatureValueStructRef
where
    Self: OwningMembershipStructRef,
{
    fn is_initial_ref(&self) -> &bool;
    fn is_default_ref(&self) -> &bool;
}
pub trait FeatureValueUpcast: FeatureValueStruct {
    fn into_feature_value(self) -> FeatureValue;
}
pub trait FeatureValueUpcastRefMut<'a>: FeatureValueStructRefMut {
    fn as_feature_value_ref_mut(self) -> FeatureValueRefMut<'a>;
}
pub trait FeatureValueUpcastRef<'a>: FeatureValueStructRef {
    fn as_feature_value_ref(self) -> FeatureValueRef<'a>;
}
impl FeatureValueStruct for FeatureValueInner {
    fn is_initial(self) -> bool {
        self.is_initial
    }
    fn is_default(self) -> bool {
        self.is_default
    }
}
impl FeatureValueStructRefMut for FeatureValueInner {
    fn is_initial_ref_mut(&mut self) -> &mut bool {
        &mut self.is_initial
    }
    fn is_default_ref_mut(&mut self) -> &mut bool {
        &mut self.is_default
    }
}
impl FeatureValueStructRef for FeatureValueInner {
    fn is_initial_ref(&self) -> &bool {
        &self.is_initial
    }
    fn is_default_ref(&self) -> &bool {
        &self.is_default
    }
}
impl OwningMembershipStruct for FeatureValueInner {}
impl OwningMembershipStructRefMut for FeatureValueInner {}
impl OwningMembershipStructRef for FeatureValueInner {}
impl MembershipStruct for FeatureValueInner {
    fn member_short_name(self) -> Option<String> {
        self.sup_owning_membership.member_short_name()
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_owning_membership.member_element()
    }
    fn member_name(self) -> Option<String> {
        self.sup_owning_membership.member_name()
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_owning_membership.visibility()
    }
}
impl MembershipStructRefMut for FeatureValueInner {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_owning_membership.member_short_name_ref_mut()
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_owning_membership.member_element_ref_mut()
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_owning_membership.member_name_ref_mut()
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_owning_membership.visibility_ref_mut()
    }
}
impl MembershipStructRef for FeatureValueInner {
    fn member_short_name_ref(&self) -> &Option<String> {
        self.sup_owning_membership.member_short_name_ref()
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_owning_membership.member_element_ref()
    }
    fn member_name_ref(&self) -> &Option<String> {
        self.sup_owning_membership.member_name_ref()
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_owning_membership.visibility_ref()
    }
}
impl RelationshipStruct for FeatureValueInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_owning_membership.is_implied()
    }
}
impl RelationshipStructRefMut for FeatureValueInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_owning_membership.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for FeatureValueInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_owning_membership.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_owning_membership.is_implied_ref()
    }
}
impl ElementStruct for FeatureValueInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_owning_membership.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_owning_membership.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_owning_membership.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_owning_membership.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_owning_membership.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_owning_membership.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_owning_membership.is_implied_included()
    }
}
impl ElementStructRefMut for FeatureValueInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_owning_membership.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_owning_membership.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_owning_membership.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_owning_membership.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_owning_membership.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_owning_membership.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_owning_membership.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FeatureValueInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_owning_membership.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_owning_membership.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_owning_membership.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_owning_membership.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_owning_membership.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_owning_membership.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_owning_membership.is_implied_included_ref()
    }
}
pub enum FeatureValue {
    Itself(FeatureValueInner),
}
pub enum FeatureValueRefMut<'a> {
    Itself(&'a mut FeatureValueInner),
}
pub enum FeatureValueRef<'a> {
    Itself(&'a FeatureValueInner),
}
impl FeatureValue {
    pub fn as_ref(&self) -> FeatureValueRef {
        match self {
            FeatureValue::Itself(inner) => FeatureValueRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureValueRefMut {
        match self {
            FeatureValue::Itself(inner) => FeatureValueRefMut::Itself(inner),
        }
    }
}
impl<'a> FeatureValueRefMut<'a> {
    pub fn as_ref(self) -> FeatureValueRef<'a> {
        match self {
            FeatureValueRefMut::Itself(inner) => FeatureValueRef::Itself(inner),
        }
    }
}
impl FeatureValueStruct for FeatureValue {
    fn is_initial(self) -> bool {
        match self {
            FeatureValue::Itself(x) => x.is_initial(),
        }
    }
    fn is_default(self) -> bool {
        match self {
            FeatureValue::Itself(x) => x.is_default(),
        }
    }
}
impl FeatureValueStructRefMut for FeatureValue {
    fn is_initial_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValue::Itself(x) => x.is_initial_ref_mut(),
        }
    }
    fn is_default_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValue::Itself(x) => x.is_default_ref_mut(),
        }
    }
}
impl FeatureValueStructRef for FeatureValue {
    fn is_initial_ref(&self) -> &bool {
        match self {
            FeatureValue::Itself(x) => x.is_initial_ref(),
        }
    }
    fn is_default_ref(&self) -> &bool {
        match self {
            FeatureValue::Itself(x) => x.is_default_ref(),
        }
    }
}
impl<'a> FeatureValueStructRefMut for FeatureValueRefMut<'a> {
    fn is_initial_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_initial_ref_mut(),
        }
    }
    fn is_default_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_default_ref_mut(),
        }
    }
}
impl<'a> FeatureValueStructRef for FeatureValueRefMut<'a> {
    fn is_initial_ref(&self) -> &bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_initial_ref(),
        }
    }
    fn is_default_ref(&self) -> &bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_default_ref(),
        }
    }
}
impl<'a> FeatureValueStructRef for FeatureValueRef<'a> {
    fn is_initial_ref(&self) -> &bool {
        match self {
            FeatureValueRef::Itself(x) => x.is_initial_ref(),
        }
    }
    fn is_default_ref(&self) -> &bool {
        match self {
            FeatureValueRef::Itself(x) => x.is_default_ref(),
        }
    }
}
impl OwningMembershipStruct for FeatureValue {}
impl OwningMembershipStructRefMut for FeatureValue {}
impl OwningMembershipStructRef for FeatureValue {}
impl<'a> OwningMembershipStructRefMut for FeatureValueRefMut<'a> {}
impl<'a> OwningMembershipStructRef for FeatureValueRefMut<'a> {}
impl<'a> OwningMembershipStructRef for FeatureValueRef<'a> {}
impl MembershipStruct for FeatureValue {
    fn member_short_name(self) -> Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValue::Itself(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValue::Itself(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for FeatureValue {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValue::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValue::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for FeatureValue {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValue::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValue::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValue::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for FeatureValueRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for FeatureValueRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for FeatureValueRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRef::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureValueRef::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRef::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureValueRef::Itself(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for FeatureValue {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for FeatureValue {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for FeatureValue {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FeatureValueRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureValueRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureValueRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureValueRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureValueRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for FeatureValue {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValue::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureValue::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureValue::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureValue {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureValue::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureValue::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureValue::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureValue {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValue::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValue::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureValue::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureValue::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValue::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureValue::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureValueRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureValueRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureValueRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureValueRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValueRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureValueRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureValueRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureValueRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValueRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureValueRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureValueRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureValueRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureValueRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureValueRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureValueUpcast for FeatureValue {
    fn into_feature_value(self) -> FeatureValue {
        self
    }
}
impl<'a> FeatureValueUpcastRefMut<'a> for FeatureValueRefMut<'a> {
    fn as_feature_value_ref_mut(self) -> FeatureValueRefMut<'a> {
        self
    }
}
impl<'a> FeatureValueUpcastRef<'a> for FeatureValueRef<'a> {
    fn as_feature_value_ref(self) -> FeatureValueRef<'a> {
        self
    }
}
impl OwningMembershipUpcast for FeatureValue {
    fn into_owning_membership(self) -> OwningMembership {
        OwningMembership::FeatureValue(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for FeatureValueRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        OwningMembershipRefMut::FeatureValue(self).as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for FeatureValueRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        OwningMembershipRef::FeatureValue(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for FeatureValue {
    fn into_membership(self) -> Membership {
        OwningMembership::FeatureValue(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for FeatureValueRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        OwningMembershipRefMut::FeatureValue(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for FeatureValueRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        OwningMembershipRef::FeatureValue(self).as_membership_ref()
    }
}
impl RelationshipUpcast for FeatureValue {
    fn into_relationship(self) -> Relationship {
        OwningMembership::FeatureValue(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FeatureValueRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        OwningMembershipRefMut::FeatureValue(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FeatureValueRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        OwningMembershipRef::FeatureValue(self).as_relationship_ref()
    }
}
impl ElementUpcast for FeatureValue {
    fn into_element(self) -> Element {
        OwningMembership::FeatureValue(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureValueRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OwningMembershipRefMut::FeatureValue(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureValueRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OwningMembershipRef::FeatureValue(self).as_element_ref()
    }
}
pub trait FeatureValueDowncast {}
pub trait FeatureValueDowncastRefMut<'a> {}
pub trait FeatureValueDowncastRef<'a> {}
impl FeatureValueDowncast for FeatureValue {}
impl<'a> FeatureValueDowncastRefMut<'a> for FeatureValueRefMut<'a> {}
impl<'a> FeatureValueDowncastRef<'a> for FeatureValueRef<'a> {}
pub trait FeatureValueMethodsDescendants
where
    Self: DescendantOf<FeatureValue>,
    Self::Via: FeatureValueMethods,
    Self: Sized,
{}
pub trait FeatureValueMethods: Sized {}
impl<T: FeatureValueMethodsDescendants> FeatureValueMethods for T
where
    T::Via: FeatureValueMethods,
{}
impl DescendantOf<OwningMembership> for FeatureValue {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl OwningMembershipMethodsDescendants for FeatureValue {}
impl DescendantOf<Membership> for FeatureValue {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl MembershipMethodsDescendants for FeatureValue {}
impl DescendantOf<Relationship> for FeatureValue {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl RelationshipMethodsDescendants for FeatureValue {}
impl DescendantOf<Element> for FeatureValue {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl ElementMethodsDescendants for FeatureValue {}
pub trait FeatureValueRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureValueRefMut<'a>>,
    Self::Via: FeatureValueRefMutMethods,
    Self: Sized,
{}
pub trait FeatureValueRefMutMethods: Sized {}
impl<'a, T: FeatureValueRefMutMethodsDescendants<'a>> FeatureValueRefMutMethods for T
where
    T::Via: FeatureValueRefMutMethods,
{}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>> for FeatureValueRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a> for FeatureValueRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for FeatureValueRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for FeatureValueRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FeatureValueRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FeatureValueRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureValueRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureValueRefMut<'a> {}
pub trait FeatureValueRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureValueRef<'a>>,
    Self::Via: FeatureValueRefMethods,
    Self: Sized,
{}
pub trait FeatureValueRefMethods: Sized {}
impl<'a, T: FeatureValueRefMethodsDescendants<'a>> FeatureValueRefMethods for T
where
    T::Via: FeatureValueRefMethods,
{}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for FeatureValueRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for FeatureValueRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for FeatureValueRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for FeatureValueRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for FeatureValueRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FeatureValueRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureValueRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureValueRef<'a> {}

