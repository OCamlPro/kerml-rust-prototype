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
pub struct FlowEndInner {
    pub(super) sup_feature: FeatureInner,
}
pub trait FlowEndStruct
where
    Self: FlowEndStructRefMut,
    Self: FlowEndStructRef,
    Self: FeatureStruct,
{}
pub trait FlowEndStructRefMut
where
    Self: FlowEndStructRef,
    Self: FeatureStructRefMut,
{}
pub trait FlowEndStructRef
where
    Self: FeatureStructRef,
{}
pub trait FlowEndUpcast: FlowEndStruct {
    fn into_flow_end(self) -> FlowEnd;
}
pub trait FlowEndUpcastRefMut<'a>: FlowEndStructRefMut {
    fn as_flow_end_ref_mut(self) -> FlowEndRefMut<'a>;
}
pub trait FlowEndUpcastRef<'a>: FlowEndStructRef {
    fn as_flow_end_ref(self) -> FlowEndRef<'a>;
}
impl FlowEndStruct for FlowEndInner {}
impl FlowEndStructRefMut for FlowEndInner {}
impl FlowEndStructRef for FlowEndInner {}
impl FeatureStruct for FlowEndInner {
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
impl FeatureStructRefMut for FlowEndInner {
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
impl FeatureStructRef for FlowEndInner {
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
impl TypeStruct for FlowEndInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for FlowEndInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FlowEndInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for FlowEndInner {}
impl NamespaceStructRefMut for FlowEndInner {}
impl NamespaceStructRef for FlowEndInner {}
impl ElementStruct for FlowEndInner {
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
impl ElementStructRefMut for FlowEndInner {
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
impl ElementStructRef for FlowEndInner {
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
pub enum FlowEnd {
    Itself(FlowEndInner),
}
pub enum FlowEndRefMut<'a> {
    Itself(&'a mut FlowEndInner),
}
pub enum FlowEndRef<'a> {
    Itself(&'a FlowEndInner),
}
impl FlowEnd {
    pub fn as_ref(&self) -> FlowEndRef {
        match self {
            FlowEnd::Itself(inner) => FlowEndRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> FlowEndRefMut {
        match self {
            FlowEnd::Itself(inner) => FlowEndRefMut::Itself(inner),
        }
    }
}
impl<'a> FlowEndRefMut<'a> {
    pub fn as_ref(self) -> FlowEndRef<'a> {
        match self {
            FlowEndRefMut::Itself(inner) => FlowEndRef::Itself(inner),
        }
    }
}
impl FlowEndStruct for FlowEnd {}
impl FlowEndStructRefMut for FlowEnd {}
impl FlowEndStructRef for FlowEnd {}
impl<'a> FlowEndStructRefMut for FlowEndRefMut<'a> {}
impl<'a> FlowEndStructRef for FlowEndRefMut<'a> {}
impl<'a> FlowEndStructRef for FlowEndRef<'a> {}
impl FeatureStruct for FlowEnd {
    fn is_unique(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_constant(),
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
            FlowEnd::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for FlowEnd {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_constant_ref_mut(),
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
            FlowEnd::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for FlowEnd {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_constant_ref(),
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
            FlowEnd::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for FlowEndRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            FlowEndRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for FlowEndRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_constant_ref(),
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
            FlowEndRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for FlowEndRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_constant_ref(),
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
            FlowEndRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for FlowEnd {
    fn is_abstract(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for FlowEnd {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for FlowEnd {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FlowEndRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FlowEndRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FlowEndRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for FlowEnd {}
impl NamespaceStructRefMut for FlowEnd {}
impl NamespaceStructRef for FlowEnd {}
impl<'a> NamespaceStructRefMut for FlowEndRefMut<'a> {}
impl<'a> NamespaceStructRef for FlowEndRefMut<'a> {}
impl<'a> NamespaceStructRef for FlowEndRef<'a> {}
impl ElementStruct for FlowEnd {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEnd::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEnd::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FlowEnd::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FlowEnd::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FlowEnd::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FlowEnd {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEnd::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FlowEnd::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FlowEnd::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FlowEnd::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEnd::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FlowEnd {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEnd::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEnd::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FlowEnd::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FlowEnd::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FlowEnd::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FlowEnd::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FlowEndRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEndRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FlowEndRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FlowEndRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FlowEndRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEndRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEndRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FlowEndRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FlowEndRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FlowEndRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FlowEndRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEndRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowEndRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FlowEndRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FlowEndRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FlowEndRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FlowEndRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FlowEndRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FlowEndUpcast for FlowEnd {
    fn into_flow_end(self) -> FlowEnd {
        self
    }
}
impl<'a> FlowEndUpcastRefMut<'a> for FlowEndRefMut<'a> {
    fn as_flow_end_ref_mut(self) -> FlowEndRefMut<'a> {
        self
    }
}
impl<'a> FlowEndUpcastRef<'a> for FlowEndRef<'a> {
    fn as_flow_end_ref(self) -> FlowEndRef<'a> {
        self
    }
}
impl FeatureUpcast for FlowEnd {
    fn into_feature(self) -> Feature {
        Feature::FlowEnd(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for FlowEndRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::FlowEnd(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for FlowEndRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::FlowEnd(self).as_feature_ref()
    }
}
impl TypeUpcast for FlowEnd {
    fn into_type_(self) -> Type {
        Feature::FlowEnd(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FlowEndRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::FlowEnd(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FlowEndRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::FlowEnd(self).as_type_ref()
    }
}
impl NamespaceUpcast for FlowEnd {
    fn into_namespace(self) -> Namespace {
        Feature::FlowEnd(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FlowEndRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::FlowEnd(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FlowEndRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::FlowEnd(self).as_namespace_ref()
    }
}
impl ElementUpcast for FlowEnd {
    fn into_element(self) -> Element {
        Feature::FlowEnd(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FlowEndRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::FlowEnd(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FlowEndRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::FlowEnd(self).as_element_ref()
    }
}
pub trait FlowEndDowncast {}
pub trait FlowEndDowncastRefMut<'a> {}
pub trait FlowEndDowncastRef<'a> {}
impl FlowEndDowncast for FlowEnd {}
impl<'a> FlowEndDowncastRefMut<'a> for FlowEndRefMut<'a> {}
impl<'a> FlowEndDowncastRef<'a> for FlowEndRef<'a> {}
pub trait FlowEndMethodsDescendants
where
    Self: DescendantOf<FlowEnd>,
    Self::Via: FlowEndMethods,
    Self: Sized,
{}
pub trait FlowEndMethods: Sized {}
impl<T: FlowEndMethodsDescendants> FlowEndMethods for T
where
    T::Via: FlowEndMethods,
{}
impl DescendantOf<Feature> for FlowEnd {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for FlowEnd {}
impl DescendantOf<Type> for FlowEnd {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for FlowEnd {}
impl DescendantOf<Namespace> for FlowEnd {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for FlowEnd {}
impl DescendantOf<Element> for FlowEnd {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl ElementMethodsDescendants for FlowEnd {}
pub trait FlowEndRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FlowEndRefMut<'a>>,
    Self::Via: FlowEndRefMutMethods,
    Self: Sized,
{}
pub trait FlowEndRefMutMethods: Sized {}
impl<'a, T: FlowEndRefMutMethodsDescendants<'a>> FlowEndRefMutMethods for T
where
    T::Via: FlowEndRefMutMethods,
{}
impl<'a> DescendantOf<FeatureRefMut<'a>> for FlowEndRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for FlowEndRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for FlowEndRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FlowEndRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FlowEndRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FlowEndRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FlowEndRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FlowEndRefMut<'a> {}
pub trait FlowEndRefMethodsDescendants<'a>
where
    Self: DescendantOf<FlowEndRef<'a>>,
    Self::Via: FlowEndRefMethods,
    Self: Sized,
{}
pub trait FlowEndRefMethods: Sized {}
impl<'a, T: FlowEndRefMethodsDescendants<'a>> FlowEndRefMethods for T
where
    T::Via: FlowEndRefMethods,
{}
impl<'a> DescendantOf<FeatureRef<'a>> for FlowEndRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for FlowEndRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for FlowEndRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FlowEndRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FlowEndRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FlowEndRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FlowEndRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FlowEndRef<'a> {}

