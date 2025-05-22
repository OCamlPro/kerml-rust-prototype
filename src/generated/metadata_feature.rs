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
use super::annotating_element::{
    AnnotatingElement, AnnotatingElementRefMut, AnnotatingElementRef,
    AnnotatingElementStruct, AnnotatingElementStructRefMut, AnnotatingElementStructRef,
    AnnotatingElementInner, AnnotatingElementUpcast, AnnotatingElementUpcastRefMut,
    AnnotatingElementUpcastRef, AnnotatingElementMethodsDescendants,
    AnnotatingElementRefMutMethodsDescendants, AnnotatingElementRefMethodsDescendants,
};
pub struct MetadataFeatureInner {
    pub(super) sup_annotating_element: AnnotatingElementInner,
    pub(super) sup_feature: FeatureInner,
}
pub trait MetadataFeatureStruct
where
    Self: MetadataFeatureStructRefMut,
    Self: MetadataFeatureStructRef,
    Self: AnnotatingElementStruct,
    Self: FeatureStruct,
{}
pub trait MetadataFeatureStructRefMut
where
    Self: MetadataFeatureStructRef,
    Self: AnnotatingElementStructRefMut,
    Self: FeatureStructRefMut,
{}
pub trait MetadataFeatureStructRef
where
    Self: AnnotatingElementStructRef,
    Self: FeatureStructRef,
{}
pub trait MetadataFeatureUpcast: MetadataFeatureStruct {
    fn into_metadata_feature(self) -> MetadataFeature;
}
pub trait MetadataFeatureUpcastRefMut<'a>: MetadataFeatureStructRefMut {
    fn as_metadata_feature_ref_mut(self) -> MetadataFeatureRefMut<'a>;
}
pub trait MetadataFeatureUpcastRef<'a>: MetadataFeatureStructRef {
    fn as_metadata_feature_ref(self) -> MetadataFeatureRef<'a>;
}
impl MetadataFeatureStruct for MetadataFeatureInner {}
impl MetadataFeatureStructRefMut for MetadataFeatureInner {}
impl MetadataFeatureStructRef for MetadataFeatureInner {}
impl FeatureStruct for MetadataFeatureInner {
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
impl FeatureStructRefMut for MetadataFeatureInner {
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
impl FeatureStructRef for MetadataFeatureInner {
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
impl TypeStruct for MetadataFeatureInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for MetadataFeatureInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for MetadataFeatureInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for MetadataFeatureInner {}
impl NamespaceStructRefMut for MetadataFeatureInner {}
impl NamespaceStructRef for MetadataFeatureInner {}
impl ElementStruct for MetadataFeatureInner {
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
impl ElementStructRefMut for MetadataFeatureInner {
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
impl ElementStructRef for MetadataFeatureInner {
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
impl AnnotatingElementStruct for MetadataFeatureInner {}
impl AnnotatingElementStructRefMut for MetadataFeatureInner {}
impl AnnotatingElementStructRef for MetadataFeatureInner {}
pub enum MetadataFeature {
    Itself(MetadataFeatureInner),
}
pub enum MetadataFeatureRefMut<'a> {
    Itself(&'a mut MetadataFeatureInner),
}
pub enum MetadataFeatureRef<'a> {
    Itself(&'a MetadataFeatureInner),
}
impl MetadataFeature {
    pub fn as_ref(&self) -> MetadataFeatureRef {
        match self {
            MetadataFeature::Itself(inner) => MetadataFeatureRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> MetadataFeatureRefMut {
        match self {
            MetadataFeature::Itself(inner) => MetadataFeatureRefMut::Itself(inner),
        }
    }
}
impl<'a> MetadataFeatureRefMut<'a> {
    pub fn as_ref(self) -> MetadataFeatureRef<'a> {
        match self {
            MetadataFeatureRefMut::Itself(inner) => MetadataFeatureRef::Itself(inner),
        }
    }
}
impl MetadataFeatureStruct for MetadataFeature {}
impl MetadataFeatureStructRefMut for MetadataFeature {}
impl MetadataFeatureStructRef for MetadataFeature {}
impl<'a> MetadataFeatureStructRefMut for MetadataFeatureRefMut<'a> {}
impl<'a> MetadataFeatureStructRef for MetadataFeatureRefMut<'a> {}
impl<'a> MetadataFeatureStructRef for MetadataFeatureRef<'a> {}
impl FeatureStruct for MetadataFeature {
    fn is_unique(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_constant(),
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
            MetadataFeature::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for MetadataFeature {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_constant_ref_mut(),
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
            MetadataFeature::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for MetadataFeature {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_constant_ref(),
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
            MetadataFeature::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for MetadataFeatureRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            MetadataFeatureRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for MetadataFeatureRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_constant_ref(),
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
            MetadataFeatureRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for MetadataFeatureRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_constant_ref(),
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
            MetadataFeatureRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for MetadataFeature {
    fn is_abstract(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for MetadataFeature {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for MetadataFeature {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for MetadataFeatureRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for MetadataFeatureRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for MetadataFeatureRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for MetadataFeature {}
impl NamespaceStructRefMut for MetadataFeature {}
impl NamespaceStructRef for MetadataFeature {}
impl<'a> NamespaceStructRefMut for MetadataFeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for MetadataFeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for MetadataFeatureRef<'a> {}
impl ElementStruct for MetadataFeature {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeature::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeature::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            MetadataFeature::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            MetadataFeature::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            MetadataFeature::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for MetadataFeature {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeature::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MetadataFeature::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MetadataFeature::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MetadataFeature::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeature::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for MetadataFeature {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeature::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeature::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataFeature::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataFeature::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeature::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataFeature::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MetadataFeatureRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MetadataFeatureRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataFeatureRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MetadataFeatureRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeatureRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataFeatureRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataFeatureRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataFeatureRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeatureRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataFeatureRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataFeatureRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl AnnotatingElementStruct for MetadataFeature {}
impl AnnotatingElementStructRefMut for MetadataFeature {}
impl AnnotatingElementStructRef for MetadataFeature {}
impl<'a> AnnotatingElementStructRefMut for MetadataFeatureRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for MetadataFeatureRefMut<'a> {}
impl<'a> AnnotatingElementStructRef for MetadataFeatureRef<'a> {}
impl MetadataFeatureUpcast for MetadataFeature {
    fn into_metadata_feature(self) -> MetadataFeature {
        self
    }
}
impl<'a> MetadataFeatureUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_metadata_feature_ref_mut(self) -> MetadataFeatureRefMut<'a> {
        self
    }
}
impl<'a> MetadataFeatureUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_metadata_feature_ref(self) -> MetadataFeatureRef<'a> {
        self
    }
}
impl FeatureUpcast for MetadataFeature {
    fn into_feature(self) -> Feature {
        Feature::MetadataFeature(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::MetadataFeature(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::MetadataFeature(self).as_feature_ref()
    }
}
impl TypeUpcast for MetadataFeature {
    fn into_type_(self) -> Type {
        Feature::MetadataFeature(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::MetadataFeature(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::MetadataFeature(self).as_type_ref()
    }
}
impl NamespaceUpcast for MetadataFeature {
    fn into_namespace(self) -> Namespace {
        Feature::MetadataFeature(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::MetadataFeature(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::MetadataFeature(self).as_namespace_ref()
    }
}
impl ElementUpcast for MetadataFeature {
    fn into_element(self) -> Element {
        Feature::MetadataFeature(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::MetadataFeature(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::MetadataFeature(self).as_element_ref()
    }
}
impl AnnotatingElementUpcast for MetadataFeature {
    fn into_annotating_element(self) -> AnnotatingElement {
        AnnotatingElement::MetadataFeature(self).into_annotating_element()
    }
}
impl<'a> AnnotatingElementUpcastRefMut<'a> for MetadataFeatureRefMut<'a> {
    fn as_annotating_element_ref_mut(self) -> AnnotatingElementRefMut<'a> {
        AnnotatingElementRefMut::MetadataFeature(self).as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementUpcastRef<'a> for MetadataFeatureRef<'a> {
    fn as_annotating_element_ref(self) -> AnnotatingElementRef<'a> {
        AnnotatingElementRef::MetadataFeature(self).as_annotating_element_ref()
    }
}
pub trait MetadataFeatureDowncast {}
pub trait MetadataFeatureDowncastRefMut<'a> {}
pub trait MetadataFeatureDowncastRef<'a> {}
impl MetadataFeatureDowncast for MetadataFeature {}
impl<'a> MetadataFeatureDowncastRefMut<'a> for MetadataFeatureRefMut<'a> {}
impl<'a> MetadataFeatureDowncastRef<'a> for MetadataFeatureRef<'a> {}
pub trait MetadataFeatureMethodsDescendants
where
    Self: DescendantOf<MetadataFeature>,
    Self::Via: MetadataFeatureMethods,
    Self: Sized,
{}
pub trait MetadataFeatureMethods: Sized {}
impl<T: MetadataFeatureMethodsDescendants> MetadataFeatureMethods for T
where
    T::Via: MetadataFeatureMethods,
{}
impl DescendantOf<AnnotatingElement> for MetadataFeature {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl AnnotatingElementMethodsDescendants for MetadataFeature {}
impl DescendantOf<Element> for MetadataFeature {
    type Via = AnnotatingElement;
    fn into_via(self) -> Self::Via {
        self.into_annotating_element()
    }
}
impl ElementMethodsDescendants for MetadataFeature {}
impl DescendantOf<Feature> for MetadataFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for MetadataFeature {}
impl DescendantOf<Type> for MetadataFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for MetadataFeature {}
impl DescendantOf<Namespace> for MetadataFeature {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for MetadataFeature {}
pub trait MetadataFeatureRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MetadataFeatureRefMut<'a>>,
    Self::Via: MetadataFeatureRefMutMethods,
    Self: Sized,
{}
pub trait MetadataFeatureRefMutMethods: Sized {}
impl<'a, T: MetadataFeatureRefMutMethodsDescendants<'a>> MetadataFeatureRefMutMethods
for T
where
    T::Via: MetadataFeatureRefMutMethods,
{}
impl<'a> DescendantOf<AnnotatingElementRefMut<'a>> for MetadataFeatureRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> AnnotatingElementRefMutMethodsDescendants<'a> for MetadataFeatureRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MetadataFeatureRefMut<'a> {
    type Via = AnnotatingElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MetadataFeatureRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for MetadataFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for MetadataFeatureRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for MetadataFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for MetadataFeatureRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for MetadataFeatureRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for MetadataFeatureRefMut<'a> {}
pub trait MetadataFeatureRefMethodsDescendants<'a>
where
    Self: DescendantOf<MetadataFeatureRef<'a>>,
    Self::Via: MetadataFeatureRefMethods,
    Self: Sized,
{
    fn evaluate_feature_impl(
        self,
        base_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.into_via().evaluate_feature(base_feature)
    }
    fn is_semantic_impl(self) -> bool {
        self.into_via().is_semantic()
    }
    fn is_syntactic_impl(self) -> bool {
        self.into_via().is_syntactic()
    }
    fn syntax_element_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.into_via().syntax_element()
    }
}
pub trait MetadataFeatureRefMethods: Sized {
    fn evaluate_feature(
        self,
        base_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn is_semantic(self) -> bool;
    fn is_syntactic(self) -> bool;
    fn syntax_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
}
impl<'a, T: MetadataFeatureRefMethodsDescendants<'a>> MetadataFeatureRefMethods for T
where
    T::Via: MetadataFeatureRefMethods,
{
    fn evaluate_feature(
        self,
        base_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        MetadataFeatureRefMethodsDescendants::evaluate_feature_impl(self, base_feature)
    }
    fn is_semantic(self) -> bool {
        MetadataFeatureRefMethodsDescendants::is_semantic_impl(self)
    }
    fn is_syntactic(self) -> bool {
        MetadataFeatureRefMethodsDescendants::is_syntactic_impl(self)
    }
    fn syntax_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        MetadataFeatureRefMethodsDescendants::syntax_element_impl(self)
    }
}
impl<'a> DescendantOf<AnnotatingElementRef<'a>> for MetadataFeatureRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> AnnotatingElementRefMethodsDescendants<'a> for MetadataFeatureRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MetadataFeatureRef<'a> {
    type Via = AnnotatingElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_annotating_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MetadataFeatureRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for MetadataFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for MetadataFeatureRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for MetadataFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for MetadataFeatureRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for MetadataFeatureRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for MetadataFeatureRef<'a> {}

