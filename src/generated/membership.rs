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
use super::owning_membership::{
    OwningMembership, OwningMembershipRefMut, OwningMembershipRef,
};
pub struct MembershipInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) member_short_name: Option<String>,
    pub(super) member_element: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    pub(super) member_name: Option<String>,
    pub(super) visibility: std::rc::Rc<
        std::cell::RefCell<super::visibility_kind::VisibilityKind>,
    >,
}
pub trait MembershipStruct
where
    Self: MembershipStructRefMut,
    Self: MembershipStructRef,
    Self: RelationshipStruct,
{
    fn member_short_name(self) -> Option<String>;
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>>;
    fn member_name(self) -> Option<String>;
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
}
pub trait MembershipStructRefMut
where
    Self: MembershipStructRef,
    Self: RelationshipStructRefMut,
{
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String>;
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>>;
    fn member_name_ref_mut(&mut self) -> &mut Option<String>;
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
}
pub trait MembershipStructRef
where
    Self: RelationshipStructRef,
{
    fn member_short_name_ref(&self) -> &Option<String>;
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>>;
    fn member_name_ref(&self) -> &Option<String>;
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
}
pub trait MembershipUpcast: MembershipStruct {
    fn into_membership(self) -> Membership;
}
pub trait MembershipUpcastRefMut<'a>: MembershipStructRefMut {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a>;
}
pub trait MembershipUpcastRef<'a>: MembershipStructRef {
    fn as_membership_ref(self) -> MembershipRef<'a>;
}
impl MembershipStruct for MembershipInner {
    fn member_short_name(self) -> Option<String> {
        self.member_short_name
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.member_element
    }
    fn member_name(self) -> Option<String> {
        self.member_name
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.visibility
    }
}
impl MembershipStructRefMut for MembershipInner {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        &mut self.member_short_name
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        &mut self.member_element
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        &mut self.member_name
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        &mut self.visibility
    }
}
impl MembershipStructRef for MembershipInner {
    fn member_short_name_ref(&self) -> &Option<String> {
        &self.member_short_name
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        &self.member_element
    }
    fn member_name_ref(&self) -> &Option<String> {
        &self.member_name
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        &self.visibility
    }
}
impl RelationshipStruct for MembershipInner {
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
impl RelationshipStructRefMut for MembershipInner {
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
impl RelationshipStructRef for MembershipInner {
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
impl ElementStruct for MembershipInner {
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
impl ElementStructRefMut for MembershipInner {
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
impl ElementStructRef for MembershipInner {
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
pub enum Membership {
    Itself(MembershipInner),
    OwningMembership(OwningMembership),
}
pub enum MembershipRefMut<'a> {
    Itself(&'a mut MembershipInner),
    OwningMembership(OwningMembershipRefMut<'a>),
}
pub enum MembershipRef<'a> {
    Itself(&'a MembershipInner),
    OwningMembership(OwningMembershipRef<'a>),
}
impl Membership {
    pub fn as_ref(&self) -> MembershipRef {
        match self {
            Membership::Itself(inner) => MembershipRef::Itself(&inner),
            Membership::OwningMembership(inner) => {
                MembershipRef::OwningMembership(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> MembershipRefMut {
        match self {
            Membership::Itself(inner) => MembershipRefMut::Itself(inner),
            Membership::OwningMembership(inner) => {
                MembershipRefMut::OwningMembership(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> MembershipRefMut<'a> {
    pub fn as_ref(self) -> MembershipRef<'a> {
        match self {
            MembershipRefMut::Itself(inner) => MembershipRef::Itself(inner),
            MembershipRefMut::OwningMembership(inner) => {
                MembershipRef::OwningMembership(inner.as_ref())
            }
        }
    }
}
impl MembershipStruct for Membership {
    fn member_short_name(self) -> Option<String> {
        match self {
            Membership::Itself(x) => x.member_short_name(),
            Membership::OwningMembership(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Membership::Itself(x) => x.member_element(),
            Membership::OwningMembership(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            Membership::Itself(x) => x.member_name(),
            Membership::OwningMembership(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Membership::Itself(x) => x.visibility(),
            Membership::OwningMembership(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for Membership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Membership::Itself(x) => x.member_short_name_ref_mut(),
            Membership::OwningMembership(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Membership::Itself(x) => x.member_element_ref_mut(),
            Membership::OwningMembership(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Membership::Itself(x) => x.member_name_ref_mut(),
            Membership::OwningMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Membership::Itself(x) => x.visibility_ref_mut(),
            Membership::OwningMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for Membership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            Membership::Itself(x) => x.member_short_name_ref(),
            Membership::OwningMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            Membership::Itself(x) => x.member_element_ref(),
            Membership::OwningMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            Membership::Itself(x) => x.member_name_ref(),
            Membership::OwningMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Membership::Itself(x) => x.visibility_ref(),
            Membership::OwningMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for MembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            MembershipRefMut::Itself(x) => x.member_element_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.member_name_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipRefMut::Itself(x) => x.visibility_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for MembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.member_short_name_ref(),
            MembershipRefMut::OwningMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            MembershipRefMut::Itself(x) => x.member_element_ref(),
            MembershipRefMut::OwningMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.member_name_ref(),
            MembershipRefMut::OwningMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipRefMut::Itself(x) => x.visibility_ref(),
            MembershipRefMut::OwningMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for MembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRef::Itself(x) => x.member_short_name_ref(),
            MembershipRef::OwningMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            MembershipRef::Itself(x) => x.member_element_ref(),
            MembershipRef::OwningMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRef::Itself(x) => x.member_name_ref(),
            MembershipRef::OwningMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipRef::Itself(x) => x.visibility_ref(),
            MembershipRef::OwningMembership(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for Membership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.target(),
            Membership::OwningMembership(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.source(),
            Membership::OwningMembership(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owning_related_element(),
            Membership::OwningMembership(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owned_related_element(),
            Membership::OwningMembership(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Membership::Itself(x) => x.is_implied(),
            Membership::OwningMembership(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Membership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.target_ref_mut(),
            Membership::OwningMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.source_ref_mut(),
            Membership::OwningMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owning_related_element_ref_mut(),
            Membership::OwningMembership(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owned_related_element_ref_mut(),
            Membership::OwningMembership(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Membership::Itself(x) => x.is_implied_ref_mut(),
            Membership::OwningMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Membership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.target_ref(),
            Membership::OwningMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.source_ref(),
            Membership::OwningMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owning_related_element_ref(),
            Membership::OwningMembership(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Membership::Itself(x) => x.owned_related_element_ref(),
            Membership::OwningMembership(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Membership::Itself(x) => x.is_implied_ref(),
            Membership::OwningMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for MembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.target_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.source_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for MembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.target_ref(),
            MembershipRefMut::OwningMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.source_ref(),
            MembershipRefMut::OwningMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owning_related_element_ref(),
            MembershipRefMut::OwningMembership(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owned_related_element_ref(),
            MembershipRefMut::OwningMembership(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            MembershipRefMut::Itself(x) => x.is_implied_ref(),
            MembershipRefMut::OwningMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for MembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRef::Itself(x) => x.target_ref(),
            MembershipRef::OwningMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRef::Itself(x) => x.source_ref(),
            MembershipRef::OwningMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRef::Itself(x) => x.owning_related_element_ref(),
            MembershipRef::OwningMembership(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipRef::Itself(x) => x.owned_related_element_ref(),
            MembershipRef::OwningMembership(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            MembershipRef::Itself(x) => x.is_implied_ref(),
            MembershipRef::OwningMembership(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Membership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Membership::Itself(x) => x.owned_relationship(),
            Membership::OwningMembership(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Membership::Itself(x) => x.owning_relationship(),
            Membership::OwningMembership(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Membership::Itself(x) => x.element_id(),
            Membership::OwningMembership(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Membership::Itself(x) => x.alias_ids(),
            Membership::OwningMembership(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Membership::Itself(x) => x.declared_short_name(),
            Membership::OwningMembership(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Membership::Itself(x) => x.declared_name(),
            Membership::OwningMembership(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Membership::Itself(x) => x.is_implied_included(),
            Membership::OwningMembership(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Membership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Membership::Itself(x) => x.owned_relationship_ref_mut(),
            Membership::OwningMembership(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Membership::Itself(x) => x.owning_relationship_ref_mut(),
            Membership::OwningMembership(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Membership::Itself(x) => x.element_id_ref_mut(),
            Membership::OwningMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Membership::Itself(x) => x.alias_ids_ref_mut(),
            Membership::OwningMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Membership::Itself(x) => x.declared_short_name_ref_mut(),
            Membership::OwningMembership(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Membership::Itself(x) => x.declared_name_ref_mut(),
            Membership::OwningMembership(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Membership::Itself(x) => x.is_implied_included_ref_mut(),
            Membership::OwningMembership(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Membership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Membership::Itself(x) => x.owned_relationship_ref(),
            Membership::OwningMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Membership::Itself(x) => x.owning_relationship_ref(),
            Membership::OwningMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Membership::Itself(x) => x.element_id_ref(),
            Membership::OwningMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Membership::Itself(x) => x.alias_ids_ref(),
            Membership::OwningMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Membership::Itself(x) => x.declared_short_name_ref(),
            Membership::OwningMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Membership::Itself(x) => x.declared_name_ref(),
            Membership::OwningMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Membership::Itself(x) => x.is_implied_included_ref(),
            Membership::OwningMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MembershipRefMut::Itself(x) => x.element_id_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            MembershipRefMut::OwningMembership(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owned_relationship_ref(),
            MembershipRefMut::OwningMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipRefMut::Itself(x) => x.owning_relationship_ref(),
            MembershipRefMut::OwningMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MembershipRefMut::Itself(x) => x.element_id_ref(),
            MembershipRefMut::OwningMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MembershipRefMut::Itself(x) => x.alias_ids_ref(),
            MembershipRefMut::OwningMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.declared_short_name_ref(),
            MembershipRefMut::OwningMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRefMut::Itself(x) => x.declared_name_ref(),
            MembershipRefMut::OwningMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MembershipRefMut::Itself(x) => x.is_implied_included_ref(),
            MembershipRefMut::OwningMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipRef::Itself(x) => x.owned_relationship_ref(),
            MembershipRef::OwningMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipRef::Itself(x) => x.owning_relationship_ref(),
            MembershipRef::OwningMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MembershipRef::Itself(x) => x.element_id_ref(),
            MembershipRef::OwningMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MembershipRef::Itself(x) => x.alias_ids_ref(),
            MembershipRef::OwningMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRef::Itself(x) => x.declared_short_name_ref(),
            MembershipRef::OwningMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MembershipRef::Itself(x) => x.declared_name_ref(),
            MembershipRef::OwningMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MembershipRef::Itself(x) => x.is_implied_included_ref(),
            MembershipRef::OwningMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl MembershipUpcast for Membership {
    fn into_membership(self) -> Membership {
        self
    }
}
impl<'a> MembershipUpcastRefMut<'a> for MembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        self
    }
}
impl<'a> MembershipUpcastRef<'a> for MembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        self
    }
}
impl RelationshipUpcast for Membership {
    fn into_relationship(self) -> Relationship {
        Relationship::Membership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for MembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Membership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for MembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Membership(self).as_relationship_ref()
    }
}
impl ElementUpcast for Membership {
    fn into_element(self) -> Element {
        Relationship::Membership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Membership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Membership(self).as_element_ref()
    }
}
pub trait MembershipDowncast {
    fn try_into_owning_membership(self) -> Result<OwningMembership, String>;
}
pub trait MembershipDowncastRefMut<'a> {
    fn try_as_owning_membership_ref_mut(
        self,
    ) -> Result<OwningMembershipRefMut<'a>, String>;
}
pub trait MembershipDowncastRef<'a> {
    fn try_as_owning_membership_ref(self) -> Result<OwningMembershipRef<'a>, String>;
}
impl MembershipDowncast for Membership {
    fn try_into_owning_membership(self) -> Result<OwningMembership, String> {
        match self {
            Membership::OwningMembership(e) => Ok(e),
            _ => Err("Not a OwningMembership".into()),
        }
    }
}
impl<'a> MembershipDowncastRefMut<'a> for MembershipRefMut<'a> {
    fn try_as_owning_membership_ref_mut(
        self,
    ) -> Result<OwningMembershipRefMut<'a>, String> {
        match self {
            MembershipRefMut::OwningMembership(e) => Ok(e),
            _ => Err("Not a OwningMembership".into()),
        }
    }
}
impl<'a> MembershipDowncastRef<'a> for MembershipRef<'a> {
    fn try_as_owning_membership_ref(self) -> Result<OwningMembershipRef<'a>, String> {
        match self {
            MembershipRef::OwningMembership(e) => Ok(e),
            _ => Err("Not a OwningMembership".into()),
        }
    }
}
pub trait MembershipMethodsDescendants
where
    Self: DescendantOf<Membership>,
    Self::Via: MembershipMethods,
    Self: Sized,
{}
pub trait MembershipMethods: Sized {}
impl<T: MembershipMethodsDescendants> MembershipMethods for T
where
    T::Via: MembershipMethods,
{}
impl DescendantOf<Relationship> for Membership {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Membership {}
impl DescendantOf<Element> for Membership {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Membership {}
pub trait MembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MembershipRefMut<'a>>,
    Self::Via: MembershipRefMutMethods,
    Self: Sized,
{}
pub trait MembershipRefMutMethods: Sized {}
impl<'a, T: MembershipRefMutMethodsDescendants<'a>> MembershipRefMutMethods for T
where
    T::Via: MembershipRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for MembershipRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for MembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MembershipRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MembershipRefMut<'a> {}
pub trait MembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<MembershipRef<'a>>,
    Self::Via: MembershipRefMethods,
    Self: Sized,
{
    fn is_distinguishable_from_impl(
        self,
        other: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> bool {
        self.into_via().is_distinguishable_from(other)
    }
}
pub trait MembershipRefMethods: Sized {
    fn is_distinguishable_from(
        self,
        other: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> bool;
}
impl<'a, T: MembershipRefMethodsDescendants<'a>> MembershipRefMethods for T
where
    T::Via: MembershipRefMethods,
{
    fn is_distinguishable_from(
        self,
        other: std::rc::Rc<std::cell::RefCell<super::membership::Membership>>,
    ) -> bool {
        MembershipRefMethodsDescendants::is_distinguishable_from_impl(self, other)
    }
}
impl<'a> DescendantOf<RelationshipRef<'a>> for MembershipRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for MembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MembershipRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MembershipRef<'a> {}

