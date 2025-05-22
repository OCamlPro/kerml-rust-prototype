#![allow(unused)]
use super::utils::DescendantOf;
use super::class::{
    Class, ClassRefMut, ClassRef, ClassStruct, ClassStructRefMut, ClassStructRef,
    ClassInner, ClassUpcast, ClassUpcastRefMut, ClassUpcastRef, ClassMethodsDescendants,
    ClassRefMutMethodsDescendants, ClassRefMethodsDescendants,
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
use super::metaclass::{Metaclass, MetaclassRefMut, MetaclassRef};
use super::association_structure::{
    AssociationStructure, AssociationStructureRefMut, AssociationStructureRef,
};
pub struct StructureInner {
    pub(super) sup_class: ClassInner,
}
pub trait StructureStruct
where
    Self: StructureStructRefMut,
    Self: StructureStructRef,
    Self: ClassStruct,
{}
pub trait StructureStructRefMut
where
    Self: StructureStructRef,
    Self: ClassStructRefMut,
{}
pub trait StructureStructRef
where
    Self: ClassStructRef,
{}
pub trait StructureUpcast: StructureStruct {
    fn into_structure(self) -> Structure;
}
pub trait StructureUpcastRefMut<'a>: StructureStructRefMut {
    fn as_structure_ref_mut(self) -> StructureRefMut<'a>;
}
pub trait StructureUpcastRef<'a>: StructureStructRef {
    fn as_structure_ref(self) -> StructureRef<'a>;
}
impl StructureStruct for StructureInner {}
impl StructureStructRefMut for StructureInner {}
impl StructureStructRef for StructureInner {}
impl ClassStruct for StructureInner {}
impl ClassStructRefMut for StructureInner {}
impl ClassStructRef for StructureInner {}
impl ClassifierStruct for StructureInner {}
impl ClassifierStructRefMut for StructureInner {}
impl ClassifierStructRef for StructureInner {}
impl TypeStruct for StructureInner {
    fn is_abstract(self) -> bool {
        self.sup_class.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_class.is_sufficient()
    }
}
impl TypeStructRefMut for StructureInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for StructureInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_class.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_class.is_sufficient_ref()
    }
}
impl NamespaceStruct for StructureInner {}
impl NamespaceStructRefMut for StructureInner {}
impl NamespaceStructRef for StructureInner {}
impl ElementStruct for StructureInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_class.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_class.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_class.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_class.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_class.is_implied_included()
    }
}
impl ElementStructRefMut for StructureInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_class.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_class.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_class.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_class.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_class.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_class.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for StructureInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_class.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_class.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_class.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_class.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_class.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_class.is_implied_included_ref()
    }
}
pub enum Structure {
    Itself(StructureInner),
    Metaclass(Metaclass),
    AssociationStructure(AssociationStructure),
}
pub enum StructureRefMut<'a> {
    Itself(&'a mut StructureInner),
    Metaclass(MetaclassRefMut<'a>),
    AssociationStructure(AssociationStructureRefMut<'a>),
}
pub enum StructureRef<'a> {
    Itself(&'a StructureInner),
    Metaclass(MetaclassRef<'a>),
    AssociationStructure(AssociationStructureRef<'a>),
}
impl Structure {
    pub fn as_ref(&self) -> StructureRef {
        match self {
            Structure::Itself(inner) => StructureRef::Itself(&inner),
            Structure::Metaclass(inner) => StructureRef::Metaclass(inner.as_ref()),
            Structure::AssociationStructure(inner) => {
                StructureRef::AssociationStructure(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> StructureRefMut {
        match self {
            Structure::Itself(inner) => StructureRefMut::Itself(inner),
            Structure::Metaclass(inner) => StructureRefMut::Metaclass(inner.as_ref_mut()),
            Structure::AssociationStructure(inner) => {
                StructureRefMut::AssociationStructure(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> StructureRefMut<'a> {
    pub fn as_ref(self) -> StructureRef<'a> {
        match self {
            StructureRefMut::Itself(inner) => StructureRef::Itself(inner),
            StructureRefMut::Metaclass(inner) => StructureRef::Metaclass(inner.as_ref()),
            StructureRefMut::AssociationStructure(inner) => {
                StructureRef::AssociationStructure(inner.as_ref())
            }
        }
    }
}
impl StructureStruct for Structure {}
impl StructureStructRefMut for Structure {}
impl StructureStructRef for Structure {}
impl<'a> StructureStructRefMut for StructureRefMut<'a> {}
impl<'a> StructureStructRef for StructureRefMut<'a> {}
impl<'a> StructureStructRef for StructureRef<'a> {}
impl ClassStruct for Structure {}
impl ClassStructRefMut for Structure {}
impl ClassStructRef for Structure {}
impl<'a> ClassStructRefMut for StructureRefMut<'a> {}
impl<'a> ClassStructRef for StructureRefMut<'a> {}
impl<'a> ClassStructRef for StructureRef<'a> {}
impl ClassifierStruct for Structure {}
impl ClassifierStructRefMut for Structure {}
impl ClassifierStructRef for Structure {}
impl<'a> ClassifierStructRefMut for StructureRefMut<'a> {}
impl<'a> ClassifierStructRef for StructureRefMut<'a> {}
impl<'a> ClassifierStructRef for StructureRef<'a> {}
impl TypeStruct for Structure {
    fn is_abstract(self) -> bool {
        match self {
            Structure::Itself(x) => x.is_abstract(),
            Structure::Metaclass(x) => x.is_abstract(),
            Structure::AssociationStructure(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Structure::Itself(x) => x.is_sufficient(),
            Structure::Metaclass(x) => x.is_sufficient(),
            Structure::AssociationStructure(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Structure {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Structure::Itself(x) => x.is_abstract_ref_mut(),
            Structure::Metaclass(x) => x.is_abstract_ref_mut(),
            Structure::AssociationStructure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Structure::Itself(x) => x.is_sufficient_ref_mut(),
            Structure::Metaclass(x) => x.is_sufficient_ref_mut(),
            Structure::AssociationStructure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Structure {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Structure::Itself(x) => x.is_abstract_ref(),
            Structure::Metaclass(x) => x.is_abstract_ref(),
            Structure::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Structure::Itself(x) => x.is_sufficient_ref(),
            Structure::Metaclass(x) => x.is_sufficient_ref(),
            Structure::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for StructureRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            StructureRefMut::Itself(x) => x.is_abstract_ref_mut(),
            StructureRefMut::Metaclass(x) => x.is_abstract_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            StructureRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            StructureRefMut::Metaclass(x) => x.is_sufficient_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for StructureRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            StructureRefMut::Itself(x) => x.is_abstract_ref(),
            StructureRefMut::Metaclass(x) => x.is_abstract_ref(),
            StructureRefMut::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            StructureRefMut::Itself(x) => x.is_sufficient_ref(),
            StructureRefMut::Metaclass(x) => x.is_sufficient_ref(),
            StructureRefMut::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for StructureRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            StructureRef::Itself(x) => x.is_abstract_ref(),
            StructureRef::Metaclass(x) => x.is_abstract_ref(),
            StructureRef::AssociationStructure(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            StructureRef::Itself(x) => x.is_sufficient_ref(),
            StructureRef::Metaclass(x) => x.is_sufficient_ref(),
            StructureRef::AssociationStructure(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Structure {}
impl NamespaceStructRefMut for Structure {}
impl NamespaceStructRef for Structure {}
impl<'a> NamespaceStructRefMut for StructureRefMut<'a> {}
impl<'a> NamespaceStructRef for StructureRefMut<'a> {}
impl<'a> NamespaceStructRef for StructureRef<'a> {}
impl ElementStruct for Structure {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Structure::Itself(x) => x.owned_relationship(),
            Structure::Metaclass(x) => x.owned_relationship(),
            Structure::AssociationStructure(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Structure::Itself(x) => x.owning_relationship(),
            Structure::Metaclass(x) => x.owning_relationship(),
            Structure::AssociationStructure(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Structure::Itself(x) => x.element_id(),
            Structure::Metaclass(x) => x.element_id(),
            Structure::AssociationStructure(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Structure::Itself(x) => x.alias_ids(),
            Structure::Metaclass(x) => x.alias_ids(),
            Structure::AssociationStructure(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Structure::Itself(x) => x.declared_short_name(),
            Structure::Metaclass(x) => x.declared_short_name(),
            Structure::AssociationStructure(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Structure::Itself(x) => x.declared_name(),
            Structure::Metaclass(x) => x.declared_name(),
            Structure::AssociationStructure(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Structure::Itself(x) => x.is_implied_included(),
            Structure::Metaclass(x) => x.is_implied_included(),
            Structure::AssociationStructure(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Structure {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Structure::Itself(x) => x.owned_relationship_ref_mut(),
            Structure::Metaclass(x) => x.owned_relationship_ref_mut(),
            Structure::AssociationStructure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Structure::Itself(x) => x.owning_relationship_ref_mut(),
            Structure::Metaclass(x) => x.owning_relationship_ref_mut(),
            Structure::AssociationStructure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Structure::Itself(x) => x.element_id_ref_mut(),
            Structure::Metaclass(x) => x.element_id_ref_mut(),
            Structure::AssociationStructure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Structure::Itself(x) => x.alias_ids_ref_mut(),
            Structure::Metaclass(x) => x.alias_ids_ref_mut(),
            Structure::AssociationStructure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Structure::Itself(x) => x.declared_short_name_ref_mut(),
            Structure::Metaclass(x) => x.declared_short_name_ref_mut(),
            Structure::AssociationStructure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Structure::Itself(x) => x.declared_name_ref_mut(),
            Structure::Metaclass(x) => x.declared_name_ref_mut(),
            Structure::AssociationStructure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Structure::Itself(x) => x.is_implied_included_ref_mut(),
            Structure::Metaclass(x) => x.is_implied_included_ref_mut(),
            Structure::AssociationStructure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Structure {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Structure::Itself(x) => x.owned_relationship_ref(),
            Structure::Metaclass(x) => x.owned_relationship_ref(),
            Structure::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Structure::Itself(x) => x.owning_relationship_ref(),
            Structure::Metaclass(x) => x.owning_relationship_ref(),
            Structure::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Structure::Itself(x) => x.element_id_ref(),
            Structure::Metaclass(x) => x.element_id_ref(),
            Structure::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Structure::Itself(x) => x.alias_ids_ref(),
            Structure::Metaclass(x) => x.alias_ids_ref(),
            Structure::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Structure::Itself(x) => x.declared_short_name_ref(),
            Structure::Metaclass(x) => x.declared_short_name_ref(),
            Structure::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Structure::Itself(x) => x.declared_name_ref(),
            Structure::Metaclass(x) => x.declared_name_ref(),
            Structure::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Structure::Itself(x) => x.is_implied_included_ref(),
            Structure::Metaclass(x) => x.is_implied_included_ref(),
            Structure::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for StructureRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StructureRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            StructureRefMut::Metaclass(x) => x.owned_relationship_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            StructureRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            StructureRefMut::Metaclass(x) => x.owning_relationship_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            StructureRefMut::Itself(x) => x.element_id_ref_mut(),
            StructureRefMut::Metaclass(x) => x.element_id_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            StructureRefMut::Itself(x) => x.alias_ids_ref_mut(),
            StructureRefMut::Metaclass(x) => x.alias_ids_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            StructureRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            StructureRefMut::Metaclass(x) => x.declared_short_name_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            StructureRefMut::Itself(x) => x.declared_name_ref_mut(),
            StructureRefMut::Metaclass(x) => x.declared_name_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            StructureRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            StructureRefMut::Metaclass(x) => x.is_implied_included_ref_mut(),
            StructureRefMut::AssociationStructure(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for StructureRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StructureRefMut::Itself(x) => x.owned_relationship_ref(),
            StructureRefMut::Metaclass(x) => x.owned_relationship_ref(),
            StructureRefMut::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StructureRefMut::Itself(x) => x.owning_relationship_ref(),
            StructureRefMut::Metaclass(x) => x.owning_relationship_ref(),
            StructureRefMut::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            StructureRefMut::Itself(x) => x.element_id_ref(),
            StructureRefMut::Metaclass(x) => x.element_id_ref(),
            StructureRefMut::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            StructureRefMut::Itself(x) => x.alias_ids_ref(),
            StructureRefMut::Metaclass(x) => x.alias_ids_ref(),
            StructureRefMut::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            StructureRefMut::Itself(x) => x.declared_short_name_ref(),
            StructureRefMut::Metaclass(x) => x.declared_short_name_ref(),
            StructureRefMut::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            StructureRefMut::Itself(x) => x.declared_name_ref(),
            StructureRefMut::Metaclass(x) => x.declared_name_ref(),
            StructureRefMut::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            StructureRefMut::Itself(x) => x.is_implied_included_ref(),
            StructureRefMut::Metaclass(x) => x.is_implied_included_ref(),
            StructureRefMut::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for StructureRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StructureRef::Itself(x) => x.owned_relationship_ref(),
            StructureRef::Metaclass(x) => x.owned_relationship_ref(),
            StructureRef::AssociationStructure(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StructureRef::Itself(x) => x.owning_relationship_ref(),
            StructureRef::Metaclass(x) => x.owning_relationship_ref(),
            StructureRef::AssociationStructure(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            StructureRef::Itself(x) => x.element_id_ref(),
            StructureRef::Metaclass(x) => x.element_id_ref(),
            StructureRef::AssociationStructure(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            StructureRef::Itself(x) => x.alias_ids_ref(),
            StructureRef::Metaclass(x) => x.alias_ids_ref(),
            StructureRef::AssociationStructure(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            StructureRef::Itself(x) => x.declared_short_name_ref(),
            StructureRef::Metaclass(x) => x.declared_short_name_ref(),
            StructureRef::AssociationStructure(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            StructureRef::Itself(x) => x.declared_name_ref(),
            StructureRef::Metaclass(x) => x.declared_name_ref(),
            StructureRef::AssociationStructure(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            StructureRef::Itself(x) => x.is_implied_included_ref(),
            StructureRef::Metaclass(x) => x.is_implied_included_ref(),
            StructureRef::AssociationStructure(x) => x.is_implied_included_ref(),
        }
    }
}
impl StructureUpcast for Structure {
    fn into_structure(self) -> Structure {
        self
    }
}
impl<'a> StructureUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_structure_ref_mut(self) -> StructureRefMut<'a> {
        self
    }
}
impl<'a> StructureUpcastRef<'a> for StructureRef<'a> {
    fn as_structure_ref(self) -> StructureRef<'a> {
        self
    }
}
impl ClassUpcast for Structure {
    fn into_class(self) -> Class {
        Class::Structure(self).into_class()
    }
}
impl<'a> ClassUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_class_ref_mut(self) -> ClassRefMut<'a> {
        ClassRefMut::Structure(self).as_class_ref_mut()
    }
}
impl<'a> ClassUpcastRef<'a> for StructureRef<'a> {
    fn as_class_ref(self) -> ClassRef<'a> {
        ClassRef::Structure(self).as_class_ref()
    }
}
impl ClassifierUpcast for Structure {
    fn into_classifier(self) -> Classifier {
        Class::Structure(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        ClassRefMut::Structure(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for StructureRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        ClassRef::Structure(self).as_classifier_ref()
    }
}
impl TypeUpcast for Structure {
    fn into_type_(self) -> Type {
        Class::Structure(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ClassRefMut::Structure(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for StructureRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ClassRef::Structure(self).as_type_ref()
    }
}
impl NamespaceUpcast for Structure {
    fn into_namespace(self) -> Namespace {
        Class::Structure(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ClassRefMut::Structure(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for StructureRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ClassRef::Structure(self).as_namespace_ref()
    }
}
impl ElementUpcast for Structure {
    fn into_element(self) -> Element {
        Class::Structure(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for StructureRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ClassRefMut::Structure(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for StructureRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ClassRef::Structure(self).as_element_ref()
    }
}
pub trait StructureDowncast {
    fn try_into_metaclass(self) -> Result<Metaclass, String>;
    fn try_into_association_structure(self) -> Result<AssociationStructure, String>;
}
pub trait StructureDowncastRefMut<'a> {
    fn try_as_metaclass_ref_mut(self) -> Result<MetaclassRefMut<'a>, String>;
    fn try_as_association_structure_ref_mut(
        self,
    ) -> Result<AssociationStructureRefMut<'a>, String>;
}
pub trait StructureDowncastRef<'a> {
    fn try_as_metaclass_ref(self) -> Result<MetaclassRef<'a>, String>;
    fn try_as_association_structure_ref(
        self,
    ) -> Result<AssociationStructureRef<'a>, String>;
}
impl StructureDowncast for Structure {
    fn try_into_metaclass(self) -> Result<Metaclass, String> {
        match self {
            Structure::Metaclass(e) => Ok(e),
            _ => Err("Not a Metaclass".into()),
        }
    }
    fn try_into_association_structure(self) -> Result<AssociationStructure, String> {
        match self {
            Structure::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
impl<'a> StructureDowncastRefMut<'a> for StructureRefMut<'a> {
    fn try_as_metaclass_ref_mut(self) -> Result<MetaclassRefMut<'a>, String> {
        match self {
            StructureRefMut::Metaclass(e) => Ok(e),
            _ => Err("Not a Metaclass".into()),
        }
    }
    fn try_as_association_structure_ref_mut(
        self,
    ) -> Result<AssociationStructureRefMut<'a>, String> {
        match self {
            StructureRefMut::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
impl<'a> StructureDowncastRef<'a> for StructureRef<'a> {
    fn try_as_metaclass_ref(self) -> Result<MetaclassRef<'a>, String> {
        match self {
            StructureRef::Metaclass(e) => Ok(e),
            _ => Err("Not a Metaclass".into()),
        }
    }
    fn try_as_association_structure_ref(
        self,
    ) -> Result<AssociationStructureRef<'a>, String> {
        match self {
            StructureRef::AssociationStructure(e) => Ok(e),
            _ => Err("Not a AssociationStructure".into()),
        }
    }
}
pub trait StructureMethodsDescendants
where
    Self: DescendantOf<Structure>,
    Self::Via: StructureMethods,
    Self: Sized,
{}
pub trait StructureMethods: Sized {}
impl<T: StructureMethodsDescendants> StructureMethods for T
where
    T::Via: StructureMethods,
{}
impl DescendantOf<Class> for Structure {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ClassMethodsDescendants for Structure {}
impl DescendantOf<Classifier> for Structure {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ClassifierMethodsDescendants for Structure {}
impl DescendantOf<Type> for Structure {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl TypeMethodsDescendants for Structure {}
impl DescendantOf<Namespace> for Structure {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl NamespaceMethodsDescendants for Structure {}
impl DescendantOf<Element> for Structure {
    type Via = Class;
    fn into_via(self) -> Self::Via {
        self.into_class()
    }
}
impl ElementMethodsDescendants for Structure {}
pub trait StructureRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<StructureRefMut<'a>>,
    Self::Via: StructureRefMutMethods,
    Self: Sized,
{}
pub trait StructureRefMutMethods: Sized {}
impl<'a, T: StructureRefMutMethodsDescendants<'a>> StructureRefMutMethods for T
where
    T::Via: StructureRefMutMethods,
{}
impl<'a> DescendantOf<ClassRefMut<'a>> for StructureRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ClassRefMutMethodsDescendants<'a> for StructureRefMut<'a> {}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for StructureRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for StructureRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for StructureRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for StructureRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for StructureRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for StructureRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for StructureRefMut<'a> {
    type Via = ClassRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for StructureRefMut<'a> {}
pub trait StructureRefMethodsDescendants<'a>
where
    Self: DescendantOf<StructureRef<'a>>,
    Self::Via: StructureRefMethods,
    Self: Sized,
{}
pub trait StructureRefMethods: Sized {}
impl<'a, T: StructureRefMethodsDescendants<'a>> StructureRefMethods for T
where
    T::Via: StructureRefMethods,
{}
impl<'a> DescendantOf<ClassRef<'a>> for StructureRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ClassRefMethodsDescendants<'a> for StructureRef<'a> {}
impl<'a> DescendantOf<ClassifierRef<'a>> for StructureRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for StructureRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for StructureRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for StructureRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for StructureRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for StructureRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for StructureRef<'a> {
    type Via = ClassRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_class_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for StructureRef<'a> {}

