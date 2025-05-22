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
use super::return_parameter_membership::{
    ReturnParameterMembership, ReturnParameterMembershipRefMut,
    ReturnParameterMembershipRef,
};
pub struct ParameterMembershipInner {
    pub(super) sup_feature_membership: FeatureMembershipInner,
}
pub trait ParameterMembershipStruct
where
    Self: ParameterMembershipStructRefMut,
    Self: ParameterMembershipStructRef,
    Self: FeatureMembershipStruct,
{}
pub trait ParameterMembershipStructRefMut
where
    Self: ParameterMembershipStructRef,
    Self: FeatureMembershipStructRefMut,
{}
pub trait ParameterMembershipStructRef
where
    Self: FeatureMembershipStructRef,
{}
pub trait ParameterMembershipUpcast: ParameterMembershipStruct {
    fn into_parameter_membership(self) -> ParameterMembership;
}
pub trait ParameterMembershipUpcastRefMut<'a>: ParameterMembershipStructRefMut {
    fn as_parameter_membership_ref_mut(self) -> ParameterMembershipRefMut<'a>;
}
pub trait ParameterMembershipUpcastRef<'a>: ParameterMembershipStructRef {
    fn as_parameter_membership_ref(self) -> ParameterMembershipRef<'a>;
}
impl ParameterMembershipStruct for ParameterMembershipInner {}
impl ParameterMembershipStructRefMut for ParameterMembershipInner {}
impl ParameterMembershipStructRef for ParameterMembershipInner {}
impl FeatureMembershipStruct for ParameterMembershipInner {}
impl FeatureMembershipStructRefMut for ParameterMembershipInner {}
impl FeatureMembershipStructRef for ParameterMembershipInner {}
impl OwningMembershipStruct for ParameterMembershipInner {}
impl OwningMembershipStructRefMut for ParameterMembershipInner {}
impl OwningMembershipStructRef for ParameterMembershipInner {}
impl MembershipStruct for ParameterMembershipInner {
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
impl MembershipStructRefMut for ParameterMembershipInner {
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
impl MembershipStructRef for ParameterMembershipInner {
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
impl RelationshipStruct for ParameterMembershipInner {
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
impl RelationshipStructRefMut for ParameterMembershipInner {
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
impl RelationshipStructRef for ParameterMembershipInner {
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
impl ElementStruct for ParameterMembershipInner {
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
impl ElementStructRefMut for ParameterMembershipInner {
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
impl ElementStructRef for ParameterMembershipInner {
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
pub enum ParameterMembership {
    Itself(ParameterMembershipInner),
    ReturnParameterMembership(ReturnParameterMembership),
}
pub enum ParameterMembershipRefMut<'a> {
    Itself(&'a mut ParameterMembershipInner),
    ReturnParameterMembership(ReturnParameterMembershipRefMut<'a>),
}
pub enum ParameterMembershipRef<'a> {
    Itself(&'a ParameterMembershipInner),
    ReturnParameterMembership(ReturnParameterMembershipRef<'a>),
}
impl ParameterMembership {
    pub fn as_ref(&self) -> ParameterMembershipRef {
        match self {
            ParameterMembership::Itself(inner) => ParameterMembershipRef::Itself(&inner),
            ParameterMembership::ReturnParameterMembership(inner) => {
                ParameterMembershipRef::ReturnParameterMembership(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ParameterMembershipRefMut {
        match self {
            ParameterMembership::Itself(inner) => {
                ParameterMembershipRefMut::Itself(inner)
            }
            ParameterMembership::ReturnParameterMembership(inner) => {
                ParameterMembershipRefMut::ReturnParameterMembership(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ParameterMembershipRefMut<'a> {
    pub fn as_ref(self) -> ParameterMembershipRef<'a> {
        match self {
            ParameterMembershipRefMut::Itself(inner) => {
                ParameterMembershipRef::Itself(inner)
            }
            ParameterMembershipRefMut::ReturnParameterMembership(inner) => {
                ParameterMembershipRef::ReturnParameterMembership(inner.as_ref())
            }
        }
    }
}
impl ParameterMembershipStruct for ParameterMembership {}
impl ParameterMembershipStructRefMut for ParameterMembership {}
impl ParameterMembershipStructRef for ParameterMembership {}
impl<'a> ParameterMembershipStructRefMut for ParameterMembershipRefMut<'a> {}
impl<'a> ParameterMembershipStructRef for ParameterMembershipRefMut<'a> {}
impl<'a> ParameterMembershipStructRef for ParameterMembershipRef<'a> {}
impl FeatureMembershipStruct for ParameterMembership {}
impl FeatureMembershipStructRefMut for ParameterMembership {}
impl FeatureMembershipStructRef for ParameterMembership {}
impl<'a> FeatureMembershipStructRefMut for ParameterMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ParameterMembershipRefMut<'a> {}
impl<'a> FeatureMembershipStructRef for ParameterMembershipRef<'a> {}
impl OwningMembershipStruct for ParameterMembership {}
impl OwningMembershipStructRefMut for ParameterMembership {}
impl OwningMembershipStructRef for ParameterMembership {}
impl<'a> OwningMembershipStructRefMut for ParameterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ParameterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ParameterMembershipRef<'a> {}
impl MembershipStruct for ParameterMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_short_name(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembership::Itself(x) => x.member_element(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_name(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembership::Itself(x) => x.visibility(),
            ParameterMembership::ReturnParameterMembership(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for ParameterMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_short_name_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.member_short_name_ref_mut()
            }
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembership::Itself(x) => x.member_element_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.member_element_ref_mut()
            }
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_name_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembership::Itself(x) => x.visibility_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for ParameterMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_short_name_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.member_short_name_ref()
            }
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembership::Itself(x) => x.member_element_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.member_name_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembership::Itself(x) => x.visibility_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for ParameterMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_short_name_ref_mut()
            }
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_element_ref_mut()
            }
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_name_ref_mut()
            }
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.visibility_ref_mut()
            }
        }
    }
}
impl<'a> MembershipStructRef for ParameterMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_short_name_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_short_name_ref()
            }
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_element_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_element_ref()
            }
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.member_name_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.member_name_ref()
            }
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.visibility_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for ParameterMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRef::Itself(x) => x.member_short_name_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.member_short_name_ref()
            }
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.member_element_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.member_element_ref()
            }
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRef::Itself(x) => x.member_name_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.visibility_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for ParameterMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.target(),
            ParameterMembership::ReturnParameterMembership(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.source(),
            ParameterMembership::ReturnParameterMembership(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owning_related_element(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owning_related_element()
            }
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_related_element(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owned_related_element()
            }
        }
    }
    fn is_implied(self) -> bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied(),
            ParameterMembership::ReturnParameterMembership(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for ParameterMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.target_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.source_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owning_related_element_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_related_element_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for ParameterMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.target_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.source_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owning_related_element_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_related_element_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ParameterMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.target_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.source_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.is_implied_ref_mut()
            }
        }
    }
}
impl<'a> RelationshipStructRef for ParameterMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.target_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.source_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.is_implied_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ParameterMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.target_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.source_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.owning_related_element_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.owned_related_element_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ParameterMembershipRef::Itself(x) => x.is_implied_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for ParameterMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_relationship(),
            ParameterMembership::ReturnParameterMembership(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembership::Itself(x) => x.owning_relationship(),
            ParameterMembership::ReturnParameterMembership(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ParameterMembership::Itself(x) => x.element_id(),
            ParameterMembership::ReturnParameterMembership(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ParameterMembership::Itself(x) => x.alias_ids(),
            ParameterMembership::ReturnParameterMembership(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_short_name(),
            ParameterMembership::ReturnParameterMembership(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_name(),
            ParameterMembership::ReturnParameterMembership(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied_included(),
            ParameterMembership::ReturnParameterMembership(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ParameterMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_relationship_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
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
            ParameterMembership::Itself(x) => x.owning_relationship_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ParameterMembership::Itself(x) => x.element_id_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ParameterMembership::Itself(x) => x.alias_ids_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_short_name_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_name_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.declared_name_ref_mut()
            }
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied_included_ref_mut(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl ElementStructRef for ParameterMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembership::Itself(x) => x.owned_relationship_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembership::Itself(x) => x.owning_relationship_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ParameterMembership::Itself(x) => x.element_id_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ParameterMembership::Itself(x) => x.alias_ids_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_short_name_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembership::Itself(x) => x.declared_name_ref(),
            ParameterMembership::ReturnParameterMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ParameterMembership::Itself(x) => x.is_implied_included_ref(),
            ParameterMembership::ReturnParameterMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRefMut for ParameterMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
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
            ParameterMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.element_id_ref_mut()
            }
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.alias_ids_ref_mut()
            }
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.declared_name_ref_mut()
            }
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for ParameterMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.element_id_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.alias_ids_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.declared_name_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.declared_name_ref()
            }
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ParameterMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
            ParameterMembershipRefMut::ReturnParameterMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRef for ParameterMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.owned_relationship_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ParameterMembershipRef::Itself(x) => x.owning_relationship_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ParameterMembershipRef::Itself(x) => x.element_id_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ParameterMembershipRef::Itself(x) => x.alias_ids_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRef::Itself(x) => x.declared_short_name_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ParameterMembershipRef::Itself(x) => x.declared_name_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ParameterMembershipRef::Itself(x) => x.is_implied_included_ref(),
            ParameterMembershipRef::ReturnParameterMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl ParameterMembershipUpcast for ParameterMembership {
    fn into_parameter_membership(self) -> ParameterMembership {
        self
    }
}
impl<'a> ParameterMembershipUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_parameter_membership_ref_mut(self) -> ParameterMembershipRefMut<'a> {
        self
    }
}
impl<'a> ParameterMembershipUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_parameter_membership_ref(self) -> ParameterMembershipRef<'a> {
        self
    }
}
impl FeatureMembershipUpcast for ParameterMembership {
    fn into_feature_membership(self) -> FeatureMembership {
        FeatureMembership::ParameterMembership(self).into_feature_membership()
    }
}
impl<'a> FeatureMembershipUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_feature_membership_ref_mut(self) -> FeatureMembershipRefMut<'a> {
        FeatureMembershipRefMut::ParameterMembership(self)
            .as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_feature_membership_ref(self) -> FeatureMembershipRef<'a> {
        FeatureMembershipRef::ParameterMembership(self).as_feature_membership_ref()
    }
}
impl OwningMembershipUpcast for ParameterMembership {
    fn into_owning_membership(self) -> OwningMembership {
        FeatureMembership::ParameterMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        FeatureMembershipRefMut::ParameterMembership(self).as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        FeatureMembershipRef::ParameterMembership(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for ParameterMembership {
    fn into_membership(self) -> Membership {
        FeatureMembership::ParameterMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        FeatureMembershipRefMut::ParameterMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        FeatureMembershipRef::ParameterMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for ParameterMembership {
    fn into_relationship(self) -> Relationship {
        FeatureMembership::ParameterMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        FeatureMembershipRefMut::ParameterMembership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        FeatureMembershipRef::ParameterMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for ParameterMembership {
    fn into_element(self) -> Element {
        FeatureMembership::ParameterMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureMembershipRefMut::ParameterMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ParameterMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureMembershipRef::ParameterMembership(self).as_element_ref()
    }
}
pub trait ParameterMembershipDowncast {
    fn try_into_return_parameter_membership(
        self,
    ) -> Result<ReturnParameterMembership, String>;
}
pub trait ParameterMembershipDowncastRefMut<'a> {
    fn try_as_return_parameter_membership_ref_mut(
        self,
    ) -> Result<ReturnParameterMembershipRefMut<'a>, String>;
}
pub trait ParameterMembershipDowncastRef<'a> {
    fn try_as_return_parameter_membership_ref(
        self,
    ) -> Result<ReturnParameterMembershipRef<'a>, String>;
}
impl ParameterMembershipDowncast for ParameterMembership {
    fn try_into_return_parameter_membership(
        self,
    ) -> Result<ReturnParameterMembership, String> {
        match self {
            ParameterMembership::ReturnParameterMembership(e) => Ok(e),
            _ => Err("Not a ReturnParameterMembership".into()),
        }
    }
}
impl<'a> ParameterMembershipDowncastRefMut<'a> for ParameterMembershipRefMut<'a> {
    fn try_as_return_parameter_membership_ref_mut(
        self,
    ) -> Result<ReturnParameterMembershipRefMut<'a>, String> {
        match self {
            ParameterMembershipRefMut::ReturnParameterMembership(e) => Ok(e),
            _ => Err("Not a ReturnParameterMembership".into()),
        }
    }
}
impl<'a> ParameterMembershipDowncastRef<'a> for ParameterMembershipRef<'a> {
    fn try_as_return_parameter_membership_ref(
        self,
    ) -> Result<ReturnParameterMembershipRef<'a>, String> {
        match self {
            ParameterMembershipRef::ReturnParameterMembership(e) => Ok(e),
            _ => Err("Not a ReturnParameterMembership".into()),
        }
    }
}
pub trait ParameterMembershipMethodsDescendants
where
    Self: DescendantOf<ParameterMembership>,
    Self::Via: ParameterMembershipMethods,
    Self: Sized,
{}
pub trait ParameterMembershipMethods: Sized {}
impl<T: ParameterMembershipMethodsDescendants> ParameterMembershipMethods for T
where
    T::Via: ParameterMembershipMethods,
{}
impl DescendantOf<FeatureMembership> for ParameterMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl FeatureMembershipMethodsDescendants for ParameterMembership {}
impl DescendantOf<OwningMembership> for ParameterMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl OwningMembershipMethodsDescendants for ParameterMembership {}
impl DescendantOf<Membership> for ParameterMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl MembershipMethodsDescendants for ParameterMembership {}
impl DescendantOf<Relationship> for ParameterMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl RelationshipMethodsDescendants for ParameterMembership {}
impl DescendantOf<Element> for ParameterMembership {
    type Via = FeatureMembership;
    fn into_via(self) -> Self::Via {
        self.into_feature_membership()
    }
}
impl ElementMethodsDescendants for ParameterMembership {}
pub trait ParameterMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ParameterMembershipRefMut<'a>>,
    Self::Via: ParameterMembershipRefMutMethods,
    Self: Sized,
{}
pub trait ParameterMembershipRefMutMethods: Sized {}
impl<
    'a,
    T: ParameterMembershipRefMutMethodsDescendants<'a>,
> ParameterMembershipRefMutMethods for T
where
    T::Via: ParameterMembershipRefMutMethods,
{}
impl<'a> DescendantOf<FeatureMembershipRefMut<'a>> for ParameterMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> FeatureMembershipRefMutMethodsDescendants<'a>
for ParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>> for ParameterMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a> for ParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for ParameterMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for ParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ParameterMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ParameterMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ParameterMembershipRefMut<'a> {
    type Via = FeatureMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ParameterMembershipRefMut<'a> {}
pub trait ParameterMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<ParameterMembershipRef<'a>>,
    Self::Via: ParameterMembershipRefMethods,
    Self: Sized,
{
    fn parameter_direction_impl(
        self,
    ) -> std::rc::Rc<
        std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
    > {
        self.into_via().parameter_direction()
    }
}
pub trait ParameterMembershipRefMethods: Sized {
    fn parameter_direction(
        self,
    ) -> std::rc::Rc<
        std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
    >;
}
impl<'a, T: ParameterMembershipRefMethodsDescendants<'a>> ParameterMembershipRefMethods
for T
where
    T::Via: ParameterMembershipRefMethods,
{
    fn parameter_direction(
        self,
    ) -> std::rc::Rc<
        std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
    > {
        ParameterMembershipRefMethodsDescendants::parameter_direction_impl(self)
    }
}
impl<'a> DescendantOf<FeatureMembershipRef<'a>> for ParameterMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> FeatureMembershipRefMethodsDescendants<'a> for ParameterMembershipRef<'a> {}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for ParameterMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for ParameterMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for ParameterMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for ParameterMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for ParameterMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ParameterMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ParameterMembershipRef<'a> {
    type Via = FeatureMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ParameterMembershipRef<'a> {}

