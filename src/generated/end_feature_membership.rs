#![allow(unused)]
use super::utils::DescendantOf;
use super::feature_membership::{
    FeatureMembership, FeatureMembershipRefMut, FeatureMembershipRef,
    FeatureMembershipStruct, FeatureMembershipStructRefMut, FeatureMembershipStructRef,
    FeatureMembershipInner, FeatureMembershipUpcast, FeatureMembershipUpcastRefMut,
    FeatureMembershipUpcastRef, FeatureMembershipMethodsDescendants,
    FeatureMembershipRefMutMethodsDescendants, FeatureMembershipRefMethodsDescendants,
};
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
pub struct EndFeatureMembershipInner {
    pub(super) sup_feature_membership: FeatureMembershipInner,
}
pub trait EndFeatureMembershipStruct
where
    Self: EndFeatureMembershipStructRefMut,
    Self: EndFeatureMembershipStructRef,
    Self: FeatureMembershipStruct,
{}
pub trait EndFeatureMembershipStructRefMut
where
    Self: EndFeatureMembershipStructRef,
    Self: FeatureMembershipStructRefMut,
{}
pub trait EndFeatureMembershipStructRef
where
    Self: FeatureMembershipStructRef,
{}
pub trait EndFeatureMembershipUpcast: EndFeatureMembershipStruct {
    fn into_end_feature_membership(self) -> EndFeatureMembership;
}
pub trait EndFeatureMembershipUpcastRefMut<'a>: EndFeatureMembershipStructRefMut {
    fn as_end_feature_membership_ref_mut(self) -> EndFeatureMembershipRefMut<'a>;
}
pub trait EndFeatureMembershipUpcastRef<'a>: EndFeatureMembershipStructRef {
    fn as_end_feature_membership_ref(self) -> EndFeatureMembershipRef<'a>;
}
impl EndFeatureMembershipStruct for EndFeatureMembershipInner {}
impl EndFeatureMembershipStructRefMut for EndFeatureMembershipInner {}
impl EndFeatureMembershipStructRef for EndFeatureMembershipInner {}
impl FeatureMembershipStruct for EndFeatureMembershipInner {}
impl FeatureMembershipStructRefMut for EndFeatureMembershipInner {}
impl FeatureMembershipStructRef for EndFeatureMembershipInner {}
impl OwningMembershipStruct for EndFeatureMembershipInner {}
impl OwningMembershipStructRefMut for EndFeatureMembershipInner {}
impl OwningMembershipStructRef for EndFeatureMembershipInner {}
impl MembershipStruct for EndFeatureMembershipInner {
    fn member_short_name(self) -> Option<String> {
        self.sup_feature_membership.member_short_name()
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_feature_membership.member_element()
    }
    fn member_name(self) -> Option<String> {
        self.sup_feature_membership.member_name()
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_feature_membership.visibility()
    }
}
impl MembershipStructRefMut for EndFeatureMembershipInner {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature_membership.member_short_name_ref_mut()
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_feature_membership.member_element_ref_mut()
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature_membership.member_name_ref_mut()
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_feature_membership.visibility_ref_mut()
    }
}
impl MembershipStructRef for EndFeatureMembershipInner {
    fn member_short_name_ref(&self) -> &Option<String> {
        self.sup_feature_membership.member_short_name_ref()
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_feature_membership.member_element_ref()
    }
    fn member_name_ref(&self) -> &Option<String> {
        self.sup_feature_membership.member_name_ref()
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_feature_membership.visibility_ref()
    }
}
impl RelationshipStruct for EndFeatureMembershipInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_feature_membership.is_implied()
    }
}
impl RelationshipStructRefMut for EndFeatureMembershipInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_feature_membership.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for EndFeatureMembershipInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_feature_membership.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_feature_membership.is_implied_ref()
    }
}
impl ElementStruct for EndFeatureMembershipInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature_membership.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature_membership.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_feature_membership.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_feature_membership.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_feature_membership.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_feature_membership.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_feature_membership.is_implied_included()
    }
}
impl ElementStructRefMut for EndFeatureMembershipInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature_membership.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_feature_membership.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_feature_membership.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_feature_membership.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature_membership.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature_membership.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_feature_membership.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for EndFeatureMembershipInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature_membership.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature_membership.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_feature_membership.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_feature_membership.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_feature_membership.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_feature_membership.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_feature_membership.is_implied_included_ref()
    }
}
pub enum EndFeatureMembership {
    Itself(EndFeatureMembershipInner),
}
pub enum EndFeatureMembershipRefMut<'a> {
    Itself(&'a mut EndFeatureMembershipInner),
}
pub enum EndFeatureMembershipRef<'a> {
    Itself(&'a EndFeatureMembershipInner),
}
impl EndFeatureMembership {
    pub fn as_ref(&self) -> EndFeatureMembershipRef {
        match self {
            EndFeatureMembership::Itself(inner) => {
                EndFeatureMembershipRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> EndFeatureMembershipRefMut {
        match self {
            EndFeatureMembership::Itself(inner) => {
                EndFeatureMembershipRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> EndFeatureMembershipRefMut<'a> {
    pub fn as_ref(self) -> EndFeatureMembershipRef<'a> {
        match self {
            EndFeatureMembershipRefMut::Itself(inner) => {
                EndFeatureMembershipRef::Itself(inner)
            }
        }
    }
}
impl EndFeatureMembershipStruct for EndFeatureMembership {}
impl EndFeatureMembershipStructRefMut for EndFeatureMembership {}
impl EndFeatureMembershipStructRef for EndFeatureMembership {}
impl<'a> EndFeatureMembershipStructRefMut for EndFeatureMembershipRefMut<'a> {}
impl<'a> EndFeatureMembershipStructRef for EndFeatureMembershipRefMut<'a> {}
impl<'a> EndFeatureMembershipStructRef for EndFeatureMembershipRef<'a> {}
impl FeatureMembershipStruct for EndFeatureMembership {}
impl FeatureMembershipStructRefMut for EndFeatureMembership {}
impl FeatureMembershipStructRef for EndFeatureMembership {}
impl<'a> FeatureMembershipStructRefMut for EndFeatureMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for EndFeatureMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for EndFeatureMembershipRef<'a> {}
impl OwningMembershipStruct for EndFeatureMembership {}
impl OwningMembershipStructRefMut for EndFeatureMembership {}
impl OwningMembershipStructRef for EndFeatureMembership {}
impl<'a> OwningMembershipStructRefMut for EndFeatureMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for EndFeatureMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for EndFeatureMembershipRef<'a> {}
impl MembershipStruct for EndFeatureMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembership::Itself(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for EndFeatureMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembership::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for EndFeatureMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembership::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for EndFeatureMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for EndFeatureMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for EndFeatureMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for EndFeatureMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for EndFeatureMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for EndFeatureMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for EndFeatureMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for EndFeatureMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for EndFeatureMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for EndFeatureMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            EndFeatureMembership::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for EndFeatureMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            EndFeatureMembership::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for EndFeatureMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembership::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            EndFeatureMembership::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembership::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            EndFeatureMembership::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for EndFeatureMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for EndFeatureMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            EndFeatureMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for EndFeatureMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            EndFeatureMembershipRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl EndFeatureMembershipUpcast for EndFeatureMembership {
    fn into_end_feature_membership(self) -> EndFeatureMembership {
        self
    }
}
impl<'a> EndFeatureMembershipUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_end_feature_membership_ref_mut(self) -> EndFeatureMembershipRefMut<'a> {
        self
    }
}
impl<'a> EndFeatureMembershipUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_end_feature_membership_ref(self) -> EndFeatureMembershipRef<'a> {
        self
    }
}
impl FeatureMembershipUpcast for EndFeatureMembership {
    fn into_feature_membership(self) -> FeatureMembership {
        FeatureMembership::EndFeatureMembership(self).into_feature_membership()
    }
}
impl<'a> FeatureMembershipUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a> {
        FeatureMembershipRefMut::EndFeatureMembership(self)
            .as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a> {
        FeatureMembershipRef::EndFeatureMembership(self).as_feature_membership_ref()
    }
}
impl OwningMembershipUpcast for EndFeatureMembership {
    fn into_owning_membership(self) -> OwningMembership {
        FeatureMembership::EndFeatureMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        FeatureMembershipRefMut::EndFeatureMembership(self)
            .as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        FeatureMembershipRef::EndFeatureMembership(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for EndFeatureMembership {
    fn into_membership(self) -> Membership {
        FeatureMembership::EndFeatureMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        FeatureMembershipRefMut::EndFeatureMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        FeatureMembershipRef::EndFeatureMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for EndFeatureMembership {
    fn into_relationship(self) -> Relationship {
        FeatureMembership::EndFeatureMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        FeatureMembershipRefMut::EndFeatureMembership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        FeatureMembershipRef::EndFeatureMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for EndFeatureMembership {
    fn into_element(self) -> Element {
        FeatureMembership::EndFeatureMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for EndFeatureMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureMembershipRefMut::EndFeatureMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for EndFeatureMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureMembershipRef::EndFeatureMembership(self).as_element_ref()
    }
}
pub trait EndFeatureMembershipDowncast {}
pub trait EndFeatureMembershipDowncastRefMut<'a> {}
pub trait EndFeatureMembershipDowncastRef<'a> {}
impl EndFeatureMembershipDowncast for EndFeatureMembership {}
impl<'a> EndFeatureMembershipDowncastRefMut<'a> for EndFeatureMembershipRefMut<'a> {}
impl<'a> EndFeatureMembershipDowncastRef<'a> for EndFeatureMembershipRef<'a> {}
pub trait EndFeatureMembershipMethodsDescendants
where
    Self: DescendantOf<EndFeatureMembership>,
    Self::Via: EndFeatureMembershipMethods,
    Self: Sized,
{}
pub trait EndFeatureMembershipMethods: Sized {}
impl<T: EndFeatureMembershipMethodsDescendants> EndFeatureMembershipMethods for T
where
    T::Via: EndFeatureMembershipMethods,
{}
impl DescendantOf<FeatureMembership> for EndFeatureMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl FeatureMembershipMethodsDescendants for EndFeatureMembership {}
impl DescendantOf<OwningMembership> for EndFeatureMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl OwningMembershipMethodsDescendants for EndFeatureMembership {}
impl DescendantOf<Membership> for EndFeatureMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl MembershipMethodsDescendants for EndFeatureMembership {}
impl DescendantOf<Relationship> for EndFeatureMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl RelationshipMethodsDescendants for EndFeatureMembership {}
impl DescendantOf<Element> for EndFeatureMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl ElementMethodsDescendants for EndFeatureMembership {}
pub trait EndFeatureMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<EndFeatureMembershipRefMut<'a>>,
    Self::Via: EndFeatureMembershipRefMutMethods,
    Self: Sized,
{}
pub trait EndFeatureMembershipRefMutMethods: Sized {}
impl<
    'a,
    T: EndFeatureMembershipRefMutMethodsDescendants<'a>,
> EndFeatureMembershipRefMutMethods for T
where
    T::Via: EndFeatureMembershipRefMutMethods,
{}
impl<'a> DescendantOf<FeatureMembershipRefMut<'a>> for EndFeatureMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipRefMutMethodsDescendants<'a>
for EndFeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>> for EndFeatureMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a>
for EndFeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for EndFeatureMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for EndFeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for EndFeatureMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for EndFeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for EndFeatureMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for EndFeatureMembershipRefMut<'a> {}
pub trait EndFeatureMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<EndFeatureMembershipRef<'a>>,
    Self::Via: EndFeatureMembershipRefMethods,
    Self: Sized,
{}
pub trait EndFeatureMembershipRefMethods: Sized {}
impl<'a, T: EndFeatureMembershipRefMethodsDescendants<'a>> EndFeatureMembershipRefMethods
for T
where
    T::Via: EndFeatureMembershipRefMethods,
{}
impl<'a> DescendantOf<FeatureMembershipRef<'a>> for EndFeatureMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> FeatureMembershipRefMethodsDescendants<'a> for EndFeatureMembershipRef<'a> {}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for EndFeatureMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for EndFeatureMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for EndFeatureMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for EndFeatureMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for EndFeatureMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for EndFeatureMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for EndFeatureMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for EndFeatureMembershipRef<'a> {}

