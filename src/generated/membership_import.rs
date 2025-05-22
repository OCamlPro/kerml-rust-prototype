#![allow(unused)]
use super::utils::DescendantOf;
use super::import::{
    Import, ImportRefMut, ImportRef, ImportStruct, ImportStructRefMut, ImportStructRef,
    ImportInner, ImportUpcast, ImportUpcastRefMut, ImportUpcastRef,
    ImportMethodsDescendants, ImportRefMutMethodsDescendants, ImportRefMethodsDescendants,
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
pub struct MembershipImportInner {
    pub(super) sup_import: ImportInner,
    pub(super) imported_membership: std::rc::Rc<
        std::cell::RefCell<super::membership::Membership>,
    >,
}
pub trait MembershipImportStruct
where
    Self: MembershipImportStructRefMut,
    Self: MembershipImportStructRef,
    Self: ImportStruct,
{
    fn imported_membership(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::membership::Membership>>;
}
pub trait MembershipImportStructRefMut
where
    Self: MembershipImportStructRef,
    Self: ImportStructRefMut,
{
    fn imported_membership_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::membership::Membership>>;
}
pub trait MembershipImportStructRef
where
    Self: ImportStructRef,
{
    fn imported_membership_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::membership::Membership>>;
}
pub trait MembershipImportUpcast: MembershipImportStruct {
    fn into_membership_import(self) -> MembershipImport;
}
pub trait MembershipImportUpcastRefMut<'a>: MembershipImportStructRefMut {
    fn as_membership_import_ref_mut(self) -> MembershipImportRefMut<'a>;
}
pub trait MembershipImportUpcastRef<'a>: MembershipImportStructRef {
    fn as_membership_import_ref(self) -> MembershipImportRef<'a>;
}
impl MembershipImportStruct for MembershipImportInner {
    fn imported_membership(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        self.imported_membership
    }
}
impl MembershipImportStructRefMut for MembershipImportInner {
    fn imported_membership_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        &mut self.imported_membership
    }
}
impl MembershipImportStructRef for MembershipImportInner {
    fn imported_membership_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        &self.imported_membership
    }
}
impl ImportStruct for MembershipImportInner {
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_import.visibility()
    }
    fn is_recursive(self) -> bool {
        self.sup_import.is_recursive()
    }
    fn is_import_all(self) -> bool {
        self.sup_import.is_import_all()
    }
}
impl ImportStructRefMut for MembershipImportInner {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_import.visibility_ref_mut()
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        self.sup_import.is_recursive_ref_mut()
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        self.sup_import.is_import_all_ref_mut()
    }
}
impl ImportStructRef for MembershipImportInner {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.sup_import.visibility_ref()
    }
    fn is_recursive_ref(&self) -> &bool {
        self.sup_import.is_recursive_ref()
    }
    fn is_import_all_ref(&self) -> &bool {
        self.sup_import.is_import_all_ref()
    }
}
impl RelationshipStruct for MembershipImportInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_import.is_implied()
    }
}
impl RelationshipStructRefMut for MembershipImportInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_import.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for MembershipImportInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_import.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_import.is_implied_ref()
    }
}
impl ElementStruct for MembershipImportInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_import.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_import.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_import.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_import.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_import.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_import.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_import.is_implied_included()
    }
}
impl ElementStructRefMut for MembershipImportInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_import.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_import.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_import.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_import.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_import.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_import.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_import.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for MembershipImportInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_import.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_import.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_import.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_import.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_import.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_import.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_import.is_implied_included_ref()
    }
}
pub enum MembershipImport {
    Itself(MembershipImportInner),
}
pub enum MembershipImportRefMut<'a> {
    Itself(&'a mut MembershipImportInner),
}
pub enum MembershipImportRef<'a> {
    Itself(&'a MembershipImportInner),
}
impl MembershipImport {
    pub fn as_ref(&self) -> MembershipImportRef {
        match self {
            MembershipImport::Itself(inner) => MembershipImportRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> MembershipImportRefMut {
        match self {
            MembershipImport::Itself(inner) => MembershipImportRefMut::Itself(inner),
        }
    }
}
impl<'a> MembershipImportRefMut<'a> {
    pub fn as_ref(self) -> MembershipImportRef<'a> {
        match self {
            MembershipImportRefMut::Itself(inner) => MembershipImportRef::Itself(inner),
        }
    }
}
impl MembershipImportStruct for MembershipImport {
    fn imported_membership(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImport::Itself(x) => x.imported_membership(),
        }
    }
}
impl MembershipImportStructRefMut for MembershipImport {
    fn imported_membership_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImport::Itself(x) => x.imported_membership_ref_mut(),
        }
    }
}
impl MembershipImportStructRef for MembershipImport {
    fn imported_membership_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImport::Itself(x) => x.imported_membership_ref(),
        }
    }
}
impl<'a> MembershipImportStructRefMut for MembershipImportRefMut<'a> {
    fn imported_membership_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.imported_membership_ref_mut(),
        }
    }
}
impl<'a> MembershipImportStructRef for MembershipImportRefMut<'a> {
    fn imported_membership_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.imported_membership_ref(),
        }
    }
}
impl<'a> MembershipImportStructRef for MembershipImportRef<'a> {
    fn imported_membership_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::membership::Membership>> {
        match self {
            MembershipImportRef::Itself(x) => x.imported_membership_ref(),
        }
    }
}
impl ImportStruct for MembershipImport {
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImport::Itself(x) => x.visibility(),
        }
    }
    fn is_recursive(self) -> bool {
        match self {
            MembershipImport::Itself(x) => x.is_recursive(),
        }
    }
    fn is_import_all(self) -> bool {
        match self {
            MembershipImport::Itself(x) => x.is_import_all(),
        }
    }
}
impl ImportStructRefMut for MembershipImport {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImport::Itself(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImport::Itself(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImport::Itself(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl ImportStructRef for MembershipImport {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImport::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            MembershipImport::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            MembershipImport::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRefMut for MembershipImportRefMut<'a> {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl<'a> ImportStructRef for MembershipImportRefMut<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRef for MembershipImportRef<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            MembershipImportRef::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            MembershipImportRef::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            MembershipImportRef::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl RelationshipStruct for MembershipImport {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for MembershipImport {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for MembershipImport {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for MembershipImportRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for MembershipImportRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for MembershipImportRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            MembershipImportRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            MembershipImportRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for MembershipImport {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImport::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            MembershipImport::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            MembershipImport::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for MembershipImport {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MembershipImport::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MembershipImport::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MembershipImport::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for MembershipImport {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImport::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImport::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MembershipImport::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MembershipImport::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImport::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MembershipImport::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MembershipImportRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MembershipImportRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MembershipImportRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MembershipImportRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImportRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MembershipImportRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImportRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MembershipImportRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MembershipImportRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImportRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MembershipImportRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MembershipImportRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MembershipImportRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImportRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MembershipImportRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MembershipImportRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl MembershipImportUpcast for MembershipImport {
    fn into_membership_import(self) -> MembershipImport {
        self
    }
}
impl<'a> MembershipImportUpcastRefMut<'a> for MembershipImportRefMut<'a> {
    fn as_membership_import_ref_mut(self) -> MembershipImportRefMut<'a> {
        self
    }
}
impl<'a> MembershipImportUpcastRef<'a> for MembershipImportRef<'a> {
    fn as_membership_import_ref(self) -> MembershipImportRef<'a> {
        self
    }
}
impl ImportUpcast for MembershipImport {
    fn into_import(self) -> Import {
        Import::MembershipImport(self).into_import()
    }
}
impl<'a> ImportUpcastRefMut<'a> for MembershipImportRefMut<'a> {
    fn as_import_ref_mut(self) -> ImportRefMut<'a> {
        ImportRefMut::MembershipImport(self).as_import_ref_mut()
    }
}
impl<'a> ImportUpcastRef<'a> for MembershipImportRef<'a> {
    fn as_import_ref(self) -> ImportRef<'a> {
        ImportRef::MembershipImport(self).as_import_ref()
    }
}
impl RelationshipUpcast for MembershipImport {
    fn into_relationship(self) -> Relationship {
        Import::MembershipImport(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for MembershipImportRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ImportRefMut::MembershipImport(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for MembershipImportRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ImportRef::MembershipImport(self).as_relationship_ref()
    }
}
impl ElementUpcast for MembershipImport {
    fn into_element(self) -> Element {
        Import::MembershipImport(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MembershipImportRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ImportRefMut::MembershipImport(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MembershipImportRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ImportRef::MembershipImport(self).as_element_ref()
    }
}
pub trait MembershipImportDowncast {}
pub trait MembershipImportDowncastRefMut<'a> {}
pub trait MembershipImportDowncastRef<'a> {}
impl MembershipImportDowncast for MembershipImport {}
impl<'a> MembershipImportDowncastRefMut<'a> for MembershipImportRefMut<'a> {}
impl<'a> MembershipImportDowncastRef<'a> for MembershipImportRef<'a> {}
pub trait MembershipImportMethodsDescendants
where
    Self: DescendantOf<MembershipImport>,
    Self::Via: MembershipImportMethods,
    Self: Sized,
{}
pub trait MembershipImportMethods: Sized {}
impl<T: MembershipImportMethodsDescendants> MembershipImportMethods for T
where
    T::Via: MembershipImportMethods,
{}
impl DescendantOf<Import> for MembershipImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl ImportMethodsDescendants for MembershipImport {}
impl DescendantOf<Relationship> for MembershipImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl RelationshipMethodsDescendants for MembershipImport {}
impl DescendantOf<Element> for MembershipImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl ElementMethodsDescendants for MembershipImport {}
pub trait MembershipImportRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MembershipImportRefMut<'a>>,
    Self::Via: MembershipImportRefMutMethods,
    Self: Sized,
{}
pub trait MembershipImportRefMutMethods: Sized {}
impl<'a, T: MembershipImportRefMutMethodsDescendants<'a>> MembershipImportRefMutMethods
for T
where
    T::Via: MembershipImportRefMutMethods,
{}
impl<'a> DescendantOf<ImportRefMut<'a>> for MembershipImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> ImportRefMutMethodsDescendants<'a> for MembershipImportRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for MembershipImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for MembershipImportRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MembershipImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MembershipImportRefMut<'a> {}
pub trait MembershipImportRefMethodsDescendants<'a>
where
    Self: DescendantOf<MembershipImportRef<'a>>,
    Self::Via: MembershipImportRefMethods,
    Self: Sized,
{}
pub trait MembershipImportRefMethods: Sized {}
impl<'a, T: MembershipImportRefMethodsDescendants<'a>> MembershipImportRefMethods for T
where
    T::Via: MembershipImportRefMethods,
{}
impl<'a> DescendantOf<ImportRef<'a>> for MembershipImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> ImportRefMethodsDescendants<'a> for MembershipImportRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for MembershipImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for MembershipImportRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MembershipImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MembershipImportRef<'a> {}

