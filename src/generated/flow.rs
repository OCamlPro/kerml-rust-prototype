#![allow(unused)]
use super::utils::DescendantOf;
use super::connector::{
    Connector, ConnectorRefMut, ConnectorRef, ConnectorStruct, ConnectorStructRefMut,
    ConnectorStructRef, ConnectorInner, ConnectorUpcast, ConnectorUpcastRefMut,
    ConnectorUpcastRef, ConnectorMethodsDescendants, ConnectorRefMutMethodsDescendants,
    ConnectorRefMethodsDescendants,
};
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
use super::relationship::{
    Relationship, RelationshipRefMut, RelationshipRef, RelationshipStruct,
    RelationshipStructRefMut, RelationshipStructRef, RelationshipInner,
    RelationshipUpcast, RelationshipUpcastRefMut, RelationshipUpcastRef,
    RelationshipMethodsDescendants, RelationshipRefMutMethodsDescendants,
    RelationshipRefMethodsDescendants,
};
use super::step::{
    Step, StepRefMut, StepRef, StepStruct, StepStructRefMut, StepStructRef, StepInner,
    StepUpcast, StepUpcastRefMut, StepUpcastRef, StepMethodsDescendants,
    StepRefMutMethodsDescendants, StepRefMethodsDescendants,
};
use super::succession_flow::{SuccessionFlow, SuccessionFlowRefMut, SuccessionFlowRef};
pub struct FlowInner {
    pub(super) sup_step: StepInner,
    pub(super) sup_connector: ConnectorInner,
}
pub trait FlowStruct
where
    Self: FlowStructRefMut,
    Self: FlowStructRef,
    Self: StepStruct,
    Self: ConnectorStruct,
{}
pub trait FlowStructRefMut
where
    Self: FlowStructRef,
    Self: StepStructRefMut,
    Self: ConnectorStructRefMut,
{}
pub trait FlowStructRef
where
    Self: StepStructRef,
    Self: ConnectorStructRef,
{}
pub trait FlowUpcast: FlowStruct {
    fn into_flow(self) -> Flow;
}
pub trait FlowUpcastRefMut<'a>: FlowStructRefMut {
    fn as_flow_ref_mut(self) -> FlowRefMut<'a>;
}
pub trait FlowUpcastRef<'a>: FlowStructRef {
    fn as_flow_ref(self) -> FlowRef<'a>;
}
impl FlowStruct for FlowInner {}
impl FlowStructRefMut for FlowInner {}
impl FlowStructRef for FlowInner {}
impl ConnectorStruct for FlowInner {}
impl ConnectorStructRefMut for FlowInner {}
impl ConnectorStructRef for FlowInner {}
impl FeatureStruct for FlowInner {
    fn is_unique(self) -> bool {
        self.sup_connector.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_connector.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_connector.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_connector.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_connector.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_connector.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_connector.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_connector.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction()
    }
}
impl FeatureStructRefMut for FlowInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction_ref_mut()
    }
}
impl FeatureStructRef for FlowInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_connector.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_connector.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_connector.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_connector.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_connector.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_connector.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_connector.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_connector.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction_ref()
    }
}
impl TypeStruct for FlowInner {
    fn is_abstract(self) -> bool {
        self.sup_connector.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_connector.is_sufficient()
    }
}
impl TypeStructRefMut for FlowInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FlowInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_connector.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_connector.is_sufficient_ref()
    }
}
impl NamespaceStruct for FlowInner {}
impl NamespaceStructRefMut for FlowInner {}
impl NamespaceStructRef for FlowInner {}
impl ElementStruct for FlowInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_connector.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_connector.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_connector.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_connector.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_connector.is_implied_included()
    }
}
impl ElementStructRefMut for FlowInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_connector.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_connector.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_connector.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_connector.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_connector.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FlowInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_connector.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_connector.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_connector.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_connector.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_connector.is_implied_included_ref()
    }
}
impl RelationshipStruct for FlowInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_connector.is_implied()
    }
}
impl RelationshipStructRefMut for FlowInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for FlowInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_connector.is_implied_ref()
    }
}
impl StepStruct for FlowInner {}
impl StepStructRefMut for FlowInner {}
impl StepStructRef for FlowInner {}
pub enum Flow {
    Itself(FlowInner),
    SuccessionFlow(SuccessionFlow),
}
pub enum FlowRefMut<'a> {
    Itself(&'a mut FlowInner),
    SuccessionFlow(SuccessionFlowRefMut<'a>),
}
pub enum FlowRef<'a> {
    Itself(&'a FlowInner),
    SuccessionFlow(SuccessionFlowRef<'a>),
}
impl Flow {
    pub fn as_ref(&self) -> FlowRef {
        match self {
            Flow::Itself(inner) => FlowRef::Itself(&inner),
            Flow::SuccessionFlow(inner) => FlowRef::SuccessionFlow(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> FlowRefMut {
        match self {
            Flow::Itself(inner) => FlowRefMut::Itself(inner),
            Flow::SuccessionFlow(inner) => FlowRefMut::SuccessionFlow(inner.as_ref_mut()),
        }
    }
}
impl<'a> FlowRefMut<'a> {
    pub fn as_ref(self) -> FlowRef<'a> {
        match self {
            FlowRefMut::Itself(inner) => FlowRef::Itself(inner),
            FlowRefMut::SuccessionFlow(inner) => FlowRef::SuccessionFlow(inner.as_ref()),
        }
    }
}
impl FlowStruct for Flow {}
impl FlowStructRefMut for Flow {}
impl FlowStructRef for Flow {}
impl<'a> FlowStructRefMut for FlowRefMut<'a> {}
impl<'a> FlowStructRef for FlowRefMut<'a> {}
impl<'a> FlowStructRef for FlowRef<'a> {}
impl ConnectorStruct for Flow {}
impl ConnectorStructRefMut for Flow {}
impl ConnectorStructRef for Flow {}
impl<'a> ConnectorStructRefMut for FlowRefMut<'a> {}
impl<'a> ConnectorStructRef for FlowRefMut<'a> {}
impl<'a> ConnectorStructRef for FlowRef<'a> {}
impl FeatureStruct for Flow {
    fn is_unique(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_unique(),
            Flow::SuccessionFlow(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_ordered(),
            Flow::SuccessionFlow(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_composite(),
            Flow::SuccessionFlow(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_end(),
            Flow::SuccessionFlow(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_derived(),
            Flow::SuccessionFlow(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_portion(),
            Flow::SuccessionFlow(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_variable(),
            Flow::SuccessionFlow(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_constant(),
            Flow::SuccessionFlow(x) => x.is_constant(),
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
            Flow::Itself(x) => x.direction(),
            Flow::SuccessionFlow(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Flow {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_unique_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_ordered_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_composite_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_end_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_derived_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_portion_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_variable_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_constant_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_constant_ref_mut(),
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
            Flow::Itself(x) => x.direction_ref_mut(),
            Flow::SuccessionFlow(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Flow {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_unique_ref(),
            Flow::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_ordered_ref(),
            Flow::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_composite_ref(),
            Flow::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_end_ref(),
            Flow::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_derived_ref(),
            Flow::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_portion_ref(),
            Flow::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_variable_ref(),
            Flow::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_constant_ref(),
            Flow::SuccessionFlow(x) => x.is_constant_ref(),
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
            Flow::Itself(x) => x.direction_ref(),
            Flow::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for FlowRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_unique_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_ordered_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_composite_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_end_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_derived_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_portion_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_variable_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_constant_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_constant_ref_mut(),
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
            FlowRefMut::Itself(x) => x.direction_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for FlowRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_unique_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_ordered_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_composite_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_end_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_derived_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_portion_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_variable_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_constant_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_constant_ref(),
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
            FlowRefMut::Itself(x) => x.direction_ref(),
            FlowRefMut::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for FlowRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_unique_ref(),
            FlowRef::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_ordered_ref(),
            FlowRef::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_composite_ref(),
            FlowRef::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_end_ref(),
            FlowRef::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_derived_ref(),
            FlowRef::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_portion_ref(),
            FlowRef::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_variable_ref(),
            FlowRef::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_constant_ref(),
            FlowRef::SuccessionFlow(x) => x.is_constant_ref(),
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
            FlowRef::Itself(x) => x.direction_ref(),
            FlowRef::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Flow {
    fn is_abstract(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_abstract(),
            Flow::SuccessionFlow(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_sufficient(),
            Flow::SuccessionFlow(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Flow {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_abstract_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_sufficient_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Flow {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_abstract_ref(),
            Flow::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_sufficient_ref(),
            Flow::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FlowRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_abstract_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FlowRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_abstract_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_sufficient_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FlowRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_abstract_ref(),
            FlowRef::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_sufficient_ref(),
            FlowRef::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Flow {}
impl NamespaceStructRefMut for Flow {}
impl NamespaceStructRef for Flow {}
impl<'a> NamespaceStructRefMut for FlowRefMut<'a> {}
impl<'a> NamespaceStructRef for FlowRefMut<'a> {}
impl<'a> NamespaceStructRef for FlowRef<'a> {}
impl ElementStruct for Flow {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Flow::Itself(x) => x.owned_relationship(),
            Flow::SuccessionFlow(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Flow::Itself(x) => x.owning_relationship(),
            Flow::SuccessionFlow(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Flow::Itself(x) => x.element_id(),
            Flow::SuccessionFlow(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Flow::Itself(x) => x.alias_ids(),
            Flow::SuccessionFlow(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Flow::Itself(x) => x.declared_short_name(),
            Flow::SuccessionFlow(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Flow::Itself(x) => x.declared_name(),
            Flow::SuccessionFlow(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_implied_included(),
            Flow::SuccessionFlow(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Flow {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Flow::Itself(x) => x.owned_relationship_ref_mut(),
            Flow::SuccessionFlow(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Flow::Itself(x) => x.owning_relationship_ref_mut(),
            Flow::SuccessionFlow(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Flow::Itself(x) => x.element_id_ref_mut(),
            Flow::SuccessionFlow(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Flow::Itself(x) => x.alias_ids_ref_mut(),
            Flow::SuccessionFlow(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Flow::Itself(x) => x.declared_short_name_ref_mut(),
            Flow::SuccessionFlow(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Flow::Itself(x) => x.declared_name_ref_mut(),
            Flow::SuccessionFlow(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_implied_included_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Flow {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Flow::Itself(x) => x.owned_relationship_ref(),
            Flow::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Flow::Itself(x) => x.owning_relationship_ref(),
            Flow::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Flow::Itself(x) => x.element_id_ref(),
            Flow::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Flow::Itself(x) => x.alias_ids_ref(),
            Flow::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Flow::Itself(x) => x.declared_short_name_ref(),
            Flow::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Flow::Itself(x) => x.declared_name_ref(),
            Flow::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_implied_included_ref(),
            Flow::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FlowRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FlowRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FlowRefMut::Itself(x) => x.element_id_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FlowRefMut::Itself(x) => x.alias_ids_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FlowRefMut::Itself(x) => x.declared_name_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FlowRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowRefMut::Itself(x) => x.owned_relationship_ref(),
            FlowRefMut::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowRefMut::Itself(x) => x.owning_relationship_ref(),
            FlowRefMut::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FlowRefMut::Itself(x) => x.element_id_ref(),
            FlowRefMut::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FlowRefMut::Itself(x) => x.alias_ids_ref(),
            FlowRefMut::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FlowRefMut::Itself(x) => x.declared_short_name_ref(),
            FlowRefMut::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FlowRefMut::Itself(x) => x.declared_name_ref(),
            FlowRefMut::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_implied_included_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FlowRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowRef::Itself(x) => x.owned_relationship_ref(),
            FlowRef::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FlowRef::Itself(x) => x.owning_relationship_ref(),
            FlowRef::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FlowRef::Itself(x) => x.element_id_ref(),
            FlowRef::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FlowRef::Itself(x) => x.alias_ids_ref(),
            FlowRef::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FlowRef::Itself(x) => x.declared_short_name_ref(),
            FlowRef::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FlowRef::Itself(x) => x.declared_name_ref(),
            FlowRef::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_implied_included_ref(),
            FlowRef::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for Flow {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.target(),
            Flow::SuccessionFlow(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.source(),
            Flow::SuccessionFlow(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owning_related_element(),
            Flow::SuccessionFlow(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owned_related_element(),
            Flow::SuccessionFlow(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Flow::Itself(x) => x.is_implied(),
            Flow::SuccessionFlow(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Flow {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.target_ref_mut(),
            Flow::SuccessionFlow(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.source_ref_mut(),
            Flow::SuccessionFlow(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owning_related_element_ref_mut(),
            Flow::SuccessionFlow(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owned_related_element_ref_mut(),
            Flow::SuccessionFlow(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Flow::Itself(x) => x.is_implied_ref_mut(),
            Flow::SuccessionFlow(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Flow {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.target_ref(),
            Flow::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.source_ref(),
            Flow::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owning_related_element_ref(),
            Flow::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Flow::Itself(x) => x.owned_related_element_ref(),
            Flow::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Flow::Itself(x) => x.is_implied_ref(),
            Flow::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for FlowRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.target_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.source_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            FlowRefMut::Itself(x) => x.is_implied_ref_mut(),
            FlowRefMut::SuccessionFlow(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for FlowRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.target_ref(),
            FlowRefMut::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.source_ref(),
            FlowRefMut::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.owning_related_element_ref(),
            FlowRefMut::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRefMut::Itself(x) => x.owned_related_element_ref(),
            FlowRefMut::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FlowRefMut::Itself(x) => x.is_implied_ref(),
            FlowRefMut::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for FlowRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRef::Itself(x) => x.target_ref(),
            FlowRef::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRef::Itself(x) => x.source_ref(),
            FlowRef::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRef::Itself(x) => x.owning_related_element_ref(),
            FlowRef::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            FlowRef::Itself(x) => x.owned_related_element_ref(),
            FlowRef::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            FlowRef::Itself(x) => x.is_implied_ref(),
            FlowRef::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl StepStruct for Flow {}
impl StepStructRefMut for Flow {}
impl StepStructRef for Flow {}
impl<'a> StepStructRefMut for FlowRefMut<'a> {}
impl<'a> StepStructRef for FlowRefMut<'a> {}
impl<'a> StepStructRef for FlowRef<'a> {}
impl FlowUpcast for Flow {
    fn into_flow(self) -> Flow {
        self
    }
}
impl<'a> FlowUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_flow_ref_mut(self) -> FlowRefMut<'a> {
        self
    }
}
impl<'a> FlowUpcastRef<'a> for FlowRef<'a> {
    fn as_flow_ref(self) -> FlowRef<'a> {
        self
    }
}
impl ConnectorUpcast for Flow {
    fn into_connector(self) -> Connector {
        Connector::Flow(self).into_connector()
    }
}
impl<'a> ConnectorUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a> {
        ConnectorRefMut::Flow(self).as_connector_ref_mut()
    }
}
impl<'a> ConnectorUpcastRef<'a> for FlowRef<'a> {
    fn as_connector_ref(self) -> ConnectorRef<'a> {
        ConnectorRef::Flow(self).as_connector_ref()
    }
}
impl FeatureUpcast for Flow {
    fn into_feature(self) -> Feature {
        Connector::Flow(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ConnectorRefMut::Flow(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for FlowRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ConnectorRef::Flow(self).as_feature_ref()
    }
}
impl TypeUpcast for Flow {
    fn into_type_(self) -> Type {
        Connector::Flow(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ConnectorRefMut::Flow(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FlowRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ConnectorRef::Flow(self).as_type_ref()
    }
}
impl NamespaceUpcast for Flow {
    fn into_namespace(self) -> Namespace {
        Connector::Flow(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ConnectorRefMut::Flow(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FlowRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ConnectorRef::Flow(self).as_namespace_ref()
    }
}
impl ElementUpcast for Flow {
    fn into_element(self) -> Element {
        Connector::Flow(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ConnectorRefMut::Flow(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FlowRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ConnectorRef::Flow(self).as_element_ref()
    }
}
impl RelationshipUpcast for Flow {
    fn into_relationship(self) -> Relationship {
        Connector::Flow(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ConnectorRefMut::Flow(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for FlowRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ConnectorRef::Flow(self).as_relationship_ref()
    }
}
impl StepUpcast for Flow {
    fn into_step(self) -> Step {
        Step::Flow(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for FlowRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        StepRefMut::Flow(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for FlowRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        StepRef::Flow(self).as_step_ref()
    }
}
pub trait FlowDowncast {
    fn try_into_succession_flow(self) -> Result<SuccessionFlow, String>;
}
pub trait FlowDowncastRefMut<'a> {
    fn try_as_succession_flow_ref_mut(self) -> Result<SuccessionFlowRefMut<'a>, String>;
}
pub trait FlowDowncastRef<'a> {
    fn try_as_succession_flow_ref(self) -> Result<SuccessionFlowRef<'a>, String>;
}
impl FlowDowncast for Flow {
    fn try_into_succession_flow(self) -> Result<SuccessionFlow, String> {
        match self {
            Flow::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
impl<'a> FlowDowncastRefMut<'a> for FlowRefMut<'a> {
    fn try_as_succession_flow_ref_mut(self) -> Result<SuccessionFlowRefMut<'a>, String> {
        match self {
            FlowRefMut::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
impl<'a> FlowDowncastRef<'a> for FlowRef<'a> {
    fn try_as_succession_flow_ref(self) -> Result<SuccessionFlowRef<'a>, String> {
        match self {
            FlowRef::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
pub trait FlowMethodsDescendants
where
    Self: DescendantOf<Flow>,
    Self::Via: FlowMethods,
    Self: Sized,
{}
pub trait FlowMethods: Sized {}
impl<T: FlowMethodsDescendants> FlowMethods for T
where
    T::Via: FlowMethods,
{}
impl DescendantOf<Step> for Flow {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl StepMethodsDescendants for Flow {}
impl DescendantOf<Feature> for Flow {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl FeatureMethodsDescendants for Flow {}
impl DescendantOf<Type> for Flow {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl TypeMethodsDescendants for Flow {}
impl DescendantOf<Namespace> for Flow {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl NamespaceMethodsDescendants for Flow {}
impl DescendantOf<Element> for Flow {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl ElementMethodsDescendants for Flow {}
impl DescendantOf<Connector> for Flow {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl ConnectorMethodsDescendants for Flow {}
impl DescendantOf<Relationship> for Flow {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl RelationshipMethodsDescendants for Flow {}
pub trait FlowRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FlowRefMut<'a>>,
    Self::Via: FlowRefMutMethods,
    Self: Sized,
{}
pub trait FlowRefMutMethods: Sized {}
impl<'a, T: FlowRefMutMethodsDescendants<'a>> FlowRefMutMethods for T
where
    T::Via: FlowRefMutMethods,
{}
impl<'a> DescendantOf<StepRefMut<'a>> for FlowRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for FlowRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for FlowRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FlowRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FlowRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<ConnectorRefMut<'a>> for FlowRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> ConnectorRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for FlowRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for FlowRefMut<'a> {}
pub trait FlowRefMethodsDescendants<'a>
where
    Self: DescendantOf<FlowRef<'a>>,
    Self::Via: FlowRefMethods,
    Self: Sized,
{}
pub trait FlowRefMethods: Sized {}
impl<'a, T: FlowRefMethodsDescendants<'a>> FlowRefMethods for T
where
    T::Via: FlowRefMethods,
{}
impl<'a> DescendantOf<StepRef<'a>> for FlowRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for FlowRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for FlowRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FlowRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FlowRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<ConnectorRef<'a>> for FlowRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> ConnectorRefMethodsDescendants<'a> for FlowRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for FlowRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for FlowRef<'a> {}

