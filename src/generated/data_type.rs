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
pub struct DataTypeInner {
    pub(super) sup_classifier: ClassifierInner,
}
pub trait DataTypeStruct
where
    Self: DataTypeStructRefMut,
    Self: DataTypeStructRef,
    Self: ClassifierStruct,
{}
pub trait DataTypeStructRefMut
where
    Self: DataTypeStructRef,
    Self: ClassifierStructRefMut,
{}
pub trait DataTypeStructRef
where
    Self: ClassifierStructRef,
{}
pub trait DataTypeUpcast: DataTypeStruct {
    fn into_data_type(self) -> DataType;
}
pub trait DataTypeUpcastRefMut<'a>: DataTypeStructRefMut {
    fn as_data_type_ref_mut(self) -> DataTypeRefMut<'a>;
}
pub trait DataTypeUpcastRef<'a>: DataTypeStructRef {
    fn as_data_type_ref(self) -> DataTypeRef<'a>;
}
impl DataTypeStruct for DataTypeInner {}
impl DataTypeStructRefMut for DataTypeInner {}
impl DataTypeStructRef for DataTypeInner {}
impl ClassifierStruct for DataTypeInner {}
impl ClassifierStructRefMut for DataTypeInner {}
impl ClassifierStructRef for DataTypeInner {}
impl TypeStruct for DataTypeInner {
    fn is_abstract(self) -> bool {
        self.sup_classifier.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_classifier.is_sufficient()
    }
}
impl TypeStructRefMut for DataTypeInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_classifier.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for DataTypeInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_classifier.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_classifier.is_sufficient_ref()
    }
}
impl NamespaceStruct for DataTypeInner {}
impl NamespaceStructRefMut for DataTypeInner {}
impl NamespaceStructRef for DataTypeInner {}
impl ElementStruct for DataTypeInner {
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
impl ElementStructRefMut for DataTypeInner {
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
impl ElementStructRef for DataTypeInner {
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
pub enum DataType {
    Itself(DataTypeInner),
}
pub enum DataTypeRefMut<'a> {
    Itself(&'a mut DataTypeInner),
}
pub enum DataTypeRef<'a> {
    Itself(&'a DataTypeInner),
}
impl DataType {
    pub fn as_ref(&self) -> DataTypeRef {
        match self {
            DataType::Itself(inner) => DataTypeRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> DataTypeRefMut {
        match self {
            DataType::Itself(inner) => DataTypeRefMut::Itself(inner),
        }
    }
}
impl<'a> DataTypeRefMut<'a> {
    pub fn as_ref(self) -> DataTypeRef<'a> {
        match self {
            DataTypeRefMut::Itself(inner) => DataTypeRef::Itself(inner),
        }
    }
}
impl DataTypeStruct for DataType {}
impl DataTypeStructRefMut for DataType {}
impl DataTypeStructRef for DataType {}
impl<'a> DataTypeStructRefMut for DataTypeRefMut<'a> {}
impl<'a> DataTypeStructRef for DataTypeRefMut<'a> {}
impl<'a> DataTypeStructRef for DataTypeRef<'a> {}
impl ClassifierStruct for DataType {}
impl ClassifierStructRefMut for DataType {}
impl ClassifierStructRef for DataType {}
impl<'a> ClassifierStructRefMut for DataTypeRefMut<'a> {}
impl<'a> ClassifierStructRef for DataTypeRefMut<'a> {}
impl<'a> ClassifierStructRef for DataTypeRef<'a> {}
impl TypeStruct for DataType {
    fn is_abstract(self) -> bool {
        match self {
            DataType::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            DataType::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for DataType {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            DataType::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            DataType::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for DataType {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            DataType::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            DataType::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for DataTypeRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for DataTypeRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for DataTypeRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            DataTypeRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            DataTypeRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for DataType {}
impl NamespaceStructRefMut for DataType {}
impl NamespaceStructRef for DataType {}
impl<'a> NamespaceStructRefMut for DataTypeRefMut<'a> {}
impl<'a> NamespaceStructRef for DataTypeRefMut<'a> {}
impl<'a> NamespaceStructRef for DataTypeRef<'a> {}
impl ElementStruct for DataType {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataType::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataType::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            DataType::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            DataType::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            DataType::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            DataType::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            DataType::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for DataType {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataType::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            DataType::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            DataType::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            DataType::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DataType::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DataType::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            DataType::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for DataType {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataType::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataType::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DataType::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DataType::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DataType::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DataType::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DataType::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for DataTypeRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataTypeRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            DataTypeRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            DataTypeRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for DataTypeRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataTypeRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataTypeRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DataTypeRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DataTypeRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DataTypeRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for DataTypeRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataTypeRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            DataTypeRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            DataTypeRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            DataTypeRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            DataTypeRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            DataTypeRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            DataTypeRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl DataTypeUpcast for DataType {
    fn into_data_type(self) -> DataType {
        self
    }
}
impl<'a> DataTypeUpcastRefMut<'a> for DataTypeRefMut<'a> {
    fn as_data_type_ref_mut(self) -> DataTypeRefMut<'a> {
        self
    }
}
impl<'a> DataTypeUpcastRef<'a> for DataTypeRef<'a> {
    fn as_data_type_ref(self) -> DataTypeRef<'a> {
        self
    }
}
impl ClassifierUpcast for DataType {
    fn into_classifier(self) -> Classifier {
        Classifier::DataType(self).into_classifier()
    }
}
impl<'a> ClassifierUpcastRefMut<'a> for DataTypeRefMut<'a> {
    fn as_classifier_ref_mut(self) -> ClassifierRefMut<'a> {
        ClassifierRefMut::DataType(self).as_classifier_ref_mut()
    }
}
impl<'a> ClassifierUpcastRef<'a> for DataTypeRef<'a> {
    fn as_classifier_ref(self) -> ClassifierRef<'a> {
        ClassifierRef::DataType(self).as_classifier_ref()
    }
}
impl TypeUpcast for DataType {
    fn into_type_(self) -> Type {
        Classifier::DataType(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for DataTypeRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ClassifierRefMut::DataType(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for DataTypeRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ClassifierRef::DataType(self).as_type_ref()
    }
}
impl NamespaceUpcast for DataType {
    fn into_namespace(self) -> Namespace {
        Classifier::DataType(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for DataTypeRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ClassifierRefMut::DataType(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for DataTypeRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ClassifierRef::DataType(self).as_namespace_ref()
    }
}
impl ElementUpcast for DataType {
    fn into_element(self) -> Element {
        Classifier::DataType(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for DataTypeRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ClassifierRefMut::DataType(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for DataTypeRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ClassifierRef::DataType(self).as_element_ref()
    }
}
pub trait DataTypeDowncast {}
pub trait DataTypeDowncastRefMut<'a> {}
pub trait DataTypeDowncastRef<'a> {}
impl DataTypeDowncast for DataType {}
impl<'a> DataTypeDowncastRefMut<'a> for DataTypeRefMut<'a> {}
impl<'a> DataTypeDowncastRef<'a> for DataTypeRef<'a> {}
pub trait DataTypeMethodsDescendants
where
    Self: DescendantOf<DataType>,
    Self::Via: DataTypeMethods,
    Self: Sized,
{}
pub trait DataTypeMethods: Sized {}
impl<T: DataTypeMethodsDescendants> DataTypeMethods for T
where
    T::Via: DataTypeMethods,
{}
impl DescendantOf<Classifier> for DataType {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl ClassifierMethodsDescendants for DataType {}
impl DescendantOf<Type> for DataType {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl TypeMethodsDescendants for DataType {}
impl DescendantOf<Namespace> for DataType {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl NamespaceMethodsDescendants for DataType {}
impl DescendantOf<Element> for DataType {
    type Via = Classifier;
    fn into_via(self) -> Self::Via {
        self.into_classifier()
    }
}
impl ElementMethodsDescendants for DataType {}
pub trait DataTypeRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<DataTypeRefMut<'a>>,
    Self::Via: DataTypeRefMutMethods,
    Self: Sized,
{}
pub trait DataTypeRefMutMethods: Sized {}
impl<'a, T: DataTypeRefMutMethodsDescendants<'a>> DataTypeRefMutMethods for T
where
    T::Via: DataTypeRefMutMethods,
{}
impl<'a> DescendantOf<ClassifierRefMut<'a>> for DataTypeRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> ClassifierRefMutMethodsDescendants<'a> for DataTypeRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for DataTypeRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for DataTypeRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for DataTypeRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for DataTypeRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for DataTypeRefMut<'a> {
    type Via = ClassifierRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for DataTypeRefMut<'a> {}
pub trait DataTypeRefMethodsDescendants<'a>
where
    Self: DescendantOf<DataTypeRef<'a>>,
    Self::Via: DataTypeRefMethods,
    Self: Sized,
{}
pub trait DataTypeRefMethods: Sized {}
impl<'a, T: DataTypeRefMethodsDescendants<'a>> DataTypeRefMethods for T
where
    T::Via: DataTypeRefMethods,
{}
impl<'a> DescendantOf<ClassifierRef<'a>> for DataTypeRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> ClassifierRefMethodsDescendants<'a> for DataTypeRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for DataTypeRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for DataTypeRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for DataTypeRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for DataTypeRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for DataTypeRef<'a> {
    type Via = ClassifierRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_classifier_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for DataTypeRef<'a> {}

