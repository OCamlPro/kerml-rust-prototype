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
use super::namespace_import::{
    NamespaceImport, NamespaceImportRefMut, NamespaceImportRef,
};
use super::membership_import::{
    MembershipImport, MembershipImportRefMut, MembershipImportRef,
};
pub struct ImportInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) visibility: std::rc::Rc<
        std::cell::RefCell<super::visibility_kind::VisibilityKind>,
    >,
    pub(super) is_recursive: bool,
    pub(super) is_import_all: bool,
}
pub trait ImportStruct
where
    Self: ImportStructRefMut,
    Self: ImportStructRef,
    Self: RelationshipStruct,
{
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
    fn is_recursive(self) -> bool;
    fn is_import_all(self) -> bool;
}
pub trait ImportStructRefMut
where
    Self: ImportStructRef,
    Self: RelationshipStructRefMut,
{
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
    fn is_recursive_ref_mut(&mut self) -> &mut bool;
    fn is_import_all_ref_mut(&mut self) -> &mut bool;
}
pub trait ImportStructRef
where
    Self: RelationshipStructRef,
{
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>>;
    fn is_recursive_ref(&self) -> &bool;
    fn is_import_all_ref(&self) -> &bool;
}
pub trait ImportUpcast: ImportStruct {
    fn into_import(self) -> Import;
}
pub trait ImportUpcastRefMut<'a>: ImportStructRefMut {
    fn as_import_ref_mut(self) -> ImportRefMut<'a>;
}
pub trait ImportUpcastRef<'a>: ImportStructRef {
    fn as_import_ref(self) -> ImportRef<'a>;
}
impl ImportStruct for ImportInner {
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        self.visibility
    }
    fn is_recursive(self) -> bool {
        self.is_recursive
    }
    fn is_import_all(self) -> bool {
        self.is_import_all
    }
}
impl ImportStructRefMut for ImportInner {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        &mut self.visibility
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        &mut self.is_recursive
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        &mut self.is_import_all
    }
}
impl ImportStructRef for ImportInner {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        &self.visibility
    }
    fn is_recursive_ref(&self) -> &bool {
        &self.is_recursive
    }
    fn is_import_all_ref(&self) -> &bool {
        &self.is_import_all
    }
}
impl RelationshipStruct for ImportInner {
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
impl RelationshipStructRefMut for ImportInner {
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
impl RelationshipStructRef for ImportInner {
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
impl ElementStruct for ImportInner {
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
impl ElementStructRefMut for ImportInner {
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
impl ElementStructRef for ImportInner {
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
pub enum Import {
    Itself(ImportInner),
    NamespaceImport(NamespaceImport),
    MembershipImport(MembershipImport),
}
pub enum ImportRefMut<'a> {
    Itself(&'a mut ImportInner),
    NamespaceImport(NamespaceImportRefMut<'a>),
    MembershipImport(MembershipImportRefMut<'a>),
}
pub enum ImportRef<'a> {
    Itself(&'a ImportInner),
    NamespaceImport(NamespaceImportRef<'a>),
    MembershipImport(MembershipImportRef<'a>),
}
impl Import {
    pub fn as_ref(&self) -> ImportRef {
        match self {
            Import::Itself(inner) => ImportRef::Itself(&inner),
            Import::NamespaceImport(inner) => ImportRef::NamespaceImport(inner.as_ref()),
            Import::MembershipImport(inner) => {
                ImportRef::MembershipImport(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ImportRefMut {
        match self {
            Import::Itself(inner) => ImportRefMut::Itself(inner),
            Import::NamespaceImport(inner) => {
                ImportRefMut::NamespaceImport(inner.as_ref_mut())
            }
            Import::MembershipImport(inner) => {
                ImportRefMut::MembershipImport(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ImportRefMut<'a> {
    pub fn as_ref(self) -> ImportRef<'a> {
        match self {
            ImportRefMut::Itself(inner) => ImportRef::Itself(inner),
            ImportRefMut::NamespaceImport(inner) => {
                ImportRef::NamespaceImport(inner.as_ref())
            }
            ImportRefMut::MembershipImport(inner) => {
                ImportRef::MembershipImport(inner.as_ref())
            }
        }
    }
}
impl ImportStruct for Import {
    fn visibility(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Import::Itself(x) => x.visibility(),
            Import::NamespaceImport(x) => x.visibility(),
            Import::MembershipImport(x) => x.visibility(),
        }
    }
    fn is_recursive(self) -> bool {
        match self {
            Import::Itself(x) => x.is_recursive(),
            Import::NamespaceImport(x) => x.is_recursive(),
            Import::MembershipImport(x) => x.is_recursive(),
        }
    }
    fn is_import_all(self) -> bool {
        match self {
            Import::Itself(x) => x.is_import_all(),
            Import::NamespaceImport(x) => x.is_import_all(),
            Import::MembershipImport(x) => x.is_import_all(),
        }
    }
}
impl ImportStructRefMut for Import {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Import::Itself(x) => x.visibility_ref_mut(),
            Import::NamespaceImport(x) => x.visibility_ref_mut(),
            Import::MembershipImport(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            Import::Itself(x) => x.is_recursive_ref_mut(),
            Import::NamespaceImport(x) => x.is_recursive_ref_mut(),
            Import::MembershipImport(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            Import::Itself(x) => x.is_import_all_ref_mut(),
            Import::NamespaceImport(x) => x.is_import_all_ref_mut(),
            Import::MembershipImport(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl ImportStructRef for Import {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            Import::Itself(x) => x.visibility_ref(),
            Import::NamespaceImport(x) => x.visibility_ref(),
            Import::MembershipImport(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            Import::Itself(x) => x.is_recursive_ref(),
            Import::NamespaceImport(x) => x.is_recursive_ref(),
            Import::MembershipImport(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            Import::Itself(x) => x.is_import_all_ref(),
            Import::NamespaceImport(x) => x.is_import_all_ref(),
            Import::MembershipImport(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRefMut for ImportRefMut<'a> {
    fn visibility_ref_mut(
        &mut self,
    ) -> &mut std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ImportRefMut::Itself(x) => x.visibility_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.visibility_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.visibility_ref_mut(),
        }
    }
    fn is_recursive_ref_mut(&mut self) -> &mut bool {
        match self {
            ImportRefMut::Itself(x) => x.is_recursive_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.is_recursive_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.is_recursive_ref_mut(),
        }
    }
    fn is_import_all_ref_mut(&mut self) -> &mut bool {
        match self {
            ImportRefMut::Itself(x) => x.is_import_all_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.is_import_all_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.is_import_all_ref_mut(),
        }
    }
}
impl<'a> ImportStructRef for ImportRefMut<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ImportRefMut::Itself(x) => x.visibility_ref(),
            ImportRefMut::NamespaceImport(x) => x.visibility_ref(),
            ImportRefMut::MembershipImport(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            ImportRefMut::Itself(x) => x.is_recursive_ref(),
            ImportRefMut::NamespaceImport(x) => x.is_recursive_ref(),
            ImportRefMut::MembershipImport(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            ImportRefMut::Itself(x) => x.is_import_all_ref(),
            ImportRefMut::NamespaceImport(x) => x.is_import_all_ref(),
            ImportRefMut::MembershipImport(x) => x.is_import_all_ref(),
        }
    }
}
impl<'a> ImportStructRef for ImportRef<'a> {
    fn visibility_ref(
        &self,
    ) -> &std::rc::Rc<std::cell::RefCell<super::visibility_kind::VisibilityKind>> {
        match self {
            ImportRef::Itself(x) => x.visibility_ref(),
            ImportRef::NamespaceImport(x) => x.visibility_ref(),
            ImportRef::MembershipImport(x) => x.visibility_ref(),
        }
    }
    fn is_recursive_ref(&self) -> &bool {
        match self {
            ImportRef::Itself(x) => x.is_recursive_ref(),
            ImportRef::NamespaceImport(x) => x.is_recursive_ref(),
            ImportRef::MembershipImport(x) => x.is_recursive_ref(),
        }
    }
    fn is_import_all_ref(&self) -> &bool {
        match self {
            ImportRef::Itself(x) => x.is_import_all_ref(),
            ImportRef::NamespaceImport(x) => x.is_import_all_ref(),
            ImportRef::MembershipImport(x) => x.is_import_all_ref(),
        }
    }
}
impl RelationshipStruct for Import {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.target(),
            Import::NamespaceImport(x) => x.target(),
            Import::MembershipImport(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.source(),
            Import::NamespaceImport(x) => x.source(),
            Import::MembershipImport(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owning_related_element(),
            Import::NamespaceImport(x) => x.owning_related_element(),
            Import::MembershipImport(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owned_related_element(),
            Import::NamespaceImport(x) => x.owned_related_element(),
            Import::MembershipImport(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Import::Itself(x) => x.is_implied(),
            Import::NamespaceImport(x) => x.is_implied(),
            Import::MembershipImport(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Import {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.target_ref_mut(),
            Import::NamespaceImport(x) => x.target_ref_mut(),
            Import::MembershipImport(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.source_ref_mut(),
            Import::NamespaceImport(x) => x.source_ref_mut(),
            Import::MembershipImport(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owning_related_element_ref_mut(),
            Import::NamespaceImport(x) => x.owning_related_element_ref_mut(),
            Import::MembershipImport(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owned_related_element_ref_mut(),
            Import::NamespaceImport(x) => x.owned_related_element_ref_mut(),
            Import::MembershipImport(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Import::Itself(x) => x.is_implied_ref_mut(),
            Import::NamespaceImport(x) => x.is_implied_ref_mut(),
            Import::MembershipImport(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Import {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.target_ref(),
            Import::NamespaceImport(x) => x.target_ref(),
            Import::MembershipImport(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.source_ref(),
            Import::NamespaceImport(x) => x.source_ref(),
            Import::MembershipImport(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owning_related_element_ref(),
            Import::NamespaceImport(x) => x.owning_related_element_ref(),
            Import::MembershipImport(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Import::Itself(x) => x.owned_related_element_ref(),
            Import::NamespaceImport(x) => x.owned_related_element_ref(),
            Import::MembershipImport(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Import::Itself(x) => x.is_implied_ref(),
            Import::NamespaceImport(x) => x.is_implied_ref(),
            Import::MembershipImport(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ImportRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.target_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.target_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.source_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.source_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.owning_related_element_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.owned_related_element_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ImportRefMut::Itself(x) => x.is_implied_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.is_implied_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ImportRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.target_ref(),
            ImportRefMut::NamespaceImport(x) => x.target_ref(),
            ImportRefMut::MembershipImport(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.source_ref(),
            ImportRefMut::NamespaceImport(x) => x.source_ref(),
            ImportRefMut::MembershipImport(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.owning_related_element_ref(),
            ImportRefMut::NamespaceImport(x) => x.owning_related_element_ref(),
            ImportRefMut::MembershipImport(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRefMut::Itself(x) => x.owned_related_element_ref(),
            ImportRefMut::NamespaceImport(x) => x.owned_related_element_ref(),
            ImportRefMut::MembershipImport(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ImportRefMut::Itself(x) => x.is_implied_ref(),
            ImportRefMut::NamespaceImport(x) => x.is_implied_ref(),
            ImportRefMut::MembershipImport(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ImportRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRef::Itself(x) => x.target_ref(),
            ImportRef::NamespaceImport(x) => x.target_ref(),
            ImportRef::MembershipImport(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRef::Itself(x) => x.source_ref(),
            ImportRef::NamespaceImport(x) => x.source_ref(),
            ImportRef::MembershipImport(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRef::Itself(x) => x.owning_related_element_ref(),
            ImportRef::NamespaceImport(x) => x.owning_related_element_ref(),
            ImportRef::MembershipImport(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ImportRef::Itself(x) => x.owned_related_element_ref(),
            ImportRef::NamespaceImport(x) => x.owned_related_element_ref(),
            ImportRef::MembershipImport(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ImportRef::Itself(x) => x.is_implied_ref(),
            ImportRef::NamespaceImport(x) => x.is_implied_ref(),
            ImportRef::MembershipImport(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Import {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Import::Itself(x) => x.owned_relationship(),
            Import::NamespaceImport(x) => x.owned_relationship(),
            Import::MembershipImport(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Import::Itself(x) => x.owning_relationship(),
            Import::NamespaceImport(x) => x.owning_relationship(),
            Import::MembershipImport(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Import::Itself(x) => x.element_id(),
            Import::NamespaceImport(x) => x.element_id(),
            Import::MembershipImport(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Import::Itself(x) => x.alias_ids(),
            Import::NamespaceImport(x) => x.alias_ids(),
            Import::MembershipImport(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Import::Itself(x) => x.declared_short_name(),
            Import::NamespaceImport(x) => x.declared_short_name(),
            Import::MembershipImport(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Import::Itself(x) => x.declared_name(),
            Import::NamespaceImport(x) => x.declared_name(),
            Import::MembershipImport(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Import::Itself(x) => x.is_implied_included(),
            Import::NamespaceImport(x) => x.is_implied_included(),
            Import::MembershipImport(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Import {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Import::Itself(x) => x.owned_relationship_ref_mut(),
            Import::NamespaceImport(x) => x.owned_relationship_ref_mut(),
            Import::MembershipImport(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Import::Itself(x) => x.owning_relationship_ref_mut(),
            Import::NamespaceImport(x) => x.owning_relationship_ref_mut(),
            Import::MembershipImport(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Import::Itself(x) => x.element_id_ref_mut(),
            Import::NamespaceImport(x) => x.element_id_ref_mut(),
            Import::MembershipImport(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Import::Itself(x) => x.alias_ids_ref_mut(),
            Import::NamespaceImport(x) => x.alias_ids_ref_mut(),
            Import::MembershipImport(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Import::Itself(x) => x.declared_short_name_ref_mut(),
            Import::NamespaceImport(x) => x.declared_short_name_ref_mut(),
            Import::MembershipImport(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Import::Itself(x) => x.declared_name_ref_mut(),
            Import::NamespaceImport(x) => x.declared_name_ref_mut(),
            Import::MembershipImport(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Import::Itself(x) => x.is_implied_included_ref_mut(),
            Import::NamespaceImport(x) => x.is_implied_included_ref_mut(),
            Import::MembershipImport(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Import {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Import::Itself(x) => x.owned_relationship_ref(),
            Import::NamespaceImport(x) => x.owned_relationship_ref(),
            Import::MembershipImport(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Import::Itself(x) => x.owning_relationship_ref(),
            Import::NamespaceImport(x) => x.owning_relationship_ref(),
            Import::MembershipImport(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Import::Itself(x) => x.element_id_ref(),
            Import::NamespaceImport(x) => x.element_id_ref(),
            Import::MembershipImport(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Import::Itself(x) => x.alias_ids_ref(),
            Import::NamespaceImport(x) => x.alias_ids_ref(),
            Import::MembershipImport(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Import::Itself(x) => x.declared_short_name_ref(),
            Import::NamespaceImport(x) => x.declared_short_name_ref(),
            Import::MembershipImport(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Import::Itself(x) => x.declared_name_ref(),
            Import::NamespaceImport(x) => x.declared_name_ref(),
            Import::MembershipImport(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Import::Itself(x) => x.is_implied_included_ref(),
            Import::NamespaceImport(x) => x.is_implied_included_ref(),
            Import::MembershipImport(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ImportRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ImportRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.owned_relationship_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ImportRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.owning_relationship_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ImportRefMut::Itself(x) => x.element_id_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.element_id_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ImportRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.alias_ids_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ImportRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.declared_short_name_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ImportRefMut::Itself(x) => x.declared_name_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.declared_name_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ImportRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ImportRefMut::NamespaceImport(x) => x.is_implied_included_ref_mut(),
            ImportRefMut::MembershipImport(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ImportRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ImportRefMut::Itself(x) => x.owned_relationship_ref(),
            ImportRefMut::NamespaceImport(x) => x.owned_relationship_ref(),
            ImportRefMut::MembershipImport(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ImportRefMut::Itself(x) => x.owning_relationship_ref(),
            ImportRefMut::NamespaceImport(x) => x.owning_relationship_ref(),
            ImportRefMut::MembershipImport(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ImportRefMut::Itself(x) => x.element_id_ref(),
            ImportRefMut::NamespaceImport(x) => x.element_id_ref(),
            ImportRefMut::MembershipImport(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ImportRefMut::Itself(x) => x.alias_ids_ref(),
            ImportRefMut::NamespaceImport(x) => x.alias_ids_ref(),
            ImportRefMut::MembershipImport(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ImportRefMut::Itself(x) => x.declared_short_name_ref(),
            ImportRefMut::NamespaceImport(x) => x.declared_short_name_ref(),
            ImportRefMut::MembershipImport(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ImportRefMut::Itself(x) => x.declared_name_ref(),
            ImportRefMut::NamespaceImport(x) => x.declared_name_ref(),
            ImportRefMut::MembershipImport(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ImportRefMut::Itself(x) => x.is_implied_included_ref(),
            ImportRefMut::NamespaceImport(x) => x.is_implied_included_ref(),
            ImportRefMut::MembershipImport(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ImportRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ImportRef::Itself(x) => x.owned_relationship_ref(),
            ImportRef::NamespaceImport(x) => x.owned_relationship_ref(),
            ImportRef::MembershipImport(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ImportRef::Itself(x) => x.owning_relationship_ref(),
            ImportRef::NamespaceImport(x) => x.owning_relationship_ref(),
            ImportRef::MembershipImport(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ImportRef::Itself(x) => x.element_id_ref(),
            ImportRef::NamespaceImport(x) => x.element_id_ref(),
            ImportRef::MembershipImport(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ImportRef::Itself(x) => x.alias_ids_ref(),
            ImportRef::NamespaceImport(x) => x.alias_ids_ref(),
            ImportRef::MembershipImport(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ImportRef::Itself(x) => x.declared_short_name_ref(),
            ImportRef::NamespaceImport(x) => x.declared_short_name_ref(),
            ImportRef::MembershipImport(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ImportRef::Itself(x) => x.declared_name_ref(),
            ImportRef::NamespaceImport(x) => x.declared_name_ref(),
            ImportRef::MembershipImport(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ImportRef::Itself(x) => x.is_implied_included_ref(),
            ImportRef::NamespaceImport(x) => x.is_implied_included_ref(),
            ImportRef::MembershipImport(x) => x.is_implied_included_ref(),
        }
    }
}
impl ImportUpcast for Import {
    fn into_import(self) -> Import {
        self
    }
}
impl<'a> ImportUpcastRefMut<'a> for ImportRefMut<'a> {
    fn as_import_ref_mut(self) -> ImportRefMut<'a> {
        self
    }
}
impl<'a> ImportUpcastRef<'a> for ImportRef<'a> {
    fn as_import_ref(self) -> ImportRef<'a> {
        self
    }
}
impl RelationshipUpcast for Import {
    fn into_relationship(self) -> Relationship {
        Relationship::Import(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ImportRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Import(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ImportRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Import(self).as_relationship_ref()
    }
}
impl ElementUpcast for Import {
    fn into_element(self) -> Element {
        Relationship::Import(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ImportRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Import(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ImportRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Import(self).as_element_ref()
    }
}
pub trait ImportDowncast {
    fn try_into_namespace_import(self) -> Result<NamespaceImport, String>;
    fn try_into_membership_import(self) -> Result<MembershipImport, String>;
}
pub trait ImportDowncastRefMut<'a> {
    fn try_as_namespace_import_ref_mut(
        self,
    ) -> Result<NamespaceImportRefMut<'a>, String>;
    fn try_as_membership_import_ref_mut(
        self,
    ) -> Result<MembershipImportRefMut<'a>, String>;
}
pub trait ImportDowncastRef<'a> {
    fn try_as_namespace_import_ref(self) -> Result<NamespaceImportRef<'a>, String>;
    fn try_as_membership_import_ref(self) -> Result<MembershipImportRef<'a>, String>;
}
impl ImportDowncast for Import {
    fn try_into_namespace_import(self) -> Result<NamespaceImport, String> {
        match self {
            Import::NamespaceImport(e) => Ok(e),
            _ => Err("Not a NamespaceImport".into()),
        }
    }
    fn try_into_membership_import(self) -> Result<MembershipImport, String> {
        match self {
            Import::MembershipImport(e) => Ok(e),
            _ => Err("Not a MembershipImport".into()),
        }
    }
}
impl<'a> ImportDowncastRefMut<'a> for ImportRefMut<'a> {
    fn try_as_namespace_import_ref_mut(
        self,
    ) -> Result<NamespaceImportRefMut<'a>, String> {
        match self {
            ImportRefMut::NamespaceImport(e) => Ok(e),
            _ => Err("Not a NamespaceImport".into()),
        }
    }
    fn try_as_membership_import_ref_mut(
        self,
    ) -> Result<MembershipImportRefMut<'a>, String> {
        match self {
            ImportRefMut::MembershipImport(e) => Ok(e),
            _ => Err("Not a MembershipImport".into()),
        }
    }
}
impl<'a> ImportDowncastRef<'a> for ImportRef<'a> {
    fn try_as_namespace_import_ref(self) -> Result<NamespaceImportRef<'a>, String> {
        match self {
            ImportRef::NamespaceImport(e) => Ok(e),
            _ => Err("Not a NamespaceImport".into()),
        }
    }
    fn try_as_membership_import_ref(self) -> Result<MembershipImportRef<'a>, String> {
        match self {
            ImportRef::MembershipImport(e) => Ok(e),
            _ => Err("Not a MembershipImport".into()),
        }
    }
}
pub trait ImportMethodsDescendants
where
    Self: DescendantOf<Import>,
    Self::Via: ImportMethods,
    Self: Sized,
{}
pub trait ImportMethods: Sized {}
impl<T: ImportMethodsDescendants> ImportMethods for T
where
    T::Via: ImportMethods,
{}
impl DescendantOf<Relationship> for Import {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Import {}
impl DescendantOf<Element> for Import {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Import {}
pub trait ImportRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ImportRefMut<'a>>,
    Self::Via: ImportRefMutMethods,
    Self: Sized,
{}
pub trait ImportRefMutMethods: Sized {}
impl<'a, T: ImportRefMutMethodsDescendants<'a>> ImportRefMutMethods for T
where
    T::Via: ImportRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ImportRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ImportRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ImportRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ImportRefMut<'a> {}
pub trait ImportRefMethodsDescendants<'a>
where
    Self: DescendantOf<ImportRef<'a>>,
    Self::Via: ImportRefMethods,
    Self: Sized,
{
    fn imported_memberships_impl(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        self.into_via().imported_memberships(excluded)
    }
}
pub trait ImportRefMethods: Sized {
    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>>;
}
impl<'a, T: ImportRefMethodsDescendants<'a>> ImportRefMethods for T
where
    T::Via: ImportRefMethods,
{
    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::membership::Membership>>> {
        ImportRefMethodsDescendants::imported_memberships_impl(self, excluded)
    }
}
impl<'a> DescendantOf<RelationshipRef<'a>> for ImportRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ImportRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ImportRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ImportRef<'a> {}

