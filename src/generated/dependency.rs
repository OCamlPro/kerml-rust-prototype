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
pub struct DependencyInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) client: Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>,
    pub(super) supplier: Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>,
}
pub trait DependencyStruct
where
    Self: DependencyStructRefMut,
    Self: DependencyStructRef,
    Self: RelationshipStruct,
{
    fn client(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn supplier(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
}
pub trait DependencyStructRefMut
where
    Self: DependencyStructRef,
    Self: RelationshipStructRefMut,
{
    fn client_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn supplier_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
}
pub trait DependencyStructRef
where
    Self: RelationshipStructRef,
{
    fn client_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn supplier_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
}
pub trait DependencyUpcast: DependencyStruct {
    fn into_dependency(self) -> Dependency;
}
pub trait DependencyUpcastRefMut<'a>: DependencyStructRefMut {
    fn as_dependency_ref_mut(self) -> DependencyRefMut<'a>;
}
pub trait DependencyUpcastRef<'a>: DependencyStructRef {
    fn as_dependency_ref(self) -> DependencyRef<'a>;
}
impl DependencyStruct for DependencyInner {
    fn client(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.client
    }
    fn supplier(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.supplier
    }
}
impl DependencyStructRefMut for DependencyInner {
    fn client_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.client
    }
    fn supplier_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.supplier
    }
}
impl DependencyStructRef for DependencyInner {
    fn client_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.client
    }
    fn supplier_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.supplier
    }
}
impl RelationshipStruct for DependencyInner {
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
impl RelationshipStructRefMut for DependencyInner {
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
impl RelationshipStructRef for DependencyInner {
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
impl ElementStruct for DependencyInner {
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
impl ElementStructRefMut for DependencyInner {
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
impl ElementStructRef for DependencyInner {
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
pub enum Dependency {
    Itself(DependencyInner),
}
pub enum DependencyRefMut<'a> {
    Itself(&'a mut DependencyInner),
}
pub enum DependencyRef<'a> {
    Itself(&'a DependencyInner),
}
impl Dependency {
    pub fn as_ref(&self) -> DependencyRef {
        match self {
            Dependency::Itself(inner) => DependencyRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> DependencyRefMut {
        match self {
            Dependency::Itself(inner) => DependencyRefMut::Itself(inner),
        }
    }
}
impl<'a> DependencyRefMut<'a> {
    pub fn as_ref(self) -> DependencyRef<'a> {
        match self {
            DependencyRefMut::Itself(inner) => DependencyRef::Itself(inner),
        }
    }
}
impl DependencyStruct for Dependency {
    fn client(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.client(),
        }
    }
    fn supplier(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.supplier(),
        }
    }
}
impl DependencyStructRefMut for Dependency {
    fn client_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.client_ref_mut(),
        }
    }
    fn supplier_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.supplier_ref_mut(),
        }
    }
}
impl DependencyStructRef for Dependency {
    fn client_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.client_ref(),
        }
    }
    fn supplier_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.supplier_ref(),
        }
    }
}
impl<'a> DependencyStructRefMut for DependencyRefMut<'a> {
    fn client_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.client_ref_mut(),
        }
    }
    fn supplier_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.supplier_ref_mut(),
        }
    }
}
impl<'a> DependencyStructRef for DependencyRefMut<'a> {
    fn client_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.client_ref(),
        }
    }
    fn supplier_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.supplier_ref(),
        }
    }
}
impl<'a> DependencyStructRef for DependencyRef<'a> {
    fn client_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.client_ref(),
        }
    }
    fn supplier_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.supplier_ref(),
        }
    }
}
impl RelationshipStruct for Dependency {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Dependency::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Dependency {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Dependency::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Dependency {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Dependency::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Dependency::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for DependencyRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            DependencyRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for DependencyRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            DependencyRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for DependencyRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            DependencyRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            DependencyRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Dependency {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Dependency::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Dependency::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Dependency::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Dependency::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Dependency::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Dependency {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Dependency::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Dependency::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Dependency::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Dependency::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Dependency::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Dependency {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Dependency::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Dependency::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Dependency::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Dependency::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Dependency::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Dependency::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for DependencyRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            DependencyRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            DependencyRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            DependencyRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DependencyRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DependencyRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            DependencyRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for DependencyRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DependencyRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DependencyRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DependencyRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DependencyRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DependencyRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DependencyRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for DependencyRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DependencyRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DependencyRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DependencyRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DependencyRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DependencyRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DependencyRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DependencyRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl DependencyUpcast for Dependency {
    fn into_dependency(self) -> Dependency {
        self
    }
}
impl<'a> DependencyUpcastRefMut<'a> for DependencyRefMut<'a> {
    fn as_dependency_ref_mut(self) -> DependencyRefMut<'a> {
        self
    }
}
impl<'a> DependencyUpcastRef<'a> for DependencyRef<'a> {
    fn as_dependency_ref(self) -> DependencyRef<'a> {
        self
    }
}
impl RelationshipUpcast for Dependency {
    fn into_relationship(self) -> Relationship {
        Relationship::Dependency(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for DependencyRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Dependency(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for DependencyRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Dependency(self).as_relationship_ref()
    }
}
impl ElementUpcast for Dependency {
    fn into_element(self) -> Element {
        Relationship::Dependency(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for DependencyRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        RelationshipRefMut::Dependency(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for DependencyRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        RelationshipRef::Dependency(self).as_element_ref()
    }
}
pub trait DependencyDowncast {}
pub trait DependencyDowncastRefMut<'a> {}
pub trait DependencyDowncastRef<'a> {}
impl DependencyDowncast for Dependency {}
impl<'a> DependencyDowncastRefMut<'a> for DependencyRefMut<'a> {}
impl<'a> DependencyDowncastRef<'a> for DependencyRef<'a> {}
pub trait DependencyMethodsDescendants
where
    Self: DescendantOf<Dependency>,
    Self::Via: DependencyMethods,
    Self: Sized,
{}
pub trait DependencyMethods: Sized {}
impl<T: DependencyMethodsDescendants> DependencyMethods for T
where
    T::Via: DependencyMethods,
{}
impl DescendantOf<Relationship> for Dependency {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Dependency {}
impl DescendantOf<Element> for Dependency {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Dependency {}
pub trait DependencyRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<DependencyRefMut<'a>>,
    Self::Via: DependencyRefMutMethods,
    Self: Sized,
{}
pub trait DependencyRefMutMethods: Sized {}
impl<'a, T: DependencyRefMutMethodsDescendants<'a>> DependencyRefMutMethods for T
where
    T::Via: DependencyRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for DependencyRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for DependencyRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for DependencyRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for DependencyRefMut<'a> {}
pub trait DependencyRefMethodsDescendants<'a>
where
    Self: DescendantOf<DependencyRef<'a>>,
    Self::Via: DependencyRefMethods,
    Self: Sized,
{}
pub trait DependencyRefMethods: Sized {}
impl<'a, T: DependencyRefMethodsDescendants<'a>> DependencyRefMethods for T
where
    T::Via: DependencyRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for DependencyRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for DependencyRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for DependencyRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for DependencyRef<'a> {}

