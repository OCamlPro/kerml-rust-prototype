#![allow(unused)]
use super::utils::DescendantOf;
use super::comment::{
    Comment, CommentRefMut, CommentRef, CommentStruct, CommentStructRefMut,
    CommentStructRef, CommentInner, CommentUpcast, CommentUpcastRefMut, CommentUpcastRef,
    CommentMethodsDescendants, CommentRefMutMethodsDescendants,
    CommentRefMethodsDescendants,
};
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
pub struct DocumentationInner {
    pub(super) sup_comment: CommentInner,
}
pub trait DocumentationStruct
where
    Self: DocumentationStructRefMut,
    Self: DocumentationStructRef,
    Self: CommentStruct,
{}
pub trait DocumentationStructRefMut
where
    Self: DocumentationStructRef,
    Self: CommentStructRefMut,
{}
pub trait DocumentationStructRef
where
    Self: CommentStructRef,
{}
pub trait DocumentationUpcast: DocumentationStruct {
    fn into_documentation(self) -> Documentation;
}
pub trait DocumentationUpcastRefMut<'a>: DocumentationStructRefMut {
    fn as_documentation_ref_mut(self) -> DocumentationRefMut<'a>;
}
pub trait DocumentationUpcastRef<'a>: DocumentationStructRef {
    fn as_documentation_ref(self) -> DocumentationRef<'a>;
}
impl DocumentationStruct for DocumentationInner {}
impl DocumentationStructRefMut for DocumentationInner {}
impl DocumentationStructRef for DocumentationInner {}
impl CommentStruct for DocumentationInner {
    fn locale(self) -> Option<String> {
        self.sup_comment.locale()
    }
    fn body(self) -> String {
        self.sup_comment.body()
    }
}
impl CommentStructRefMut for DocumentationInner {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_comment.locale_ref_mut()
    }
    fn body_ref_mut(&mut self) -> &mut String {
        self.sup_comment.body_ref_mut()
    }
}
impl CommentStructRef for DocumentationInner {
    fn locale_ref(&self) -> &Option<String> {
        self.sup_comment.locale_ref()
    }
    fn body_ref(&self) -> &String {
        self.sup_comment.body_ref()
    }
}
impl AnnotatingElementStruct for DocumentationInner {}
impl AnnotatingElementStructRefMut for DocumentationInner {}
impl AnnotatingElementStructRef for DocumentationInner {}
impl ElementStruct for DocumentationInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_comment.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_comment.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_comment.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_comment.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_comment.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_comment.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_comment.is_implied_included()
    }
}
impl ElementStructRefMut for DocumentationInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_comment.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_comment.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_comment.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_comment.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_comment.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_comment.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_comment.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for DocumentationInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_comment.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_comment.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_comment.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_comment.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_comment.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_comment.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_comment.is_implied_included_ref()
    }
}
pub enum Documentation {
    Itself(DocumentationInner),
}
pub enum DocumentationRefMut<'a> {
    Itself(&'a mut DocumentationInner),
}
pub enum DocumentationRef<'a> {
    Itself(&'a DocumentationInner),
}
impl Documentation {
    pub fn as_ref(&self) -> DocumentationRef {
        match self {
            Documentation::Itself(inner) => DocumentationRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> DocumentationRefMut {
        match self {
            Documentation::Itself(inner) => DocumentationRefMut::Itself(inner),
        }
    }
}
impl<'a> DocumentationRefMut<'a> {
    pub fn as_ref(self) -> DocumentationRef<'a> {
        match self {
            DocumentationRefMut::Itself(inner) => DocumentationRef::Itself(inner),
        }
    }
}
impl DocumentationStruct for Documentation {}
impl DocumentationStructRefMut for Documentation {}
impl DocumentationStructRef for Documentation {}
impl<'a> DocumentationStructRefMut for DocumentationRefMut<'a> {}
impl<'a> DocumentationStructRef for DocumentationRefMut<'a> {}
impl<'a> DocumentationStructRef for DocumentationRef<'a> {}
impl CommentStruct for Documentation {
    fn locale(self) -> Option<String> {
        match self {
            Documentation::Itself(x) => x.locale(),
        }
    }
    fn body(self) -> String {
        match self {
            Documentation::Itself(x) => x.body(),
        }
    }
}
impl CommentStructRefMut for Documentation {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Documentation::Itself(x) => x.locale_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            Documentation::Itself(x) => x.body_ref_mut(),
        }
    }
}
impl CommentStructRef for Documentation {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            Documentation::Itself(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            Documentation::Itself(x) => x.body_ref(),
        }
    }
}
impl<'a> CommentStructRefMut for DocumentationRefMut<'a> {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.locale_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            DocumentationRefMut::Itself(x) => x.body_ref_mut(),
        }
    }
}
impl<'a> CommentStructRef for DocumentationRefMut<'a> {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            DocumentationRefMut::Itself(x) => x.body_ref(),
        }
    }
}
impl<'a> CommentStructRef for DocumentationRef<'a> {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            DocumentationRef::Itself(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            DocumentationRef::Itself(x) => x.body_ref(),
        }
    }
}
impl AnnotatingElementStruct for Documentation {}
impl AnnotatingElementStructRefMut for Documentation {}
impl AnnotatingElementStructRef for Documentation {}
impl<'a> AnnotatingElementStructRefMut for DocumentationRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for DocumentationRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for DocumentationRef<'a> {}
impl ElementStruct for Documentation {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Documentation::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Documentation::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Documentation::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Documentation::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Documentation::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Documentation {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Documentation::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Documentation::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Documentation::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Documentation::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Documentation::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Documentation {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Documentation::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Documentation::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Documentation::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Documentation::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Documentation::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Documentation::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for DocumentationRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DocumentationRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            DocumentationRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            DocumentationRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            DocumentationRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for DocumentationRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DocumentationRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DocumentationRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DocumentationRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DocumentationRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DocumentationRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for DocumentationRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DocumentationRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DocumentationRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DocumentationRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DocumentationRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DocumentationRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DocumentationRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DocumentationRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl DocumentationUpcast for Documentation {
    fn into_documentation(self) -> Documentation {
        self
    }
}
impl<'a> DocumentationUpcastRefMut<'a> for DocumentationRefMut<'a> {
    fn as_documentation_ref_mut(self) -> DocumentationRefMut<'a> {
        self
    }
}
impl<'a> DocumentationUpcastRef<'a> for DocumentationRef<'a> {
    fn as_documentation_ref(self) -> DocumentationRef<'a> {
        self
    }
}
impl CommentUpcast for Documentation {
    fn into_comment(self) -> Comment {
        Comment::Documentation(self).into_comment()
    }
}
impl<'a> CommentUpcastRefMut<'a> for DocumentationRefMut<'a> {
    fn as_comment_ref_mut(self) -> CommentRefMut<'a> {
        CommentRefMut::Documentation(self).as_comment_ref_mut()
    }
}
impl<'a> CommentUpcastRef<'a> for DocumentationRef<'a> {
    fn as_comment_ref(self) -> CommentRef<'a> {
        CommentRef::Documentation(self).as_comment_ref()
    }
}
impl AnnotatingElementUpcast for Documentation {
    fn into_annotating_element(self) -> AnnotatingElement {
        Comment::Documentation(self).into_annotating_element()
    }
}
impl<'a> AnnotatingElementUpcastRefMut<'a> for DocumentationRefMut<'a> {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a> {
        CommentRefMut::Documentation(self).as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementUpcastRef<'a> for DocumentationRef<'a> {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a> {
        CommentRef::Documentation(self).as_annotating_element_ref()
    }
}
impl ElementUpcast for Documentation {
    fn into_element(self) -> Element {
        Comment::Documentation(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for DocumentationRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        CommentRefMut::Documentation(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for DocumentationRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        CommentRef::Documentation(self).as_element_ref()
    }
}
pub trait DocumentationDowncast {}
pub trait DocumentationDowncastRefMut<'a> {}
pub trait DocumentationDowncastRef<'a> {}
impl DocumentationDowncast for Documentation {}
impl<'a> DocumentationDowncastRefMut<'a> for DocumentationRefMut<'a> {}
impl<'a> DocumentationDowncastRef<'a> for DocumentationRef<'a> {}
pub trait DocumentationMethodsDescendants
where
    Self: DescendantOf<Documentation>,
    Self::Via: DocumentationMethods,
    Self: Sized,
{}
pub trait DocumentationMethods: Sized {}
impl<T: DocumentationMethodsDescendants> DocumentationMethods for T
where
    T::Via: DocumentationMethods,
{}
impl DescendantOf<Comment> for Documentation {
    type Via = Comment;
    fn into_via(self) -> Self::Via {
        self.into_comment()
    }
}
impl CommentMethodsDescendants for Documentation {}
impl DescendantOf<AnnotatingElement> for Documentation {
    type Via = Comment;
    fn into_via(self) -> Self::Via {
        self.into_comment()
    }
}
impl AnnotatingElementMethodsDescendants for Documentation {}
impl DescendantOf<Element> for Documentation {
    type Via = Comment;
    fn into_via(self) -> Self::Via {
        self.into_comment()
    }
}
impl ElementMethodsDescendants for Documentation {}
pub trait DocumentationRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<DocumentationRefMut<'a>>,
    Self::Via: DocumentationRefMutMethods,
    Self: Sized,
{}
pub trait DocumentationRefMutMethods: Sized {}
impl<'a, T: DocumentationRefMutMethodsDescendants<'a>> DocumentationRefMutMethods for T
where
    T::Via: DocumentationRefMutMethods,
{}
impl<'a> DescendantOf<CommentRefMut<'a>> for DocumentationRefMut<'a> {
    type Via = CommentRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref_mut()
    }
}
impl<'a> CommentRefMutMethodsDescendants<'a> for DocumentationRefMut<'a> {}
impl<'a> DescendantOf<AnnotatingElementRefMut<'a>> for DocumentationRefMut<'a> {
    type Via = CommentRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref_mut()
    }
}
impl<'a> AnnotatingElementRefMutMethodsDescendants<'a> for DocumentationRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for DocumentationRefMut<'a> {
    type Via = CommentRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for DocumentationRefMut<'a> {}
pub trait DocumentationRefMethodsDescendants<'a>
where
    Self: DescendantOf<DocumentationRef<'a>>,
    Self::Via: DocumentationRefMethods,
    Self: Sized,
{}
pub trait DocumentationRefMethods: Sized {}
impl<'a, T: DocumentationRefMethodsDescendants<'a>> DocumentationRefMethods for T
where
    T::Via: DocumentationRefMethods,
{}
impl<'a> DescendantOf<CommentRef<'a>> for DocumentationRef<'a> {
    type Via = CommentRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref()
    }
}
impl<'a> CommentRefMethodsDescendants<'a> for DocumentationRef<'a> {}
impl<'a> DescendantOf<AnnotatingElementRef<'a>> for DocumentationRef<'a> {
    type Via = CommentRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref()
    }
}
impl<'a> AnnotatingElementRefMethodsDescendants<'a> for DocumentationRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for DocumentationRef<'a> {
    type Via = CommentRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_comment_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for DocumentationRef<'a> {}

