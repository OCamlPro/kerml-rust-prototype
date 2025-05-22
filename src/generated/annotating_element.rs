#![allow(unused)]
use super::utils::DescendantOf;
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
use super::metadata_feature::{
    MetadataFeature, MetadataFeatureRefMut, MetadataFeatureRef,
};
use super::textual_representation::{
    TextualRepresentation, TextualRepresentationRefMut, TextualRepresentationRef,
};
use super::comment::{Comment, CommentRefMut, CommentRef};
pub struct AnnotatingElementInner {
    pub(super) sup_element: ElementInner,
}
pub trait AnnotatingElementStruct
where
    Self: AnnotatingElementStructRefMut,
    Self: AnnotatingElementStructRef,
    Self: ElementStruct,
{}
pub trait AnnotatingElementStructRefMut
where
    Self: AnnotatingElementStructRef,
    Self: ElementStructRefMut,
{}
pub trait AnnotatingElementStructRef
where
    Self: ElementStructRef,
{}
pub trait AnnotatingElementUpcast: AnnotatingElementStruct {
    fn into_annotating_element(self) -> AnnotatingElement;
}
pub trait AnnotatingElementUpcastRefMut<'a>: AnnotatingElementStructRefMut {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a>;
}
pub trait AnnotatingElementUpcastRef<'a>: AnnotatingElementStructRef {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a>;
}
impl AnnotatingElementStruct for AnnotatingElementInner {}
impl AnnotatingElementStructRefMut for AnnotatingElementInner {}
impl AnnotatingElementStructRef for AnnotatingElementInner {}
impl ElementStruct for AnnotatingElementInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_element.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_element.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_element.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_element.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_element.is_implied_included()
    }
}
impl ElementStructRefMut for AnnotatingElementInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_element.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_element.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_element.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_element.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for AnnotatingElementInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_element.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_element.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_element.is_implied_included_ref()
    }
}
pub enum AnnotatingElement {
    Itself(AnnotatingElementInner),
    MetadataFeature(MetadataFeature),
    TextualRepresentation(TextualRepresentation),
    Comment(Comment),
}
pub enum AnnotatingElementRefMut<'a> {
    Itself(&'a mut AnnotatingElementInner),
    MetadataFeature(MetadataFeatureRefMut<'a>),
    TextualRepresentation(TextualRepresentationRefMut<'a>),
    Comment(CommentRefMut<'a>),
}
pub enum AnnotatingElementRef<'a> {
    Itself(&'a AnnotatingElementInner),
    MetadataFeature(MetadataFeatureRef<'a>),
    TextualRepresentation(TextualRepresentationRef<'a>),
    Comment(CommentRef<'a>),
}
impl AnnotatingElement {
    pub fn as_ref(&self) -> AnnotatingElementRef {
        match self {
            AnnotatingElement::Itself(inner) => AnnotatingElementRef::Itself(&inner),
            AnnotatingElement::MetadataFeature(inner) => {
                AnnotatingElementRef::MetadataFeature(inner.as_ref())
            }
            AnnotatingElement::TextualRepresentation(inner) => {
                AnnotatingElementRef::TextualRepresentation(inner.as_ref())
            }
            AnnotatingElement::Comment(inner) => {
                AnnotatingElementRef::Comment(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> AnnotatingElementRefMut {
        match self {
            AnnotatingElement::Itself(inner) => AnnotatingElementRefMut::Itself(inner),
            AnnotatingElement::MetadataFeature(inner) => {
                AnnotatingElementRefMut::MetadataFeature(inner.as_ref_mut())
            }
            AnnotatingElement::TextualRepresentation(inner) => {
                AnnotatingElementRefMut::TextualRepresentation(inner.as_ref_mut())
            }
            AnnotatingElement::Comment(inner) => {
                AnnotatingElementRefMut::Comment(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> AnnotatingElementRefMut<'a> {
    pub fn as_ref(self) -> AnnotatingElementRef<'a> {
        match self {
            AnnotatingElementRefMut::Itself(inner) => AnnotatingElementRef::Itself(inner),
            AnnotatingElementRefMut::MetadataFeature(inner) => {
                AnnotatingElementRef::MetadataFeature(inner.as_ref())
            }
            AnnotatingElementRefMut::TextualRepresentation(inner) => {
                AnnotatingElementRef::TextualRepresentation(inner.as_ref())
            }
            AnnotatingElementRefMut::Comment(inner) => {
                AnnotatingElementRef::Comment(inner.as_ref())
            }
        }
    }
}
impl AnnotatingElementStruct for AnnotatingElement {}
impl AnnotatingElementStructRefMut for AnnotatingElement {}
impl AnnotatingElementStructRef for AnnotatingElement {}
impl<'a> AnnotatingElementStructRefMut for AnnotatingElementRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for AnnotatingElementRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for AnnotatingElementRef<'a> {}
impl ElementStruct for AnnotatingElement {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElement::Itself(x) => x.owned_relationship(),
            AnnotatingElement::MetadataFeature(x) => x.owned_relationship(),
            AnnotatingElement::TextualRepresentation(x) => x.owned_relationship(),
            AnnotatingElement::Comment(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElement::Itself(x) => x.owning_relationship(),
            AnnotatingElement::MetadataFeature(x) => x.owning_relationship(),
            AnnotatingElement::TextualRepresentation(x) => x.owning_relationship(),
            AnnotatingElement::Comment(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            AnnotatingElement::Itself(x) => x.element_id(),
            AnnotatingElement::MetadataFeature(x) => x.element_id(),
            AnnotatingElement::TextualRepresentation(x) => x.element_id(),
            AnnotatingElement::Comment(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            AnnotatingElement::Itself(x) => x.alias_ids(),
            AnnotatingElement::MetadataFeature(x) => x.alias_ids(),
            AnnotatingElement::TextualRepresentation(x) => x.alias_ids(),
            AnnotatingElement::Comment(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_short_name(),
            AnnotatingElement::MetadataFeature(x) => x.declared_short_name(),
            AnnotatingElement::TextualRepresentation(x) => x.declared_short_name(),
            AnnotatingElement::Comment(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_name(),
            AnnotatingElement::MetadataFeature(x) => x.declared_name(),
            AnnotatingElement::TextualRepresentation(x) => x.declared_name(),
            AnnotatingElement::Comment(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            AnnotatingElement::Itself(x) => x.is_implied_included(),
            AnnotatingElement::MetadataFeature(x) => x.is_implied_included(),
            AnnotatingElement::TextualRepresentation(x) => x.is_implied_included(),
            AnnotatingElement::Comment(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for AnnotatingElement {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElement::Itself(x) => x.owned_relationship_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.owned_relationship_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => x.owned_relationship_ref_mut(),
            AnnotatingElement::Comment(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AnnotatingElement::Itself(x) => x.owning_relationship_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.owning_relationship_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => {
                x.owning_relationship_ref_mut()
            }
            AnnotatingElement::Comment(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AnnotatingElement::Itself(x) => x.element_id_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.element_id_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => x.element_id_ref_mut(),
            AnnotatingElement::Comment(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AnnotatingElement::Itself(x) => x.alias_ids_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.alias_ids_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => x.alias_ids_ref_mut(),
            AnnotatingElement::Comment(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_short_name_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.declared_short_name_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => {
                x.declared_short_name_ref_mut()
            }
            AnnotatingElement::Comment(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_name_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.declared_name_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => x.declared_name_ref_mut(),
            AnnotatingElement::Comment(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AnnotatingElement::Itself(x) => x.is_implied_included_ref_mut(),
            AnnotatingElement::MetadataFeature(x) => x.is_implied_included_ref_mut(),
            AnnotatingElement::TextualRepresentation(x) => {
                x.is_implied_included_ref_mut()
            }
            AnnotatingElement::Comment(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for AnnotatingElement {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElement::Itself(x) => x.owned_relationship_ref(),
            AnnotatingElement::MetadataFeature(x) => x.owned_relationship_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.owned_relationship_ref(),
            AnnotatingElement::Comment(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElement::Itself(x) => x.owning_relationship_ref(),
            AnnotatingElement::MetadataFeature(x) => x.owning_relationship_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.owning_relationship_ref(),
            AnnotatingElement::Comment(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AnnotatingElement::Itself(x) => x.element_id_ref(),
            AnnotatingElement::MetadataFeature(x) => x.element_id_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.element_id_ref(),
            AnnotatingElement::Comment(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AnnotatingElement::Itself(x) => x.alias_ids_ref(),
            AnnotatingElement::MetadataFeature(x) => x.alias_ids_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.alias_ids_ref(),
            AnnotatingElement::Comment(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_short_name_ref(),
            AnnotatingElement::MetadataFeature(x) => x.declared_short_name_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.declared_short_name_ref(),
            AnnotatingElement::Comment(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElement::Itself(x) => x.declared_name_ref(),
            AnnotatingElement::MetadataFeature(x) => x.declared_name_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.declared_name_ref(),
            AnnotatingElement::Comment(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AnnotatingElement::Itself(x) => x.is_implied_included_ref(),
            AnnotatingElement::MetadataFeature(x) => x.is_implied_included_ref(),
            AnnotatingElement::TextualRepresentation(x) => x.is_implied_included_ref(),
            AnnotatingElement::Comment(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for AnnotatingElementRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.owned_relationship_ref_mut(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.owned_relationship_ref_mut()
            }
            AnnotatingElementRefMut::Comment(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => {
                x.owning_relationship_ref_mut()
            }
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.owning_relationship_ref_mut()
            }
            AnnotatingElementRefMut::Comment(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.element_id_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.element_id_ref_mut(),
            AnnotatingElementRefMut::TextualRepresentation(x) => x.element_id_ref_mut(),
            AnnotatingElementRefMut::Comment(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.alias_ids_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.alias_ids_ref_mut(),
            AnnotatingElementRefMut::TextualRepresentation(x) => x.alias_ids_ref_mut(),
            AnnotatingElementRefMut::Comment(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => {
                x.declared_short_name_ref_mut()
            }
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.declared_short_name_ref_mut()
            }
            AnnotatingElementRefMut::Comment(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.declared_name_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.declared_name_ref_mut(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.declared_name_ref_mut()
            }
            AnnotatingElementRefMut::Comment(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            AnnotatingElementRefMut::MetadataFeature(x) => {
                x.is_implied_included_ref_mut()
            }
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.is_implied_included_ref_mut()
            }
            AnnotatingElementRefMut::Comment(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for AnnotatingElementRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.owned_relationship_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.owned_relationship_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.owned_relationship_ref()
            }
            AnnotatingElementRefMut::Comment(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.owning_relationship_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.owning_relationship_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.owning_relationship_ref()
            }
            AnnotatingElementRefMut::Comment(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.element_id_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.element_id_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => x.element_id_ref(),
            AnnotatingElementRefMut::Comment(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.alias_ids_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.alias_ids_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => x.alias_ids_ref(),
            AnnotatingElementRefMut::Comment(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.declared_short_name_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.declared_short_name_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.declared_short_name_ref()
            }
            AnnotatingElementRefMut::Comment(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.declared_name_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.declared_name_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => x.declared_name_ref(),
            AnnotatingElementRefMut::Comment(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AnnotatingElementRefMut::Itself(x) => x.is_implied_included_ref(),
            AnnotatingElementRefMut::MetadataFeature(x) => x.is_implied_included_ref(),
            AnnotatingElementRefMut::TextualRepresentation(x) => {
                x.is_implied_included_ref()
            }
            AnnotatingElementRefMut::Comment(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for AnnotatingElementRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElementRef::Itself(x) => x.owned_relationship_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.owned_relationship_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.owned_relationship_ref(),
            AnnotatingElementRef::Comment(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            AnnotatingElementRef::Itself(x) => x.owning_relationship_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.owning_relationship_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.owning_relationship_ref(),
            AnnotatingElementRef::Comment(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            AnnotatingElementRef::Itself(x) => x.element_id_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.element_id_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.element_id_ref(),
            AnnotatingElementRef::Comment(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            AnnotatingElementRef::Itself(x) => x.alias_ids_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.alias_ids_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.alias_ids_ref(),
            AnnotatingElementRef::Comment(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElementRef::Itself(x) => x.declared_short_name_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.declared_short_name_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.declared_short_name_ref(),
            AnnotatingElementRef::Comment(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            AnnotatingElementRef::Itself(x) => x.declared_name_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.declared_name_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.declared_name_ref(),
            AnnotatingElementRef::Comment(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            AnnotatingElementRef::Itself(x) => x.is_implied_included_ref(),
            AnnotatingElementRef::MetadataFeature(x) => x.is_implied_included_ref(),
            AnnotatingElementRef::TextualRepresentation(x) => x.is_implied_included_ref(),
            AnnotatingElementRef::Comment(x) => x.is_implied_included_ref(),
        }
    }
}
impl AnnotatingElementUpcast for AnnotatingElement {
    fn into_annotating_element(self) -> AnnotatingElement {
        self
    }
}
impl<'a> AnnotatingElementUpcastRefMut<'a> for AnnotatingElementRefMut<'a> {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a> {
        self
    }
}
impl<'a> AnnotatingElementUpcastRef<'a> for AnnotatingElementRef<'a> {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a> {
        self
    }
}
impl ElementUpcast for AnnotatingElement {
    fn into_element(self) -> Element {
        Element::AnnotatingElement(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for AnnotatingElementRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ElementRefMut::AnnotatingElement(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for AnnotatingElementRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ElementRef::AnnotatingElement(self).as_element_ref()
    }
}
pub trait AnnotatingElementDowncast {
    fn try_into_metadata_feature(self) -> Result<MetadataFeature, String>;
    fn try_into_textual_representation(self) -> Result<TextualRepresentation, String>;
    fn try_into_comment(self) -> Result<Comment, String>;
}
pub trait AnnotatingElementDowncastRefMut<'a> {
    fn try_as_metadata_feature_ref_mut(
        self,
    ) -> Result<MetadataFeatureRefMut<'a>, String>;
    fn try_as_textual_representation_ref_mut(
        self,
    ) -> Result<TextualRepresentationRefMut<'a>, String>;
    fn try_as_comment_ref_mut(self) -> Result<CommentRefMut<'a>, String>;
}
pub trait AnnotatingElementDowncastRef<'a> {
    fn try_as_metadata_feature_ref(self) -> Result<MetadataFeatureRef<'a>, String>;
    fn try_as_textual_representation_ref(
        self,
    ) -> Result<TextualRepresentationRef<'a>, String>;
    fn try_as_comment_ref(self) -> Result<CommentRef<'a>, String>;
}
impl AnnotatingElementDowncast for AnnotatingElement {
    fn try_into_metadata_feature(self) -> Result<MetadataFeature, String> {
        match self {
            AnnotatingElement::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_into_textual_representation(self) -> Result<TextualRepresentation, String> {
        match self {
            AnnotatingElement::TextualRepresentation(e) => Ok(e),
            _ => Err("Not a TextualRepresentation".into()),
        }
    }
    fn try_into_comment(self) -> Result<Comment, String> {
        match self {
            AnnotatingElement::Comment(e) => Ok(e),
            _ => Err("Not a Comment".into()),
        }
    }
}
impl<'a> AnnotatingElementDowncastRefMut<'a> for AnnotatingElementRefMut<'a> {
    fn try_as_metadata_feature_ref_mut(
        self,
    ) -> Result<MetadataFeatureRefMut<'a>, String> {
        match self {
            AnnotatingElementRefMut::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_as_textual_representation_ref_mut(
        self,
    ) -> Result<TextualRepresentationRefMut<'a>, String> {
        match self {
            AnnotatingElementRefMut::TextualRepresentation(e) => Ok(e),
            _ => Err("Not a TextualRepresentation".into()),
        }
    }
    fn try_as_comment_ref_mut(self) -> Result<CommentRefMut<'a>, String> {
        match self {
            AnnotatingElementRefMut::Comment(e) => Ok(e),
            _ => Err("Not a Comment".into()),
        }
    }
}
impl<'a> AnnotatingElementDowncastRef<'a> for AnnotatingElementRef<'a> {
    fn try_as_metadata_feature_ref(self) -> Result<MetadataFeatureRef<'a>, String> {
        match self {
            AnnotatingElementRef::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_as_textual_representation_ref(
        self,
    ) -> Result<TextualRepresentationRef<'a>, String> {
        match self {
            AnnotatingElementRef::TextualRepresentation(e) => Ok(e),
            _ => Err("Not a TextualRepresentation".into()),
        }
    }
    fn try_as_comment_ref(self) -> Result<CommentRef<'a>, String> {
        match self {
            AnnotatingElementRef::Comment(e) => Ok(e),
            _ => Err("Not a Comment".into()),
        }
    }
}
pub trait AnnotatingElementMethodsDescendants
where
    Self: DescendantOf<AnnotatingElement>,
    Self::Via: AnnotatingElementMethods,
    Self: Sized,
{}
pub trait AnnotatingElementMethods: Sized {}
impl<T: AnnotatingElementMethodsDescendants> AnnotatingElementMethods for T
where
    T::Via: AnnotatingElementMethods,
{}
impl DescendantOf<Element> for AnnotatingElement {
    type Via = Element;
    fn into_via(self) -> Self::Via {
        self.into_element()
    }
}
impl ElementMethodsDescendants for AnnotatingElement {}
pub trait AnnotatingElementRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<AnnotatingElementRefMut<'a>>,
    Self::Via: AnnotatingElementRefMutMethods,
    Self: Sized,
{}
pub trait AnnotatingElementRefMutMethods: Sized {}
impl<'a, T: AnnotatingElementRefMutMethodsDescendants<'a>> AnnotatingElementRefMutMethods
for T
where
    T::Via: AnnotatingElementRefMutMethods,
{}
impl<'a> DescendantOf<ElementRefMut<'a>> for AnnotatingElementRefMut<'a> {
    type Via = ElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for AnnotatingElementRefMut<'a> {}
pub trait AnnotatingElementRefMethodsDescendants<'a>
where
    Self: DescendantOf<AnnotatingElementRef<'a>>,
    Self::Via: AnnotatingElementRefMethods,
    Self: Sized,
{}
pub trait AnnotatingElementRefMethods: Sized {}
impl<'a, T: AnnotatingElementRefMethodsDescendants<'a>> AnnotatingElementRefMethods for T
where
    T::Via: AnnotatingElementRefMethods,
{}
impl<'a> DescendantOf<ElementRef<'a>> for AnnotatingElementRef<'a> {
    type Via = ElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for AnnotatingElementRef<'a> {}

