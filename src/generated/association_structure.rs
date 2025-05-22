#![allow(unused)]
use super::utils::DescendantOf;
use super::association::{
    Association, AssociationRefMut, AssociationRef, AssociationStruct,
    AssociationStructRefMut, AssociationStructRef, AssociationInner, AssociationUpcast,
    AssociationUpcastRefMut, AssociationUpcastRef, AssociationMethodsDescendants,
    AssociationRefMutMethodsDescendants, AssociationRefMethodsDescendants,
};
use super::classifier::{
    Classifier, ClassifierRefMut, ClassifierRef, ClassifierStruct,
    ClassifierStructRefMut, ClassifierStructRef, ClassifierInner, ClassifierUpcast,
    ClassifierUpcastRefMut, ClassifierUpcastRef, ClassifierMethodsDescendants,
    ClassifierRefMutMethodsDescendants, ClassifierRefMethodsDescendants,
};
use super::type_::{
    Type, TypeRefMut, TypeRef, TypeStruct, TypeStructRefMut, TypeStructRef, TypeInner,
    TypeUpcast, TypeUpcastRefMut, TypeUpcastRef, TypeMethodsDescendants,
    TypeRefMutMethodsDescendants, TypeRefMethodsDescendants,
};
use super::namespace::{
    Namespace, NamespaceRefMut, NamespaceRef, NamespaceStruct, NamespaceStructRefMut,
    NamespaceStructRef, NamespaceInner, NamespaceUpcast, NamespaceUpcastRefMut,
    NamespaceUpcastRef, NamespaceMethodsDescendants, NamespaceRefMutMethodsDescendants,
    NamespaceRefMethodsDescendants,
};
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
use super::relationship::{
    Relationship, RelationshipRefMut, RelationshipRef, RelationshipStruct,
    RelationshipStructRefMut, RelationshipStructRef, RelationshipInner,
    RelationshipUpcast, RelationshipUpcastRefMut, RelationshipUpcastRef,
    RelationshipMethodsDescendants, RelationshipRefMutMethodsDescendants,
    RelationshipRefMethodsDescendants,
};
use super::structure::{
    Structure, StructureRefMut, StructureRef, StructureStruct, StructureStructRefMut,
    StructureStructRef, StructureInner, StructureUpcast, StructureUpcastRefMut,
    StructureUpcastRef, StructureMethodsDescendants, StructureRefMutMethodsDescendants,
    StructureRefMethodsDescendants,
};
use super::class::{
    Class, ClassRefMut, ClassRef, ClassStruct, ClassStructRefMut, ClassStructRef,
    ClassInner, ClassUpcast, ClassUpcastRefMut, ClassUpcastRef, ClassMethodsDescendants,
    ClassRefMutMethodsDescendants, ClassRefMethodsDescendants,
};
pub struct AssociationStructureInner {
    pub(super) sup_structure: StructureInner,
    pub(super) sup_association: AssociationInner,
}
pub trait AssociationStructureStruct
where
    Self: AssociationStructureStructRefMut,
    Self: AssociationStructureStructRef,
    Self: StructureStruct,
    Self: AssociationStruct,
{}
pub trait AssociationStructureStructRefMut
where
    Self: AssociationStructureStructRef,
    Self: StructureStructRefMut,
    Self: AssociationStructRefMut,
{}
pub trait AssociationStructureStructRef
where
    Self: StructureStructRef,
    Self: AssociationStructRef,
{}
pub trait AssociationStructureUpcast: AssociationStructureStruct {
    fn into_association_structure(self) -> AssociationStructure;
}
pub trait AssociationStructureUpcastRefMut<'a>: AssociationStructureStructRefMut {
    fn as_association_structure_ref_mut(self) -> AssociationStructureRefMut<'a>;
}
pub trait AssociationStructureUpcastRef<'a>: AssociationStructureStructRef {
    fn as_association_structure_ref(self) -> AssociationStructureRef<'a>;
}
impl AssociationStructureStruct for AssociationStructureInner {}
impl AssociationStructureStructRefMut for AssociationStructureInner {}
impl AssociationStructureStructRef for AssociationStructureInner {}
impl AssociationStruct for AssociationStructureInner {}
impl AssociationStructRefMut for AssociationStructureInner {}
impl AssociationStructRef for AssociationStructureInner {}
impl ClassifierStruct for AssociationStructureInner {}
impl ClassifierStructRefMut for AssociationStructureInner {}
impl ClassifierStructRef for AssociationStructureInner {}
impl TypeStruct for AssociationStructureInner {
    fn is_abstract(self) -> bool {
        self.sup_association.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_association.is_sufficient()
    }
}
impl TypeStructRefMut for AssociationStructureInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for AssociationStructureInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_association.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_association.is_sufficient_ref()
    }
}
impl NamespaceStruct for AssociationStructureInner {}
impl NamespaceStructRefMut for AssociationStructureInner {}
impl NamespaceStructRef for AssociationStructureInner {}
impl ElementStruct for AssociationStructureInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_association.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_association.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_association.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_association.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_association.is_implied_included()
    }
}
impl ElementStructRefMut for AssociationStructureInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_association.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_association.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_association.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_association.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_association.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for AssociationStructureInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_association.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_association.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_association.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_association.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_association.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_association.is_implied_included_ref()
    }
}
impl RelationshipStruct for AssociationStructureInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_association.is_implied()
    }
}
impl RelationshipStructRefMut for AssociationStructureInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_association.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for AssociationStructureInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_association.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_association.is_implied_ref()
    }
}
impl StructureStruct for AssociationStructureInner {}
impl StructureStructRefMut for AssociationStructureInner {}
impl StructureStructRef for AssociationStructureInner {}
impl ClassStruct for AssociationStructureInner {}
impl ClassStructRefMut for AssociationStructureInner {}
impl ClassStructRef for AssociationStructureInner {}
pub enum AssociationStructure {
    Itself(AssociationStructureInner),
}
pub enum AssociationStructureRefMut<'a> {
    Itself(&'a mut AssociationStructureInner),
}
pub enum AssociationStructureRef<'a> {
    Itself(&'a AssociationStructureInner),
}
impl AssociationStructure {
    pub fn as_ref(&self) -> AssociationStructureRef {
        match self {
            AssociationStructure::Itself(inner) => {
                AssociationStructureRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> AssociationStructureRefMut {
        match self {
            AssociationStructure::Itself(inner) => {
                AssociationStructureRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> AssociationStructureRefMut<'a> {
    pub fn as_ref(self) -> AssociationStructureRef<'a> {
        match self {
            AssociationStructureRefMut::Itself(inner) => {
                AssociationStructureRef::Itself(inner)
            }
        }
    }
}
impl AssociationStructureStruct for AssociationStructure {}
impl AssociationStructureStructRefMut for AssociationStructure {}
impl AssociationStructureStructRef for AssociationStructure {}
impl<'a> AssociationStructureStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> AssociationStructureStructRef for AssociationStructureRefMut<'a> {}
impl<'a> AssociationStructureStructRef for AssociationStructureRef<'a> {}
impl AssociationStruct for AssociationStructure {}
impl AssociationStructRefMut for AssociationStructure {}
impl AssociationStructRef for AssociationStructure {}
impl<'a> AssociationStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> AssociationStructRef for AssociationStructureRefMut<'a> {}
impl<'a> AssociationStructRef for AssociationStructureRef<'a> {}
impl ClassifierStruct for AssociationStructure {}
impl ClassifierStructRefMut for AssociationStructure {}
impl ClassifierStructRef for AssociationStructure {}
impl<'a> ClassifierStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> ClassifierStructRef for AssociationStructureRefMut<'a> {}
impl<'a> ClassifierStructRef for AssociationStructureRef<'a> {}
impl TypeStruct for AssociationStructure {
    fn is_abstract(self) -> bool {
        match self {
            AssociationStructure::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            AssociationStructure::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for AssociationStructure {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructure::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructure::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for AssociationStructure {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            AssociationStructure::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            AssociationStructure::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for AssociationStructureRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for AssociationStructureRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for AssociationStructureRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            AssociationStructureRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            AssociationStructureRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for AssociationStructure {}
impl NamespaceStructRefMut for AssociationStructure {}
impl NamespaceStructRef for AssociationStructure {}
impl<'a> NamespaceStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> NamespaceStructRef for AssociationStructureRefMut<'a> {}
impl<'a> NamespaceStructRef for AssociationStructureRef<'a> {}
impl ElementStruct for AssociationStructure {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructure::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            AssociationStructure::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            AssociationStructure::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for AssociationStructure {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AssociationStructure::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AssociationStructure::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AssociationStructure::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for AssociationStructure {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructure::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AssociationStructure::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AssociationStructure::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructure::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for AssociationStructureRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AssociationStructureRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for AssociationStructureRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AssociationStructureRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for AssociationStructureRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AssociationStructureRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AssociationStructureRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructureRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AssociationStructureRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AssociationStructureRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for AssociationStructure {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for AssociationStructure {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for AssociationStructure {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructure::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AssociationStructure::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for AssociationStructureRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for AssociationStructureRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AssociationStructureRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for AssociationStructureRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationStructureRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AssociationStructureRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl StructureStruct for AssociationStructure {}
impl StructureStructRefMut for AssociationStructure {}
impl StructureStructRef for AssociationStructure {}
impl<'a> StructureStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> StructureStructRef for AssociationStructureRefMut<'a> {}
impl<'a> StructureStructRef for AssociationStructureRef<'a> {}
impl ClassStruct for AssociationStructure {}
impl ClassStructRefMut for AssociationStructure {}
impl ClassStructRef for AssociationStructure {}
impl<'a> ClassStructRefMut for AssociationStructureRefMut<'a> {}
impl<'a> ClassStructRef for AssociationStructureRefMut<'a> {}
impl<'a> ClassStructRef for AssociationStructureRef<'a> {}
impl AssociationStructureUpcast for AssociationStructure {
    fn into_association_structure(self) -> AssociationStructure {
        self
    }
}
impl<'a> AssociationStructureUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_association_structure_ref_mut(self) -> AssociationStructureRefMut<'a> {
        self
    }
}
impl<'a> AssociationStructureUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_association_structure_ref(self) -> AssociationStructureRef<'a> {
        self
    }
}
impl AssociationUpcast for AssociationStructure {
    fn into_association(self) -> Association {
        Association::AssociationStructure(self).into_association()
    }
}
impl<'a> AssociationUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_association_ref_mut(self) -> AssociationRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_association_ref_mut()
    }
}
impl<'a> AssociationUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_association_ref(self) -> AssociationRef<'a> {
        AssociationRef::AssociationStructure(self).as_association_ref()
    }
}
impl ClassifierUpcast for AssociationStructure {
    fn into_classifier(self) -> Classifier {
        Association::AssociationStructure(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        AssociationRef::AssociationStructure(self).as_classifier_ref()
    }
}
impl TypeUpcast for AssociationStructure {
    fn into_type_(self) -> Type {
        Association::AssociationStructure(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        AssociationRef::AssociationStructure(self).as_type_ref()
    }
}
impl NamespaceUpcast for AssociationStructure {
    fn into_namespace(self) -> Namespace {
        Association::AssociationStructure(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        AssociationRef::AssociationStructure(self).as_namespace_ref()
    }
}
impl ElementUpcast for AssociationStructure {
    fn into_element(self) -> Element {
        Association::AssociationStructure(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        AssociationRef::AssociationStructure(self).as_element_ref()
    }
}
impl RelationshipUpcast for AssociationStructure {
    fn into_relationship(self) -> Relationship {
        Association::AssociationStructure(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        AssociationRefMut::AssociationStructure(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        AssociationRef::AssociationStructure(self).as_relationship_ref()
    }
}
impl StructureUpcast for AssociationStructure {
    fn into_structure(self) -> Structure {
        Structure::AssociationStructure(self).into_structure()
    }
}
impl<'a> StructureUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_structure_ref_mut(self) -> StructureRefMut<'a> {
        StructureRefMut::AssociationStructure(self).as_structure_ref_mut()
    }
}
impl<'a> StructureUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_structure_ref(self) -> StructureRef<'a> {
        StructureRef::AssociationStructure(self).as_structure_ref()
    }
}
impl ClassUpcast for AssociationStructure {
    fn into_class(self) -> Class {
        Structure::AssociationStructure(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for AssociationStructureRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        StructureRefMut::AssociationStructure(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for AssociationStructureRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        StructureRef::AssociationStructure(self).as_class_ref()
    }
}
pub trait AssociationStructureDowncast {}
pub trait AssociationStructureDowncastRefMut<'a> {}
pub trait AssociationStructureDowncastRef<'a> {}
impl AssociationStructureDowncast for AssociationStructure {}
impl<'a> AssociationStructureDowncastRefMut<'a> for AssociationStructureRefMut<'a> {}
impl<'a> AssociationStructureDowncastRef<'a> for AssociationStructureRef<'a> {}
pub trait AssociationStructureMethodsDescendants
where
    Self: DescendantOf<AssociationStructure>,
    Self::Via: AssociationStructureMethods,
    Self: Sized,
{}
pub trait AssociationStructureMethods: Sized {}
impl<T: AssociationStructureMethodsDescendants> AssociationStructureMethods for T
where
    T::Via: AssociationStructureMethods,
{}
impl DescendantOf<Structure> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl StructureMethodsDescendants for AssociationStructure {}
impl DescendantOf<Class> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ClassMethodsDescendants for AssociationStructure {}
impl DescendantOf<Classifier> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ClassifierMethodsDescendants for AssociationStructure {}
impl DescendantOf<Type> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl TypeMethodsDescendants for AssociationStructure {}
impl DescendantOf<Namespace> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl NamespaceMethodsDescendants for AssociationStructure {}
impl DescendantOf<Element> for AssociationStructure {
    type Via = Structure;
    fn into_via(self) -> Self::Via {
        self.into_structure()
    }
}
impl ElementMethodsDescendants for AssociationStructure {}
impl DescendantOf<Association> for AssociationStructure {
    type Via = Association;
    fn into_via(self) -> Self::Via {
        self.into_association()
    }
}
impl AssociationMethodsDescendants for AssociationStructure {}
impl DescendantOf<Relationship> for AssociationStructure {
    type Via = Association;
    fn into_via(self) -> Self::Via {
        self.into_association()
    }
}
impl RelationshipMethodsDescendants for AssociationStructure {}
pub trait AssociationStructureRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<AssociationStructureRefMut<'a>>,
    Self::Via: AssociationStructureRefMutMethods,
    Self: Sized,
{}
pub trait AssociationStructureRefMutMethods: Sized {}
impl<
    'a,
    T: AssociationStructureRefMutMethodsDescendants<'a>,
> AssociationStructureRefMutMethods for T
where
    T::Via: AssociationStructureRefMutMethods,
{}
impl<'a> DescendantOf<StructureRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> StructureRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<ClassRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = StructureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<AssociationRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = AssociationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref_mut()
    }
}
impl<'a> AssociationRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for AssociationStructureRefMut<'a> {
    type Via = AssociationRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for AssociationStructureRefMut<'a> {}
pub trait AssociationStructureRefMethodsDescendants<'a>
where
    Self: DescendantOf<AssociationStructureRef<'a>>,
    Self::Via: AssociationStructureRefMethods,
    Self: Sized,
{}
pub trait AssociationStructureRefMethods: Sized {}
impl<'a, T: AssociationStructureRefMethodsDescendants<'a>> AssociationStructureRefMethods
for T
where
    T::Via: AssociationStructureRefMethods,
{}
impl<'a> DescendantOf<StructureRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> StructureRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<ClassRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for AssociationStructureRef<'a> {
    type Via = StructureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_structure_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<AssociationRef<'a>> for AssociationStructureRef<'a> {
    type Via = AssociationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref()
    }
}
impl<'a> AssociationRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for AssociationStructureRef<'a> {
    type Via = AssociationRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_association_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for AssociationStructureRef<'a> {}

