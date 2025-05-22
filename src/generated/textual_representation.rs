#![allow(unused)]
use super::utils::DescendantOf;
use super::annotating_element::{
    AnnotatingElement, AnnotatingElementRefMut, AnnotatingElementRef,
    AnnotatingElementStruct, AnnotatingElementStructRefMut, AnnotatingElementStructRef,
    AnnotatingElementInner, AnnotatingElementUpcast, AnnotatingElementUpcastRefMut,
    AnnotatingElementUpcastRef, AnnotatingElementMethodsDescendants,
    AnnotatingElementRefMutMethodsDescendants, AnnotatingElementRefMethodsDescendants,
};
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
pub struct TextualRepresentationInner {
    pub(super) sup_annotating_element: AnnotatingElementInner,
    pub(super) language: String,
    pub(super) body: String,
}
pub trait TextualRepresentationStruct
where
    Self: TextualRepresentationStructRefMut,
    Self: TextualRepresentationStructRef,
    Self: AnnotatingElementStruct,
{
    fn language(self) -> String;
    fn body(self) -> String;
}
pub trait TextualRepresentationStructRefMut
where
    Self: TextualRepresentationStructRef,
    Self: AnnotatingElementStructRefMut,
{
    fn language_ref_mut(&mut self) -> &mut String;
    fn body_ref_mut(&mut self) -> &mut String;
}
pub trait TextualRepresentationStructRef
where
    Self: AnnotatingElementStructRef,
{
    fn language_ref(&self) -> &String;
    fn body_ref(&self) -> &String;
}
pub trait TextualRepresentationUpcast: TextualRepresentationStruct {
    fn into_textual_representation(self) -> TextualRepresentation;
}
pub trait TextualRepresentationUpcastRefMut<'a>: TextualRepresentationStructRefMut {
    fn as_textual_representation_ref_mut(self) -> TextualRepresentationRefMut<'a>;
}
pub trait TextualRepresentationUpcastRef<'a>: TextualRepresentationStructRef {
    fn as_textual_representation_ref(self) -> TextualRepresentationRef<'a>;
}
impl TextualRepresentationStruct for TextualRepresentationInner {
    fn language(self) -> String {
        self.language
    }
    fn body(self) -> String {
        self.body
    }
}
impl TextualRepresentationStructRefMut for TextualRepresentationInner {
    fn language_ref_mut(&mut self) -> &mut String {
        &mut self.language
    }
    fn body_ref_mut(&mut self) -> &mut String {
        &mut self.body
    }
}
impl TextualRepresentationStructRef for TextualRepresentationInner {
    fn language_ref(&self) -> &String {
        &self.language
    }
    fn body_ref(&self) -> &String {
        &self.body
    }
}
impl AnnotatingElementStruct for TextualRepresentationInner {}
impl AnnotatingElementStructRefMut for TextualRepresentationInner {}
impl AnnotatingElementStructRef for TextualRepresentationInner {}
impl ElementStruct for TextualRepresentationInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_annotating_element.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_annotating_element.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_annotating_element.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_annotating_element.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_annotating_element.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_annotating_element.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_annotating_element.is_implied_included()
    }
}
impl ElementStructRefMut for TextualRepresentationInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_annotating_element.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_annotating_element.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_annotating_element.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_annotating_element.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_annotating_element.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_annotating_element.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_annotating_element.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for TextualRepresentationInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_annotating_element.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_annotating_element.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_annotating_element.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_annotating_element.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_annotating_element.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_annotating_element.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_annotating_element.is_implied_included_ref()
    }
}
pub enum TextualRepresentation {
    Itself(TextualRepresentationInner),
}
pub enum TextualRepresentationRefMut<'a> {
    Itself(&'a mut TextualRepresentationInner),
}
pub enum TextualRepresentationRef<'a> {
    Itself(&'a TextualRepresentationInner),
}
impl TextualRepresentation {
    pub fn as_ref(&self) -> TextualRepresentationRef {
        match self {
            TextualRepresentation::Itself(inner) => {
                TextualRepresentationRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> TextualRepresentationRefMut {
        match self {
            TextualRepresentation::Itself(inner) => {
                TextualRepresentationRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> TextualRepresentationRefMut<'a> {
    pub fn as_ref(self) -> TextualRepresentationRef<'a> {
        match self {
            TextualRepresentationRefMut::Itself(inner) => {
                TextualRepresentationRef::Itself(inner)
            }
        }
    }
}
impl TextualRepresentationStruct for TextualRepresentation {
    fn language(self) -> String {
        match self {
            TextualRepresentation::Itself(x) => x.language(),
        }
    }
    fn body(self) -> String {
        match self {
            TextualRepresentation::Itself(x) => x.body(),
        }
    }
}
impl TextualRepresentationStructRefMut for TextualRepresentation {
    fn language_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentation::Itself(x) => x.language_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentation::Itself(x) => x.body_ref_mut(),
        }
    }
}
impl TextualRepresentationStructRef for TextualRepresentation {
    fn language_ref(&self) -> &String {
        match self {
            TextualRepresentation::Itself(x) => x.language_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            TextualRepresentation::Itself(x) => x.body_ref(),
        }
    }
}
impl<'a> TextualRepresentationStructRefMut for TextualRepresentationRefMut<'a> {
    fn language_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.language_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.body_ref_mut(),
        }
    }
}
impl<'a> TextualRepresentationStructRef for TextualRepresentationRefMut<'a> {
    fn language_ref(&self) -> &String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.language_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.body_ref(),
        }
    }
}
impl<'a> TextualRepresentationStructRef for TextualRepresentationRef<'a> {
    fn language_ref(&self) -> &String {
        match self {
            TextualRepresentationRef::Itself(x) => x.language_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            TextualRepresentationRef::Itself(x) => x.body_ref(),
        }
    }
}
impl AnnotatingElementStruct for TextualRepresentation {}
impl AnnotatingElementStructRefMut for TextualRepresentation {}
impl AnnotatingElementStructRef for TextualRepresentation {}
impl<'a> AnnotatingElementStructRefMut for TextualRepresentationRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for TextualRepresentationRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for TextualRepresentationRef<'a> {}
impl ElementStruct for TextualRepresentation {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentation::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentation::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            TextualRepresentation::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            TextualRepresentation::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            TextualRepresentation::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for TextualRepresentation {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentation::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            TextualRepresentation::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentation::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            TextualRepresentation::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            TextualRepresentation::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for TextualRepresentation {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentation::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentation::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TextualRepresentation::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TextualRepresentation::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentation::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TextualRepresentation::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for TextualRepresentationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for TextualRepresentationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TextualRepresentationRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for TextualRepresentationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentationRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            TextualRepresentationRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            TextualRepresentationRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            TextualRepresentationRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentationRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            TextualRepresentationRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            TextualRepresentationRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl TextualRepresentationUpcast for TextualRepresentation {
    fn into_textual_representation(self) -> TextualRepresentation {
        self
    }
}
impl<'a> TextualRepresentationUpcastRefMut<'a> for TextualRepresentationRefMut<'a> {
    fn as_textual_representation_ref_mut(self) -> TextualRepresentationRefMut<'a> {
        self
    }
}
impl<'a> TextualRepresentationUpcastRef<'a> for TextualRepresentationRef<'a> {
    fn as_textual_representation_ref(self) -> TextualRepresentationRef<'a> {
        self
    }
}
impl AnnotatingElementUpcast for TextualRepresentation {
    fn into_annotating_element(self) -> AnnotatingElement {
        AnnotatingElement::TextualRepresentation(self).into_annotating_element()
    }
}
impl<'a> AnnotatingElementUpcastRefMut<'a> for TextualRepresentationRefMut<'a> {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a> {
        AnnotatingElementRefMut::TextualRepresentation(self)
            .as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementUpcastRef<'a> for TextualRepresentationRef<'a> {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a> {
        AnnotatingElementRef::TextualRepresentation(self).as_annotating_element_ref()
    }
}
impl ElementUpcast for TextualRepresentation {
    fn into_element(self) -> Element {
        AnnotatingElement::TextualRepresentation(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for TextualRepresentationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        AnnotatingElementRefMut::TextualRepresentation(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for TextualRepresentationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        AnnotatingElementRef::TextualRepresentation(self).as_element_ref()
    }
}
pub trait TextualRepresentationDowncast {}
pub trait TextualRepresentationDowncastRefMut<'a> {}
pub trait TextualRepresentationDowncastRef<'a> {}
impl TextualRepresentationDowncast for TextualRepresentation {}
impl<'a> TextualRepresentationDowncastRefMut<'a> for TextualRepresentationRefMut<'a> {}
impl<'a> TextualRepresentationDowncastRef<'a> for TextualRepresentationRef<'a> {}
pub trait TextualRepresentationMethodsDescendants
where
    Self: DescendantOf<TextualRepresentation>,
    Self::Via: TextualRepresentationMethods,
    Self: Sized,
{}
pub trait TextualRepresentationMethods: Sized {}
impl<T: TextualRepresentationMethodsDescendants> TextualRepresentationMethods for T
where
    T::Via: TextualRepresentationMethods,
{}
impl DescendantOf<AnnotatingElement> for TextualRepresentation {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl AnnotatingElementMethodsDescendants for TextualRepresentation {}
impl DescendantOf<Element> for TextualRepresentation {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl ElementMethodsDescendants for TextualRepresentation {}
pub trait TextualRepresentationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<TextualRepresentationRefMut<'a>>,
    Self::Via: TextualRepresentationRefMutMethods,
    Self: Sized,
{}
pub trait TextualRepresentationRefMutMethods: Sized {}
impl<
    'a,
    T: TextualRepresentationRefMutMethodsDescendants<'a>,
> TextualRepresentationRefMutMethods for T
where
    T::Via: TextualRepresentationRefMutMethods,
{}
impl<'a> DescendantOf<AnnotatingElementRefMut<'a>> for TextualRepresentationRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementRefMutMethodsDescendants<'a>
for TextualRepresentationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for TextualRepresentationRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for TextualRepresentationRefMut<'a> {}
pub trait TextualRepresentationRefMethodsDescendants<'a>
where
    Self: DescendantOf<TextualRepresentationRef<'a>>,
    Self::Via: TextualRepresentationRefMethods,
    Self: Sized,
{}
pub trait TextualRepresentationRefMethods: Sized {}
impl<
    'a,
    T: TextualRepresentationRefMethodsDescendants<'a>,
> TextualRepresentationRefMethods for T
where
    T::Via: TextualRepresentationRefMethods,
{}
impl<'a> DescendantOf<AnnotatingElementRef<'a>> for TextualRepresentationRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> AnnotatingElementRefMethodsDescendants<'a> for TextualRepresentationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for TextualRepresentationRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for TextualRepresentationRef<'a> {}

