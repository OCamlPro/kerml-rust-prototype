#![allow(unused)]
use super::utils::DescendantOf;
use super::flow::{
    Flow, FlowRefMut, FlowRef, FlowStruct, FlowStructRefMut, FlowStructRef, FlowInner,
    FlowUpcast, FlowUpcastRefMut, FlowUpcastRef, FlowMethodsDescendants,
    FlowRefMutMethodsDescendants, FlowRefMethodsDescendants,
};
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
use super::succession::{
    Succession, SuccessionRefMut, SuccessionRef, SuccessionStruct,
    SuccessionStructRefMut, SuccessionStructRef, SuccessionInner, SuccessionUpcast,
    SuccessionUpcastRefMut, SuccessionUpcastRef, SuccessionMethodsDescendants,
    SuccessionRefMutMethodsDescendants, SuccessionRefMethodsDescendants,
};
pub struct SuccessionFlowInner {
    pub(super) sup_succession: SuccessionInner,
    pub(super) sup_flow: FlowInner,
}
pub trait SuccessionFlowStruct
where
    Self: SuccessionFlowStructRefMut,
    Self: SuccessionFlowStructRef,
    Self: SuccessionStruct,
    Self: FlowStruct,
{}
pub trait SuccessionFlowStructRefMut
where
    Self: SuccessionFlowStructRef,
    Self: SuccessionStructRefMut,
    Self: FlowStructRefMut,
{}
pub trait SuccessionFlowStructRef
where
    Self: SuccessionStructRef,
    Self: FlowStructRef,
{}
pub trait SuccessionFlowUpcast: SuccessionFlowStruct {
    fn into_succession_flow(self) -> SuccessionFlow;
}
pub trait SuccessionFlowUpcastRefMut<'a>: SuccessionFlowStructRefMut {
    fn as_succession_flow_ref_mut(self) -> SuccessionFlowRefMut<'a>;
}
pub trait SuccessionFlowUpcastRef<'a>: SuccessionFlowStructRef {
    fn as_succession_flow_ref(self) -> SuccessionFlowRef<'a>;
}
impl SuccessionFlowStruct for SuccessionFlowInner {}
impl SuccessionFlowStructRefMut for SuccessionFlowInner {}
impl SuccessionFlowStructRef for SuccessionFlowInner {}
impl FlowStruct for SuccessionFlowInner {}
impl FlowStructRefMut for SuccessionFlowInner {}
impl FlowStructRef for SuccessionFlowInner {}
impl ConnectorStruct for SuccessionFlowInner {}
impl ConnectorStructRefMut for SuccessionFlowInner {}
impl ConnectorStructRef for SuccessionFlowInner {}
impl FeatureStruct for SuccessionFlowInner {
    fn is_unique(self) -> bool {
        self.sup_flow.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_flow.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_flow.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_flow.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_flow.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_flow.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_flow.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_flow.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_flow.direction()
    }
}
impl FeatureStructRefMut for SuccessionFlowInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_flow.direction_ref_mut()
    }
}
impl FeatureStructRef for SuccessionFlowInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_flow.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_flow.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_flow.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_flow.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_flow.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_flow.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_flow.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_flow.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_flow.direction_ref()
    }
}
impl TypeStruct for SuccessionFlowInner {
    fn is_abstract(self) -> bool {
        self.sup_flow.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_flow.is_sufficient()
    }
}
impl TypeStructRefMut for SuccessionFlowInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for SuccessionFlowInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_flow.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_flow.is_sufficient_ref()
    }
}
impl NamespaceStruct for SuccessionFlowInner {}
impl NamespaceStructRefMut for SuccessionFlowInner {}
impl NamespaceStructRef for SuccessionFlowInner {}
impl ElementStruct for SuccessionFlowInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_flow.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_flow.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_flow.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_flow.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_flow.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_flow.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_flow.is_implied_included()
    }
}
impl ElementStructRefMut for SuccessionFlowInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_flow.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_flow.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_flow.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_flow.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_flow.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_flow.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for SuccessionFlowInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_flow.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_flow.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_flow.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_flow.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_flow.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_flow.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_flow.is_implied_included_ref()
    }
}
impl RelationshipStruct for SuccessionFlowInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_flow.is_implied()
    }
}
impl RelationshipStructRefMut for SuccessionFlowInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_flow.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for SuccessionFlowInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_flow.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_flow.is_implied_ref()
    }
}
impl StepStruct for SuccessionFlowInner {}
impl StepStructRefMut for SuccessionFlowInner {}
impl StepStructRef for SuccessionFlowInner {}
impl SuccessionStruct for SuccessionFlowInner {}
impl SuccessionStructRefMut for SuccessionFlowInner {}
impl SuccessionStructRef for SuccessionFlowInner {}
pub enum SuccessionFlow {
    Itself(SuccessionFlowInner),
}
pub enum SuccessionFlowRefMut<'a> {
    Itself(&'a mut SuccessionFlowInner),
}
pub enum SuccessionFlowRef<'a> {
    Itself(&'a SuccessionFlowInner),
}
impl SuccessionFlow {
    pub fn as_ref(&self) -> SuccessionFlowRef {
        match self {
            SuccessionFlow::Itself(inner) => SuccessionFlowRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> SuccessionFlowRefMut {
        match self {
            SuccessionFlow::Itself(inner) => SuccessionFlowRefMut::Itself(inner),
        }
    }
}
impl<'a> SuccessionFlowRefMut<'a> {
    pub fn as_ref(self) -> SuccessionFlowRef<'a> {
        match self {
            SuccessionFlowRefMut::Itself(inner) => SuccessionFlowRef::Itself(inner),
        }
    }
}
impl SuccessionFlowStruct for SuccessionFlow {}
impl SuccessionFlowStructRefMut for SuccessionFlow {}
impl SuccessionFlowStructRef for SuccessionFlow {}
impl<'a> SuccessionFlowStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> SuccessionFlowStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> SuccessionFlowStructRef for SuccessionFlowRef<'a> {}
impl FlowStruct for SuccessionFlow {}
impl FlowStructRefMut for SuccessionFlow {}
impl FlowStructRef for SuccessionFlow {}
impl<'a> FlowStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> FlowStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> FlowStructRef for SuccessionFlowRef<'a> {}
impl ConnectorStruct for SuccessionFlow {}
impl ConnectorStructRefMut for SuccessionFlow {}
impl ConnectorStructRef for SuccessionFlow {}
impl<'a> ConnectorStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> ConnectorStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> ConnectorStructRef for SuccessionFlowRef<'a> {}
impl FeatureStruct for SuccessionFlow {
    fn is_unique(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_constant(),
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
            SuccessionFlow::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for SuccessionFlow {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_constant_ref_mut(),
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
            SuccessionFlow::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for SuccessionFlow {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_constant_ref(),
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
            SuccessionFlow::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for SuccessionFlowRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            SuccessionFlowRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for SuccessionFlowRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_constant_ref(),
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
            SuccessionFlowRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for SuccessionFlowRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_constant_ref(),
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
            SuccessionFlowRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for SuccessionFlow {
    fn is_abstract(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for SuccessionFlow {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for SuccessionFlow {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for SuccessionFlowRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for SuccessionFlowRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for SuccessionFlowRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for SuccessionFlow {}
impl NamespaceStructRefMut for SuccessionFlow {}
impl NamespaceStructRef for SuccessionFlow {}
impl<'a> NamespaceStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> NamespaceStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> NamespaceStructRef for SuccessionFlowRef<'a> {}
impl ElementStruct for SuccessionFlow {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            SuccessionFlow::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            SuccessionFlow::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for SuccessionFlow {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SuccessionFlow::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SuccessionFlow::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SuccessionFlow::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for SuccessionFlow {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SuccessionFlow::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SuccessionFlow::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlow::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SuccessionFlowRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SuccessionFlowRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SuccessionFlowRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SuccessionFlowRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SuccessionFlowRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlowRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionFlowRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for SuccessionFlow {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for SuccessionFlow {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for SuccessionFlow {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlow::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SuccessionFlow::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for SuccessionFlowRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for SuccessionFlowRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SuccessionFlowRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for SuccessionFlowRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionFlowRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SuccessionFlowRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl StepStruct for SuccessionFlow {}
impl StepStructRefMut for SuccessionFlow {}
impl StepStructRef for SuccessionFlow {}
impl<'a> StepStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> StepStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> StepStructRef for SuccessionFlowRef<'a> {}
impl SuccessionStruct for SuccessionFlow {}
impl SuccessionStructRefMut for SuccessionFlow {}
impl SuccessionStructRef for SuccessionFlow {}
impl<'a> SuccessionStructRefMut for SuccessionFlowRefMut<'a> {}
impl<'a> SuccessionStructRef for SuccessionFlowRefMut<'a> {}
impl<'a> SuccessionStructRef for SuccessionFlowRef<'a> {}
impl SuccessionFlowUpcast for SuccessionFlow {
    fn into_succession_flow(self) -> SuccessionFlow {
        self
    }
}
impl<'a> SuccessionFlowUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_succession_flow_ref_mut(self) -> SuccessionFlowRefMut<'a> {
        self
    }
}
impl<'a> SuccessionFlowUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_succession_flow_ref(self) -> SuccessionFlowRef<'a> {
        self
    }
}
impl FlowUpcast for SuccessionFlow {
    fn into_flow(self) -> Flow {
        Flow::SuccessionFlow(self).into_flow()
    }
}
impl<'a> FlowUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_flow_ref_mut(self) -> FlowRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_flow_ref_mut()
    }
}
impl<'a> FlowUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_flow_ref(self) -> FlowRef<'a> {
        FlowRef::SuccessionFlow(self).as_flow_ref()
    }
}
impl ConnectorUpcast for SuccessionFlow {
    fn into_connector(self) -> Connector {
        Flow::SuccessionFlow(self).into_connector()
    }
}
impl<'a> ConnectorUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_connector_ref_mut()
    }
}
impl<'a> ConnectorUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_connector_ref(self) -> ConnectorRef<'a> {
        FlowRef::SuccessionFlow(self).as_connector_ref()
    }
}
impl FeatureUpcast for SuccessionFlow {
    fn into_feature(self) -> Feature {
        Flow::SuccessionFlow(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FlowRef::SuccessionFlow(self).as_feature_ref()
    }
}
impl TypeUpcast for SuccessionFlow {
    fn into_type_(self) -> Type {
        Flow::SuccessionFlow(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FlowRef::SuccessionFlow(self).as_type_ref()
    }
}
impl NamespaceUpcast for SuccessionFlow {
    fn into_namespace(self) -> Namespace {
        Flow::SuccessionFlow(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FlowRef::SuccessionFlow(self).as_namespace_ref()
    }
}
impl ElementUpcast for SuccessionFlow {
    fn into_element(self) -> Element {
        Flow::SuccessionFlow(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FlowRef::SuccessionFlow(self).as_element_ref()
    }
}
impl RelationshipUpcast for SuccessionFlow {
    fn into_relationship(self) -> Relationship {
        Flow::SuccessionFlow(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        FlowRef::SuccessionFlow(self).as_relationship_ref()
    }
}
impl StepUpcast for SuccessionFlow {
    fn into_step(self) -> Step {
        Flow::SuccessionFlow(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        FlowRefMut::SuccessionFlow(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        FlowRef::SuccessionFlow(self).as_step_ref()
    }
}
impl SuccessionUpcast for SuccessionFlow {
    fn into_succession(self) -> Succession {
        Succession::SuccessionFlow(self).into_succession()
    }
}
impl<'a> SuccessionUpcastRefMut<'a> for SuccessionFlowRefMut<'a> {
    fn as_succession_ref_mut(self) -> SuccessionRefMut<'a> {
        SuccessionRefMut::SuccessionFlow(self).as_succession_ref_mut()
    }
}
impl<'a> SuccessionUpcastRef<'a> for SuccessionFlowRef<'a> {
    fn as_succession_ref(self) -> SuccessionRef<'a> {
        SuccessionRef::SuccessionFlow(self).as_succession_ref()
    }
}
pub trait SuccessionFlowDowncast {}
pub trait SuccessionFlowDowncastRefMut<'a> {}
pub trait SuccessionFlowDowncastRef<'a> {}
impl SuccessionFlowDowncast for SuccessionFlow {}
impl<'a> SuccessionFlowDowncastRefMut<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> SuccessionFlowDowncastRef<'a> for SuccessionFlowRef<'a> {}
pub trait SuccessionFlowMethodsDescendants
where
    Self: DescendantOf<SuccessionFlow>,
    Self::Via: SuccessionFlowMethods,
    Self: Sized,
{}
pub trait SuccessionFlowMethods: Sized {}
impl<T: SuccessionFlowMethodsDescendants> SuccessionFlowMethods for T
where
    T::Via: SuccessionFlowMethods,
{}
impl DescendantOf<Succession> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl SuccessionMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Connector> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl ConnectorMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Feature> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl FeatureMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Type> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl TypeMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Namespace> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl NamespaceMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Element> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl ElementMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Relationship> for SuccessionFlow {
    type Via = Succession;
    fn into_via(self) -> Self::Via {
        self.into_succession()
    }
}
impl RelationshipMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Flow> for SuccessionFlow {
    type Via = Flow;
    fn into_via(self) -> Self::Via {
        self.into_flow()
    }
}
impl FlowMethodsDescendants for SuccessionFlow {}
impl DescendantOf<Step> for SuccessionFlow {
    type Via = Flow;
    fn into_via(self) -> Self::Via {
        self.into_flow()
    }
}
impl StepMethodsDescendants for SuccessionFlow {}
pub trait SuccessionFlowRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SuccessionFlowRefMut<'a>>,
    Self::Via: SuccessionFlowRefMutMethods,
    Self: Sized,
{}
pub trait SuccessionFlowRefMutMethods: Sized {}
impl<'a, T: SuccessionFlowRefMutMethodsDescendants<'a>> SuccessionFlowRefMutMethods for T
where
    T::Via: SuccessionFlowRefMutMethods,
{}
impl<'a> DescendantOf<SuccessionRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> SuccessionRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<ConnectorRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> ConnectorRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = SuccessionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<FlowRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = FlowRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_flow_ref_mut()
    }
}
impl<'a> FlowRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for SuccessionFlowRefMut<'a> {
    type Via = FlowRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_flow_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for SuccessionFlowRefMut<'a> {}
pub trait SuccessionFlowRefMethodsDescendants<'a>
where
    Self: DescendantOf<SuccessionFlowRef<'a>>,
    Self::Via: SuccessionFlowRefMethods,
    Self: Sized,
{}
pub trait SuccessionFlowRefMethods: Sized {}
impl<'a, T: SuccessionFlowRefMethodsDescendants<'a>> SuccessionFlowRefMethods for T
where
    T::Via: SuccessionFlowRefMethods,
{}
impl<'a> DescendantOf<SuccessionRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> SuccessionRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<ConnectorRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> ConnectorRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for SuccessionFlowRef<'a> {
    type Via = SuccessionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_succession_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<FlowRef<'a>> for SuccessionFlowRef<'a> {
    type Via = FlowRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_flow_ref()
    }
}
impl<'a> FlowRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for SuccessionFlowRef<'a> {
    type Via = FlowRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_flow_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for SuccessionFlowRef<'a> {}

