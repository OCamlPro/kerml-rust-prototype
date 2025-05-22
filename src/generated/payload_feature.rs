#![allow(unused)]
use super::utils::DescendantOf;
use super::feature::{
    Feature, FeatureRefMut, FeatureRef, FeatureStruct, FeatureStructRefMut,
    FeatureStructRef, FeatureInner, FeatureUpcast, FeatureUpcastRefMut, FeatureUpcastRef,
    FeatureMethodsDescendants, FeatureRefMutMethodsDescendants,
    FeatureRefMethodsDescendants,
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
pub struct PayloadFeatureInner {
    pub(super) sup_feature: FeatureInner,
}
pub trait PayloadFeatureStruct
where
    Self: PayloadFeatureStructRefMut,
    Self: PayloadFeatureStructRef,
    Self: FeatureStruct,
{}
pub trait PayloadFeatureStructRefMut
where
    Self: PayloadFeatureStructRef,
    Self: FeatureStructRefMut,
{}
pub trait PayloadFeatureStructRef
where
    Self: FeatureStructRef,
{}
pub trait PayloadFeatureUpcast: PayloadFeatureStruct {
    fn into_payload_feature(self) -> PayloadFeature;
}
pub trait PayloadFeatureUpcastRefMut<'a>: PayloadFeatureStructRefMut {
    fn as_payload_feature_ref_mut(self) -> PayloadFeatureRefMut<'a>;
}
pub trait PayloadFeatureUpcastRef<'a>: PayloadFeatureStructRef {
    fn as_payload_feature_ref(self) -> PayloadFeatureRef<'a>;
}
impl PayloadFeatureStruct for PayloadFeatureInner {}
impl PayloadFeatureStructRefMut for PayloadFeatureInner {}
impl PayloadFeatureStructRef for PayloadFeatureInner {}
impl FeatureStruct for PayloadFeatureInner {
    fn is_unique(self) -> bool {
        self.sup_feature.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_feature.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_feature.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_feature.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_feature.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_feature.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_feature.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_feature.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction()
    }
}
impl FeatureStructRefMut for PayloadFeatureInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction_ref_mut()
    }
}
impl FeatureStructRef for PayloadFeatureInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_feature.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_feature.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_feature.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_feature.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_feature.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_feature.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_feature.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_feature.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction_ref()
    }
}
impl TypeStruct for PayloadFeatureInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for PayloadFeatureInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for PayloadFeatureInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for PayloadFeatureInner {}
impl NamespaceStructRefMut for PayloadFeatureInner {}
impl NamespaceStructRef for PayloadFeatureInner {}
impl ElementStruct for PayloadFeatureInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_feature.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_feature.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_feature.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_feature.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_feature.is_implied_included()
    }
}
impl ElementStructRefMut for PayloadFeatureInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_feature.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_feature.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_feature.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for PayloadFeatureInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_feature.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_feature.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_feature.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_feature.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_feature.is_implied_included_ref()
    }
}
pub enum PayloadFeature {
    Itself(PayloadFeatureInner),
}
pub enum PayloadFeatureRefMut<'a> {
    Itself(&'a mut PayloadFeatureInner),
}
pub enum PayloadFeatureRef<'a> {
    Itself(&'a PayloadFeatureInner),
}
impl PayloadFeature {
    pub fn as_ref(&self) -> PayloadFeatureRef {
        match self {
            PayloadFeature::Itself(inner) => PayloadFeatureRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> PayloadFeatureRefMut {
        match self {
            PayloadFeature::Itself(inner) => PayloadFeatureRefMut::Itself(inner),
        }
    }
}
impl<'a> PayloadFeatureRefMut<'a> {
    pub fn as_ref(self) -> PayloadFeatureRef<'a> {
        match self {
            PayloadFeatureRefMut::Itself(inner) => PayloadFeatureRef::Itself(inner),
        }
    }
}
impl PayloadFeatureStruct for PayloadFeature {}
impl PayloadFeatureStructRefMut for PayloadFeature {}
impl PayloadFeatureStructRef for PayloadFeature {}
impl<'a> PayloadFeatureStructRefMut for PayloadFeatureRefMut<'a> {}
impl<'a> PayloadFeatureStructRef for PayloadFeatureRefMut<'a> {}
impl<'a> PayloadFeatureStructRef for PayloadFeatureRef<'a> {}
impl FeatureStruct for PayloadFeature {
    fn is_unique(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_constant(),
        }
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeature::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for PayloadFeature {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_constant_ref_mut(),
        }
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeature::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for PayloadFeature {
    fn is_unique_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeature::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for PayloadFeatureRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_constant_ref_mut(),
        }
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for PayloadFeatureRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for PayloadFeatureRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            PayloadFeatureRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for PayloadFeature {
    fn is_abstract(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for PayloadFeature {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for PayloadFeature {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for PayloadFeatureRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for PayloadFeatureRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for PayloadFeatureRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for PayloadFeature {}
impl NamespaceStructRefMut for PayloadFeature {}
impl NamespaceStructRef for PayloadFeature {}
impl<'a> NamespaceStructRefMut for PayloadFeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for PayloadFeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for PayloadFeatureRef<'a> {}
impl ElementStruct for PayloadFeature {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeature::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeature::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            PayloadFeature::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            PayloadFeature::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            PayloadFeature::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for PayloadFeature {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeature::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            PayloadFeature::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            PayloadFeature::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            PayloadFeature::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeature::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for PayloadFeature {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeature::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeature::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PayloadFeature::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PayloadFeature::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeature::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PayloadFeature::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for PayloadFeatureRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for PayloadFeatureRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PayloadFeatureRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for PayloadFeatureRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeatureRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            PayloadFeatureRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            PayloadFeatureRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            PayloadFeatureRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeatureRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            PayloadFeatureRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            PayloadFeatureRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl PayloadFeatureUpcast for PayloadFeature {
    fn into_payload_feature(self) -> PayloadFeature {
        self
    }
}
impl<'a> PayloadFeatureUpcastRefMut<'a> for PayloadFeatureRefMut<'a> {
    fn as_payload_feature_ref_mut(self) -> PayloadFeatureRefMut<'a> {
        self
    }
}
impl<'a> PayloadFeatureUpcastRef<'a> for PayloadFeatureRef<'a> {
    fn as_payload_feature_ref(self) -> PayloadFeatureRef<'a> {
        self
    }
}
impl FeatureUpcast for PayloadFeature {
    fn into_feature(self) -> Feature {
        Feature::PayloadFeature(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for PayloadFeatureRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::PayloadFeature(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for PayloadFeatureRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::PayloadFeature(self).as_feature_ref()
    }
}
impl TypeUpcast for PayloadFeature {
    fn into_type_(self) -> Type {
        Feature::PayloadFeature(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for PayloadFeatureRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::PayloadFeature(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for PayloadFeatureRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::PayloadFeature(self).as_type_ref()
    }
}
impl NamespaceUpcast for PayloadFeature {
    fn into_namespace(self) -> Namespace {
        Feature::PayloadFeature(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for PayloadFeatureRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::PayloadFeature(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for PayloadFeatureRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::PayloadFeature(self).as_namespace_ref()
    }
}
impl ElementUpcast for PayloadFeature {
    fn into_element(self) -> Element {
        Feature::PayloadFeature(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for PayloadFeatureRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::PayloadFeature(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for PayloadFeatureRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::PayloadFeature(self).as_element_ref()
    }
}
pub trait PayloadFeatureDowncast {}
pub trait PayloadFeatureDowncastRefMut<'a> {}
pub trait PayloadFeatureDowncastRef<'a> {}
impl PayloadFeatureDowncast for PayloadFeature {}
impl<'a> PayloadFeatureDowncastRefMut<'a> for PayloadFeatureRefMut<'a> {}
impl<'a> PayloadFeatureDowncastRef<'a> for PayloadFeatureRef<'a> {}
pub trait PayloadFeatureMethodsDescendants
where
    Self: DescendantOf<PayloadFeature>,
    Self::Via: PayloadFeatureMethods,
    Self: Sized,
{}
pub trait PayloadFeatureMethods: Sized {}
impl<T: PayloadFeatureMethodsDescendants> PayloadFeatureMethods for T
where
    T::Via: PayloadFeatureMethods,
{}
impl DescendantOf<Feature> for PayloadFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for PayloadFeature {}
impl DescendantOf<Type> for PayloadFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for PayloadFeature {}
impl DescendantOf<Namespace> for PayloadFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for PayloadFeature {}
impl DescendantOf<Element> for PayloadFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl ElementMethodsDescendants for PayloadFeature {}
pub trait PayloadFeatureRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<PayloadFeatureRefMut<'a>>,
    Self::Via: PayloadFeatureRefMutMethods,
    Self: Sized,
{}
pub trait PayloadFeatureRefMutMethods: Sized {}
impl<'a, T: PayloadFeatureRefMutMethodsDescendants<'a>> PayloadFeatureRefMutMethods for T
where
    T::Via: PayloadFeatureRefMutMethods,
{}
impl<'a> DescendantOf<FeatureRefMut<'a>> for PayloadFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for PayloadFeatureRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for PayloadFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for PayloadFeatureRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for PayloadFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for PayloadFeatureRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for PayloadFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for PayloadFeatureRefMut<'a> {}
pub trait PayloadFeatureRefMethodsDescendants<'a>
where
    Self: DescendantOf<PayloadFeatureRef<'a>>,
    Self::Via: PayloadFeatureRefMethods,
    Self: Sized,
{}
pub trait PayloadFeatureRefMethods: Sized {}
impl<'a, T: PayloadFeatureRefMethodsDescendants<'a>> PayloadFeatureRefMethods for T
where
    T::Via: PayloadFeatureRefMethods,
{}
impl<'a> DescendantOf<FeatureRef<'a>> for PayloadFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for PayloadFeatureRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for PayloadFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for PayloadFeatureRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for PayloadFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for PayloadFeatureRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for PayloadFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for PayloadFeatureRef<'a> {}

