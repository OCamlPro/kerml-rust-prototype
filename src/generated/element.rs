#![allow(unused)]
use super::utils::DescendantOf;
use super::namespace::{Namespace, NamespaceRefMut, NamespaceRef};
use super::relationship::{Relationship, RelationshipRefMut, RelationshipRef};
use super::annotating_element::{
    AnnotatingElement, AnnotatingElementRefMut, AnnotatingElementRef,
};
pub struct ElementInner {
    pub(super) owned_relationship: Vec<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    >,
    pub(super) owning_relationship: Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    >,
    pub(super) element_id: String,
    pub(super) alias_ids: Vec<String>,
    pub(super) declared_short_name: Option<String>,
    pub(super) declared_name: Option<String>,
    pub(super) is_implied_included: bool,
}
pub trait ElementStruct
where
    Self: ElementStructRefMut,
    Self: ElementStructRef,
{
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn element_id(self) -> String;
    fn alias_ids(self) -> Vec<String>;
    fn declared_short_name(self) -> Option<String>;
    fn declared_name(self) -> Option<String>;
    fn is_implied_included(self) -> bool;
}
pub trait ElementStructRefMut
where
    Self: ElementStructRef,
{
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn element_id_ref_mut(&mut self) -> &mut String;
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String>;
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String>;
    fn declared_name_ref_mut(&mut self) -> &mut Option<String>;
    fn is_implied_included_ref_mut(&mut self) -> &mut bool;
}
pub trait ElementStructRef {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>>;
    fn element_id_ref(&self) -> &String;
    fn alias_ids_ref(&self) -> &Vec<String>;
    fn declared_short_name_ref(&self) -> &Option<String>;
    fn declared_name_ref(&self) -> &Option<String>;
    fn is_implied_included_ref(&self) -> &bool;
}
pub trait ElementUpcast: ElementStruct {
    fn into_element(self) -> Element;
}
pub trait ElementUpcastRefMut<'a>: ElementStructRefMut {
    fn as_element_ref_mut(self) -> ElementRefMut<'a>;
}
pub trait ElementUpcastRef<'a>: ElementStructRef {
    fn as_element_ref(self) -> ElementRef<'a>;
}
impl ElementStruct for ElementInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.owned_relationship
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.owning_relationship
    }
    fn element_id(self) -> String {
        self.element_id
    }
    fn alias_ids(self) -> Vec<String> {
        self.alias_ids
    }
    fn declared_short_name(self) -> Option<String> {
        self.declared_short_name
    }
    fn declared_name(self) -> Option<String> {
        self.declared_name
    }
    fn is_implied_included(self) -> bool {
        self.is_implied_included
    }
}
impl ElementStructRefMut for ElementInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        &mut self.owned_relationship
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        &mut self.owning_relationship
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        &mut self.element_id
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        &mut self.alias_ids
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        &mut self.declared_short_name
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        &mut self.declared_name
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        &mut self.is_implied_included
    }
}
impl ElementStructRef for ElementInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        &self.owned_relationship
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        &self.owning_relationship
    }
    fn element_id_ref(&self) -> &String {
        &self.element_id
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        &self.alias_ids
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        &self.declared_short_name
    }
    fn declared_name_ref(&self) -> &Option<String> {
        &self.declared_name
    }
    fn is_implied_included_ref(&self) -> &bool {
        &self.is_implied_included
    }
}
pub enum Element {
    Itself(ElementInner),
    Namespace(Namespace),
    Relationship(Relationship),
    AnnotatingElement(AnnotatingElement),
}
pub enum ElementRefMut<'a> {
    Itself(&'a mut ElementInner),
    Namespace(NamespaceRefMut<'a>),
    Relationship(RelationshipRefMut<'a>),
    AnnotatingElement(AnnotatingElementRefMut<'a>),
}
pub enum ElementRef<'a> {
    Itself(&'a ElementInner),
    Namespace(NamespaceRef<'a>),
    Relationship(RelationshipRef<'a>),
    AnnotatingElement(AnnotatingElementRef<'a>),
}
impl Element {
    pub fn as_ref(&self) -> ElementRef {
        match self {
            Element::Itself(inner) => ElementRef::Itself(&inner),
            Element::Namespace(inner) => ElementRef::Namespace(inner.as_ref()),
            Element::Relationship(inner) => ElementRef::Relationship(inner.as_ref()),
            Element::AnnotatingElement(inner) => {
                ElementRef::AnnotatingElement(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ElementRefMut {
        match self {
            Element::Itself(inner) => ElementRefMut::Itself(inner),
            Element::Namespace(inner) => ElementRefMut::Namespace(inner.as_ref_mut()),
            Element::Relationship(inner) => {
                ElementRefMut::Relationship(inner.as_ref_mut())
            }
            Element::AnnotatingElement(inner) => {
                ElementRefMut::AnnotatingElement(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ElementRefMut<'a> {
    pub fn as_ref(self) -> ElementRef<'a> {
        match self {
            ElementRefMut::Itself(inner) => ElementRef::Itself(inner),
            ElementRefMut::Namespace(inner) => ElementRef::Namespace(inner.as_ref()),
            ElementRefMut::Relationship(inner) => {
                ElementRef::Relationship(inner.as_ref())
            }
            ElementRefMut::AnnotatingElement(inner) => {
                ElementRef::AnnotatingElement(inner.as_ref())
            }
        }
    }
}
impl ElementStruct for Element {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Element::Itself(x) => x.owned_relationship(),
            Element::Namespace(x) => x.owned_relationship(),
            Element::Relationship(x) => x.owned_relationship(),
            Element::AnnotatingElement(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Element::Itself(x) => x.owning_relationship(),
            Element::Namespace(x) => x.owning_relationship(),
            Element::Relationship(x) => x.owning_relationship(),
            Element::AnnotatingElement(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Element::Itself(x) => x.element_id(),
            Element::Namespace(x) => x.element_id(),
            Element::Relationship(x) => x.element_id(),
            Element::AnnotatingElement(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Element::Itself(x) => x.alias_ids(),
            Element::Namespace(x) => x.alias_ids(),
            Element::Relationship(x) => x.alias_ids(),
            Element::AnnotatingElement(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Element::Itself(x) => x.declared_short_name(),
            Element::Namespace(x) => x.declared_short_name(),
            Element::Relationship(x) => x.declared_short_name(),
            Element::AnnotatingElement(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Element::Itself(x) => x.declared_name(),
            Element::Namespace(x) => x.declared_name(),
            Element::Relationship(x) => x.declared_name(),
            Element::AnnotatingElement(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Element::Itself(x) => x.is_implied_included(),
            Element::Namespace(x) => x.is_implied_included(),
            Element::Relationship(x) => x.is_implied_included(),
            Element::AnnotatingElement(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Element {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Element::Itself(x) => x.owned_relationship_ref_mut(),
            Element::Namespace(x) => x.owned_relationship_ref_mut(),
            Element::Relationship(x) => x.owned_relationship_ref_mut(),
            Element::AnnotatingElement(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Element::Itself(x) => x.owning_relationship_ref_mut(),
            Element::Namespace(x) => x.owning_relationship_ref_mut(),
            Element::Relationship(x) => x.owning_relationship_ref_mut(),
            Element::AnnotatingElement(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Element::Itself(x) => x.element_id_ref_mut(),
            Element::Namespace(x) => x.element_id_ref_mut(),
            Element::Relationship(x) => x.element_id_ref_mut(),
            Element::AnnotatingElement(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Element::Itself(x) => x.alias_ids_ref_mut(),
            Element::Namespace(x) => x.alias_ids_ref_mut(),
            Element::Relationship(x) => x.alias_ids_ref_mut(),
            Element::AnnotatingElement(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Element::Itself(x) => x.declared_short_name_ref_mut(),
            Element::Namespace(x) => x.declared_short_name_ref_mut(),
            Element::Relationship(x) => x.declared_short_name_ref_mut(),
            Element::AnnotatingElement(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Element::Itself(x) => x.declared_name_ref_mut(),
            Element::Namespace(x) => x.declared_name_ref_mut(),
            Element::Relationship(x) => x.declared_name_ref_mut(),
            Element::AnnotatingElement(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Element::Itself(x) => x.is_implied_included_ref_mut(),
            Element::Namespace(x) => x.is_implied_included_ref_mut(),
            Element::Relationship(x) => x.is_implied_included_ref_mut(),
            Element::AnnotatingElement(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Element {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Element::Itself(x) => x.owned_relationship_ref(),
            Element::Namespace(x) => x.owned_relationship_ref(),
            Element::Relationship(x) => x.owned_relationship_ref(),
            Element::AnnotatingElement(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Element::Itself(x) => x.owning_relationship_ref(),
            Element::Namespace(x) => x.owning_relationship_ref(),
            Element::Relationship(x) => x.owning_relationship_ref(),
            Element::AnnotatingElement(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Element::Itself(x) => x.element_id_ref(),
            Element::Namespace(x) => x.element_id_ref(),
            Element::Relationship(x) => x.element_id_ref(),
            Element::AnnotatingElement(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Element::Itself(x) => x.alias_ids_ref(),
            Element::Namespace(x) => x.alias_ids_ref(),
            Element::Relationship(x) => x.alias_ids_ref(),
            Element::AnnotatingElement(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Element::Itself(x) => x.declared_short_name_ref(),
            Element::Namespace(x) => x.declared_short_name_ref(),
            Element::Relationship(x) => x.declared_short_name_ref(),
            Element::AnnotatingElement(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Element::Itself(x) => x.declared_name_ref(),
            Element::Namespace(x) => x.declared_name_ref(),
            Element::Relationship(x) => x.declared_name_ref(),
            Element::AnnotatingElement(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Element::Itself(x) => x.is_implied_included_ref(),
            Element::Namespace(x) => x.is_implied_included_ref(),
            Element::Relationship(x) => x.is_implied_included_ref(),
            Element::AnnotatingElement(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ElementRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ElementRefMut::Namespace(x) => x.owned_relationship_ref_mut(),
            ElementRefMut::Relationship(x) => x.owned_relationship_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ElementRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ElementRefMut::Namespace(x) => x.owning_relationship_ref_mut(),
            ElementRefMut::Relationship(x) => x.owning_relationship_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ElementRefMut::Itself(x) => x.element_id_ref_mut(),
            ElementRefMut::Namespace(x) => x.element_id_ref_mut(),
            ElementRefMut::Relationship(x) => x.element_id_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ElementRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ElementRefMut::Namespace(x) => x.alias_ids_ref_mut(),
            ElementRefMut::Relationship(x) => x.alias_ids_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ElementRefMut::Namespace(x) => x.declared_short_name_ref_mut(),
            ElementRefMut::Relationship(x) => x.declared_short_name_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ElementRefMut::Itself(x) => x.declared_name_ref_mut(),
            ElementRefMut::Namespace(x) => x.declared_name_ref_mut(),
            ElementRefMut::Relationship(x) => x.declared_name_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ElementRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ElementRefMut::Namespace(x) => x.is_implied_included_ref_mut(),
            ElementRefMut::Relationship(x) => x.is_implied_included_ref_mut(),
            ElementRefMut::AnnotatingElement(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ElementRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementRefMut::Itself(x) => x.owned_relationship_ref(),
            ElementRefMut::Namespace(x) => x.owned_relationship_ref(),
            ElementRefMut::Relationship(x) => x.owned_relationship_ref(),
            ElementRefMut::AnnotatingElement(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementRefMut::Itself(x) => x.owning_relationship_ref(),
            ElementRefMut::Namespace(x) => x.owning_relationship_ref(),
            ElementRefMut::Relationship(x) => x.owning_relationship_ref(),
            ElementRefMut::AnnotatingElement(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ElementRefMut::Itself(x) => x.element_id_ref(),
            ElementRefMut::Namespace(x) => x.element_id_ref(),
            ElementRefMut::Relationship(x) => x.element_id_ref(),
            ElementRefMut::AnnotatingElement(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ElementRefMut::Itself(x) => x.alias_ids_ref(),
            ElementRefMut::Namespace(x) => x.alias_ids_ref(),
            ElementRefMut::Relationship(x) => x.alias_ids_ref(),
            ElementRefMut::AnnotatingElement(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementRefMut::Itself(x) => x.declared_short_name_ref(),
            ElementRefMut::Namespace(x) => x.declared_short_name_ref(),
            ElementRefMut::Relationship(x) => x.declared_short_name_ref(),
            ElementRefMut::AnnotatingElement(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ElementRefMut::Itself(x) => x.declared_name_ref(),
            ElementRefMut::Namespace(x) => x.declared_name_ref(),
            ElementRefMut::Relationship(x) => x.declared_name_ref(),
            ElementRefMut::AnnotatingElement(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ElementRefMut::Itself(x) => x.is_implied_included_ref(),
            ElementRefMut::Namespace(x) => x.is_implied_included_ref(),
            ElementRefMut::Relationship(x) => x.is_implied_included_ref(),
            ElementRefMut::AnnotatingElement(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ElementRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementRef::Itself(x) => x.owned_relationship_ref(),
            ElementRef::Namespace(x) => x.owned_relationship_ref(),
            ElementRef::Relationship(x) => x.owned_relationship_ref(),
            ElementRef::AnnotatingElement(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ElementRef::Itself(x) => x.owning_relationship_ref(),
            ElementRef::Namespace(x) => x.owning_relationship_ref(),
            ElementRef::Relationship(x) => x.owning_relationship_ref(),
            ElementRef::AnnotatingElement(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ElementRef::Itself(x) => x.element_id_ref(),
            ElementRef::Namespace(x) => x.element_id_ref(),
            ElementRef::Relationship(x) => x.element_id_ref(),
            ElementRef::AnnotatingElement(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ElementRef::Itself(x) => x.alias_ids_ref(),
            ElementRef::Namespace(x) => x.alias_ids_ref(),
            ElementRef::Relationship(x) => x.alias_ids_ref(),
            ElementRef::AnnotatingElement(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ElementRef::Itself(x) => x.declared_short_name_ref(),
            ElementRef::Namespace(x) => x.declared_short_name_ref(),
            ElementRef::Relationship(x) => x.declared_short_name_ref(),
            ElementRef::AnnotatingElement(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ElementRef::Itself(x) => x.declared_name_ref(),
            ElementRef::Namespace(x) => x.declared_name_ref(),
            ElementRef::Relationship(x) => x.declared_name_ref(),
            ElementRef::AnnotatingElement(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ElementRef::Itself(x) => x.is_implied_included_ref(),
            ElementRef::Namespace(x) => x.is_implied_included_ref(),
            ElementRef::Relationship(x) => x.is_implied_included_ref(),
            ElementRef::AnnotatingElement(x) => x.is_implied_included_ref(),
        }
    }
}
impl ElementUpcast for Element {
    fn into_element(self) -> Element {
        self
    }
}
impl<'a> ElementUpcastRefMut<'a> for ElementRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        self
    }
}
impl<'a> ElementUpcastRef<'a> for ElementRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        self
    }
}
pub trait ElementDowncast {
    fn try_into_namespace(self) -> Result<Namespace, String>;
    fn try_into_relationship(self) -> Result<Relationship, String>;
    fn try_into_annotating_element(self) -> Result<AnnotatingElement, String>;
}
pub trait ElementDowncastRefMut<'a> {
    fn try_as_namespace_ref_mut(self) -> Result<NamespaceRefMut<'a>, String>;
    fn try_as_relationship_ref_mut(self) -> Result<RelationshipRefMut<'a>, String>;
    fn try_as_annotating_element_ref_mut(
        self,
    ) -> Result<AnnotatingElementRefMut<'a>, String>;
}
pub trait ElementDowncastRef<'a> {
    fn try_as_namespace_ref(self) -> Result<NamespaceRef<'a>, String>;
    fn try_as_relationship_ref(self) -> Result<RelationshipRef<'a>, String>;
    fn try_as_annotating_element_ref(self) -> Result<AnnotatingElementRef<'a>, String>;
}
impl ElementDowncast for Element {
    fn try_into_namespace(self) -> Result<Namespace, String> {
        match self {
            Element::Namespace(e) => Ok(e),
            _ => Err("Not a Namespace".into()),
        }
    }
    fn try_into_relationship(self) -> Result<Relationship, String> {
        match self {
            Element::Relationship(e) => Ok(e),
            _ => Err("Not a Relationship".into()),
        }
    }
    fn try_into_annotating_element(self) -> Result<AnnotatingElement, String> {
        match self {
            Element::AnnotatingElement(e) => Ok(e),
            _ => Err("Not a AnnotatingElement".into()),
        }
    }
}
impl<'a> ElementDowncastRefMut<'a> for ElementRefMut<'a> {
    fn try_as_namespace_ref_mut(self) -> Result<NamespaceRefMut<'a>, String> {
        match self {
            ElementRefMut::Namespace(e) => Ok(e),
            _ => Err("Not a Namespace".into()),
        }
    }
    fn try_as_relationship_ref_mut(self) -> Result<RelationshipRefMut<'a>, String> {
        match self {
            ElementRefMut::Relationship(e) => Ok(e),
            _ => Err("Not a Relationship".into()),
        }
    }
    fn try_as_annotating_element_ref_mut(
        self,
    ) -> Result<AnnotatingElementRefMut<'a>, String> {
        match self {
            ElementRefMut::AnnotatingElement(e) => Ok(e),
            _ => Err("Not a AnnotatingElement".into()),
        }
    }
}
impl<'a> ElementDowncastRef<'a> for ElementRef<'a> {
    fn try_as_namespace_ref(self) -> Result<NamespaceRef<'a>, String> {
        match self {
            ElementRef::Namespace(e) => Ok(e),
            _ => Err("Not a Namespace".into()),
        }
    }
    fn try_as_relationship_ref(self) -> Result<RelationshipRef<'a>, String> {
        match self {
            ElementRef::Relationship(e) => Ok(e),
            _ => Err("Not a Relationship".into()),
        }
    }
    fn try_as_annotating_element_ref(self) -> Result<AnnotatingElementRef<'a>, String> {
        match self {
            ElementRef::AnnotatingElement(e) => Ok(e),
            _ => Err("Not a AnnotatingElement".into()),
        }
    }
}
pub trait ElementMethodsDescendants
where
    Self: DescendantOf<Element>,
    Self::Via: ElementMethods,
    Self: Sized,
{}
pub trait ElementMethods: Sized {}
impl<T: ElementMethodsDescendants> ElementMethods for T
where
    T::Via: ElementMethods,
{}
pub trait ElementRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ElementRefMut<'a>>,
    Self::Via: ElementRefMutMethods,
    Self: Sized,
{}
pub trait ElementRefMutMethods: Sized {}
impl<'a, T: ElementRefMutMethodsDescendants<'a>> ElementRefMutMethods for T
where
    T::Via: ElementRefMutMethods,
{}
pub trait ElementRefMethodsDescendants<'a>
where
    Self: DescendantOf<ElementRef<'a>>,
    Self::Via: ElementRefMethods,
    Self: Sized,
{
    fn escaped_name_impl(self) -> Option<String> {
        self.into_via().escaped_name()
    }
    fn effective_short_name_impl(self) -> Option<String> {
        self.into_via().effective_short_name()
    }
    fn effective_name_impl(self) -> Option<String> {
        self.into_via().effective_name()
    }
    fn library_namespace_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>> {
        self.into_via().library_namespace()
    }
    fn path_impl(self) -> String {
        self.into_via().path()
    }
}
pub trait ElementRefMethods: Sized {
    fn escaped_name(self) -> Option<String>;
    fn effective_short_name(self) -> Option<String>;
    fn effective_name(self) -> Option<String>;
    fn library_namespace(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>>;
    fn path(self) -> String;
}
impl<'a, T: ElementRefMethodsDescendants<'a>> ElementRefMethods for T
where
    T::Via: ElementRefMethods,
{
    fn escaped_name(self) -> Option<String> {
        ElementRefMethodsDescendants::escaped_name_impl(self)
    }
    fn effective_short_name(self) -> Option<String> {
        ElementRefMethodsDescendants::effective_short_name_impl(self)
    }
    fn effective_name(self) -> Option<String> {
        ElementRefMethodsDescendants::effective_name_impl(self)
    }
    fn library_namespace(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::namespace::Namespace>>> {
        ElementRefMethodsDescendants::library_namespace_impl(self)
    }
    fn path(self) -> String {
        ElementRefMethodsDescendants::path_impl(self)
    }
}

