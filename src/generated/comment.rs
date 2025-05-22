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
use super::documentation::{Documentation, DocumentationRefMut, DocumentationRef};
pub struct CommentInner {
    pub(super) sup_annotating_element: AnnotatingElementInner,
    pub(super) locale: Option<String>,
    pub(super) body: String,
}
pub trait CommentStruct
where
    Self: CommentStructRefMut,
    Self: CommentStructRef,
    Self: AnnotatingElementStruct,
{
    fn locale(self) -> Option<String>;
    fn body(self) -> String;
}
pub trait CommentStructRefMut
where
    Self: CommentStructRef,
    Self: AnnotatingElementStructRefMut,
{
    fn locale_ref_mut(&mut self) -> &mut Option<String>;
    fn body_ref_mut(&mut self) -> &mut String;
}
pub trait CommentStructRef
where
    Self: AnnotatingElementStructRef,
{
    fn locale_ref(&self) -> &Option<String>;
    fn body_ref(&self) -> &String;
}
pub trait CommentUpcast: CommentStruct {
    fn into_comment(self) -> Comment;
}
pub trait CommentUpcastRefMut<'a>: CommentStructRefMut {
    fn as_comment_ref_mut(self) -> CommentRefMut<'a>;
}
pub trait CommentUpcastRef<'a>: CommentStructRef {
    fn as_comment_ref(self) -> CommentRef<'a>;
}
impl CommentStruct for CommentInner {
    fn locale(self) -> Option<String> {
        self.locale
    }
    fn body(self) -> String {
        self.body
    }
}
impl CommentStructRefMut for CommentInner {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        &mut self.locale
    }
    fn body_ref_mut(&mut self) -> &mut String {
        &mut self.body
    }
}
impl CommentStructRef for CommentInner {
    fn locale_ref(&self) -> &Option<String> {
        &self.locale
    }
    fn body_ref(&self) -> &String {
        &self.body
    }
}
impl AnnotatingElementStruct for CommentInner {}
impl AnnotatingElementStructRefMut for CommentInner {}
impl AnnotatingElementStructRef for CommentInner {}
impl ElementStruct for CommentInner {
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
impl ElementStructRefMut for CommentInner {
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
impl ElementStructRef for CommentInner {
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
pub enum Comment {
    Itself(CommentInner),
    Documentation(Documentation),
}
pub enum CommentRefMut<'a> {
    Itself(&'a mut CommentInner),
    Documentation(DocumentationRefMut<'a>),
}
pub enum CommentRef<'a> {
    Itself(&'a CommentInner),
    Documentation(DocumentationRef<'a>),
}
impl Comment {
    pub fn as_ref(&self) -> CommentRef {
        match self {
            Comment::Itself(inner) => CommentRef::Itself(&inner),
            Comment::Documentation(inner) => CommentRef::Documentation(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> CommentRefMut {
        match self {
            Comment::Itself(inner) => CommentRefMut::Itself(inner),
            Comment::Documentation(inner) => {
                CommentRefMut::Documentation(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> CommentRefMut<'a> {
    pub fn as_ref(self) -> CommentRef<'a> {
        match self {
            CommentRefMut::Itself(inner) => CommentRef::Itself(inner),
            CommentRefMut::Documentation(inner) => {
                CommentRef::Documentation(inner.as_ref())
            }
        }
    }
}
impl CommentStruct for Comment {
    fn locale(self) -> Option<String> {
        match self {
            Comment::Itself(x) => x.locale(),
            Comment::Documentation(x) => x.locale(),
        }
    }
    fn body(self) -> String {
        match self {
            Comment::Itself(x) => x.body(),
            Comment::Documentation(x) => x.body(),
        }
    }
}
impl CommentStructRefMut for Comment {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Comment::Itself(x) => x.locale_ref_mut(),
            Comment::Documentation(x) => x.locale_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            Comment::Itself(x) => x.body_ref_mut(),
            Comment::Documentation(x) => x.body_ref_mut(),
        }
    }
}
impl CommentStructRef for Comment {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            Comment::Itself(x) => x.locale_ref(),
            Comment::Documentation(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            Comment::Itself(x) => x.body_ref(),
            Comment::Documentation(x) => x.body_ref(),
        }
    }
}
impl<'a> CommentStructRefMut for CommentRefMut<'a> {
    fn locale_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.locale_ref_mut(),
            CommentRefMut::Documentation(x) => x.locale_ref_mut(),
        }
    }
    fn body_ref_mut(&mut self) -> &mut String {
        match self {
            CommentRefMut::Itself(x) => x.body_ref_mut(),
            CommentRefMut::Documentation(x) => x.body_ref_mut(),
        }
    }
}
impl<'a> CommentStructRef for CommentRefMut<'a> {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.locale_ref(),
            CommentRefMut::Documentation(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            CommentRefMut::Itself(x) => x.body_ref(),
            CommentRefMut::Documentation(x) => x.body_ref(),
        }
    }
}
impl<'a> CommentStructRef for CommentRef<'a> {
    fn locale_ref(&self) -> &Option<String> {
        match self {
            CommentRef::Itself(x) => x.locale_ref(),
            CommentRef::Documentation(x) => x.locale_ref(),
        }
    }
    fn body_ref(&self) -> &String {
        match self {
            CommentRef::Itself(x) => x.body_ref(),
            CommentRef::Documentation(x) => x.body_ref(),
        }
    }
}
impl AnnotatingElementStruct for Comment {}
impl AnnotatingElementStructRefMut for Comment {}
impl AnnotatingElementStructRef for Comment {}
impl<'a> AnnotatingElementStructRefMut for CommentRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for CommentRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for CommentRef<'a> {}
impl ElementStruct for Comment {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Comment::Itself(x) => x.owned_relationship(),
            Comment::Documentation(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Comment::Itself(x) => x.owning_relationship(),
            Comment::Documentation(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Comment::Itself(x) => x.element_id(),
            Comment::Documentation(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Comment::Itself(x) => x.alias_ids(),
            Comment::Documentation(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Comment::Itself(x) => x.declared_short_name(),
            Comment::Documentation(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Comment::Itself(x) => x.declared_name(),
            Comment::Documentation(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Comment::Itself(x) => x.is_implied_included(),
            Comment::Documentation(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Comment {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Comment::Itself(x) => x.owned_relationship_ref_mut(),
            Comment::Documentation(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Comment::Itself(x) => x.owning_relationship_ref_mut(),
            Comment::Documentation(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Comment::Itself(x) => x.element_id_ref_mut(),
            Comment::Documentation(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Comment::Itself(x) => x.alias_ids_ref_mut(),
            Comment::Documentation(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Comment::Itself(x) => x.declared_short_name_ref_mut(),
            Comment::Documentation(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Comment::Itself(x) => x.declared_name_ref_mut(),
            Comment::Documentation(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Comment::Itself(x) => x.is_implied_included_ref_mut(),
            Comment::Documentation(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Comment {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Comment::Itself(x) => x.owned_relationship_ref(),
            Comment::Documentation(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Comment::Itself(x) => x.owning_relationship_ref(),
            Comment::Documentation(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Comment::Itself(x) => x.element_id_ref(),
            Comment::Documentation(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Comment::Itself(x) => x.alias_ids_ref(),
            Comment::Documentation(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Comment::Itself(x) => x.declared_short_name_ref(),
            Comment::Documentation(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Comment::Itself(x) => x.declared_name_ref(),
            Comment::Documentation(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Comment::Itself(x) => x.is_implied_included_ref(),
            Comment::Documentation(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for CommentRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CommentRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            CommentRefMut::Documentation(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            CommentRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            CommentRefMut::Documentation(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            CommentRefMut::Itself(x) => x.element_id_ref_mut(),
            CommentRefMut::Documentation(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            CommentRefMut::Itself(x) => x.alias_ids_ref_mut(),
            CommentRefMut::Documentation(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            CommentRefMut::Documentation(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.declared_name_ref_mut(),
            CommentRefMut::Documentation(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            CommentRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            CommentRefMut::Documentation(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for CommentRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CommentRefMut::Itself(x) => x.owned_relationship_ref(),
            CommentRefMut::Documentation(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CommentRefMut::Itself(x) => x.owning_relationship_ref(),
            CommentRefMut::Documentation(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CommentRefMut::Itself(x) => x.element_id_ref(),
            CommentRefMut::Documentation(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CommentRefMut::Itself(x) => x.alias_ids_ref(),
            CommentRefMut::Documentation(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.declared_short_name_ref(),
            CommentRefMut::Documentation(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CommentRefMut::Itself(x) => x.declared_name_ref(),
            CommentRefMut::Documentation(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CommentRefMut::Itself(x) => x.is_implied_included_ref(),
            CommentRefMut::Documentation(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for CommentRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CommentRef::Itself(x) => x.owned_relationship_ref(),
            CommentRef::Documentation(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CommentRef::Itself(x) => x.owning_relationship_ref(),
            CommentRef::Documentation(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CommentRef::Itself(x) => x.element_id_ref(),
            CommentRef::Documentation(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CommentRef::Itself(x) => x.alias_ids_ref(),
            CommentRef::Documentation(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CommentRef::Itself(x) => x.declared_short_name_ref(),
            CommentRef::Documentation(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CommentRef::Itself(x) => x.declared_name_ref(),
            CommentRef::Documentation(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CommentRef::Itself(x) => x.is_implied_included_ref(),
            CommentRef::Documentation(x) => x.is_implied_included_ref(),
        }
    }
}
impl CommentUpcast for Comment {
    fn into_comment(self) -> Comment {
        self
    }
}
impl<'a> CommentUpcastRefMut<'a> for CommentRefMut<'a> {
    fn as_comment_ref_mut(self) -> CommentRefMut<'a> {
        self
    }
}
impl<'a> CommentUpcastRef<'a> for CommentRef<'a> {
    fn as_comment_ref(self) -> CommentRef<'a> {
        self
    }
}
impl AnnotatingElementUpcast for Comment {
    fn into_annotating_element(self) -> AnnotatingElement {
        AnnotatingElement::Comment(self).into_annotating_element()
    }
}
impl<'a> AnnotatingElementUpcastRefMut<'a> for CommentRefMut<'a> {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a> {
        AnnotatingElementRefMut::Comment(self).as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementUpcastRef<'a> for CommentRef<'a> {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a> {
        AnnotatingElementRef::Comment(self).as_annotating_element_ref()
    }
}
impl ElementUpcast for Comment {
    fn into_element(self) -> Element {
        AnnotatingElement::Comment(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for CommentRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        AnnotatingElementRefMut::Comment(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for CommentRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        AnnotatingElementRef::Comment(self).as_element_ref()
    }
}
pub trait CommentDowncast {
    fn try_into_documentation(self) -> Result<Documentation, String>;
}
pub trait CommentDowncastRefMut<'a> {
    fn try_as_documentation_ref_mut(self) -> Result<DocumentationRefMut<'a>, String>;
}
pub trait CommentDowncastRef<'a> {
    fn try_as_documentation_ref(self) -> Result<DocumentationRef<'a>, String>;
}
impl CommentDowncast for Comment {
    fn try_into_documentation(self) -> Result<Documentation, String> {
        match self {
            Comment::Documentation(e) => Ok(e),
            _ => Err("Not a Documentation".into()),
        }
    }
}
impl<'a> CommentDowncastRefMut<'a> for CommentRefMut<'a> {
    fn try_as_documentation_ref_mut(self) -> Result<DocumentationRefMut<'a>, String> {
        match self {
            CommentRefMut::Documentation(e) => Ok(e),
            _ => Err("Not a Documentation".into()),
        }
    }
}
impl<'a> CommentDowncastRef<'a> for CommentRef<'a> {
    fn try_as_documentation_ref(self) -> Result<DocumentationRef<'a>, String> {
        match self {
            CommentRef::Documentation(e) => Ok(e),
            _ => Err("Not a Documentation".into()),
        }
    }
}
pub trait CommentMethodsDescendants
where
    Self: DescendantOf<Comment>,
    Self::Via: CommentMethods,
    Self: Sized,
{}
pub trait CommentMethods: Sized {}
impl<T: CommentMethodsDescendants> CommentMethods for T
where
    T::Via: CommentMethods,
{}
impl DescendantOf<AnnotatingElement> for Comment {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl AnnotatingElementMethodsDescendants for Comment {}
impl DescendantOf<Element> for Comment {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl ElementMethodsDescendants for Comment {}
pub trait CommentRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<CommentRefMut<'a>>,
    Self::Via: CommentRefMutMethods,
    Self: Sized,
{}
pub trait CommentRefMutMethods: Sized {}
impl<'a, T: CommentRefMutMethodsDescendants<'a>> CommentRefMutMethods for T
where
    T::Via: CommentRefMutMethods,
{}
impl<'a> DescendantOf<AnnotatingElementRefMut<'a>> for CommentRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementRefMutMethodsDescendants<'a> for CommentRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for CommentRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for CommentRefMut<'a> {}
pub trait CommentRefMethodsDescendants<'a>
where
    Self: DescendantOf<CommentRef<'a>>,
    Self::Via: CommentRefMethods,
    Self: Sized,
{}
pub trait CommentRefMethods: Sized {}
impl<'a, T: CommentRefMethodsDescendants<'a>> CommentRefMethods for T
where
    T::Via: CommentRefMethods,
{}
impl<'a> DescendantOf<AnnotatingElementRef<'a>> for CommentRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> AnnotatingElementRefMethodsDescendants<'a> for CommentRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for CommentRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for CommentRef<'a> {}

