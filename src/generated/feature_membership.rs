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
use super::result_expression_membership::{
    ResultExpressionMembership, ResultExpressionMembershipRefMut,
    ResultExpressionMembershipRef,
};
use super::parameter_membership::{
    ParameterMembership, ParameterMembershipRefMut, ParameterMembershipRef,
};
use super::end_feature_membership::{
    EndFeatureMembership, EndFeatureMembershipRefMut, EndFeatureMembershipRef,
};
pub struct FeatureMembershipInner {
    pub(super) sup_owning_membership: OwningMembershipInner,
}
pub trait FeatureMembershipStruct
where
    Self: FeatureMembershipStructRefMut,
    Self: FeatureMembershipStructRef,
    Self: OwningMembershipStruct,
{}
pub trait FeatureMembershipStructRefMut
where
    Self: FeatureMembershipStructRef,
    Self: OwningMembershipStructRefMut,
{}
pub trait FeatureMembershipStructRef
where
    Self: OwningMembershipStructRef,
{}
pub trait FeatureMembershipUpcast: FeatureMembershipStruct {
    fn into_feature_membership(self) -> FeatureMembership;
}
pub trait FeatureMembershipUpcastRefMut<'a>: FeatureMembershipStructRefMut {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a>;
}
pub trait FeatureMembershipUpcastRef<'a>: FeatureMembershipStructRef {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a>;
}
impl FeatureMembershipStruct for FeatureMembershipInner {}
impl FeatureMembershipStructRefMut for FeatureMembershipInner {}
impl FeatureMembershipStructRef for FeatureMembershipInner {}
impl OwningMembershipStruct for FeatureMembershipInner {}
impl OwningMembershipStructRefMut for FeatureMembershipInner {}
impl OwningMembershipStructRef for FeatureMembershipInner {}
impl MembershipStruct for FeatureMembershipInner {
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
impl MembershipStructRefMut for FeatureMembershipInner {
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
impl MembershipStructRef for FeatureMembershipInner {
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
impl RelationshipStruct for FeatureMembershipInner {
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
impl RelationshipStructRefMut for FeatureMembershipInner {
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
impl RelationshipStructRef for FeatureMembershipInner {
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
impl ElementStruct for FeatureMembershipInner {
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
impl ElementStructRefMut for FeatureMembershipInner {
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
impl ElementStructRef for FeatureMembershipInner {
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
pub enum FeatureMembership {
    Itself(FeatureMembershipInner),
    ResultExpressionMembership(ResultExpressionMembership),
    ParameterMembership(ParameterMembership),
    EndFeatureMembership(EndFeatureMembership),
}
pub enum FeatureMembershipRefMut<'a> {
    Itself(&'a mut FeatureMembershipInner),
    ResultExpressionMembership(ResultExpressionMembershipRefMut<'a>),
    ParameterMembership(ParameterMembershipRefMut<'a>),
    EndFeatureMembership(EndFeatureMembershipRefMut<'a>),
}
pub enum FeatureMembershipRef<'a> {
    Itself(&'a FeatureMembershipInner),
    ResultExpressionMembership(ResultExpressionMembershipRef<'a>),
    ParameterMembership(ParameterMembershipRef<'a>),
    EndFeatureMembership(EndFeatureMembershipRef<'a>),
}
impl FeatureMembership {
    pub fn as_ref(&self) -> FeatureMembershipRef {
        match self {
            FeatureMembership::Itself(inner) => FeatureMembershipRef::Itself(&inner),
            FeatureMembership::ResultExpressionMembership(inner) => {
                FeatureMembershipRef::ResultExpressionMembership(inner.as_ref())
            }
            FeatureMembership::ParameterMembership(inner) => {
                FeatureMembershipRef::ParameterMembership(inner.as_ref())
            }
            FeatureMembership::EndFeatureMembership(inner) => {
                FeatureMembershipRef::EndFeatureMembership(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureMembershipRefMut {
        match self {
            FeatureMembership::Itself(inner) => FeatureMembershipRefMut::Itself(inner),
            FeatureMembership::ResultExpressionMembership(inner) => {
                FeatureMembershipRefMut::ResultExpressionMembership(inner.as_ref_mut())
            }
            FeatureMembership::ParameterMembership(inner) => {
                FeatureMembershipRefMut::ParameterMembership(inner.as_ref_mut())
            }
            FeatureMembership::EndFeatureMembership(inner) => {
                FeatureMembershipRefMut::EndFeatureMembership(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> FeatureMembershipRefMut<'a> {
    pub fn as_ref(self) -> FeatureMembershipRef<'a> {
        match self {
            FeatureMembershipRefMut::Itself(inner) => FeatureMembershipRef::Itself(inner),
            FeatureMembershipRefMut::ResultExpressionMembership(inner) => {
                FeatureMembershipRef::ResultExpressionMembership(inner.as_ref())
            }
            FeatureMembershipRefMut::ParameterMembership(inner) => {
                FeatureMembershipRef::ParameterMembership(inner.as_ref())
            }
            FeatureMembershipRefMut::EndFeatureMembership(inner) => {
                FeatureMembershipRef::EndFeatureMembership(inner.as_ref())
            }
        }
    }
}
impl FeatureMembershipStruct for FeatureMembership {}
impl FeatureMembershipStructRefMut for FeatureMembership {}
impl FeatureMembershipStructRef for FeatureMembership {}
impl<'a> FeatureMembershipStructRefMut for FeatureMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for FeatureMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for FeatureMembershipRef<'a> {}
impl OwningMembershipStruct for FeatureMembership {}
impl OwningMembershipStructRefMut for FeatureMembership {}
impl OwningMembershipStructRef for FeatureMembership {}
impl<'a> OwningMembershipStructRefMut for FeatureMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for FeatureMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for FeatureMembershipRef<'a> {}
impl MembershipStruct for FeatureMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_short_name(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_short_name(),
            FeatureMembership::ParameterMembership(x) => x.member_short_name(),
            FeatureMembership::EndFeatureMembership(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembership::Itself(x) => x.member_element(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_element(),
            FeatureMembership::ParameterMembership(x) => x.member_element(),
            FeatureMembership::EndFeatureMembership(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_name(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_name(),
            FeatureMembership::ParameterMembership(x) => x.member_name(),
            FeatureMembership::EndFeatureMembership(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembership::Itself(x) => x.visibility(),
            FeatureMembership::ResultExpressionMembership(x) => x.visibility(),
            FeatureMembership::ParameterMembership(x) => x.visibility(),
            FeatureMembership::EndFeatureMembership(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for FeatureMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_short_name_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.member_short_name_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.member_short_name_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembership::Itself(x) => x.member_element_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.member_element_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.member_element_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_name_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_name_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.member_name_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembership::Itself(x) => x.visibility_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.visibility_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.visibility_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for FeatureMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_short_name_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_short_name_ref(),
            FeatureMembership::ParameterMembership(x) => x.member_short_name_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembership::Itself(x) => x.member_element_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_element_ref(),
            FeatureMembership::ParameterMembership(x) => x.member_element_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.member_name_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.member_name_ref(),
            FeatureMembership::ParameterMembership(x) => x.member_name_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembership::Itself(x) => x.visibility_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.visibility_ref(),
            FeatureMembership::ParameterMembership(x) => x.visibility_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for FeatureMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.member_short_name_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.member_short_name_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.member_short_name_ref_mut()
            }
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.member_element_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.member_element_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.member_element_ref_mut()
            }
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.member_name_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.member_name_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.visibility_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.visibility_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for FeatureMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_short_name_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.member_short_name_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.member_short_name_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_element_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.member_element_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.member_element_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.member_name_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.member_name_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.member_name_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.visibility_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.visibility_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.visibility_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for FeatureMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRef::Itself(x) => x.member_short_name_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.member_short_name_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.member_short_name_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.member_element_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.member_element_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.member_element_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRef::Itself(x) => x.member_name_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.member_name_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.member_name_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.visibility_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.visibility_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.visibility_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for FeatureMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.target(),
            FeatureMembership::ResultExpressionMembership(x) => x.target(),
            FeatureMembership::ParameterMembership(x) => x.target(),
            FeatureMembership::EndFeatureMembership(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.source(),
            FeatureMembership::ResultExpressionMembership(x) => x.source(),
            FeatureMembership::ParameterMembership(x) => x.source(),
            FeatureMembership::EndFeatureMembership(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owning_related_element(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owning_related_element()
            }
            FeatureMembership::ParameterMembership(x) => x.owning_related_element(),
            FeatureMembership::EndFeatureMembership(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_related_element(),
            FeatureMembership::ResultExpressionMembership(x) => x.owned_related_element(),
            FeatureMembership::ParameterMembership(x) => x.owned_related_element(),
            FeatureMembership::EndFeatureMembership(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied(),
            FeatureMembership::ResultExpressionMembership(x) => x.is_implied(),
            FeatureMembership::ParameterMembership(x) => x.is_implied(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for FeatureMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.target_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.target_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.target_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.source_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.source_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.source_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owning_related_element_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owning_related_element_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
            FeatureMembership::EndFeatureMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_related_element_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owned_related_element_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
            FeatureMembership::EndFeatureMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.is_implied_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.is_implied_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for FeatureMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.target_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.target_ref(),
            FeatureMembership::ParameterMembership(x) => x.target_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.source_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.source_ref(),
            FeatureMembership::ParameterMembership(x) => x.source_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owning_related_element_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owning_related_element_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.owning_related_element_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_related_element_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owned_related_element_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.owned_related_element_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.is_implied_ref(),
            FeatureMembership::ParameterMembership(x) => x.is_implied_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FeatureMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.target_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.target_ref_mut(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.target_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.source_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.source_ref_mut(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.source_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owning_related_element_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owned_related_element_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.is_implied_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.is_implied_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.target_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.target_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.target_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.source_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.source_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.source_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owning_related_element_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owning_related_element_ref()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owned_related_element_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owned_related_element_ref()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.is_implied_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.is_implied_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.is_implied_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FeatureMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.target_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.target_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.target_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.source_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.source_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.source_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.owning_related_element_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.owning_related_element_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => {
                x.owning_related_element_ref()
            }
            FeatureMembershipRef::EndFeatureMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.owned_related_element_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.owned_related_element_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.owned_related_element_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FeatureMembershipRef::Itself(x) => x.is_implied_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.is_implied_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.is_implied_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for FeatureMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_relationship(),
            FeatureMembership::ResultExpressionMembership(x) => x.owned_relationship(),
            FeatureMembership::ParameterMembership(x) => x.owned_relationship(),
            FeatureMembership::EndFeatureMembership(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembership::Itself(x) => x.owning_relationship(),
            FeatureMembership::ResultExpressionMembership(x) => x.owning_relationship(),
            FeatureMembership::ParameterMembership(x) => x.owning_relationship(),
            FeatureMembership::EndFeatureMembership(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureMembership::Itself(x) => x.element_id(),
            FeatureMembership::ResultExpressionMembership(x) => x.element_id(),
            FeatureMembership::ParameterMembership(x) => x.element_id(),
            FeatureMembership::EndFeatureMembership(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureMembership::Itself(x) => x.alias_ids(),
            FeatureMembership::ResultExpressionMembership(x) => x.alias_ids(),
            FeatureMembership::ParameterMembership(x) => x.alias_ids(),
            FeatureMembership::EndFeatureMembership(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_short_name(),
            FeatureMembership::ResultExpressionMembership(x) => x.declared_short_name(),
            FeatureMembership::ParameterMembership(x) => x.declared_short_name(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_name(),
            FeatureMembership::ResultExpressionMembership(x) => x.declared_name(),
            FeatureMembership::ParameterMembership(x) => x.declared_name(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied_included(),
            FeatureMembership::ResultExpressionMembership(x) => x.is_implied_included(),
            FeatureMembership::ParameterMembership(x) => x.is_implied_included(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_relationship_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owned_relationship_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.owned_relationship_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureMembership::Itself(x) => x.owning_relationship_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owning_relationship_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.owning_relationship_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureMembership::Itself(x) => x.element_id_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.element_id_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.element_id_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureMembership::Itself(x) => x.alias_ids_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.alias_ids_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.alias_ids_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_short_name_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.declared_short_name_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.declared_short_name_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_name_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => x.declared_name_ref_mut(),
            FeatureMembership::ParameterMembership(x) => x.declared_name_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied_included_ref_mut(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.is_implied_included_ref_mut()
            }
            FeatureMembership::ParameterMembership(x) => x.is_implied_included_ref_mut(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembership::Itself(x) => x.owned_relationship_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owned_relationship_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.owned_relationship_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembership::Itself(x) => x.owning_relationship_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.owning_relationship_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.owning_relationship_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureMembership::Itself(x) => x.element_id_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.element_id_ref(),
            FeatureMembership::ParameterMembership(x) => x.element_id_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureMembership::Itself(x) => x.alias_ids_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.alias_ids_ref(),
            FeatureMembership::ParameterMembership(x) => x.alias_ids_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_short_name_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.declared_short_name_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.declared_short_name_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembership::Itself(x) => x.declared_name_ref(),
            FeatureMembership::ResultExpressionMembership(x) => x.declared_name_ref(),
            FeatureMembership::ParameterMembership(x) => x.declared_name_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureMembership::Itself(x) => x.is_implied_included_ref(),
            FeatureMembership::ResultExpressionMembership(x) => {
                x.is_implied_included_ref()
            }
            FeatureMembership::ParameterMembership(x) => x.is_implied_included_ref(),
            FeatureMembership::EndFeatureMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owned_relationship_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owned_relationship_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owned_relationship_ref_mut()
            }
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owning_relationship_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owning_relationship_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.element_id_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.element_id_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.alias_ids_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.alias_ids_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.declared_short_name_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.declared_short_name_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.declared_name_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.declared_name_ref_mut(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.is_implied_included_ref_mut()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.is_implied_included_ref_mut()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for FeatureMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owned_relationship_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.owned_relationship_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.owning_relationship_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.owning_relationship_ref()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.element_id_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.element_id_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.element_id_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.alias_ids_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => x.alias_ids_ref(),
            FeatureMembershipRefMut::ParameterMembership(x) => x.alias_ids_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.declared_short_name_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.declared_short_name_ref()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.declared_name_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.declared_name_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => x.declared_name_ref(),
            FeatureMembershipRefMut::EndFeatureMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
            FeatureMembershipRefMut::ResultExpressionMembership(x) => {
                x.is_implied_included_ref()
            }
            FeatureMembershipRefMut::ParameterMembership(x) => {
                x.is_implied_included_ref()
            }
            FeatureMembershipRefMut::EndFeatureMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRef for FeatureMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.owned_relationship_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.owned_relationship_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.owned_relationship_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureMembershipRef::Itself(x) => x.owning_relationship_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.owning_relationship_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.owning_relationship_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureMembershipRef::Itself(x) => x.element_id_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.element_id_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.element_id_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureMembershipRef::Itself(x) => x.alias_ids_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.alias_ids_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.alias_ids_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRef::Itself(x) => x.declared_short_name_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.declared_short_name_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.declared_short_name_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureMembershipRef::Itself(x) => x.declared_name_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => x.declared_name_ref(),
            FeatureMembershipRef::ParameterMembership(x) => x.declared_name_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureMembershipRef::Itself(x) => x.is_implied_included_ref(),
            FeatureMembershipRef::ResultExpressionMembership(x) => {
                x.is_implied_included_ref()
            }
            FeatureMembershipRef::ParameterMembership(x) => x.is_implied_included_ref(),
            FeatureMembershipRef::EndFeatureMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureMembershipUpcast for FeatureMembership {
    fn into_feature_membership(self) -> FeatureMembership {
        self
    }
}
impl<'a> FeatureMembershipUpcastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a> {
        self
    }
}
impl<'a> FeatureMembershipUpcastRef<'a> for FeatureMembershipRef<'a> {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a> {
        self
    }
}
impl OwningMembershipUpcast for FeatureMembership {
    fn into_owning_membership(self) -> OwningMembership {
        OwningMembership::FeatureMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        OwningMembershipRefMut::FeatureMembership(self).as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for FeatureMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        OwningMembershipRef::FeatureMembership(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for FeatureMembership {
    fn into_membership(self) -> Membership {
        OwningMembership::FeatureMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        OwningMembershipRefMut::FeatureMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for FeatureMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        OwningMembershipRef::FeatureMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for FeatureMembership {
    fn into_relationship(self) -> Relationship {
        OwningMembership::FeatureMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        OwningMembershipRefMut::FeatureMembership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FeatureMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        OwningMembershipRef::FeatureMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for FeatureMembership {
    fn into_element(self) -> Element {
        OwningMembership::FeatureMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OwningMembershipRefMut::FeatureMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OwningMembershipRef::FeatureMembership(self).as_element_ref()
    }
}
pub trait FeatureMembershipDowncast {
    fn try_into_result_expression_membership(
        self,
    ) -> Result<ResultExpressionMembership, String>;
    fn try_into_parameter_membership(self) -> Result<ParameterMembership, String>;
    fn try_into_end_feature_membership(self) -> Result<EndFeatureMembership, String>;
}
pub trait FeatureMembershipDowncastRefMut<'a> {
    fn try_as_result_expression_membership_ref_mut(
        self,
    ) -> Result<ResultExpressionMembershipRefMut<'a>, String>;
    fn try_as_parameter_membership_ref_mut(
        self,
    ) -> Result<ParameterMembershipRefMut<'a>, String>;
    fn try_as_end_feature_membership_ref_mut(
        self,
    ) -> Result<EndFeatureMembershipRefMut<'a>, String>;
}
pub trait FeatureMembershipDowncastRef<'a> {
    fn try_as_result_expression_membership_ref(
        self,
    ) -> Result<ResultExpressionMembershipRef<'a>, String>;
    fn try_as_parameter_membership_ref(
        self,
    ) -> Result<ParameterMembershipRef<'a>, String>;
    fn try_as_end_feature_membership_ref(
        self,
    ) -> Result<EndFeatureMembershipRef<'a>, String>;
}
impl FeatureMembershipDowncast for FeatureMembership {
    fn try_into_result_expression_membership(
        self,
    ) -> Result<ResultExpressionMembership, String> {
        match self {
            FeatureMembership::ResultExpressionMembership(e) => Ok(e),
            _ => Err("Not a ResultExpressionMembership".into()),
        }
    }
    fn try_into_parameter_membership(self) -> Result<ParameterMembership, String> {
        match self {
            FeatureMembership::ParameterMembership(e) => Ok(e),
            _ => Err("Not a ParameterMembership".into()),
        }
    }
    fn try_into_end_feature_membership(self) -> Result<EndFeatureMembership, String> {
        match self {
            FeatureMembership::EndFeatureMembership(e) => Ok(e),
            _ => Err("Not a EndFeatureMembership".into()),
        }
    }
}
impl<'a> FeatureMembershipDowncastRefMut<'a> for FeatureMembershipRefMut<'a> {
    fn try_as_result_expression_membership_ref_mut(
        self,
    ) -> Result<ResultExpressionMembershipRefMut<'a>, String> {
        match self {
            FeatureMembershipRefMut::ResultExpressionMembership(e) => Ok(e),
            _ => Err("Not a ResultExpressionMembership".into()),
        }
    }
    fn try_as_parameter_membership_ref_mut(
        self,
    ) -> Result<ParameterMembershipRefMut<'a>, String> {
        match self {
            FeatureMembershipRefMut::ParameterMembership(e) => Ok(e),
            _ => Err("Not a ParameterMembership".into()),
        }
    }
    fn try_as_end_feature_membership_ref_mut(
        self,
    ) -> Result<EndFeatureMembershipRefMut<'a>, String> {
        match self {
            FeatureMembershipRefMut::EndFeatureMembership(e) => Ok(e),
            _ => Err("Not a EndFeatureMembership".into()),
        }
    }
}
impl<'a> FeatureMembershipDowncastRef<'a> for FeatureMembershipRef<'a> {
    fn try_as_result_expression_membership_ref(
        self,
    ) -> Result<ResultExpressionMembershipRef<'a>, String> {
        match self {
            FeatureMembershipRef::ResultExpressionMembership(e) => Ok(e),
            _ => Err("Not a ResultExpressionMembership".into()),
        }
    }
    fn try_as_parameter_membership_ref(
        self,
    ) -> Result<ParameterMembershipRef<'a>, String> {
        match self {
            FeatureMembershipRef::ParameterMembership(e) => Ok(e),
            _ => Err("Not a ParameterMembership".into()),
        }
    }
    fn try_as_end_feature_membership_ref(
        self,
    ) -> Result<EndFeatureMembershipRef<'a>, String> {
        match self {
            FeatureMembershipRef::EndFeatureMembership(e) => Ok(e),
            _ => Err("Not a EndFeatureMembership".into()),
        }
    }
}
pub trait FeatureMembershipMethodsDescendants
where
    Self: DescendantOf<FeatureMembership>,
    Self::Via: FeatureMembershipMethods,
    Self: Sized,
{}
pub trait FeatureMembershipMethods: Sized {}
impl<T: FeatureMembershipMethodsDescendants> FeatureMembershipMethods for T
where
    T::Via: FeatureMembershipMethods,
{}
impl DescendantOf<OwningMembership> for FeatureMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl OwningMembershipMethodsDescendants for FeatureMembership {}
impl DescendantOf<Membership> for FeatureMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl MembershipMethodsDescendants for FeatureMembership {}
impl DescendantOf<Relationship> for FeatureMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl RelationshipMethodsDescendants for FeatureMembership {}
impl DescendantOf<Element> for FeatureMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl ElementMethodsDescendants for FeatureMembership {}
pub trait FeatureMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureMembershipRefMut<'a>>,
    Self::Via: FeatureMembershipRefMutMethods,
    Self: Sized,
{}
pub trait FeatureMembershipRefMutMethods: Sized {}
impl<'a, T: FeatureMembershipRefMutMethodsDescendants<'a>> FeatureMembershipRefMutMethods
for T
where
    T::Via: FeatureMembershipRefMutMethods,
{}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>> for FeatureMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a> for FeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for FeatureMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for FeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FeatureMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FeatureMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureMembershipRefMut<'a> {}
pub trait FeatureMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureMembershipRef<'a>>,
    Self::Via: FeatureMembershipRefMethods,
    Self: Sized,
{}
pub trait FeatureMembershipRefMethods: Sized {}
impl<'a, T: FeatureMembershipRefMethodsDescendants<'a>> FeatureMembershipRefMethods for T
where
    T::Via: FeatureMembershipRefMethods,
{}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for FeatureMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for FeatureMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for FeatureMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for FeatureMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for FeatureMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FeatureMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureMembershipRef<'a> {}

