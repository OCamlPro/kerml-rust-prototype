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
pub struct NamespaceImportInner {
    pub(super) sup_import: ImportInner,
    pub(super) imported_namespace: std::rc::Rc<
        std::cell::RefCell<super::namespace::Namespace>,
    >,
}
pub trait NamespaceImportStruct
where
    Self: NamespaceImportStructRefMut,
    Self: NamespaceImportStructRef,
    Self: ImportStruct,
{
    fn imported_namespace(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>;
}
pub trait NamespaceImportStructRefMut
where
    Self: NamespaceImportStructRef,
    Self: ImportStructRefMut,
{
    fn imported_namespace_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>;
}
pub trait NamespaceImportStructRef
where
    Self: ImportStructRef,
{
    fn imported_namespace_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>;
}
pub trait NamespaceImportUpcast: NamespaceImportStruct {
    fn into_namespace_import(self) -> NamespaceImport;
}
pub trait NamespaceImportUpcastRefMut<'a>: NamespaceImportStructRefMut {
    fn as_namespace_import_ref_mut(self) -> NamespaceImportRefMut<'a>;
}
pub trait NamespaceImportUpcastRef<'a>: NamespaceImportStructRef {
    fn as_namespace_import_ref(self) -> NamespaceImportRef<'a>;
}
impl NamespaceImportStruct for NamespaceImportInner {
    fn imported_namespace(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        self.imported_namespace
    }
}
impl NamespaceImportStructRefMut for NamespaceImportInner {
    fn imported_namespace_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        &mut self.imported_namespace
    }
}
impl NamespaceImportStructRef for NamespaceImportInner {
    fn imported_namespace_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        &self.imported_namespace
    }
}
impl ImportStruct for NamespaceImportInner {
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
impl ImportStructRefMut for NamespaceImportInner {
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
impl ImportStructRef for NamespaceImportInner {
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
impl RelationshipStruct for NamespaceImportInner {
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
impl RelationshipStructRefMut for NamespaceImportInner {
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
impl RelationshipStructRef for NamespaceImportInner {
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
impl ElementStruct for NamespaceImportInner {
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
impl ElementStructRefMut for NamespaceImportInner {
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
impl ElementStructRef for NamespaceImportInner {
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
pub enum NamespaceImport {
    Itself(NamespaceImportInner),
}
pub enum NamespaceImportRefMut<'a> {
    Itself(&'a mut NamespaceImportInner),
}
pub enum NamespaceImportRef<'a> {
    Itself(&'a NamespaceImportInner),
}
impl NamespaceImport {
    pub fn as_ref(&self) -> NamespaceImportRef {
        match self {
            NamespaceImport::Itself(inner) => NamespaceImportRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> NamespaceImportRefMut {
        match self {
            NamespaceImport::Itself(inner) => NamespaceImportRefMut::Itself(inner),
        }
    }
}
impl<'a> NamespaceImportRefMut<'a> {
    pub fn as_ref(self) -> NamespaceImportRef<'a> {
        match self {
            NamespaceImportRefMut::Itself(inner) => NamespaceImportRef::Itself(inner),
        }
    }
}
impl NamespaceImportStruct for NamespaceImport {
    fn imported_namespace(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImport::Itself(x) => x.imported_namespace(),
        }
    }
}
impl NamespaceImportStructRefMut for NamespaceImport {
    fn imported_namespace_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImport::Itself(x) => x.imported_namespace_ref_mut(),
        }
    }
}
impl NamespaceImportStructRef for NamespaceImport {
    fn imported_namespace_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImport::Itself(x) => x.imported_namespace_ref(),
        }
    }
}
impl<'a> NamespaceImportStructRefMut for NamespaceImportRefMut<'a> {
    fn imported_namespace_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.imported_namespace_ref_mut(),
        }
    }
}
impl<'a> NamespaceImportStructRef for NamespaceImportRefMut<'a> {
    fn imported_namespace_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.imported_namespace_ref(),
        }
    }
}
impl<'a> NamespaceImportStructRef for NamespaceImportRef<'a> {
    fn imported_namespace_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>> {
        match self {
            NamespaceImportRef::Itself(x) => x.imported_namespace_ref(),
        }
    }
}
impl ImportStruct for NamespaceImport {
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImport::Itself(x) => x.visibility(),
        }
    }
    fn is_recursive(self) -> bool {
        match self {
            NamespaceImport::Itself(x) => x.is_recursive(),
        }
    }
    fn is_import_all(self) -> bool {
        match self {
            NamespaceImport::Itself(x) => x.is_import_all(),
        }
    }
}
impl ImportStructRefMut for NamespaceImport {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImport::Itself(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImport::Itself(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImport::Itself(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl ImportStructRef for NamespaceImport {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImport::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            NamespaceImport::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            NamespaceImport::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRefMut for NamespaceImportRefMut<'a> {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl<'a> ImportStructRef for NamespaceImportRefMut<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRef for NamespaceImportRef<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            NamespaceImportRef::Itself(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            NamespaceImportRef::Itself(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            NamespaceImportRef::Itself(x) => x.is_import_all_ref(),
        }
    }
}
impl RelationshipStruct for NamespaceImport {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for NamespaceImport {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for NamespaceImport {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for NamespaceImportRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for NamespaceImportRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for NamespaceImportRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            NamespaceImportRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for NamespaceImport {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImport::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            NamespaceImport::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            NamespaceImport::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for NamespaceImport {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            NamespaceImport::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            NamespaceImport::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            NamespaceImport::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for NamespaceImport {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImport::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImport::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NamespaceImport::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NamespaceImport::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImport::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NamespaceImport::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for NamespaceImportRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            NamespaceImportRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for NamespaceImportRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NamespaceImportRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImportRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NamespaceImportRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for NamespaceImportRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NamespaceImportRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NamespaceImportRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NamespaceImportRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImportRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NamespaceImportRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NamespaceImportRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl NamespaceImportUpcast for NamespaceImport {
    fn into_namespace_import(self) -> NamespaceImport {
        self
    }
}
impl<'a> NamespaceImportUpcastRefMut<'a> for NamespaceImportRefMut<'a> {
    fn as_namespace_import_ref_mut(self) -> NamespaceImportRefMut<'a> {
        self
    }
}
impl<'a> NamespaceImportUpcastRef<'a> for NamespaceImportRef<'a> {
    fn as_namespace_import_ref(self) -> NamespaceImportRef<'a> {
        self
    }
}
impl ImportUpcast for NamespaceImport {
    fn into_import(self) -> Import {
        Import::NamespaceImport(self).into_import()
    }
}
impl<'a> ImportUpcastRefMut<'a> for NamespaceImportRefMut<'a> {
    fn as_import_ref_mut(self) -> ImportRefMut<'a> {
        ImportRefMut::NamespaceImport(self).as_import_ref_mut()
    }
}
impl<'a> ImportUpcastRef<'a> for NamespaceImportRef<'a> {
    fn as_import_ref(self) -> ImportRef<'a> {
        ImportRef::NamespaceImport(self).as_import_ref()
    }
}
impl RelationshipUpcast for NamespaceImport {
    fn into_relationship(self) -> Relationship {
        Import::NamespaceImport(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for NamespaceImportRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ImportRefMut::NamespaceImport(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for NamespaceImportRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ImportRef::NamespaceImport(self).as_relationship_ref()
    }
}
impl ElementUpcast for NamespaceImport {
    fn into_element(self) -> Element {
        Import::NamespaceImport(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for NamespaceImportRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ImportRefMut::NamespaceImport(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for NamespaceImportRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ImportRef::NamespaceImport(self).as_element_ref()
    }
}
pub trait NamespaceImportDowncast {}
pub trait NamespaceImportDowncastRefMut<'a> {}
pub trait NamespaceImportDowncastRef<'a> {}
impl NamespaceImportDowncast for NamespaceImport {}
impl<'a> NamespaceImportDowncastRefMut<'a> for NamespaceImportRefMut<'a> {}
impl<'a> NamespaceImportDowncastRef<'a> for NamespaceImportRef<'a> {}
pub trait NamespaceImportMethodsDescendants
where
    Self: DescendantOf<NamespaceImport>,
    Self::Via: NamespaceImportMethods,
    Self: Sized,
{}
pub trait NamespaceImportMethods: Sized {}
impl<T: NamespaceImportMethodsDescendants> NamespaceImportMethods for T
where
    T::Via: NamespaceImportMethods,
{}
impl DescendantOf<Import> for NamespaceImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl ImportMethodsDescendants for NamespaceImport {}
impl DescendantOf<Relationship> for NamespaceImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl RelationshipMethodsDescendants for NamespaceImport {}
impl DescendantOf<Element> for NamespaceImport {
    type Via = Import;
    fn into_via(self) -> Self::Via {
        self.into_import()
    }
}
impl ElementMethodsDescendants for NamespaceImport {}
pub trait NamespaceImportRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<NamespaceImportRefMut<'a>>,
    Self::Via: NamespaceImportRefMutMethods,
    Self: Sized,
{}
pub trait NamespaceImportRefMutMethods: Sized {}
impl<'a, T: NamespaceImportRefMutMethodsDescendants<'a>> NamespaceImportRefMutMethods
for T
where
    T::Via: NamespaceImportRefMutMethods,
{}
impl<'a> DescendantOf<ImportRefMut<'a>> for NamespaceImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> ImportRefMutMethodsDescendants<'a> for NamespaceImportRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for NamespaceImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for NamespaceImportRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for NamespaceImportRefMut<'a> {
    type Via = ImportRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for NamespaceImportRefMut<'a> {}
pub trait NamespaceImportRefMethodsDescendants<'a>
where
    Self: DescendantOf<NamespaceImportRef<'a>>,
    Self::Via: NamespaceImportRefMethods,
    Self: Sized,
{}
pub trait NamespaceImportRefMethods: Sized {}
impl<'a, T: NamespaceImportRefMethodsDescendants<'a>> NamespaceImportRefMethods for T
where
    T::Via: NamespaceImportRefMethods,
{}
impl<'a> DescendantOf<ImportRef<'a>> for NamespaceImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> ImportRefMethodsDescendants<'a> for NamespaceImportRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for NamespaceImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for NamespaceImportRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for NamespaceImportRef<'a> {
    type Via = ImportRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_import_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for NamespaceImportRef<'a> {}

