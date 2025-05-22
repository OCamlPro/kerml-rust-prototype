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
use super::flow::{Flow, FlowRefMut, FlowRef};
use super::expression::{Expression, ExpressionRefMut, ExpressionRef};
pub struct StepInner {
    pub(super) sup_feature: FeatureInner,
}
pub trait StepStruct
where
    Self: StepStructRefMut,
    Self: StepStructRef,
    Self: FeatureStruct,
{}
pub trait StepStructRefMut
where
    Self: StepStructRef,
    Self: FeatureStructRefMut,
{}
pub trait StepStructRef
where
    Self: FeatureStructRef,
{}
pub trait StepUpcast: StepStruct {
    fn into_step(self) -> Step;
}
pub trait StepUpcastRefMut<'a>: StepStructRefMut {
    fn as_step_ref_mut(self) -> StepRefMut<'a>;
}
pub trait StepUpcastRef<'a>: StepStructRef {
    fn as_step_ref(self) -> StepRef<'a>;
}
impl StepStruct for StepInner {}
impl StepStructRefMut for StepInner {}
impl StepStructRef for StepInner {}
impl FeatureStruct for StepInner {
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
impl FeatureStructRefMut for StepInner {
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
impl FeatureStructRef for StepInner {
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
impl TypeStruct for StepInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for StepInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for StepInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for StepInner {}
impl NamespaceStructRefMut for StepInner {}
impl NamespaceStructRef for StepInner {}
impl ElementStruct for StepInner {
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
impl ElementStructRefMut for StepInner {
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
impl ElementStructRef for StepInner {
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
pub enum Step {
    Itself(StepInner),
    Flow(Flow),
    Expression(Expression),
}
pub enum StepRefMut<'a> {
    Itself(&'a mut StepInner),
    Flow(FlowRefMut<'a>),
    Expression(ExpressionRefMut<'a>),
}
pub enum StepRef<'a> {
    Itself(&'a StepInner),
    Flow(FlowRef<'a>),
    Expression(ExpressionRef<'a>),
}
impl Step {
    pub fn as_ref(&self) -> StepRef {
        match self {
            Step::Itself(inner) => StepRef::Itself(&inner),
            Step::Flow(inner) => StepRef::Flow(inner.as_ref()),
            Step::Expression(inner) => StepRef::Expression(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> StepRefMut {
        match self {
            Step::Itself(inner) => StepRefMut::Itself(inner),
            Step::Flow(inner) => StepRefMut::Flow(inner.as_ref_mut()),
            Step::Expression(inner) => StepRefMut::Expression(inner.as_ref_mut()),
        }
    }
}
impl<'a> StepRefMut<'a> {
    pub fn as_ref(self) -> StepRef<'a> {
        match self {
            StepRefMut::Itself(inner) => StepRef::Itself(inner),
            StepRefMut::Flow(inner) => StepRef::Flow(inner.as_ref()),
            StepRefMut::Expression(inner) => StepRef::Expression(inner.as_ref()),
        }
    }
}
impl StepStruct for Step {}
impl StepStructRefMut for Step {}
impl StepStructRef for Step {}
impl<'a> StepStructRefMut for StepRefMut<'a> {}
impl<'a> StepStructRef for StepRefMut<'a> {}
impl<'a> StepStructRef for StepRef<'a> {}
impl FeatureStruct for Step {
    fn is_unique(self) -> bool {
        match self {
            Step::Itself(x) => x.is_unique(),
            Step::Flow(x) => x.is_unique(),
            Step::Expression(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Step::Itself(x) => x.is_ordered(),
            Step::Flow(x) => x.is_ordered(),
            Step::Expression(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Step::Itself(x) => x.is_composite(),
            Step::Flow(x) => x.is_composite(),
            Step::Expression(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Step::Itself(x) => x.is_end(),
            Step::Flow(x) => x.is_end(),
            Step::Expression(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Step::Itself(x) => x.is_derived(),
            Step::Flow(x) => x.is_derived(),
            Step::Expression(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Step::Itself(x) => x.is_portion(),
            Step::Flow(x) => x.is_portion(),
            Step::Expression(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Step::Itself(x) => x.is_variable(),
            Step::Flow(x) => x.is_variable(),
            Step::Expression(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Step::Itself(x) => x.is_constant(),
            Step::Flow(x) => x.is_constant(),
            Step::Expression(x) => x.is_constant(),
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
            Step::Itself(x) => x.direction(),
            Step::Flow(x) => x.direction(),
            Step::Expression(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Step {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_unique_ref_mut(),
            Step::Flow(x) => x.is_unique_ref_mut(),
            Step::Expression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_ordered_ref_mut(),
            Step::Flow(x) => x.is_ordered_ref_mut(),
            Step::Expression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_composite_ref_mut(),
            Step::Flow(x) => x.is_composite_ref_mut(),
            Step::Expression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_end_ref_mut(),
            Step::Flow(x) => x.is_end_ref_mut(),
            Step::Expression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_derived_ref_mut(),
            Step::Flow(x) => x.is_derived_ref_mut(),
            Step::Expression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_portion_ref_mut(),
            Step::Flow(x) => x.is_portion_ref_mut(),
            Step::Expression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_variable_ref_mut(),
            Step::Flow(x) => x.is_variable_ref_mut(),
            Step::Expression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_constant_ref_mut(),
            Step::Flow(x) => x.is_constant_ref_mut(),
            Step::Expression(x) => x.is_constant_ref_mut(),
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
            Step::Itself(x) => x.direction_ref_mut(),
            Step::Flow(x) => x.direction_ref_mut(),
            Step::Expression(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Step {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_unique_ref(),
            Step::Flow(x) => x.is_unique_ref(),
            Step::Expression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_ordered_ref(),
            Step::Flow(x) => x.is_ordered_ref(),
            Step::Expression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_composite_ref(),
            Step::Flow(x) => x.is_composite_ref(),
            Step::Expression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_end_ref(),
            Step::Flow(x) => x.is_end_ref(),
            Step::Expression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_derived_ref(),
            Step::Flow(x) => x.is_derived_ref(),
            Step::Expression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_portion_ref(),
            Step::Flow(x) => x.is_portion_ref(),
            Step::Expression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_variable_ref(),
            Step::Flow(x) => x.is_variable_ref(),
            Step::Expression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_constant_ref(),
            Step::Flow(x) => x.is_constant_ref(),
            Step::Expression(x) => x.is_constant_ref(),
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
            Step::Itself(x) => x.direction_ref(),
            Step::Flow(x) => x.direction_ref(),
            Step::Expression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for StepRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_unique_ref_mut(),
            StepRefMut::Flow(x) => x.is_unique_ref_mut(),
            StepRefMut::Expression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_ordered_ref_mut(),
            StepRefMut::Flow(x) => x.is_ordered_ref_mut(),
            StepRefMut::Expression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_composite_ref_mut(),
            StepRefMut::Flow(x) => x.is_composite_ref_mut(),
            StepRefMut::Expression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_end_ref_mut(),
            StepRefMut::Flow(x) => x.is_end_ref_mut(),
            StepRefMut::Expression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_derived_ref_mut(),
            StepRefMut::Flow(x) => x.is_derived_ref_mut(),
            StepRefMut::Expression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_portion_ref_mut(),
            StepRefMut::Flow(x) => x.is_portion_ref_mut(),
            StepRefMut::Expression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_variable_ref_mut(),
            StepRefMut::Flow(x) => x.is_variable_ref_mut(),
            StepRefMut::Expression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_constant_ref_mut(),
            StepRefMut::Flow(x) => x.is_constant_ref_mut(),
            StepRefMut::Expression(x) => x.is_constant_ref_mut(),
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
            StepRefMut::Itself(x) => x.direction_ref_mut(),
            StepRefMut::Flow(x) => x.direction_ref_mut(),
            StepRefMut::Expression(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for StepRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_unique_ref(),
            StepRefMut::Flow(x) => x.is_unique_ref(),
            StepRefMut::Expression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_ordered_ref(),
            StepRefMut::Flow(x) => x.is_ordered_ref(),
            StepRefMut::Expression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_composite_ref(),
            StepRefMut::Flow(x) => x.is_composite_ref(),
            StepRefMut::Expression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_end_ref(),
            StepRefMut::Flow(x) => x.is_end_ref(),
            StepRefMut::Expression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_derived_ref(),
            StepRefMut::Flow(x) => x.is_derived_ref(),
            StepRefMut::Expression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_portion_ref(),
            StepRefMut::Flow(x) => x.is_portion_ref(),
            StepRefMut::Expression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_variable_ref(),
            StepRefMut::Flow(x) => x.is_variable_ref(),
            StepRefMut::Expression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_constant_ref(),
            StepRefMut::Flow(x) => x.is_constant_ref(),
            StepRefMut::Expression(x) => x.is_constant_ref(),
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
            StepRefMut::Itself(x) => x.direction_ref(),
            StepRefMut::Flow(x) => x.direction_ref(),
            StepRefMut::Expression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for StepRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_unique_ref(),
            StepRef::Flow(x) => x.is_unique_ref(),
            StepRef::Expression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_ordered_ref(),
            StepRef::Flow(x) => x.is_ordered_ref(),
            StepRef::Expression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_composite_ref(),
            StepRef::Flow(x) => x.is_composite_ref(),
            StepRef::Expression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_end_ref(),
            StepRef::Flow(x) => x.is_end_ref(),
            StepRef::Expression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_derived_ref(),
            StepRef::Flow(x) => x.is_derived_ref(),
            StepRef::Expression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_portion_ref(),
            StepRef::Flow(x) => x.is_portion_ref(),
            StepRef::Expression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_variable_ref(),
            StepRef::Flow(x) => x.is_variable_ref(),
            StepRef::Expression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_constant_ref(),
            StepRef::Flow(x) => x.is_constant_ref(),
            StepRef::Expression(x) => x.is_constant_ref(),
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
            StepRef::Itself(x) => x.direction_ref(),
            StepRef::Flow(x) => x.direction_ref(),
            StepRef::Expression(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Step {
    fn is_abstract(self) -> bool {
        match self {
            Step::Itself(x) => x.is_abstract(),
            Step::Flow(x) => x.is_abstract(),
            Step::Expression(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Step::Itself(x) => x.is_sufficient(),
            Step::Flow(x) => x.is_sufficient(),
            Step::Expression(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Step {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_abstract_ref_mut(),
            Step::Flow(x) => x.is_abstract_ref_mut(),
            Step::Expression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_sufficient_ref_mut(),
            Step::Flow(x) => x.is_sufficient_ref_mut(),
            Step::Expression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Step {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_abstract_ref(),
            Step::Flow(x) => x.is_abstract_ref(),
            Step::Expression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_sufficient_ref(),
            Step::Flow(x) => x.is_sufficient_ref(),
            Step::Expression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for StepRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_abstract_ref_mut(),
            StepRefMut::Flow(x) => x.is_abstract_ref_mut(),
            StepRefMut::Expression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            StepRefMut::Flow(x) => x.is_sufficient_ref_mut(),
            StepRefMut::Expression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for StepRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_abstract_ref(),
            StepRefMut::Flow(x) => x.is_abstract_ref(),
            StepRefMut::Expression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_sufficient_ref(),
            StepRefMut::Flow(x) => x.is_sufficient_ref(),
            StepRefMut::Expression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for StepRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_abstract_ref(),
            StepRef::Flow(x) => x.is_abstract_ref(),
            StepRef::Expression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_sufficient_ref(),
            StepRef::Flow(x) => x.is_sufficient_ref(),
            StepRef::Expression(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Step {}
impl NamespaceStructRefMut for Step {}
impl NamespaceStructRef for Step {}
impl<'a> NamespaceStructRefMut for StepRefMut<'a> {}
impl<'a> NamespaceStructRef for StepRefMut<'a> {}
impl<'a> NamespaceStructRef for StepRef<'a> {}
impl ElementStruct for Step {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Step::Itself(x) => x.owned_relationship(),
            Step::Flow(x) => x.owned_relationship(),
            Step::Expression(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Step::Itself(x) => x.owning_relationship(),
            Step::Flow(x) => x.owning_relationship(),
            Step::Expression(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Step::Itself(x) => x.element_id(),
            Step::Flow(x) => x.element_id(),
            Step::Expression(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Step::Itself(x) => x.alias_ids(),
            Step::Flow(x) => x.alias_ids(),
            Step::Expression(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Step::Itself(x) => x.declared_short_name(),
            Step::Flow(x) => x.declared_short_name(),
            Step::Expression(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Step::Itself(x) => x.declared_name(),
            Step::Flow(x) => x.declared_name(),
            Step::Expression(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Step::Itself(x) => x.is_implied_included(),
            Step::Flow(x) => x.is_implied_included(),
            Step::Expression(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Step {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Step::Itself(x) => x.owned_relationship_ref_mut(),
            Step::Flow(x) => x.owned_relationship_ref_mut(),
            Step::Expression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Step::Itself(x) => x.owning_relationship_ref_mut(),
            Step::Flow(x) => x.owning_relationship_ref_mut(),
            Step::Expression(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Step::Itself(x) => x.element_id_ref_mut(),
            Step::Flow(x) => x.element_id_ref_mut(),
            Step::Expression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Step::Itself(x) => x.alias_ids_ref_mut(),
            Step::Flow(x) => x.alias_ids_ref_mut(),
            Step::Expression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Step::Itself(x) => x.declared_short_name_ref_mut(),
            Step::Flow(x) => x.declared_short_name_ref_mut(),
            Step::Expression(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Step::Itself(x) => x.declared_name_ref_mut(),
            Step::Flow(x) => x.declared_name_ref_mut(),
            Step::Expression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Step::Itself(x) => x.is_implied_included_ref_mut(),
            Step::Flow(x) => x.is_implied_included_ref_mut(),
            Step::Expression(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Step {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Step::Itself(x) => x.owned_relationship_ref(),
            Step::Flow(x) => x.owned_relationship_ref(),
            Step::Expression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Step::Itself(x) => x.owning_relationship_ref(),
            Step::Flow(x) => x.owning_relationship_ref(),
            Step::Expression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Step::Itself(x) => x.element_id_ref(),
            Step::Flow(x) => x.element_id_ref(),
            Step::Expression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Step::Itself(x) => x.alias_ids_ref(),
            Step::Flow(x) => x.alias_ids_ref(),
            Step::Expression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Step::Itself(x) => x.declared_short_name_ref(),
            Step::Flow(x) => x.declared_short_name_ref(),
            Step::Expression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Step::Itself(x) => x.declared_name_ref(),
            Step::Flow(x) => x.declared_name_ref(),
            Step::Expression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Step::Itself(x) => x.is_implied_included_ref(),
            Step::Flow(x) => x.is_implied_included_ref(),
            Step::Expression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for StepRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StepRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            StepRefMut::Flow(x) => x.owned_relationship_ref_mut(),
            StepRefMut::Expression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            StepRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            StepRefMut::Flow(x) => x.owning_relationship_ref_mut(),
            StepRefMut::Expression(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            StepRefMut::Itself(x) => x.element_id_ref_mut(),
            StepRefMut::Flow(x) => x.element_id_ref_mut(),
            StepRefMut::Expression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            StepRefMut::Itself(x) => x.alias_ids_ref_mut(),
            StepRefMut::Flow(x) => x.alias_ids_ref_mut(),
            StepRefMut::Expression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            StepRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            StepRefMut::Flow(x) => x.declared_short_name_ref_mut(),
            StepRefMut::Expression(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            StepRefMut::Itself(x) => x.declared_name_ref_mut(),
            StepRefMut::Flow(x) => x.declared_name_ref_mut(),
            StepRefMut::Expression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            StepRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            StepRefMut::Flow(x) => x.is_implied_included_ref_mut(),
            StepRefMut::Expression(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for StepRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StepRefMut::Itself(x) => x.owned_relationship_ref(),
            StepRefMut::Flow(x) => x.owned_relationship_ref(),
            StepRefMut::Expression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StepRefMut::Itself(x) => x.owning_relationship_ref(),
            StepRefMut::Flow(x) => x.owning_relationship_ref(),
            StepRefMut::Expression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            StepRefMut::Itself(x) => x.element_id_ref(),
            StepRefMut::Flow(x) => x.element_id_ref(),
            StepRefMut::Expression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            StepRefMut::Itself(x) => x.alias_ids_ref(),
            StepRefMut::Flow(x) => x.alias_ids_ref(),
            StepRefMut::Expression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            StepRefMut::Itself(x) => x.declared_short_name_ref(),
            StepRefMut::Flow(x) => x.declared_short_name_ref(),
            StepRefMut::Expression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            StepRefMut::Itself(x) => x.declared_name_ref(),
            StepRefMut::Flow(x) => x.declared_name_ref(),
            StepRefMut::Expression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            StepRefMut::Itself(x) => x.is_implied_included_ref(),
            StepRefMut::Flow(x) => x.is_implied_included_ref(),
            StepRefMut::Expression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for StepRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StepRef::Itself(x) => x.owned_relationship_ref(),
            StepRef::Flow(x) => x.owned_relationship_ref(),
            StepRef::Expression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            StepRef::Itself(x) => x.owning_relationship_ref(),
            StepRef::Flow(x) => x.owning_relationship_ref(),
            StepRef::Expression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            StepRef::Itself(x) => x.element_id_ref(),
            StepRef::Flow(x) => x.element_id_ref(),
            StepRef::Expression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            StepRef::Itself(x) => x.alias_ids_ref(),
            StepRef::Flow(x) => x.alias_ids_ref(),
            StepRef::Expression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            StepRef::Itself(x) => x.declared_short_name_ref(),
            StepRef::Flow(x) => x.declared_short_name_ref(),
            StepRef::Expression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            StepRef::Itself(x) => x.declared_name_ref(),
            StepRef::Flow(x) => x.declared_name_ref(),
            StepRef::Expression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            StepRef::Itself(x) => x.is_implied_included_ref(),
            StepRef::Flow(x) => x.is_implied_included_ref(),
            StepRef::Expression(x) => x.is_implied_included_ref(),
        }
    }
}
impl StepUpcast for Step {
    fn into_step(self) -> Step {
        self
    }
}
impl<'a> StepUpcastRefMut<'a> for StepRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        self
    }
}
impl<'a> StepUpcastRef<'a> for StepRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        self
    }
}
impl FeatureUpcast for Step {
    fn into_feature(self) -> Feature {
        Feature::Step(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for StepRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::Step(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for StepRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::Step(self).as_feature_ref()
    }
}
impl TypeUpcast for Step {
    fn into_type_(self) -> Type {
        Feature::Step(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for StepRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::Step(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for StepRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::Step(self).as_type_ref()
    }
}
impl NamespaceUpcast for Step {
    fn into_namespace(self) -> Namespace {
        Feature::Step(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for StepRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::Step(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for StepRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::Step(self).as_namespace_ref()
    }
}
impl ElementUpcast for Step {
    fn into_element(self) -> Element {
        Feature::Step(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for StepRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::Step(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for StepRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::Step(self).as_element_ref()
    }
}
pub trait StepDowncast {
    fn try_into_flow(self) -> Result<Flow, String>;
    fn try_into_expression(self) -> Result<Expression, String>;
}
pub trait StepDowncastRefMut<'a> {
    fn try_as_flow_ref_mut(self) -> Result<FlowRefMut<'a>, String>;
    fn try_as_expression_ref_mut(self) -> Result<ExpressionRefMut<'a>, String>;
}
pub trait StepDowncastRef<'a> {
    fn try_as_flow_ref(self) -> Result<FlowRef<'a>, String>;
    fn try_as_expression_ref(self) -> Result<ExpressionRef<'a>, String>;
}
impl StepDowncast for Step {
    fn try_into_flow(self) -> Result<Flow, String> {
        match self {
            Step::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_into_expression(self) -> Result<Expression, String> {
        match self {
            Step::Expression(e) => Ok(e),
            _ => Err("Not a Expression".into()),
        }
    }
}
impl<'a> StepDowncastRefMut<'a> for StepRefMut<'a> {
    fn try_as_flow_ref_mut(self) -> Result<FlowRefMut<'a>, String> {
        match self {
            StepRefMut::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_as_expression_ref_mut(self) -> Result<ExpressionRefMut<'a>, String> {
        match self {
            StepRefMut::Expression(e) => Ok(e),
            _ => Err("Not a Expression".into()),
        }
    }
}
impl<'a> StepDowncastRef<'a> for StepRef<'a> {
    fn try_as_flow_ref(self) -> Result<FlowRef<'a>, String> {
        match self {
            StepRef::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_as_expression_ref(self) -> Result<ExpressionRef<'a>, String> {
        match self {
            StepRef::Expression(e) => Ok(e),
            _ => Err("Not a Expression".into()),
        }
    }
}
pub trait StepMethodsDescendants
where
    Self: DescendantOf<Step>,
    Self::Via: StepMethods,
    Self: Sized,
{}
pub trait StepMethods: Sized {}
impl<T: StepMethodsDescendants> StepMethods for T
where
    T::Via: StepMethods,
{}
impl DescendantOf<Feature> for Step {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for Step {}
impl DescendantOf<Type> for Step {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for Step {}
impl DescendantOf<Namespace> for Step {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for Step {}
impl DescendantOf<Element> for Step {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl ElementMethodsDescendants for Step {}
pub trait StepRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<StepRefMut<'a>>,
    Self::Via: StepRefMutMethods,
    Self: Sized,
{}
pub trait StepRefMutMethods: Sized {}
impl<'a, T: StepRefMutMethodsDescendants<'a>> StepRefMutMethods for T
where
    T::Via: StepRefMutMethods,
{}
impl<'a> DescendantOf<FeatureRefMut<'a>> for StepRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for StepRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for StepRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for StepRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for StepRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for StepRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for StepRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for StepRefMut<'a> {}
pub trait StepRefMethodsDescendants<'a>
where
    Self: DescendantOf<StepRef<'a>>,
    Self::Via: StepRefMethods,
    Self: Sized,
{}
pub trait StepRefMethods: Sized {}
impl<'a, T: StepRefMethodsDescendants<'a>> StepRefMethods for T
where
    T::Via: StepRefMethods,
{}
impl<'a> DescendantOf<FeatureRef<'a>> for StepRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for StepRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for StepRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for StepRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for StepRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for StepRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for StepRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for StepRef<'a> {}

