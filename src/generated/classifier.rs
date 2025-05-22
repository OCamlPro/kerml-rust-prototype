#![allow(unused)]
use super::utils::DescendantOf;
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
use super::class::{Class, ClassRefMut, ClassRef};
use super::data_type::{DataType, DataTypeRefMut, DataTypeRef};
use super::association::{Association, AssociationRefMut, AssociationRef};
pub struct ClassifierInner {
    pub(super) sup_type_: TypeInner,
}
pub trait ClassifierStruct
where
    Self: ClassifierStructRefMut,
    Self: ClassifierStructRef,
    Self: TypeStruct,
{}
pub trait ClassifierStructRefMut
where
    Self: ClassifierStructRef,
    Self: TypeStructRefMut,
{}
pub trait ClassifierStructRef
where
    Self: TypeStructRef,
{}
pub trait ClassifierUpcast: ClassifierStruct {
    fn into_classifier(self) -> Classifier;
}
pub trait ClassifierUpcastRefMut<'a>: ClassifierStructRefMut {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a>;
}
pub trait ClassifierUpcastRef<'a>: ClassifierStructRef {
    fn as_classifier_ref(self) -> ClassifierRef<'a>;
}
impl ClassifierStruct for ClassifierInner {}
impl ClassifierStructRefMut for ClassifierInner {}
impl ClassifierStructRef for ClassifierInner {}
impl TypeStruct for ClassifierInner {
    fn is_abstract(self) -> bool {
        self.sup_type_.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_type_.is_sufficient()
    }
}
impl TypeStructRefMut for ClassifierInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for ClassifierInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_type_.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_type_.is_sufficient_ref()
    }
}
impl NamespaceStruct for ClassifierInner {}
impl NamespaceStructRefMut for ClassifierInner {}
impl NamespaceStructRef for ClassifierInner {}
impl ElementStruct for ClassifierInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_type_.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_type_.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_type_.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_type_.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_type_.is_implied_included()
    }
}
impl ElementStructRefMut for ClassifierInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_type_.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_type_.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_type_.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_type_.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_type_.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for ClassifierInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_type_.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_type_.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_type_.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_type_.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_type_.is_implied_included_ref()
    }
}
pub enum Classifier {
    Itself(ClassifierInner),
    Class(Class),
    DataType(DataType),
    Association(Association),
}
pub enum ClassifierRefMut<'a> {
    Itself(&'a mut ClassifierInner),
    Class(ClassRefMut<'a>),
    DataType(DataTypeRefMut<'a>),
    Association(AssociationRefMut<'a>),
}
pub enum ClassifierRef<'a> {
    Itself(&'a ClassifierInner),
    Class(ClassRef<'a>),
    DataType(DataTypeRef<'a>),
    Association(AssociationRef<'a>),
}
impl Classifier {
    pub fn as_ref(&self) -> ClassifierRef {
        match self {
            Classifier::Itself(inner) => ClassifierRef::Itself(&inner),
            Classifier::Class(inner) => ClassifierRef::Class(inner.as_ref()),
            Classifier::DataType(inner) => ClassifierRef::DataType(inner.as_ref()),
            Classifier::Association(inner) => ClassifierRef::Association(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> ClassifierRefMut {
        match self {
            Classifier::Itself(inner) => ClassifierRefMut::Itself(inner),
            Classifier::Class(inner) => ClassifierRefMut::Class(inner.as_ref_mut()),
            Classifier::DataType(inner) => ClassifierRefMut::DataType(inner.as_ref_mut()),
            Classifier::Association(inner) => {
                ClassifierRefMut::Association(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ClassifierRefMut<'a> {
    pub fn as_ref(self) -> ClassifierRef<'a> {
        match self {
            ClassifierRefMut::Itself(inner) => ClassifierRef::Itself(inner),
            ClassifierRefMut::Class(inner) => ClassifierRef::Class(inner.as_ref()),
            ClassifierRefMut::DataType(inner) => ClassifierRef::DataType(inner.as_ref()),
            ClassifierRefMut::Association(inner) => {
                ClassifierRef::Association(inner.as_ref())
            }
        }
    }
}
impl ClassifierStruct for Classifier {}
impl ClassifierStructRefMut for Classifier {}
impl ClassifierStructRef for Classifier {}
impl<'a> ClassifierStructRefMut for ClassifierRefMut<'a> {}
impl<'a> ClassifierStructRef for ClassifierRefMut<'a> {}
impl<'a> ClassifierStructRef for ClassifierRef<'a> {}
impl TypeStruct for Classifier {
    fn is_abstract(self) -> bool {
        match self {
            Classifier::Itself(x) => x.is_abstract(),
            Classifier::Class(x) => x.is_abstract(),
            Classifier::DataType(x) => x.is_abstract(),
            Classifier::Association(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Classifier::Itself(x) => x.is_sufficient(),
            Classifier::Class(x) => x.is_sufficient(),
            Classifier::DataType(x) => x.is_sufficient(),
            Classifier::Association(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Classifier {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Classifier::Itself(x) => x.is_abstract_ref_mut(),
            Classifier::Class(x) => x.is_abstract_ref_mut(),
            Classifier::DataType(x) => x.is_abstract_ref_mut(),
            Classifier::Association(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Classifier::Itself(x) => x.is_sufficient_ref_mut(),
            Classifier::Class(x) => x.is_sufficient_ref_mut(),
            Classifier::DataType(x) => x.is_sufficient_ref_mut(),
            Classifier::Association(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Classifier {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Classifier::Itself(x) => x.is_abstract_ref(),
            Classifier::Class(x) => x.is_abstract_ref(),
            Classifier::DataType(x) => x.is_abstract_ref(),
            Classifier::Association(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Classifier::Itself(x) => x.is_sufficient_ref(),
            Classifier::Class(x) => x.is_sufficient_ref(),
            Classifier::DataType(x) => x.is_sufficient_ref(),
            Classifier::Association(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for ClassifierRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_abstract_ref_mut(),
            ClassifierRefMut::Class(x) => x.is_abstract_ref_mut(),
            ClassifierRefMut::DataType(x) => x.is_abstract_ref_mut(),
            ClassifierRefMut::Association(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            ClassifierRefMut::Class(x) => x.is_sufficient_ref_mut(),
            ClassifierRefMut::DataType(x) => x.is_sufficient_ref_mut(),
            ClassifierRefMut::Association(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for ClassifierRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_abstract_ref(),
            ClassifierRefMut::Class(x) => x.is_abstract_ref(),
            ClassifierRefMut::DataType(x) => x.is_abstract_ref(),
            ClassifierRefMut::Association(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_sufficient_ref(),
            ClassifierRefMut::Class(x) => x.is_sufficient_ref(),
            ClassifierRefMut::DataType(x) => x.is_sufficient_ref(),
            ClassifierRefMut::Association(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for ClassifierRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ClassifierRef::Itself(x) => x.is_abstract_ref(),
            ClassifierRef::Class(x) => x.is_abstract_ref(),
            ClassifierRef::DataType(x) => x.is_abstract_ref(),
            ClassifierRef::Association(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ClassifierRef::Itself(x) => x.is_sufficient_ref(),
            ClassifierRef::Class(x) => x.is_sufficient_ref(),
            ClassifierRef::DataType(x) => x.is_sufficient_ref(),
            ClassifierRef::Association(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Classifier {}
impl NamespaceStructRefMut for Classifier {}
impl NamespaceStructRef for Classifier {}
impl<'a> NamespaceStructRefMut for ClassifierRefMut<'a> {}
impl<'a> NamespaceStructRef for ClassifierRefMut<'a> {}
impl<'a> NamespaceStructRef for ClassifierRef<'a> {}
impl ElementStruct for Classifier {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Classifier::Itself(x) => x.owned_relationship(),
            Classifier::Class(x) => x.owned_relationship(),
            Classifier::DataType(x) => x.owned_relationship(),
            Classifier::Association(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Classifier::Itself(x) => x.owning_relationship(),
            Classifier::Class(x) => x.owning_relationship(),
            Classifier::DataType(x) => x.owning_relationship(),
            Classifier::Association(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Classifier::Itself(x) => x.element_id(),
            Classifier::Class(x) => x.element_id(),
            Classifier::DataType(x) => x.element_id(),
            Classifier::Association(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Classifier::Itself(x) => x.alias_ids(),
            Classifier::Class(x) => x.alias_ids(),
            Classifier::DataType(x) => x.alias_ids(),
            Classifier::Association(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_short_name(),
            Classifier::Class(x) => x.declared_short_name(),
            Classifier::DataType(x) => x.declared_short_name(),
            Classifier::Association(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_name(),
            Classifier::Class(x) => x.declared_name(),
            Classifier::DataType(x) => x.declared_name(),
            Classifier::Association(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Classifier::Itself(x) => x.is_implied_included(),
            Classifier::Class(x) => x.is_implied_included(),
            Classifier::DataType(x) => x.is_implied_included(),
            Classifier::Association(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Classifier {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Classifier::Itself(x) => x.owned_relationship_ref_mut(),
            Classifier::Class(x) => x.owned_relationship_ref_mut(),
            Classifier::DataType(x) => x.owned_relationship_ref_mut(),
            Classifier::Association(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Classifier::Itself(x) => x.owning_relationship_ref_mut(),
            Classifier::Class(x) => x.owning_relationship_ref_mut(),
            Classifier::DataType(x) => x.owning_relationship_ref_mut(),
            Classifier::Association(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Classifier::Itself(x) => x.element_id_ref_mut(),
            Classifier::Class(x) => x.element_id_ref_mut(),
            Classifier::DataType(x) => x.element_id_ref_mut(),
            Classifier::Association(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Classifier::Itself(x) => x.alias_ids_ref_mut(),
            Classifier::Class(x) => x.alias_ids_ref_mut(),
            Classifier::DataType(x) => x.alias_ids_ref_mut(),
            Classifier::Association(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_short_name_ref_mut(),
            Classifier::Class(x) => x.declared_short_name_ref_mut(),
            Classifier::DataType(x) => x.declared_short_name_ref_mut(),
            Classifier::Association(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_name_ref_mut(),
            Classifier::Class(x) => x.declared_name_ref_mut(),
            Classifier::DataType(x) => x.declared_name_ref_mut(),
            Classifier::Association(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Classifier::Itself(x) => x.is_implied_included_ref_mut(),
            Classifier::Class(x) => x.is_implied_included_ref_mut(),
            Classifier::DataType(x) => x.is_implied_included_ref_mut(),
            Classifier::Association(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Classifier {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Classifier::Itself(x) => x.owned_relationship_ref(),
            Classifier::Class(x) => x.owned_relationship_ref(),
            Classifier::DataType(x) => x.owned_relationship_ref(),
            Classifier::Association(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Classifier::Itself(x) => x.owning_relationship_ref(),
            Classifier::Class(x) => x.owning_relationship_ref(),
            Classifier::DataType(x) => x.owning_relationship_ref(),
            Classifier::Association(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Classifier::Itself(x) => x.element_id_ref(),
            Classifier::Class(x) => x.element_id_ref(),
            Classifier::DataType(x) => x.element_id_ref(),
            Classifier::Association(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Classifier::Itself(x) => x.alias_ids_ref(),
            Classifier::Class(x) => x.alias_ids_ref(),
            Classifier::DataType(x) => x.alias_ids_ref(),
            Classifier::Association(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_short_name_ref(),
            Classifier::Class(x) => x.declared_short_name_ref(),
            Classifier::DataType(x) => x.declared_short_name_ref(),
            Classifier::Association(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Classifier::Itself(x) => x.declared_name_ref(),
            Classifier::Class(x) => x.declared_name_ref(),
            Classifier::DataType(x) => x.declared_name_ref(),
            Classifier::Association(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Classifier::Itself(x) => x.is_implied_included_ref(),
            Classifier::Class(x) => x.is_implied_included_ref(),
            Classifier::DataType(x) => x.is_implied_included_ref(),
            Classifier::Association(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ClassifierRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassifierRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ClassifierRefMut::Class(x) => x.owned_relationship_ref_mut(),
            ClassifierRefMut::DataType(x) => x.owned_relationship_ref_mut(),
            ClassifierRefMut::Association(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ClassifierRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ClassifierRefMut::Class(x) => x.owning_relationship_ref_mut(),
            ClassifierRefMut::DataType(x) => x.owning_relationship_ref_mut(),
            ClassifierRefMut::Association(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ClassifierRefMut::Itself(x) => x.element_id_ref_mut(),
            ClassifierRefMut::Class(x) => x.element_id_ref_mut(),
            ClassifierRefMut::DataType(x) => x.element_id_ref_mut(),
            ClassifierRefMut::Association(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ClassifierRefMut::Class(x) => x.alias_ids_ref_mut(),
            ClassifierRefMut::DataType(x) => x.alias_ids_ref_mut(),
            ClassifierRefMut::Association(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ClassifierRefMut::Class(x) => x.declared_short_name_ref_mut(),
            ClassifierRefMut::DataType(x) => x.declared_short_name_ref_mut(),
            ClassifierRefMut::Association(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.declared_name_ref_mut(),
            ClassifierRefMut::Class(x) => x.declared_name_ref_mut(),
            ClassifierRefMut::DataType(x) => x.declared_name_ref_mut(),
            ClassifierRefMut::Association(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ClassifierRefMut::Class(x) => x.is_implied_included_ref_mut(),
            ClassifierRefMut::DataType(x) => x.is_implied_included_ref_mut(),
            ClassifierRefMut::Association(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ClassifierRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassifierRefMut::Itself(x) => x.owned_relationship_ref(),
            ClassifierRefMut::Class(x) => x.owned_relationship_ref(),
            ClassifierRefMut::DataType(x) => x.owned_relationship_ref(),
            ClassifierRefMut::Association(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassifierRefMut::Itself(x) => x.owning_relationship_ref(),
            ClassifierRefMut::Class(x) => x.owning_relationship_ref(),
            ClassifierRefMut::DataType(x) => x.owning_relationship_ref(),
            ClassifierRefMut::Association(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ClassifierRefMut::Itself(x) => x.element_id_ref(),
            ClassifierRefMut::Class(x) => x.element_id_ref(),
            ClassifierRefMut::DataType(x) => x.element_id_ref(),
            ClassifierRefMut::Association(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.alias_ids_ref(),
            ClassifierRefMut::Class(x) => x.alias_ids_ref(),
            ClassifierRefMut::DataType(x) => x.alias_ids_ref(),
            ClassifierRefMut::Association(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.declared_short_name_ref(),
            ClassifierRefMut::Class(x) => x.declared_short_name_ref(),
            ClassifierRefMut::DataType(x) => x.declared_short_name_ref(),
            ClassifierRefMut::Association(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ClassifierRefMut::Itself(x) => x.declared_name_ref(),
            ClassifierRefMut::Class(x) => x.declared_name_ref(),
            ClassifierRefMut::DataType(x) => x.declared_name_ref(),
            ClassifierRefMut::Association(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ClassifierRefMut::Itself(x) => x.is_implied_included_ref(),
            ClassifierRefMut::Class(x) => x.is_implied_included_ref(),
            ClassifierRefMut::DataType(x) => x.is_implied_included_ref(),
            ClassifierRefMut::Association(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ClassifierRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassifierRef::Itself(x) => x.owned_relationship_ref(),
            ClassifierRef::Class(x) => x.owned_relationship_ref(),
            ClassifierRef::DataType(x) => x.owned_relationship_ref(),
            ClassifierRef::Association(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ClassifierRef::Itself(x) => x.owning_relationship_ref(),
            ClassifierRef::Class(x) => x.owning_relationship_ref(),
            ClassifierRef::DataType(x) => x.owning_relationship_ref(),
            ClassifierRef::Association(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ClassifierRef::Itself(x) => x.element_id_ref(),
            ClassifierRef::Class(x) => x.element_id_ref(),
            ClassifierRef::DataType(x) => x.element_id_ref(),
            ClassifierRef::Association(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ClassifierRef::Itself(x) => x.alias_ids_ref(),
            ClassifierRef::Class(x) => x.alias_ids_ref(),
            ClassifierRef::DataType(x) => x.alias_ids_ref(),
            ClassifierRef::Association(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ClassifierRef::Itself(x) => x.declared_short_name_ref(),
            ClassifierRef::Class(x) => x.declared_short_name_ref(),
            ClassifierRef::DataType(x) => x.declared_short_name_ref(),
            ClassifierRef::Association(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ClassifierRef::Itself(x) => x.declared_name_ref(),
            ClassifierRef::Class(x) => x.declared_name_ref(),
            ClassifierRef::DataType(x) => x.declared_name_ref(),
            ClassifierRef::Association(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ClassifierRef::Itself(x) => x.is_implied_included_ref(),
            ClassifierRef::Class(x) => x.is_implied_included_ref(),
            ClassifierRef::DataType(x) => x.is_implied_included_ref(),
            ClassifierRef::Association(x) => x.is_implied_included_ref(),
        }
    }
}
impl ClassifierUpcast for Classifier {
    fn into_classifier(self) -> Classifier {
        self
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for ClassifierRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        self
    }
}
impl<'a> ClassifierUpcastRef<'a> for ClassifierRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        self
    }
}
impl TypeUpcast for Classifier {
    fn into_type_(self) -> Type {
        Type::Classifier(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for ClassifierRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        TypeRefMut::Classifier(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for ClassifierRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        TypeRef::Classifier(self).as_type_ref()
    }
}
impl NamespaceUpcast for Classifier {
    fn into_namespace(self) -> Namespace {
        Type::Classifier(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for ClassifierRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        TypeRefMut::Classifier(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for ClassifierRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        TypeRef::Classifier(self).as_namespace_ref()
    }
}
impl ElementUpcast for Classifier {
    fn into_element(self) -> Element {
        Type::Classifier(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ClassifierRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        TypeRefMut::Classifier(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ClassifierRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        TypeRef::Classifier(self).as_element_ref()
    }
}
pub trait ClassifierDowncast {
    fn try_into_class(self) -> Result<Class, String>;
    fn try_into_data_type(self) -> Result<DataType, String>;
    fn try_into_association(self) -> Result<Association, String>;
}
pub trait ClassifierDowncastRefMut<'a> {
    fn try_as_class_ref_mut(self) -> Result<ClassRefMut<'a>, String>;
    fn try_as_data_type_ref_mut(self) -> Result<DataTypeRefMut<'a>, String>;
    fn try_as_association_ref_mut(self) -> Result<AssociationRefMut<'a>, String>;
}
pub trait ClassifierDowncastRef<'a> {
    fn try_as_class_ref(self) -> Result<ClassRef<'a>, String>;
    fn try_as_data_type_ref(self) -> Result<DataTypeRef<'a>, String>;
    fn try_as_association_ref(self) -> Result<AssociationRef<'a>, String>;
}
impl ClassifierDowncast for Classifier {
    fn try_into_class(self) -> Result<Class, String> {
        match self {
            Classifier::Class(e) => Ok(e),
            _ => Err("Not a Class".into()),
        }
    }
    fn try_into_data_type(self) -> Result<DataType, String> {
        match self {
            Classifier::DataType(e) => Ok(e),
            _ => Err("Not a DataType".into()),
        }
    }
    fn try_into_association(self) -> Result<Association, String> {
        match self {
            Classifier::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
}
impl<'a> ClassifierDowncastRefMut<'a> for ClassifierRefMut<'a> {
    fn try_as_class_ref_mut(self) -> Result<ClassRefMut<'a>, String> {
        match self {
            ClassifierRefMut::Class(e) => Ok(e),
            _ => Err("Not a Class".into()),
        }
    }
    fn try_as_data_type_ref_mut(self) -> Result<DataTypeRefMut<'a>, String> {
        match self {
            ClassifierRefMut::DataType(e) => Ok(e),
            _ => Err("Not a DataType".into()),
        }
    }
    fn try_as_association_ref_mut(self) -> Result<AssociationRefMut<'a>, String> {
        match self {
            ClassifierRefMut::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
}
impl<'a> ClassifierDowncastRef<'a> for ClassifierRef<'a> {
    fn try_as_class_ref(self) -> Result<ClassRef<'a>, String> {
        match self {
            ClassifierRef::Class(e) => Ok(e),
            _ => Err("Not a Class".into()),
        }
    }
    fn try_as_data_type_ref(self) -> Result<DataTypeRef<'a>, String> {
        match self {
            ClassifierRef::DataType(e) => Ok(e),
            _ => Err("Not a DataType".into()),
        }
    }
    fn try_as_association_ref(self) -> Result<AssociationRef<'a>, String> {
        match self {
            ClassifierRef::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
}
pub trait ClassifierMethodsDescendants
where
    Self: DescendantOf<Classifier>,
    Self::Via: ClassifierMethods,
    Self: Sized,
{}
pub trait ClassifierMethods: Sized {}
impl<T: ClassifierMethodsDescendants> ClassifierMethods for T
where
    T::Via: ClassifierMethods,
{}
impl DescendantOf<Type> for Classifier {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl TypeMethodsDescendants for Classifier {}
impl DescendantOf<Namespace> for Classifier {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl NamespaceMethodsDescendants for Classifier {}
impl DescendantOf<Element> for Classifier {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl ElementMethodsDescendants for Classifier {}
pub trait ClassifierRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ClassifierRefMut<'a>>,
    Self::Via: ClassifierRefMutMethods,
    Self: Sized,
{}
pub trait ClassifierRefMutMethods: Sized {}
impl<'a, T: ClassifierRefMutMethodsDescendants<'a>> ClassifierRefMutMethods for T
where
    T::Via: ClassifierRefMutMethods,
{}
impl<'a> DescendantOf<TypeRefMut<'a>> for ClassifierRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for ClassifierRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for ClassifierRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for ClassifierRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ClassifierRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ClassifierRefMut<'a> {}
pub trait ClassifierRefMethodsDescendants<'a>
where
    Self: DescendantOf<ClassifierRef<'a>>,
    Self::Via: ClassifierRefMethods,
    Self: Sized,
{}
pub trait ClassifierRefMethods: Sized {}
impl<'a, T: ClassifierRefMethodsDescendants<'a>> ClassifierRefMethods for T
where
    T::Via: ClassifierRefMethods,
{}
impl<'a> DescendantOf<TypeRef<'a>> for ClassifierRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for ClassifierRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for ClassifierRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for ClassifierRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ClassifierRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ClassifierRef<'a> {}

