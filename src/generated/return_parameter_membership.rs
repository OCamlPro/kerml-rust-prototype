#![allow(unused)]
use super::utils::DescendantOf;
use super::parameter_membership::{
    ParameterMembership, ParameterMembershipRefMut, ParameterMembershipRef,
    ParameterMembershipStruct, ParameterMembershipStructRefMut,
    ParameterMembershipStructRef, ParameterMembershipInner, ParameterMembershipUpcast,
    ParameterMembershipUpcastRefMut, ParameterMembershipUpcastRef,
    ParameterMembershipMethodsDescendants, ParameterMembershipRefMutMethodsDescendants,
    ParameterMembershipRefMethodsDescendants,
};
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
pub struct ReturnParameterMembershipInner {
    pub(super) sup_parameter_membership: ParameterMembershipInner,
}
pub trait ReturnParameterMembershipStruct
where
    Self: ReturnParameterMembershipStructRefMut,
    Self: ReturnParameterMembershipStructRef,
    Self: ParameterMembershipStruct,
{}
pub trait ReturnParameterMembershipStructRefMut
where
    Self: ReturnParameterMembershipStructRef,
    Self: ParameterMembershipStructRefMut,
{}
pub trait ReturnParameterMembershipStructRef
where
    Self: ParameterMembershipStructRef,
{}
pub trait ReturnParameterMembershipUpcast: ReturnParameterMembershipStruct {
    fn into_return_parameter_membership(self) -> ReturnParameterMembership;
}
pub trait ReturnParameterMembershipUpcastRefMut<
    'a,
