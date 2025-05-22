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
pub struct ResultExpressionMembershipInner {
    pub(super) sup_feature_membership: FeatureMembershipInner,
}
pub trait ResultExpressionMembershipStruct
where
    Self: ResultExpressionMembershipStructRefMut,
    Self: ResultExpressionMembershipStructRef,
    Self: FeatureMembershipStruct,
{}
pub trait ResultExpressionMembershipStructRefMut
where
    Self: ResultExpressionMembershipStructRef,
    Self: FeatureMembershipStructRefMut,
{}
pub trait ResultExpressionMembershipStructRef
where
    Self: FeatureMembershipStructRef,
{}
pub trait ResultExpressionMembershipUpcast: ResultExpressionMembershipStruct {
    fn into_result_expression_membership(self) -> ResultExpressionMembership;
}
pub trait ResultExpressionMembershipUpcastRefMut<
    'a,
>: ResultExpressionMembershipStructRefMut {
    fn as_result_expression_membership_ref_mut(
        self,
    ) -> ResultExpressionMembershipRefMut<'a>;
}
pub trait ResultExpressionMembershipUpcastRef<'a>: ResultExpressionMembershipStructRef {
    fn as_result_expression_membership_ref(self) -> ResultExpressionMembershipRef<'a>;
}
impl ResultExpressionMembershipStruct for ResultExpressionMembershipInner {}
impl ResultExpressionMembershipStructRefMut for ResultExpressionMembershipInner {}
impl ResultExpressionMembershipStructRef for ResultExpressionMembershipInner {}
impl FeatureMembershipStruct for ResultExpressionMembershipInner {}
impl FeatureMembershipStructRefMut for ResultExpressionMembershipInner {}
impl FeatureMembershipStructRef for ResultExpressionMembershipInner {}
impl OwningMembershipStruct for ResultExpressionMembershipInner {}
impl OwningMembershipStructRefMut for ResultExpressionMembershipInner {}
impl OwningMembershipStructRef for ResultExpressionMembershipInner {}
impl MembershipStruct for ResultExpressionMembershipInner {
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
impl MembershipStructRefMut for ResultExpressionMembershipInner {
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
impl MembershipStructRef for ResultExpressionMembershipInner {
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
impl RelationshipStruct for ResultExpressionMembershipInner {
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
impl RelationshipStructRefMut for ResultExpressionMembershipInner {
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
impl RelationshipStructRef for ResultExpressionMembershipInner {
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
impl ElementStruct for ResultExpressionMembershipInner {
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
impl ElementStructRefMut for ResultExpressionMembershipInner {
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
impl ElementStructRef for ResultExpressionMembershipInner {
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
pub enum ResultExpressionMembership {
    Itself(ResultExpressionMembershipInner),
}
pub enum ResultExpressionMembershipRefMut<'a> {
    Itself(&'a mut ResultExpressionMembershipInner),
}
pub enum ResultExpressionMembershipRef<'a> {
    Itself(&'a ResultExpressionMembershipInner),
}
impl ResultExpressionMembership {
    pub fn as_ref(&self) -> ResultExpressionMembershipRef {
        match self {
            ResultExpressionMembership::Itself(inner) => {
                ResultExpressionMembershipRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ResultExpressionMembershipRefMut {
        match self {
            ResultExpressionMembership::Itself(inner) => {
                ResultExpressionMembershipRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> ResultExpressionMembershipRefMut<'a> {
    pub fn as_ref(self) -> ResultExpressionMembershipRef<'a> {
        match self {
            ResultExpressionMembershipRefMut::Itself(inner) => {
                ResultExpressionMembershipRef::Itself(inner)
            }
        }
    }
}
impl ResultExpressionMembershipStruct for ResultExpressionMembership {}
impl ResultExpressionMembershipStructRefMut for ResultExpressionMembership {}
impl ResultExpressionMembershipStructRef for ResultExpressionMembership {}
impl<'a> ResultExpressionMembershipStructRefMut
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> ResultExpressionMembershipStructRef for ResultExpressionMembershipRefMut<'a> {}
impl<'a> ResultExpressionMembershipStructRef for ResultExpressionMembershipRef<'a> {}
impl FeatureMembershipStruct for ResultExpressionMembership {}
impl FeatureMembershipStructRefMut for ResultExpressionMembership {}
impl FeatureMembershipStructRef for ResultExpressionMembership {}
impl<'a> FeatureMembershipStructRefMut for ResultExpressionMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ResultExpressionMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ResultExpressionMembershipRef<'a> {}
impl OwningMembershipStruct for ResultExpressionMembership {}
impl OwningMembershipStructRefMut for ResultExpressionMembership {}
impl OwningMembershipStructRef for ResultExpressionMembership {}
impl<'a> OwningMembershipStructRefMut for ResultExpressionMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ResultExpressionMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ResultExpressionMembershipRef<'a> {}
impl MembershipStruct for ResultExpressionMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for ResultExpressionMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for ResultExpressionMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for ResultExpressionMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for ResultExpressionMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for ResultExpressionMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for ResultExpressionMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for ResultExpressionMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for ResultExpressionMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ResultExpressionMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ResultExpressionMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ResultExpressionMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for ResultExpressionMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ResultExpressionMembership::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ResultExpressionMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ResultExpressionMembership::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for ResultExpressionMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembership::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ResultExpressionMembership::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembership::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ResultExpressionMembership::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ResultExpressionMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for ResultExpressionMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ResultExpressionMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ResultExpressionMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ResultExpressionMembershipRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ResultExpressionMembershipUpcast for ResultExpressionMembership {
    fn into_result_expression_membership(self) -> ResultExpressionMembership {
        self
    }
}
impl<'a> ResultExpressionMembershipUpcastRefMut<'a>
for ResultExpressionMembershipRefMut<'a> {
    fn as_result_expression_membership_ref_mut(
        self,
    ) -> ResultExpressionMembershipRefMut<'a> {
        self
    }
}
impl<'a> ResultExpressionMembershipUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_result_expression_membership_ref(self) -> ResultExpressionMembershipRef<'a> {
        self
    }
}
impl FeatureMembershipUpcast for ResultExpressionMembership {
    fn into_feature_membership(self) -> FeatureMembership {
        FeatureMembership::ResultExpressionMembership(self).into_feature_membership()
    }
}
impl<'a> FeatureMembershipUpcastRefMut<'a> for ResultExpressionMembershipRefMut<'a> {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a> {
        FeatureMembershipRefMut::ResultExpressionMembership(self)
            .as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a> {
        FeatureMembershipRef::ResultExpressionMembership(self)
            .as_feature_membership_ref()
    }
}
impl OwningMembershipUpcast for ResultExpressionMembership {
    fn into_owning_membership(self) -> OwningMembership {
        FeatureMembership::ResultExpressionMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for ResultExpressionMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        FeatureMembershipRefMut::ResultExpressionMembership(self)
            .as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        FeatureMembershipRef::ResultExpressionMembership(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for ResultExpressionMembership {
    fn into_membership(self) -> Membership {
        FeatureMembership::ResultExpressionMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for ResultExpressionMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        FeatureMembershipRefMut::ResultExpressionMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        FeatureMembershipRef::ResultExpressionMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for ResultExpressionMembership {
    fn into_relationship(self) -> Relationship {
        FeatureMembership::ResultExpressionMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ResultExpressionMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        FeatureMembershipRefMut::ResultExpressionMembership(self)
            .as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        FeatureMembershipRef::ResultExpressionMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for ResultExpressionMembership {
    fn into_element(self) -> Element {
        FeatureMembership::ResultExpressionMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ResultExpressionMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureMembershipRefMut::ResultExpressionMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ResultExpressionMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureMembershipRef::ResultExpressionMembership(self).as_element_ref()
    }
}
pub trait ResultExpressionMembershipDowncast {}
pub trait ResultExpressionMembershipDowncastRefMut<'a> {}
pub trait ResultExpressionMembershipDowncastRef<'a> {}
impl ResultExpressionMembershipDowncast for ResultExpressionMembership {}
impl<'a> ResultExpressionMembershipDowncastRefMut<'a>
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> ResultExpressionMembershipDowncastRef<'a>
for ResultExpressionMembershipRef<'a> {}
pub trait ResultExpressionMembershipMethodsDescendants
where
    Self: DescendantOf<ResultExpressionMembership>,
    Self::Via: ResultExpressionMembershipMethods,
    Self: Sized,
{}
pub trait ResultExpressionMembershipMethods: Sized {}
impl<T: ResultExpressionMembershipMethodsDescendants> ResultExpressionMembershipMethods
for T
where
    T::Via: ResultExpressionMembershipMethods,
{}
impl DescendantOf<FeatureMembership> for ResultExpressionMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl FeatureMembershipMethodsDescendants for ResultExpressionMembership {}
impl DescendantOf<OwningMembership> for ResultExpressionMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl OwningMembershipMethodsDescendants for ResultExpressionMembership {}
impl DescendantOf<Membership> for ResultExpressionMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl MembershipMethodsDescendants for ResultExpressionMembership {}
impl DescendantOf<Relationship> for ResultExpressionMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl RelationshipMethodsDescendants for ResultExpressionMembership {}
impl DescendantOf<Element> for ResultExpressionMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl ElementMethodsDescendants for ResultExpressionMembership {}
pub trait ResultExpressionMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ResultExpressionMembershipRefMut<'a>>,
    Self::Via: ResultExpressionMembershipRefMutMethods,
    Self: Sized,
{}
pub trait ResultExpressionMembershipRefMutMethods: Sized {}
impl<
    'a,
    T: ResultExpressionMembershipRefMutMethodsDescendants<'a>,
> ResultExpressionMembershipRefMutMethods for T
where
    T::Via: ResultExpressionMembershipRefMutMethods,
{}
impl<'a> DescendantOf<FeatureMembershipRefMut<'a>>
for ResultExpressionMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipRefMutMethodsDescendants<'a>
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>>
for ResultExpressionMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a>
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for ResultExpressionMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a>
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ResultExpressionMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a>
for ResultExpressionMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ResultExpressionMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ResultExpressionMembershipRefMut<'a> {}
pub trait ResultExpressionMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<ResultExpressionMembershipRef<'a>>,
    Self::Via: ResultExpressionMembershipRefMethods,
    Self: Sized,
{}
pub trait ResultExpressionMembershipRefMethods: Sized {}
impl<
    'a,
    T: ResultExpressionMembershipRefMethodsDescendants<'a>,
> ResultExpressionMembershipRefMethods for T
where
    T::Via: ResultExpressionMembershipRefMethods,
{}
impl<'a> DescendantOf<FeatureMembershipRef<'a>> for ResultExpressionMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> FeatureMembershipRefMethodsDescendants<'a>
for ResultExpressionMembershipRef<'a> {}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for ResultExpressionMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a>
for ResultExpressionMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for ResultExpressionMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for ResultExpressionMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for ResultExpressionMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ResultExpressionMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ResultExpressionMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ResultExpressionMembershipRef<'a> {}

