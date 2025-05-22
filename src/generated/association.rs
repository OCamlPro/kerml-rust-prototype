#![allow(unused)]
use super::utils::DescendantOf;
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
use super::interaction::{Interaction, InteractionRefMut, InteractionRef};
use super::association_structure::{
    AssociationStructure, AssociationStructureRefMut, AssociationStructureRef,
};
pub struct AssociationInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) sup_classifier: ClassifierInner,
}
pub trait AssociationStruct
where
    Self: AssociationStructRefMut,
    Self: AssociationStructRef,
    Self: RelationshipStruct,
    Self: ClassifierStruct,
{}
pub trait AssociationStructRefMut
where
    Self: AssociationStructRef,
    Self: RelationshipStructRefMut,
    Self: ClassifierStructRefMut,
{}
pub trait AssociationStructRef
where
    Self: RelationshipStructRef,
    Self: ClassifierStructRef,
{}
pub trait AssociationUpcast: AssociationStruct {
    fn into_association(self) -> Association;
}
pub trait AssociationUpcastRefMut<'a>: AssociationStructRefMut {
    fn as_association_ref_mut(self) -> AssociationRefMut<'a>;
}
pub trait AssociationUpcastRef<'a>: AssociationStructRef {
    fn as_association_ref(self) -> AssociationRef<'a>;
}
impl AssociationStruct for AssociationInner {}
impl AssociationStructRefMut for AssociationInner {}
impl AssociationStructRef for AssociationInner {}
impl ClassifierStruct for AssociationInner {}
impl ClassifierStructRefMut for AssociationInner {}
impl ClassifierStructRef for AssociationInner {}
impl TypeStruct for AssociationInner {
    fn is_abstract(self) -> bool {
        self.sup_classifier.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_classifier.is_sufficient()
    }
}
impl TypeStructRefMut for AssociationInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for AssociationInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_classifier.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_classifier.is_sufficient_ref()
    }
}
impl NamespaceStruct for AssociationInner {}
impl NamespaceStructRefMut for AssociationInner {}
impl NamespaceStructRef for AssociationInner {}
impl ElementStruct for AssociationInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_classifier.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_classifier.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_classifier.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_classifier.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_classifier.is_implied_included()
    }
}
impl ElementStructRefMut for AssociationInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_classifier.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_classifier.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_classifier.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_classifier.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_classifier.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for AssociationInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_classifier.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_classifier.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_classifier.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_classifier.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_classifier.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_classifier.is_implied_included_ref()
    }
}
impl RelationshipStruct for AssociationInner {
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
impl RelationshipStructRefMut for AssociationInner {
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
impl RelationshipStructRef for AssociationInner {
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
pub enum Association {
    Itself(AssociationInner),
    Interaction(Interaction),
    AssociationStructure(AssociationStructure),
}
pub enum AssociationRefMut<'a> {
    Itself(&'a mut AssociationInner),
    Interaction(InteractionRefMut<'a>),
    AssociationStructure(AssociationStructureRefMut<'a>),
}
pub enum AssociationRef<'a> {
    Itself(&'a AssociationInner),
    Interaction(InteractionRef<'a>),
    AssociationStructure(AssociationStructureRef<'a>),
}
impl Association {
    pub fn as_ref(&self) -> AssociationRef {
        match self {
            Association::Itself(inner) => AssociationRef::Itself(&inner),
            Association::Interaction(inner) => {
                AssociationRef::Interaction(inner.as_ref())
            }
            Association::AssociationStructure(inner) => {
                AssociationRef::AssociationStructure(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> AssociationRefMut {
        match self {
            Association::Itself(inner) => AssociationRefMut::Itself(inner),
            Association::Interaction(inner) => {
                AssociationRefMut::Interaction(inner.as_ref_mut())
            }
            Association::AssociationStructure(inner) => {
                AssociationRefMut::AssociationStructure(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> AssociationRefMut<'a> {
    pub fn as_ref(self) -> AssociationRef<'a> {
        match self {
            AssociationRefMut::Itself(inner) => AssociationRef::Itself(inner),
            AssociationRefMut::Interaction(inner) => {
                AssociationRef::Interaction(inner.as_ref())
            }
            AssociationRefMut::AssociationStructure(inner) => {
                AssociationRef::AssociationStructure(inner.as_ref())
            }
        }
    }
}
impl AssociationStruct for Association {}
impl AssociationStructRefMut for Association {}
impl AssociationStructRef for Association {}
impl<'a> AssociationStructRefMut for AssociationRefMut<'a> {}
impl<'a> AssociationStructRef for AssociationRefMut<'a> {}
impl<'a> AssociationStructRef for AssociationRef<'a> {}
impl ClassifierStruct for Association {}
impl ClassifierStructRefMut for Association {}
impl ClassifierStructRef for Association {}
impl<'a> ClassifierStructRefMut for AssociationRefMut<'a> {}
impl<'a> ClassifierStructRef for AssociationRefMut<'a> {}
impl<'a> ClassifierStructRef for AssociationRef<'a> {}
impl TypeStruct for Association {
    fn is_abstract(self) -> bool {
        match self {
            Association::Itself(x) => x.is_abstract(),
            Association::Interaction(x) => x.is_abstract(),
            Association::AssociationStructure(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Association::Itself(x) => x.is_sufficient(),
            Association::Interaction(x) => x.is_sufficient(),
            Association::AssociationStructure(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Association {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Association::Itself(x) => x.is_abstract_ref_mut(),
            Association::Interaction(x) => x.is_abstract_ref_mut(),
            Association::AssociationStructure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Association::Itself(x) => x.is_sufficient_ref_mut(),
            Association::Interaction(x) => x.is_sufficient_ref_mut(),
            Association::AssociationStructure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Association {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Association::Itself(x) => x.is_abstract_ref(),
            Association::Interaction(x) => x.is_abstract_ref(),
            Association::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Association::Itself(x) => x.is_sufficient_ref(),
            Association::Interaction(x) => x.is_sufficient_ref(),
            Association::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for AssociationRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_abstract_ref_mut(),
            AssociationRefMut::Interaction(x) => x.is_abstract_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            AssociationRefMut::Interaction(x) => x.is_sufficient_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for AssociationRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_abstract_ref(),
            AssociationRefMut::Interaction(x) => x.is_abstract_ref(),
            AssociationRefMut::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_sufficient_ref(),
            AssociationRefMut::Interaction(x) => x.is_sufficient_ref(),
            AssociationRefMut::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for AssociationRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            AssociationRef::Itself(x) => x.is_abstract_ref(),
            AssociationRef::Interaction(x) => x.is_abstract_ref(),
            AssociationRef::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            AssociationRef::Itself(x) => x.is_sufficient_ref(),
            AssociationRef::Interaction(x) => x.is_sufficient_ref(),
            AssociationRef::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Association {}
impl NamespaceStructRefMut for Association {}
impl NamespaceStructRef for Association {}
impl<'a> NamespaceStructRefMut for AssociationRefMut<'a> {}
impl<'a> NamespaceStructRef for AssociationRefMut<'a> {}
impl<'a> NamespaceStructRef for AssociationRef<'a> {}
impl ElementStruct for Association {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Association::Itself(x) => x.owned_relationship(),
            Association::Interaction(x) => x.owned_relationship(),
            Association::AssociationStructure(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Association::Itself(x) => x.owning_relationship(),
            Association::Interaction(x) => x.owning_relationship(),
            Association::AssociationStructure(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Association::Itself(x) => x.element_id(),
            Association::Interaction(x) => x.element_id(),
            Association::AssociationStructure(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Association::Itself(x) => x.alias_ids(),
            Association::Interaction(x) => x.alias_ids(),
            Association::AssociationStructure(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Association::Itself(x) => x.declared_short_name(),
            Association::Interaction(x) => x.declared_short_name(),
            Association::AssociationStructure(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Association::Itself(x) => x.declared_name(),
            Association::Interaction(x) => x.declared_name(),
            Association::AssociationStructure(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Association::Itself(x) => x.is_implied_included(),
            Association::Interaction(x) => x.is_implied_included(),
            Association::AssociationStructure(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Association {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Association::Itself(x) => x.owned_relationship_ref_mut(),
            Association::Interaction(x) => x.owned_relationship_ref_mut(),
            Association::AssociationStructure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Association::Itself(x) => x.owning_relationship_ref_mut(),
            Association::Interaction(x) => x.owning_relationship_ref_mut(),
            Association::AssociationStructure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Association::Itself(x) => x.element_id_ref_mut(),
            Association::Interaction(x) => x.element_id_ref_mut(),
            Association::AssociationStructure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Association::Itself(x) => x.alias_ids_ref_mut(),
            Association::Interaction(x) => x.alias_ids_ref_mut(),
            Association::AssociationStructure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Association::Itself(x) => x.declared_short_name_ref_mut(),
            Association::Interaction(x) => x.declared_short_name_ref_mut(),
            Association::AssociationStructure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Association::Itself(x) => x.declared_name_ref_mut(),
            Association::Interaction(x) => x.declared_name_ref_mut(),
            Association::AssociationStructure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Association::Itself(x) => x.is_implied_included_ref_mut(),
            Association::Interaction(x) => x.is_implied_included_ref_mut(),
            Association::AssociationStructure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Association {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Association::Itself(x) => x.owned_relationship_ref(),
            Association::Interaction(x) => x.owned_relationship_ref(),
            Association::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Association::Itself(x) => x.owning_relationship_ref(),
            Association::Interaction(x) => x.owning_relationship_ref(),
            Association::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Association::Itself(x) => x.element_id_ref(),
            Association::Interaction(x) => x.element_id_ref(),
            Association::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Association::Itself(x) => x.alias_ids_ref(),
            Association::Interaction(x) => x.alias_ids_ref(),
            Association::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Association::Itself(x) => x.declared_short_name_ref(),
            Association::Interaction(x) => x.declared_short_name_ref(),
            Association::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Association::Itself(x) => x.declared_name_ref(),
            Association::Interaction(x) => x.declared_name_ref(),
            Association::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Association::Itself(x) => x.is_implied_included_ref(),
            Association::Interaction(x) => x.is_implied_included_ref(),
            Association::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for AssociationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            AssociationRefMut::Interaction(x) => x.owned_relationship_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AssociationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            AssociationRefMut::Interaction(x) => x.owning_relationship_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AssociationRefMut::Itself(x) => x.element_id_ref_mut(),
            AssociationRefMut::Interaction(x) => x.element_id_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AssociationRefMut::Itself(x) => x.alias_ids_ref_mut(),
            AssociationRefMut::Interaction(x) => x.alias_ids_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            AssociationRefMut::Interaction(x) => x.declared_short_name_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AssociationRefMut::Itself(x) => x.declared_name_ref_mut(),
            AssociationRefMut::Interaction(x) => x.declared_name_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            AssociationRefMut::Interaction(x) => x.is_implied_included_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for AssociationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owned_relationship_ref(),
            AssociationRefMut::Interaction(x) => x.owned_relationship_ref(),
            AssociationRefMut::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owning_relationship_ref(),
            AssociationRefMut::Interaction(x) => x.owning_relationship_ref(),
            AssociationRefMut::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AssociationRefMut::Itself(x) => x.element_id_ref(),
            AssociationRefMut::Interaction(x) => x.element_id_ref(),
            AssociationRefMut::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AssociationRefMut::Itself(x) => x.alias_ids_ref(),
            AssociationRefMut::Interaction(x) => x.alias_ids_ref(),
            AssociationRefMut::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AssociationRefMut::Itself(x) => x.declared_short_name_ref(),
            AssociationRefMut::Interaction(x) => x.declared_short_name_ref(),
            AssociationRefMut::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AssociationRefMut::Itself(x) => x.declared_name_ref(),
            AssociationRefMut::Interaction(x) => x.declared_name_ref(),
            AssociationRefMut::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_implied_included_ref(),
            AssociationRefMut::Interaction(x) => x.is_implied_included_ref(),
            AssociationRefMut::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for AssociationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationRef::Itself(x) => x.owned_relationship_ref(),
            AssociationRef::Interaction(x) => x.owned_relationship_ref(),
            AssociationRef::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AssociationRef::Itself(x) => x.owning_relationship_ref(),
            AssociationRef::Interaction(x) => x.owning_relationship_ref(),
            AssociationRef::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AssociationRef::Itself(x) => x.element_id_ref(),
            AssociationRef::Interaction(x) => x.element_id_ref(),
            AssociationRef::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AssociationRef::Itself(x) => x.alias_ids_ref(),
            AssociationRef::Interaction(x) => x.alias_ids_ref(),
            AssociationRef::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AssociationRef::Itself(x) => x.declared_short_name_ref(),
            AssociationRef::Interaction(x) => x.declared_short_name_ref(),
            AssociationRef::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AssociationRef::Itself(x) => x.declared_name_ref(),
            AssociationRef::Interaction(x) => x.declared_name_ref(),
            AssociationRef::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AssociationRef::Itself(x) => x.is_implied_included_ref(),
            AssociationRef::Interaction(x) => x.is_implied_included_ref(),
            AssociationRef::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for Association {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.target(),
            Association::Interaction(x) => x.target(),
            Association::AssociationStructure(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.source(),
            Association::Interaction(x) => x.source(),
            Association::AssociationStructure(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owning_related_element(),
            Association::Interaction(x) => x.owning_related_element(),
            Association::AssociationStructure(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owned_related_element(),
            Association::Interaction(x) => x.owned_related_element(),
            Association::AssociationStructure(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Association::Itself(x) => x.is_implied(),
            Association::Interaction(x) => x.is_implied(),
            Association::AssociationStructure(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Association {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.target_ref_mut(),
            Association::Interaction(x) => x.target_ref_mut(),
            Association::AssociationStructure(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.source_ref_mut(),
            Association::Interaction(x) => x.source_ref_mut(),
            Association::AssociationStructure(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owning_related_element_ref_mut(),
            Association::Interaction(x) => x.owning_related_element_ref_mut(),
            Association::AssociationStructure(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owned_related_element_ref_mut(),
            Association::Interaction(x) => x.owned_related_element_ref_mut(),
            Association::AssociationStructure(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Association::Itself(x) => x.is_implied_ref_mut(),
            Association::Interaction(x) => x.is_implied_ref_mut(),
            Association::AssociationStructure(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Association {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.target_ref(),
            Association::Interaction(x) => x.target_ref(),
            Association::AssociationStructure(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.source_ref(),
            Association::Interaction(x) => x.source_ref(),
            Association::AssociationStructure(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owning_related_element_ref(),
            Association::Interaction(x) => x.owning_related_element_ref(),
            Association::AssociationStructure(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Association::Itself(x) => x.owned_related_element_ref(),
            Association::Interaction(x) => x.owned_related_element_ref(),
            Association::AssociationStructure(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Association::Itself(x) => x.is_implied_ref(),
            Association::Interaction(x) => x.is_implied_ref(),
            Association::AssociationStructure(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for AssociationRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.target_ref_mut(),
            AssociationRefMut::Interaction(x) => x.target_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.source_ref_mut(),
            AssociationRefMut::Interaction(x) => x.source_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            AssociationRefMut::Interaction(x) => x.owning_related_element_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => {
                x.owning_related_element_ref_mut()
            }
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            AssociationRefMut::Interaction(x) => x.owned_related_element_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => {
                x.owned_related_element_ref_mut()
            }
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_implied_ref_mut(),
            AssociationRefMut::Interaction(x) => x.is_implied_ref_mut(),
            AssociationRefMut::AssociationStructure(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for AssociationRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.target_ref(),
            AssociationRefMut::Interaction(x) => x.target_ref(),
            AssociationRefMut::AssociationStructure(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.source_ref(),
            AssociationRefMut::Interaction(x) => x.source_ref(),
            AssociationRefMut::AssociationStructure(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owning_related_element_ref(),
            AssociationRefMut::Interaction(x) => x.owning_related_element_ref(),
            AssociationRefMut::AssociationStructure(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRefMut::Itself(x) => x.owned_related_element_ref(),
            AssociationRefMut::Interaction(x) => x.owned_related_element_ref(),
            AssociationRefMut::AssociationStructure(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AssociationRefMut::Itself(x) => x.is_implied_ref(),
            AssociationRefMut::Interaction(x) => x.is_implied_ref(),
            AssociationRefMut::AssociationStructure(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for AssociationRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRef::Itself(x) => x.target_ref(),
            AssociationRef::Interaction(x) => x.target_ref(),
            AssociationRef::AssociationStructure(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRef::Itself(x) => x.source_ref(),
            AssociationRef::Interaction(x) => x.source_ref(),
            AssociationRef::AssociationStructure(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRef::Itself(x) => x.owning_related_element_ref(),
            AssociationRef::Interaction(x) => x.owning_related_element_ref(),
            AssociationRef::AssociationStructure(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            AssociationRef::Itself(x) => x.owned_related_element_ref(),
            AssociationRef::Interaction(x) => x.owned_related_element_ref(),
            AssociationRef::AssociationStructure(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            AssociationRef::Itself(x) => x.is_implied_ref(),
            AssociationRef::Interaction(x) => x.is_implied_ref(),
            AssociationRef::AssociationStructure(x) => x.is_implied_ref(),
        }
    }
}
impl AssociationUpcast for Association {
    fn into_association(self) -> Association {
        self
    }
}
impl<'a> AssociationUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_association_ref_mut(self) -> AssociationRefMut<'a> {
        self
    }
}
impl<'a> AssociationUpcastRef<'a> for AssociationRef<'a> {
    fn as_association_ref(self) -> AssociationRef<'a> {
        self
    }
}
impl ClassifierUpcast for Association {
    fn into_classifier(self) -> Classifier {
        Classifier::Association(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        ClassifierRefMut::Association(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for AssociationRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        ClassifierRef::Association(self).as_classifier_ref()
    }
}
impl TypeUpcast for Association {
    fn into_type_(self) -> Type {
        Classifier::Association(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ClassifierRefMut::Association(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for AssociationRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ClassifierRef::Association(self).as_type_ref()
    }
}
impl NamespaceUpcast for Association {
    fn into_namespace(self) -> Namespace {
        Classifier::Association(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ClassifierRefMut::Association(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for AssociationRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ClassifierRef::Association(self).as_namespace_ref()
    }
}
impl ElementUpcast for Association {
    fn into_element(self) -> Element {
        Classifier::Association(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ClassifierRefMut::Association(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for AssociationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ClassifierRef::Association(self).as_element_ref()
    }
}
impl RelationshipUpcast for Association {
    fn into_relationship(self) -> Relationship {
        Relationship::Association(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for AssociationRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Association(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for AssociationRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Association(self).as_relationship_ref()
    }
}
pub trait AssociationDowncast {
    fn try_into_interaction(self) -> Result<Interaction, String>;
    fn try_into_association_structure(self) -> Result<AssociationStructure, String>;
}
pub trait AssociationDowncastRefMut<'a> {
    fn try_as_interaction_ref_mut(self) -> Result<InteractionRefMut<'a>, String>;
    fn try_as_association_structure_ref_mut(
        self,
    ) -> Result<AssociationStructureRefMut<'a>, String>;
}
pub trait AssociationDowncastRef<'a> {
    fn try_as_interaction_ref(self) -> Result<InteractionRef<'a>, String>;
    fn try_as_association_structure_ref(
        self,
    ) -> Result<AssociationStructureRef<'a>, String>;
}
impl AssociationDowncast for Association {
    fn try_into_interaction(self) -> Result<Interaction, String> {
        match self {
            Association::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
    fn try_into_association_structure(self) -> Result<AssociationStructure, String> {
        match self {
            Association::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
impl<'a> AssociationDowncastRefMut<'a> for AssociationRefMut<'a> {
    fn try_as_interaction_ref_mut(self) -> Result<InteractionRefMut<'a>, String> {
        match self {
            AssociationRefMut::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
    fn try_as_association_structure_ref_mut(
        self,
    ) -> Result<AssociationStructureRefMut<'a>, String> {
        match self {
            AssociationRefMut::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
impl<'a> AssociationDowncastRef<'a> for AssociationRef<'a> {
    fn try_as_interaction_ref(self) -> Result<InteractionRef<'a>, String> {
        match self {
            AssociationRef::Interaction(e) => Ok(e),
            _ => Err("Not a Interaction".into()),
        }
    }
    fn try_as_association_structure_ref(
        self,
    ) -> Result<AssociationStructureRef<'a>, String> {
        match self {
            AssociationRef::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
pub trait AssociationMethodsDescendants
where
    Self: DescendantOf<Association>,
    Self::Via: AssociationMethods,
    Self: Sized,
{}
pub trait AssociationMethods: Sized {}
impl<T: AssociationMethodsDescendants> AssociationMethods for T
where
    T::Via: AssociationMethods,
{}
impl DescendantOf<Relationship> for Association {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Association {}
impl DescendantOf<Element> for Association {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Association {}
impl DescendantOf<Classifier> for Association {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl ClassifierMethodsDescendants for Association {}
impl DescendantOf<Type> for Association {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl TypeMethodsDescendants for Association {}
impl DescendantOf<Namespace> for Association {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl NamespaceMethodsDescendants for Association {}
pub trait AssociationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<AssociationRefMut<'a>>,
    Self::Via: AssociationRefMutMethods,
    Self: Sized,
{}
pub trait AssociationRefMutMethods: Sized {}
impl<'a, T: AssociationRefMutMethodsDescendants<'a>> AssociationRefMutMethods for T
where
    T::Via: AssociationRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for AssociationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for AssociationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for AssociationRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for AssociationRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for AssociationRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for AssociationRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for AssociationRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for AssociationRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for AssociationRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for AssociationRefMut<'a> {}
pub trait AssociationRefMethodsDescendants<'a>
where
    Self: DescendantOf<AssociationRef<'a>>,
    Self::Via: AssociationRefMethods,
    Self: Sized,
{}
pub trait AssociationRefMethods: Sized {}
impl<'a, T: AssociationRefMethodsDescendants<'a>> AssociationRefMethods for T
where
    T::Via: AssociationRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for AssociationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for AssociationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for AssociationRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for AssociationRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for AssociationRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for AssociationRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for AssociationRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for AssociationRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for AssociationRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for AssociationRef<'a> {}