>: ReturnParameterMembershipStructRefMut {
    fn as_return_parameter_membership_ref_mut(
        self,
    ) -> ReturnParameterMembershipRefMut<'a>;
}
pub trait ReturnParameterMembershipUpcastRef<'a>: ReturnParameterMembershipStructRef {
    fn as_return_parameter_membership_ref(self) -> ReturnParameterMembershipRef<'a>;
}
impl ReturnParameterMembershipStruct for ReturnParameterMembershipInner {}
impl ReturnParameterMembershipStructRefMut for ReturnParameterMembershipInner {}
impl ReturnParameterMembershipStructRef for ReturnParameterMembershipInner {}
impl ParameterMembershipStruct for ReturnParameterMembershipInner {}
impl ParameterMembershipStructRefMut for ReturnParameterMembershipInner {}
impl ParameterMembershipStructRef for ReturnParameterMembershipInner {}
impl FeatureMembershipStruct for ReturnParameterMembershipInner {}
impl FeatureMembershipStructRefMut for ReturnParameterMembershipInner {}
impl FeatureMembershipStructRef for ReturnParameterMembershipInner {}
impl OwningMembershipStruct for ReturnParameterMembershipInner {}
impl OwningMembershipStructRefMut for ReturnParameterMembershipInner {}
impl OwningMembershipStructRef for ReturnParameterMembershipInner {}
impl MembershipStruct for ReturnParameterMembershipInner {
    fn member_short_name(self) -> Option<String> {
        self.sup_parameter_membership.member_short_name()
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_parameter_membership.member_element()
    }
    fn member_name(self) -> Option<String> {
        self.sup_parameter_membership.member_name()
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_parameter_membership.visibility()
    }
}
impl MembershipStructRefMut for ReturnParameterMembershipInner {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_parameter_membership.member_short_name_ref_mut()
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_parameter_membership.member_element_ref_mut()
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_parameter_membership.member_name_ref_mut()
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_parameter_membership.visibility_ref_mut()
    }
}
impl MembershipStructRef for ReturnParameterMembershipInner {
    fn member_short_name_ref(&self) -> &Option<String> {
        self.sup_parameter_membership.member_short_name_ref()
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_parameter_membership.member_element_ref()
    }
    fn member_name_ref(&self) -> &Option<String> {
        self.sup_parameter_membership.member_name_ref()
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_parameter_membership.visibility_ref()
    }
}
impl RelationshipStruct for ReturnParameterMembershipInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_parameter_membership.is_implied()
    }
}
impl RelationshipStructRefMut for ReturnParameterMembershipInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_parameter_membership.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for ReturnParameterMembershipInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_parameter_membership.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_parameter_membership.is_implied_ref()
    }
}
impl ElementStruct for ReturnParameterMembershipInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_parameter_membership.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_parameter_membership.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_parameter_membership.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_parameter_membership.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_parameter_membership.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_parameter_membership.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_parameter_membership.is_implied_included()
    }
}
impl ElementStructRefMut for ReturnParameterMembershipInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_parameter_membership.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_parameter_membership.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_parameter_membership.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_parameter_membership.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_parameter_membership.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_parameter_membership.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_parameter_membership.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for ReturnParameterMembershipInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_parameter_membership.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_parameter_membership.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_parameter_membership.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_parameter_membership.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_parameter_membership.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_parameter_membership.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_parameter_membership.is_implied_included_ref()
    }
}
pub enum ReturnParameterMembership {
    Itself(ReturnParameterMembershipInner),
}
pub enum ReturnParameterMembershipRefMut<'a> {
    Itself(&'a mut ReturnParameterMembershipInner),
}
pub enum ReturnParameterMembershipRef<'a> {
    Itself(&'a ReturnParameterMembershipInner),
}
impl ReturnParameterMembership {
    pub fn as_ref(&self) -> ReturnParameterMembershipRef {
        match self {
            ReturnParameterMembership::Itself(inner) => {
                ReturnParameterMembershipRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ReturnParameterMembershipRefMut {
        match self {
            ReturnParameterMembership::Itself(inner) => {
                ReturnParameterMembershipRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> ReturnParameterMembershipRefMut<'a> {
    pub fn as_ref(self) -> ReturnParameterMembershipRef<'a> {
        match self {
            ReturnParameterMembershipRefMut::Itself(inner) => {
                ReturnParameterMembershipRef::Itself(inner)
            }
        }
    }
}
impl ReturnParameterMembershipStruct for ReturnParameterMembership {}
impl ReturnParameterMembershipStructRefMut for ReturnParameterMembership {}
impl ReturnParameterMembershipStructRef for ReturnParameterMembership {}
impl<'a> ReturnParameterMembershipStructRefMut for ReturnParameterMembershipRefMut<'a> {}
impl<'a> ReturnParameterMembershipStructRef for ReturnParameterMembershipRefMut<'a> {}
impl<'a> ReturnParameterMembershipStructRef for ReturnParameterMembershipRef<'a> {}
impl ParameterMembershipStruct for ReturnParameterMembership {}
impl ParameterMembershipStructRefMut for ReturnParameterMembership {}
impl ParameterMembershipStructRef for ReturnParameterMembership {}
impl<'a> ParameterMembershipStructRefMut for ReturnParameterMembershipRefMut<'a> {}
impl<'a> ParameterMembershipStructRef for ReturnParameterMembershipRefMut<'a> {}
impl<'a> ParameterMembershipStructRef for ReturnParameterMembershipRef<'a> {}
impl FeatureMembershipStruct for ReturnParameterMembership {}
impl FeatureMembershipStructRefMut for ReturnParameterMembership {}
impl FeatureMembershipStructRef for ReturnParameterMembership {}
impl<'a> FeatureMembershipStructRefMut for ReturnParameterMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ReturnParameterMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ReturnParameterMembershipRef<'a> {}
impl OwningMembershipStruct for ReturnParameterMembership {}
impl OwningMembershipStructRefMut for ReturnParameterMembership {}
impl OwningMembershipStructRef for ReturnParameterMembership {}
impl<'a> OwningMembershipStructRefMut for ReturnParameterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ReturnParameterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ReturnParameterMembershipRef<'a> {}
impl MembershipStruct for ReturnParameterMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for ReturnParameterMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for ReturnParameterMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for ReturnParameterMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for ReturnParameterMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for ReturnParameterMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for ReturnParameterMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for ReturnParameterMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for ReturnParameterMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ReturnParameterMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ReturnParameterMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ReturnParameterMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for ReturnParameterMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ReturnParameterMembership::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ReturnParameterMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ReturnParameterMembership::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for ReturnParameterMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembership::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReturnParameterMembership::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembership::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReturnParameterMembership::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ReturnParameterMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ReturnParameterMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReturnParameterMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ReturnParameterMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ReturnParameterMembershipRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ReturnParameterMembershipUpcast for ReturnParameterMembership {
    fn into_return_parameter_membership(self) -> ReturnParameterMembership {
        self
    }
}
impl<'a> ReturnParameterMembershipUpcastRefMut<'a>
for ReturnParameterMembershipRefMut<'a> {
    fn as_return_parameter_membership_ref_mut(
        self,
    ) -> ReturnParameterMembershipRefMut<'a> {
        self
    }
}
impl<'a> ReturnParameterMembershipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_return_parameter_membership_ref(self) -> ReturnParameterMembershipRef<'a> {
        self
    }
}
impl ParameterMembershipUpcast for ReturnParameterMembership {
    fn into_parameter_membership(self) -> ParameterMembership {
        ParameterMembership::ReturnParameterMembership(self).into_parameter_membership()
    }
}
impl<'a> ParameterMembershipUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_parameter_membership_ref_mut(self) -> ParameterMembershipRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self)
            .as_parameter_membership_ref_mut()
    }
}
impl<'a> ParameterMembershipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_parameter_membership_ref(self) -> ParameterMembershipRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self)
            .as_parameter_membership_ref()
    }
}
impl FeatureMembershipUpcast for ReturnParameterMembership {
    fn into_feature_membership(self) -> FeatureMembership {
        ParameterMembership::ReturnParameterMembership(self).into_feature_membership()
    }
}
impl<'a> FeatureMembershipUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self)
            .as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self)
            .as_feature_membership_ref()
    }
}
impl OwningMembershipUpcast for ReturnParameterMembership {
    fn into_owning_membership(self) -> OwningMembership {
        ParameterMembership::ReturnParameterMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self)
            .as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self)
            .as_owning_membership_ref()
    }
}
impl MembershipUpcast for ReturnParameterMembership {
    fn into_membership(self) -> Membership {
        ParameterMembership::ReturnParameterMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self)
            .as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for ReturnParameterMembership {
    fn into_relationship(self) -> Relationship {
        ParameterMembership::ReturnParameterMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self)
            .as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for ReturnParameterMembership {
    fn into_element(self) -> Element {
        ParameterMembership::ReturnParameterMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ReturnParameterMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ParameterMembershipRefMut::ReturnParameterMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ReturnParameterMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ParameterMembershipRef::ReturnParameterMembership(self).as_element_ref()
    }
}
pub trait ReturnParameterMembershipDowncast {}
pub trait ReturnParameterMembershipDowncastRefMut<'a> {}
pub trait ReturnParameterMembershipDowncastRef<'a> {}
impl ReturnParameterMembershipDowncast for ReturnParameterMembership {}
impl<'a> ReturnParameterMembershipDowncastRefMut<'a>
for ReturnParameterMembershipRefMut<'a> {}
impl<'a> ReturnParameterMembershipDowncastRef<'a> for ReturnParameterMembershipRef<'a> {}
pub trait ReturnParameterMembershipMethodsDescendants
where
    Self: DescendantOf<ReturnParameterMembership>,
    Self::Via: ReturnParameterMembershipMethods,
    Self: Sized,
{}
pub trait ReturnParameterMembershipMethods: Sized {}
impl<T: ReturnParameterMembershipMethodsDescendants> ReturnParameterMembershipMethods
for T
where
    T::Via: ReturnParameterMembershipMethods,
{}
impl DescendantOf<ParameterMembership> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl ParameterMembershipMethodsDescendants for ReturnParameterMembership {}
impl DescendantOf<FeatureMembership> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl FeatureMembershipMethodsDescendants for ReturnParameterMembership {}
impl DescendantOf<OwningMembership> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl OwningMembershipMethodsDescendants for ReturnParameterMembership {}
impl DescendantOf<Membership> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl MembershipMethodsDescendants for ReturnParameterMembership {}
impl DescendantOf<Relationship> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl RelationshipMethodsDescendants for ReturnParameterMembership {}
impl DescendantOf<Element> for ReturnParameterMembership {
    type Via = ParameterMembership;
    fn into_via(self) -> Self::Via {
        self.into_parameter_membership()
    }
}
impl ElementMethodsDescendants for ReturnParameterMembership {}
pub trait ReturnParameterMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ReturnParameterMembershipRefMut<'a>>,
    Self::Via: ReturnParameterMembershipRefMutMethods,
    Self: Sized,
{}
pub trait ReturnParameterMembershipRefMutMethods: Sized {}
impl<
    'a,
    T: ReturnParameterMembershipRefMutMethodsDescendants<'a>,
> ReturnParameterMembershipRefMutMethods for T
where
    T::Via: ReturnParameterMembershipRefMutMethods,
{}
impl<'a> DescendantOf<ParameterMembershipRefMut<'a>>
for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> ParameterMembershipRefMutMethodsDescendants<'a>
for ReturnParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<FeatureMembershipRefMut<'a>>
for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipRefMutMethodsDescendants<'a>
for ReturnParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>>
for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a>
for ReturnParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for ReturnParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a>
for ReturnParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ReturnParameterMembershipRefMut<'a> {
    type Via = ParameterMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ReturnParameterMembershipRefMut<'a> {}
pub trait ReturnParameterMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<ReturnParameterMembershipRef<'a>>,
    Self::Via: ReturnParameterMembershipRefMethods,
    Self: Sized,
{}
pub trait ReturnParameterMembershipRefMethods: Sized {}
impl<
    'a,
    T: ReturnParameterMembershipRefMethodsDescendants<'a>,
> ReturnParameterMembershipRefMethods for T
where
    T::Via: ReturnParameterMembershipRefMethods,
{}
impl<'a> DescendantOf<ParameterMembershipRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> ParameterMembershipRefMethodsDescendants<'a>
for ReturnParameterMembershipRef<'a> {}
impl<'a> DescendantOf<FeatureMembershipRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> FeatureMembershipRefMethodsDescendants<'a>
for ReturnParameterMembershipRef<'a> {}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for ReturnParameterMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for ReturnParameterMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ReturnParameterMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ReturnParameterMembershipRef<'a> {
    type Via = ParameterMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_parameter_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ReturnParameterMembershipRef<'a> {}

