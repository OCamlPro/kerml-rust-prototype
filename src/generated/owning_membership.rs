#![allow(unused)]
use super::utils::DescendantOf;
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
use super::feature_value::{FeatureValue, FeatureValueRefMut, FeatureValueRef};
use super::feature_membership::{
    FeatureMembership, FeatureMembershipRefMut, FeatureMembershipRef,
};
use super::element_filter_membership::{
    ElementFilterMembership, ElementFilterMembershipRefMut, ElementFilterMembershipRef,
};
pub struct OwningMembershipInner {
    pub(super) sup_membership: MembershipInner,
}
pub trait OwningMembershipStruct
where
    Self: OwningMembershipStructRefMut,
    Self: OwningMembershipStructRef,
    Self: MembershipStruct,
{}
pub trait OwningMembershipStructRefMut
where
    Self: OwningMembershipStructRef,
    Self: MembershipStructRefMut,
{}
pub trait OwningMembershipStructRef
where
    Self: MembershipStructRef,
{}
pub trait OwningMembershipUpcast: OwningMembershipStruct {
    fn into_owning_membership(self) -> OwningMembership;
}
pub trait OwningMembershipUpcastRefMut<'a>: OwningMembershipStructRefMut {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a>;
}
pub trait OwningMembershipUpcastRef<'a>: OwningMembershipStructRef {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a>;
}
impl OwningMembershipStruct for OwningMembershipInner {}
impl OwningMembershipStructRefMut for OwningMembershipInner {}
impl OwningMembershipStructRef for OwningMembershipInner {}
impl MembershipStruct for OwningMembershipInner {
    fn member_short_name(self) -> Option<String> {
        self.sup_membership.member_short_name()
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_membership.member_element()
    }
    fn member_name(self) -> Option<String> {
        self.sup_membership.member_name()
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_membership.visibility()
    }
}
impl MembershipStructRefMut for OwningMembershipInner {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_membership.member_short_name_ref_mut()
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_membership.member_element_ref_mut()
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_membership.member_name_ref_mut()
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_membership.visibility_ref_mut()
    }
}
impl MembershipStructRef for OwningMembershipInner {
    fn member_short_name_ref(&self) -> &Option<String> {
        self.sup_membership.member_short_name_ref()
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        self.sup_membership.member_element_ref()
    }
    fn member_name_ref(&self) -> &Option<String> {
        self.sup_membership.member_name_ref()
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_membership.visibility_ref()
    }
}
impl RelationshipStruct for OwningMembershipInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_membership.is_implied()
    }
}
impl RelationshipStructRefMut for OwningMembershipInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_membership.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for OwningMembershipInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_membership.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_membership.is_implied_ref()
    }
}
impl ElementStruct for OwningMembershipInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_membership.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_membership.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_membership.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_membership.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_membership.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_membership.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_membership.is_implied_included()
    }
}
impl ElementStructRefMut for OwningMembershipInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_membership.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_membership.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_membership.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_membership.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_membership.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_membership.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_membership.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for OwningMembershipInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_membership.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_membership.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_membership.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_membership.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_membership.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_membership.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_membership.is_implied_included_ref()
    }
}
pub enum OwningMembership {
    Itself(OwningMembershipInner),
    FeatureValue(FeatureValue),
    FeatureMembership(FeatureMembership),
    ElementFilterMembership(ElementFilterMembership),
}
pub enum OwningMembershipRefMut<'a> {
    Itself(&'a mut OwningMembershipInner),
    FeatureValue(FeatureValueRefMut<'a>),
    FeatureMembership(FeatureMembershipRefMut<'a>),
    ElementFilterMembership(ElementFilterMembershipRefMut<'a>),
}
pub enum OwningMembershipRef<'a> {
    Itself(&'a OwningMembershipInner),
    FeatureValue(FeatureValueRef<'a>),
    FeatureMembership(FeatureMembershipRef<'a>),
    ElementFilterMembership(ElementFilterMembershipRef<'a>),
}
impl OwningMembership {
    pub fn as_ref(&self) -> OwningMembershipRef {
        match self {
            OwningMembership::Itself(inner) => OwningMembershipRef::Itself(&inner),
            OwningMembership::FeatureValue(inner) => {
                OwningMembershipRef::FeatureValue(inner.as_ref())
            }
            OwningMembership::FeatureMembership(inner) => {
                OwningMembershipRef::FeatureMembership(inner.as_ref())
            }
            OwningMembership::ElementFilterMembership(inner) => {
                OwningMembershipRef::ElementFilterMembership(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> OwningMembershipRefMut {
        match self {
            OwningMembership::Itself(inner) => OwningMembershipRefMut::Itself(inner),
            OwningMembership::FeatureValue(inner) => {
                OwningMembershipRefMut::FeatureValue(inner.as_ref_mut())
            }
            OwningMembership::FeatureMembership(inner) => {
                OwningMembershipRefMut::FeatureMembership(inner.as_ref_mut())
            }
            OwningMembership::ElementFilterMembership(inner) => {
                OwningMembershipRefMut::ElementFilterMembership(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> OwningMembershipRefMut<'a> {
    pub fn as_ref(self) -> OwningMembershipRef<'a> {
        match self {
            OwningMembershipRefMut::Itself(inner) => OwningMembershipRef::Itself(inner),
            OwningMembershipRefMut::FeatureValue(inner) => {
                OwningMembershipRef::FeatureValue(inner.as_ref())
            }
            OwningMembershipRefMut::FeatureMembership(inner) => {
                OwningMembershipRef::FeatureMembership(inner.as_ref())
            }
            OwningMembershipRefMut::ElementFilterMembership(inner) => {
                OwningMembershipRef::ElementFilterMembership(inner.as_ref())
            }
        }
    }
}
impl OwningMembershipStruct for OwningMembership {}
impl OwningMembershipStructRefMut for OwningMembership {}
impl OwningMembershipStructRef for OwningMembership {}
impl<'a> OwningMembershipStructRefMut for OwningMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for OwningMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for OwningMembershipRef<'a> {}
impl MembershipStruct for OwningMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_short_name(),
            OwningMembership::FeatureValue(x) => x.member_short_name(),
            OwningMembership::FeatureMembership(x) => x.member_short_name(),
            OwningMembership::ElementFilterMembership(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembership::Itself(x) => x.member_element(),
            OwningMembership::FeatureValue(x) => x.member_element(),
            OwningMembership::FeatureMembership(x) => x.member_element(),
            OwningMembership::ElementFilterMembership(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_name(),
            OwningMembership::FeatureValue(x) => x.member_name(),
            OwningMembership::FeatureMembership(x) => x.member_name(),
            OwningMembership::ElementFilterMembership(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembership::Itself(x) => x.visibility(),
            OwningMembership::FeatureValue(x) => x.visibility(),
            OwningMembership::FeatureMembership(x) => x.visibility(),
            OwningMembership::ElementFilterMembership(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for OwningMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_short_name_ref_mut(),
            OwningMembership::FeatureValue(x) => x.member_short_name_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.member_short_name_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembership::Itself(x) => x.member_element_ref_mut(),
            OwningMembership::FeatureValue(x) => x.member_element_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.member_element_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_name_ref_mut(),
            OwningMembership::FeatureValue(x) => x.member_name_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.member_name_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembership::Itself(x) => x.visibility_ref_mut(),
            OwningMembership::FeatureValue(x) => x.visibility_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.visibility_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for OwningMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_short_name_ref(),
            OwningMembership::FeatureValue(x) => x.member_short_name_ref(),
            OwningMembership::FeatureMembership(x) => x.member_short_name_ref(),
            OwningMembership::ElementFilterMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembership::Itself(x) => x.member_element_ref(),
            OwningMembership::FeatureValue(x) => x.member_element_ref(),
            OwningMembership::FeatureMembership(x) => x.member_element_ref(),
            OwningMembership::ElementFilterMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembership::Itself(x) => x.member_name_ref(),
            OwningMembership::FeatureValue(x) => x.member_name_ref(),
            OwningMembership::FeatureMembership(x) => x.member_name_ref(),
            OwningMembership::ElementFilterMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembership::Itself(x) => x.visibility_ref(),
            OwningMembership::FeatureValue(x) => x.visibility_ref(),
            OwningMembership::FeatureMembership(x) => x.visibility_ref(),
            OwningMembership::ElementFilterMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for OwningMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_short_name_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_short_name_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.member_short_name_ref_mut()
            }
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_element_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_element_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.member_element_ref_mut()
            }
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_name_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_name_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.visibility_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.visibility_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for OwningMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_short_name_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_short_name_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_short_name_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.member_short_name_ref()
            }
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_element_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_element_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_element_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.member_name_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.member_name_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.member_name_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.visibility_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.visibility_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.visibility_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for OwningMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRef::Itself(x) => x.member_short_name_ref(),
            OwningMembershipRef::FeatureValue(x) => x.member_short_name_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.member_short_name_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            OwningMembershipRef::Itself(x) => x.member_element_ref(),
            OwningMembershipRef::FeatureValue(x) => x.member_element_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.member_element_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRef::Itself(x) => x.member_name_ref(),
            OwningMembershipRef::FeatureValue(x) => x.member_name_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.member_name_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            OwningMembershipRef::Itself(x) => x.visibility_ref(),
            OwningMembershipRef::FeatureValue(x) => x.visibility_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.visibility_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for OwningMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.target(),
            OwningMembership::FeatureValue(x) => x.target(),
            OwningMembership::FeatureMembership(x) => x.target(),
            OwningMembership::ElementFilterMembership(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.source(),
            OwningMembership::FeatureValue(x) => x.source(),
            OwningMembership::FeatureMembership(x) => x.source(),
            OwningMembership::ElementFilterMembership(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owning_related_element(),
            OwningMembership::FeatureValue(x) => x.owning_related_element(),
            OwningMembership::FeatureMembership(x) => x.owning_related_element(),
            OwningMembership::ElementFilterMembership(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_related_element(),
            OwningMembership::FeatureValue(x) => x.owned_related_element(),
            OwningMembership::FeatureMembership(x) => x.owned_related_element(),
            OwningMembership::ElementFilterMembership(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied(),
            OwningMembership::FeatureValue(x) => x.is_implied(),
            OwningMembership::FeatureMembership(x) => x.is_implied(),
            OwningMembership::ElementFilterMembership(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for OwningMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.target_ref_mut(),
            OwningMembership::FeatureValue(x) => x.target_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.target_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.source_ref_mut(),
            OwningMembership::FeatureValue(x) => x.source_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.source_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owning_related_element_ref_mut(),
            OwningMembership::FeatureValue(x) => x.owning_related_element_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.owning_related_element_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_related_element_ref_mut(),
            OwningMembership::FeatureValue(x) => x.owned_related_element_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.owned_related_element_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied_ref_mut(),
            OwningMembership::FeatureValue(x) => x.is_implied_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.is_implied_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for OwningMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.target_ref(),
            OwningMembership::FeatureValue(x) => x.target_ref(),
            OwningMembership::FeatureMembership(x) => x.target_ref(),
            OwningMembership::ElementFilterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.source_ref(),
            OwningMembership::FeatureValue(x) => x.source_ref(),
            OwningMembership::FeatureMembership(x) => x.source_ref(),
            OwningMembership::ElementFilterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owning_related_element_ref(),
            OwningMembership::FeatureValue(x) => x.owning_related_element_ref(),
            OwningMembership::FeatureMembership(x) => x.owning_related_element_ref(),
            OwningMembership::ElementFilterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_related_element_ref(),
            OwningMembership::FeatureValue(x) => x.owned_related_element_ref(),
            OwningMembership::FeatureMembership(x) => x.owned_related_element_ref(),
            OwningMembership::ElementFilterMembership(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied_ref(),
            OwningMembership::FeatureValue(x) => x.is_implied_ref(),
            OwningMembership::FeatureMembership(x) => x.is_implied_ref(),
            OwningMembership::ElementFilterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for OwningMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.target_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.target_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.target_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.source_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.source_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.source_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.owning_related_element_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.owning_related_element_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.owned_related_element_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.owned_related_element_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            OwningMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.is_implied_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.is_implied_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for OwningMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.target_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.target_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.target_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.source_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.source_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.source_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.owning_related_element_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.owning_related_element_ref()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.owned_related_element_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.owned_related_element_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            OwningMembershipRefMut::Itself(x) => x.is_implied_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.is_implied_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.is_implied_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for OwningMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.target_ref(),
            OwningMembershipRef::FeatureValue(x) => x.target_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.target_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.source_ref(),
            OwningMembershipRef::FeatureValue(x) => x.source_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.source_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.owning_related_element_ref(),
            OwningMembershipRef::FeatureValue(x) => x.owning_related_element_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.owning_related_element_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => {
                x.owning_related_element_ref()
            }
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.owned_related_element_ref(),
            OwningMembershipRef::FeatureValue(x) => x.owned_related_element_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.owned_related_element_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => {
                x.owned_related_element_ref()
            }
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            OwningMembershipRef::Itself(x) => x.is_implied_ref(),
            OwningMembershipRef::FeatureValue(x) => x.is_implied_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.is_implied_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for OwningMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_relationship(),
            OwningMembership::FeatureValue(x) => x.owned_relationship(),
            OwningMembership::FeatureMembership(x) => x.owned_relationship(),
            OwningMembership::ElementFilterMembership(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembership::Itself(x) => x.owning_relationship(),
            OwningMembership::FeatureValue(x) => x.owning_relationship(),
            OwningMembership::FeatureMembership(x) => x.owning_relationship(),
            OwningMembership::ElementFilterMembership(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            OwningMembership::Itself(x) => x.element_id(),
            OwningMembership::FeatureValue(x) => x.element_id(),
            OwningMembership::FeatureMembership(x) => x.element_id(),
            OwningMembership::ElementFilterMembership(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            OwningMembership::Itself(x) => x.alias_ids(),
            OwningMembership::FeatureValue(x) => x.alias_ids(),
            OwningMembership::FeatureMembership(x) => x.alias_ids(),
            OwningMembership::ElementFilterMembership(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_short_name(),
            OwningMembership::FeatureValue(x) => x.declared_short_name(),
            OwningMembership::FeatureMembership(x) => x.declared_short_name(),
            OwningMembership::ElementFilterMembership(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_name(),
            OwningMembership::FeatureValue(x) => x.declared_name(),
            OwningMembership::FeatureMembership(x) => x.declared_name(),
            OwningMembership::ElementFilterMembership(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied_included(),
            OwningMembership::FeatureValue(x) => x.is_implied_included(),
            OwningMembership::FeatureMembership(x) => x.is_implied_included(),
            OwningMembership::ElementFilterMembership(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for OwningMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_relationship_ref_mut(),
            OwningMembership::FeatureValue(x) => x.owned_relationship_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.owned_relationship_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
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
            OwningMembership::Itself(x) => x.owning_relationship_ref_mut(),
            OwningMembership::FeatureValue(x) => x.owning_relationship_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.owning_relationship_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            OwningMembership::Itself(x) => x.element_id_ref_mut(),
            OwningMembership::FeatureValue(x) => x.element_id_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.element_id_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            OwningMembership::Itself(x) => x.alias_ids_ref_mut(),
            OwningMembership::FeatureValue(x) => x.alias_ids_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.alias_ids_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_short_name_ref_mut(),
            OwningMembership::FeatureValue(x) => x.declared_short_name_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.declared_short_name_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_name_ref_mut(),
            OwningMembership::FeatureValue(x) => x.declared_name_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.declared_name_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied_included_ref_mut(),
            OwningMembership::FeatureValue(x) => x.is_implied_included_ref_mut(),
            OwningMembership::FeatureMembership(x) => x.is_implied_included_ref_mut(),
            OwningMembership::ElementFilterMembership(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl ElementStructRef for OwningMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembership::Itself(x) => x.owned_relationship_ref(),
            OwningMembership::FeatureValue(x) => x.owned_relationship_ref(),
            OwningMembership::FeatureMembership(x) => x.owned_relationship_ref(),
            OwningMembership::ElementFilterMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembership::Itself(x) => x.owning_relationship_ref(),
            OwningMembership::FeatureValue(x) => x.owning_relationship_ref(),
            OwningMembership::FeatureMembership(x) => x.owning_relationship_ref(),
            OwningMembership::ElementFilterMembership(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OwningMembership::Itself(x) => x.element_id_ref(),
            OwningMembership::FeatureValue(x) => x.element_id_ref(),
            OwningMembership::FeatureMembership(x) => x.element_id_ref(),
            OwningMembership::ElementFilterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OwningMembership::Itself(x) => x.alias_ids_ref(),
            OwningMembership::FeatureValue(x) => x.alias_ids_ref(),
            OwningMembership::FeatureMembership(x) => x.alias_ids_ref(),
            OwningMembership::ElementFilterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_short_name_ref(),
            OwningMembership::FeatureValue(x) => x.declared_short_name_ref(),
            OwningMembership::FeatureMembership(x) => x.declared_short_name_ref(),
            OwningMembership::ElementFilterMembership(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembership::Itself(x) => x.declared_name_ref(),
            OwningMembership::FeatureValue(x) => x.declared_name_ref(),
            OwningMembership::FeatureMembership(x) => x.declared_name_ref(),
            OwningMembership::ElementFilterMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OwningMembership::Itself(x) => x.is_implied_included_ref(),
            OwningMembership::FeatureValue(x) => x.is_implied_included_ref(),
            OwningMembership::FeatureMembership(x) => x.is_implied_included_ref(),
            OwningMembership::ElementFilterMembership(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for OwningMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.owned_relationship_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.owned_relationship_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
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
            OwningMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.owning_relationship_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.owning_relationship_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            OwningMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.element_id_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.element_id_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.alias_ids_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.alias_ids_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.declared_short_name_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.declared_short_name_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.declared_name_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => x.declared_name_ref_mut(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.declared_name_ref_mut()
            }
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            OwningMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            OwningMembershipRefMut::FeatureValue(x) => x.is_implied_included_ref_mut(),
            OwningMembershipRefMut::FeatureMembership(x) => {
                x.is_implied_included_ref_mut()
            }
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for OwningMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.owned_relationship_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.owned_relationship_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.owning_relationship_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.owning_relationship_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OwningMembershipRefMut::Itself(x) => x.element_id_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.element_id_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.element_id_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.alias_ids_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.alias_ids_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.alias_ids_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.declared_short_name_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.declared_short_name_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRefMut::Itself(x) => x.declared_name_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.declared_name_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.declared_name_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OwningMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
            OwningMembershipRefMut::FeatureValue(x) => x.is_implied_included_ref(),
            OwningMembershipRefMut::FeatureMembership(x) => x.is_implied_included_ref(),
            OwningMembershipRefMut::ElementFilterMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRef for OwningMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.owned_relationship_ref(),
            OwningMembershipRef::FeatureValue(x) => x.owned_relationship_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.owned_relationship_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OwningMembershipRef::Itself(x) => x.owning_relationship_ref(),
            OwningMembershipRef::FeatureValue(x) => x.owning_relationship_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.owning_relationship_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OwningMembershipRef::Itself(x) => x.element_id_ref(),
            OwningMembershipRef::FeatureValue(x) => x.element_id_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.element_id_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OwningMembershipRef::Itself(x) => x.alias_ids_ref(),
            OwningMembershipRef::FeatureValue(x) => x.alias_ids_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.alias_ids_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRef::Itself(x) => x.declared_short_name_ref(),
            OwningMembershipRef::FeatureValue(x) => x.declared_short_name_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.declared_short_name_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OwningMembershipRef::Itself(x) => x.declared_name_ref(),
            OwningMembershipRef::FeatureValue(x) => x.declared_name_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.declared_name_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OwningMembershipRef::Itself(x) => x.is_implied_included_ref(),
            OwningMembershipRef::FeatureValue(x) => x.is_implied_included_ref(),
            OwningMembershipRef::FeatureMembership(x) => x.is_implied_included_ref(),
            OwningMembershipRef::ElementFilterMembership(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl OwningMembershipUpcast for OwningMembership {
    fn into_owning_membership(self) -> OwningMembership {
        self
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for OwningMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        self
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for OwningMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        self
    }
}
impl MembershipUpcast for OwningMembership {
    fn into_membership(self) -> Membership {
        Membership::OwningMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for OwningMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        MembershipRefMut::OwningMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for OwningMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        MembershipRef::OwningMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for OwningMembership {
    fn into_relationship(self) -> Relationship {
        Membership::OwningMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for OwningMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        MembershipRefMut::OwningMembership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for OwningMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        MembershipRef::OwningMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for OwningMembership {
    fn into_element(self) -> Element {
        Membership::OwningMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for OwningMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        MembershipRefMut::OwningMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for OwningMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        MembershipRef::OwningMembership(self).as_element_ref()
    }
}
pub trait OwningMembershipDowncast {
    fn try_into_feature_value(self) -> Result<FeatureValue, String>;
    fn try_into_feature_membership(self) -> Result<FeatureMembership, String>;
    fn try_into_element_filter_membership(
        self,
    ) -> Result<ElementFilterMembership, String>;
}
pub trait OwningMembershipDowncastRefMut<'a> {
    fn try_as_feature_value_ref_mut(self) -> Result<FeatureValueRefMut<'a>, String>;
    fn try_as_feature_membership_ref_mut(
        self,
    ) -> Result<FeatureMembershipRefMut<'a>, String>;
    fn try_as_element_filter_membership_ref_mut(
        self,
    ) -> Result<ElementFilterMembershipRefMut<'a>, String>;
}
pub trait OwningMembershipDowncastRef<'a> {
    fn try_as_feature_value_ref(self) -> Result<FeatureValueRef<'a>, String>;
    fn try_as_feature_membership_ref(self) -> Result<FeatureMembershipRef<'a>, String>;
    fn try_as_element_filter_membership_ref(
        self,
    ) -> Result<ElementFilterMembershipRef<'a>, String>;
}
impl OwningMembershipDowncast for OwningMembership {
    fn try_into_feature_value(self) -> Result<FeatureValue, String> {
        match self {
            OwningMembership::FeatureValue(e) => Ok(e),
            _ => Err("Not a FeatureValue".into()),
        }
    }
    fn try_into_feature_membership(self) -> Result<FeatureMembership, String> {
        match self {
            OwningMembership::FeatureMembership(e) => Ok(e),
            _ => Err("Not a FeatureMembership".into()),
        }
    }
    fn try_into_element_filter_membership(
        self,
    ) -> Result<ElementFilterMembership, String> {
        match self {
            OwningMembership::ElementFilterMembership(e) => Ok(e),
            _ => Err("Not a ElementFilterMembership".into()),
        }
    }
}
impl<'a> OwningMembershipDowncastRefMut<'a> for OwningMembershipRefMut<'a> {
    fn try_as_feature_value_ref_mut(self) -> Result<FeatureValueRefMut<'a>, String> {
        match self {
            OwningMembershipRefMut::FeatureValue(e) => Ok(e),
            _ => Err("Not a FeatureValue".into()),
        }
    }
    fn try_as_feature_membership_ref_mut(
        self,
    ) -> Result<FeatureMembershipRefMut<'a>, String> {
        match self {
            OwningMembershipRefMut::FeatureMembership(e) => Ok(e),
            _ => Err("Not a FeatureMembership".into()),
        }
    }
    fn try_as_element_filter_membership_ref_mut(
        self,
    ) -> Result<ElementFilterMembershipRefMut<'a>, String> {
        match self {
            OwningMembershipRefMut::ElementFilterMembership(e) => Ok(e),
            _ => Err("Not a ElementFilterMembership".into()),
        }
    }
}
impl<'a> OwningMembershipDowncastRef<'a> for OwningMembershipRef<'a> {
    fn try_as_feature_value_ref(self) -> Result<FeatureValueRef<'a>, String> {
        match self {
            OwningMembershipRef::FeatureValue(e) => Ok(e),
            _ => Err("Not a FeatureValue".into()),
        }
    }
    fn try_as_feature_membership_ref(self) -> Result<FeatureMembershipRef<'a>, String> {
        match self {
            OwningMembershipRef::FeatureMembership(e) => Ok(e),
            _ => Err("Not a FeatureMembership".into()),
        }
    }
    fn try_as_element_filter_membership_ref(
        self,
    ) -> Result<ElementFilterMembershipRef<'a>, String> {
        match self {
            OwningMembershipRef::ElementFilterMembership(e) => Ok(e),
            _ => Err("Not a ElementFilterMembership".into()),
        }
    }
}
pub trait OwningMembershipMethodsDescendants
where
    Self: DescendantOf<OwningMembership>,
    Self::Via: OwningMembershipMethods,
    Self: Sized,
{}
pub trait OwningMembershipMethods: Sized {}
impl<T: OwningMembershipMethodsDescendants> OwningMembershipMethods for T
where
    T::Via: OwningMembershipMethods,
{}
impl DescendantOf<Membership> for OwningMembership {
    type Via = Membership;
    fn into_via(self) -> Self::Via {
        self.into_membership()
    }
}
impl MembershipMethodsDescendants for OwningMembership {}
impl DescendantOf<Relationship> for OwningMembership {
    type Via = Membership;
    fn into_via(self) -> Self::Via {
        self.into_membership()
    }
}
impl RelationshipMethodsDescendants for OwningMembership {}
impl DescendantOf<Element> for OwningMembership {
    type Via = Membership;
    fn into_via(self) -> Self::Via {
        self.into_membership()
    }
}
impl ElementMethodsDescendants for OwningMembership {}
pub trait OwningMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<OwningMembershipRefMut<'a>>,
    Self::Via: OwningMembershipRefMutMethods,
    Self: Sized,
{}
pub trait OwningMembershipRefMutMethods: Sized {}
impl<'a, T: OwningMembershipRefMutMethodsDescendants<'a>> OwningMembershipRefMutMethods
for T
where
    T::Via: OwningMembershipRefMutMethods,
{}
impl<'a> DescendantOf<MembershipRefMut<'a>> for OwningMembershipRefMut<'a> {
    type Via = MembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for OwningMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for OwningMembershipRefMut<'a> {
    type Via = MembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for OwningMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for OwningMembershipRefMut<'a> {
    type Via = MembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for OwningMembershipRefMut<'a> {}
pub trait OwningMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<OwningMembershipRef<'a>>,
    Self::Via: OwningMembershipRefMethods,
    Self: Sized,
{}
pub trait OwningMembershipRefMethods: Sized {}
impl<'a, T: OwningMembershipRefMethodsDescendants<'a>> OwningMembershipRefMethods for T
where
    T::Via: OwningMembershipRefMethods,
{}
impl<'a> DescendantOf<MembershipRef<'a>> for OwningMembershipRef<'a> {
    type Via = MembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for OwningMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for OwningMembershipRef<'a> {
    type Via = MembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for OwningMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for OwningMembershipRef<'a> {
    type Via = MembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for OwningMembershipRef<'a> {}

