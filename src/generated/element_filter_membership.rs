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
pub struct ElementFilterMembershipInner {
    pub(super) sup_owning_membership: OwningMembershipInner,
}
pub trait ElementFilterMembershipStruct
where
    Self: ElementFilterMembershipStructRefMut,
    Self: ElementFilterMembershipStructRef,
    Self: OwningMembershipStruct,
{}
pub trait ElementFilterMembershipStructRefMut
where
    Self: ElementFilterMembershipStructRef,
    Self: OwningMembershipStructRefMut,
{}
pub trait ElementFilterMembershipStructRef
where
    Self: OwningMembershipStructRef,
{}
pub trait ElementFilterMembershipUpcast: ElementFilterMembershipStruct {
    fn into_element_filter_membership(self) -> ElementFilterMembership;
}
pub trait ElementFilterMembershipUpcastRefMut<'a>: ElementFilterMembershipStructRefMut {
    fn as_element_filter_membership_ref_mut(self) -> ElementFilterMembershipRefMut<'a>;
}
pub trait ElementFilterMembershipUpcastRef<'a>: ElementFilterMembershipStructRef {
    fn as_element_filter_membership_ref(self) -> ElementFilterMembershipRef<'a>;
}
impl ElementFilterMembershipStruct for ElementFilterMembershipInner {}
impl ElementFilterMembershipStructRefMut for ElementFilterMembershipInner {}
impl ElementFilterMembershipStructRef for ElementFilterMembershipInner {}
impl OwningMembershipStruct for ElementFilterMembershipInner {}
impl OwningMembershipStructRefMut for ElementFilterMembershipInner {}
impl OwningMembershipStructRef for ElementFilterMembershipInner {}
impl MembershipStruct for ElementFilterMembershipInner {
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
impl MembershipStructRefMut for ElementFilterMembershipInner {
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
impl MembershipStructRef for ElementFilterMembershipInner {
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
impl RelationshipStruct for ElementFilterMembershipInner {
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
impl RelationshipStructRefMut for ElementFilterMembershipInner {
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
impl RelationshipStructRef for ElementFilterMembershipInner {
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
impl ElementStruct for ElementFilterMembershipInner {
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
impl ElementStructRefMut for ElementFilterMembershipInner {
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
impl ElementStructRef for ElementFilterMembershipInner {
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
pub enum ElementFilterMembership {
    Itself(ElementFilterMembershipInner),
}
pub enum ElementFilterMembershipRefMut<'a> {
    Itself(&'a mut ElementFilterMembershipInner),
}
pub enum ElementFilterMembershipRef<'a> {
    Itself(&'a ElementFilterMembershipInner),
}
impl ElementFilterMembership {
    pub fn as_ref(&self) -> ElementFilterMembershipRef {
        match self {
            ElementFilterMembership::Itself(inner) => {
                ElementFilterMembershipRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ElementFilterMembershipRefMut {
        match self {
            ElementFilterMembership::Itself(inner) => {
                ElementFilterMembershipRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> ElementFilterMembershipRefMut<'a> {
    pub fn as_ref(self) -> ElementFilterMembershipRef<'a> {
        match self {
            ElementFilterMembershipRefMut::Itself(inner) => {
                ElementFilterMembershipRef::Itself(inner)
            }
        }
    }
}
impl ElementFilterMembershipStruct for ElementFilterMembership {}
impl ElementFilterMembershipStructRefMut for ElementFilterMembership {}
impl ElementFilterMembershipStructRef for ElementFilterMembership {}
impl<'a> ElementFilterMembershipStructRefMut for ElementFilterMembershipRefMut<'a> {}
impl<'a> ElementFilterMembershipStructRef for ElementFilterMembershipRefMut<'a> {}
impl<'a> ElementFilterMembershipStructRef for ElementFilterMembershipRef<'a> {}
impl OwningMembershipStruct for ElementFilterMembership {}
impl OwningMembershipStructRefMut for ElementFilterMembership {}
impl OwningMembershipStructRef for ElementFilterMembership {}
impl<'a> OwningMembershipStructRefMut for ElementFilterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ElementFilterMembershipRefMut<'a> {}
impl<'a> OwningMembershipStructRef for ElementFilterMembershipRef<'a> {}
impl MembershipStruct for ElementFilterMembership {
    fn member_short_name(self) -> Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_short_name(),
        }
    }
    fn member_element(self) -> std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_element(),
        }
    }
    fn member_name(self) -> Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_name(),
        }
    }
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembership::Itself(x) => x.visibility(),
        }
    }
}
impl MembershipStructRefMut for ElementFilterMembership {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembership::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl MembershipStructRef for ElementFilterMembership {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembership::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRefMut for ElementFilterMembershipRefMut<'a> {
    fn member_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_short_name_ref_mut(),
        }
    }
    fn member_element_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_element_ref_mut(),
        }
    }
    fn member_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_name_ref_mut(),
        }
    }
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
}
impl<'a> MembershipStructRef for ElementFilterMembershipRefMut<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.visibility_ref(),
        }
    }
}
impl<'a> MembershipStructRef for ElementFilterMembershipRef<'a> {
    fn member_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.member_short_name_ref(),
        }
    }
    fn member_element_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::element::Element>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.member_element_ref(),
        }
    }
    fn member_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.member_name_ref(),
        }
    }
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.visibility_ref(),
        }
    }
}
impl RelationshipStruct for ElementFilterMembership {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for ElementFilterMembership {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for ElementFilterMembership {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ElementFilterMembershipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ElementFilterMembershipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ElementFilterMembershipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for ElementFilterMembership {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ElementFilterMembership::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ElementFilterMembership {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ElementFilterMembership::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for ElementFilterMembership {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembership::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ElementFilterMembership::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembership::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ElementFilterMembership::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ElementFilterMembershipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ElementFilterMembershipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ElementFilterMembershipRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ElementFilterMembershipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ElementFilterMembershipRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ElementFilterMembershipUpcast for ElementFilterMembership {
    fn into_element_filter_membership(self) -> ElementFilterMembership {
        self
    }
}
impl<'a> ElementFilterMembershipUpcastRefMut<'a> for ElementFilterMembershipRefMut<'a> {
    fn as_element_filter_membership_ref_mut(self) -> ElementFilterMembershipRefMut<'a> {
        self
    }
}
impl<'a> ElementFilterMembershipUpcastRef<'a> for ElementFilterMembershipRef<'a> {
    fn as_element_filter_membership_ref(self) -> ElementFilterMembershipRef<'a> {
        self
    }
}
impl OwningMembershipUpcast for ElementFilterMembership {
    fn into_owning_membership(self) -> OwningMembership {
        OwningMembership::ElementFilterMembership(self).into_owning_membership()
    }
}
impl<'a> OwningMembershipUpcastRefMut<'a> for ElementFilterMembershipRefMut<'a> {
    fn as_owning_membership_ref_mut(self) -> OwningMembershipRefMut<'a> {
        OwningMembershipRefMut::ElementFilterMembership(self)
            .as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipUpcastRef<'a> for ElementFilterMembershipRef<'a> {
    fn as_owning_membership_ref(self) -> OwningMembershipRef<'a> {
        OwningMembershipRef::ElementFilterMembership(self).as_owning_membership_ref()
    }
}
impl MembershipUpcast for ElementFilterMembership {
    fn into_membership(self) -> Membership {
        OwningMembership::ElementFilterMembership(self).into_membership()
    }
}
impl<'a> MembershipUpcastRefMut<'a> for ElementFilterMembershipRefMut<'a> {
    fn as_membership_ref_mut(self) -> MembershipRefMut<'a> {
        OwningMembershipRefMut::ElementFilterMembership(self).as_membership_ref_mut()
    }
}
impl<'a> MembershipUpcastRef<'a> for ElementFilterMembershipRef<'a> {
    fn as_membership_ref(self) -> MembershipRef<'a> {
        OwningMembershipRef::ElementFilterMembership(self).as_membership_ref()
    }
}
impl RelationshipUpcast for ElementFilterMembership {
    fn into_relationship(self) -> Relationship {
        OwningMembership::ElementFilterMembership(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ElementFilterMembershipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        OwningMembershipRefMut::ElementFilterMembership(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ElementFilterMembershipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        OwningMembershipRef::ElementFilterMembership(self).as_relationship_ref()
    }
}
impl ElementUpcast for ElementFilterMembership {
    fn into_element(self) -> Element {
        OwningMembership::ElementFilterMembership(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ElementFilterMembershipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OwningMembershipRefMut::ElementFilterMembership(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ElementFilterMembershipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OwningMembershipRef::ElementFilterMembership(self).as_element_ref()
    }
}
pub trait ElementFilterMembershipDowncast {}
pub trait ElementFilterMembershipDowncastRefMut<'a> {}
pub trait ElementFilterMembershipDowncastRef<'a> {}
impl ElementFilterMembershipDowncast for ElementFilterMembership {}
impl<'a> ElementFilterMembershipDowncastRefMut<'a>
for ElementFilterMembershipRefMut<'a> {}
impl<'a> ElementFilterMembershipDowncastRef<'a> for ElementFilterMembershipRef<'a> {}
pub trait ElementFilterMembershipMethodsDescendants
where
    Self: DescendantOf<ElementFilterMembership>,
    Self::Via: ElementFilterMembershipMethods,
    Self: Sized,
{}
pub trait ElementFilterMembershipMethods: Sized {}
impl<T: ElementFilterMembershipMethodsDescendants> ElementFilterMembershipMethods for T
where
    T::Via: ElementFilterMembershipMethods,
{}
impl DescendantOf<OwningMembership> for ElementFilterMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl OwningMembershipMethodsDescendants for ElementFilterMembership {}
impl DescendantOf<Membership> for ElementFilterMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl MembershipMethodsDescendants for ElementFilterMembership {}
impl DescendantOf<Relationship> for ElementFilterMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl RelationshipMethodsDescendants for ElementFilterMembership {}
impl DescendantOf<Element> for ElementFilterMembership {
    type Via = OwningMembership;
    fn into_via(self) -> Self::Via {
        self.into_owning_membership()
    }
}
impl ElementMethodsDescendants for ElementFilterMembership {}
pub trait ElementFilterMembershipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ElementFilterMembershipRefMut<'a>>,
    Self::Via: ElementFilterMembershipRefMutMethods,
    Self: Sized,
{}
pub trait ElementFilterMembershipRefMutMethods: Sized {}
impl<
    'a,
    T: ElementFilterMembershipRefMutMethodsDescendants<'a>,
> ElementFilterMembershipRefMutMethods for T
where
    T::Via: ElementFilterMembershipRefMutMethods,
{}
impl<'a> DescendantOf<OwningMembershipRefMut<'a>> for ElementFilterMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> OwningMembershipRefMutMethodsDescendants<'a>
for ElementFilterMembershipRefMut<'a> {}
impl<'a> DescendantOf<MembershipRefMut<'a>> for ElementFilterMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> MembershipRefMutMethodsDescendants<'a> for ElementFilterMembershipRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ElementFilterMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ElementFilterMembershipRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ElementFilterMembershipRefMut<'a> {
    type Via = OwningMembershipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ElementFilterMembershipRefMut<'a> {}
pub trait ElementFilterMembershipRefMethodsDescendants<'a>
where
    Self: DescendantOf<ElementFilterMembershipRef<'a>>,
    Self::Via: ElementFilterMembershipRefMethods,
    Self: Sized,
{}
pub trait ElementFilterMembershipRefMethods: Sized {}
impl<
    'a,
    T: ElementFilterMembershipRefMethodsDescendants<'a>,
> ElementFilterMembershipRefMethods for T
where
    T::Via: ElementFilterMembershipRefMethods,
{}
impl<'a> DescendantOf<OwningMembershipRef<'a>> for ElementFilterMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> OwningMembershipRefMethodsDescendants<'a> for ElementFilterMembershipRef<'a> {}
impl<'a> DescendantOf<MembershipRef<'a>> for ElementFilterMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> MembershipRefMethodsDescendants<'a> for ElementFilterMembershipRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for ElementFilterMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ElementFilterMembershipRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ElementFilterMembershipRef<'a> {
    type Via = OwningMembershipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_owning_membership_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ElementFilterMembershipRef<'a> {}

