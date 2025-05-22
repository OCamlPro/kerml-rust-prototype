#![allow(unused)]
use super::utils::DescendantOf;
use super::operator_expression::{
    OperatorExpression, OperatorExpressionRefMut, OperatorExpressionRef,
    OperatorExpressionStruct, OperatorExpressionStructRefMut,
    OperatorExpressionStructRef, OperatorExpressionInner, OperatorExpressionUpcast,
    OperatorExpressionUpcastRefMut, OperatorExpressionUpcastRef,
    OperatorExpressionMethodsDescendants, OperatorExpressionRefMutMethodsDescendants,
    OperatorExpressionRefMethodsDescendants,
};
use super::invocation_expression::{
    InvocationExpression, InvocationExpressionRefMut, InvocationExpressionRef,
    InvocationExpressionStruct, InvocationExpressionStructRefMut,
    InvocationExpressionStructRef, InvocationExpressionInner, InvocationExpressionUpcast,
    InvocationExpressionUpcastRefMut, InvocationExpressionUpcastRef,
    InvocationExpressionMethodsDescendants, InvocationExpressionRefMutMethodsDescendants,
    InvocationExpressionRefMethodsDescendants,
};
use super::instantiation_expression::{
    InstantiationExpression, InstantiationExpressionRefMut, InstantiationExpressionRef,
    InstantiationExpressionStruct, InstantiationExpressionStructRefMut,
    InstantiationExpressionStructRef, InstantiationExpressionInner,
    InstantiationExpressionUpcast, InstantiationExpressionUpcastRefMut,
    InstantiationExpressionUpcastRef, InstantiationExpressionMethodsDescendants,
    InstantiationExpressionRefMutMethodsDescendants,
    InstantiationExpressionRefMethodsDescendants,
};
use super::expression::{
    Expression, ExpressionRefMut, ExpressionRef, ExpressionStruct,
    ExpressionStructRefMut, ExpressionStructRef, ExpressionInner, ExpressionUpcast,
    ExpressionUpcastRefMut, ExpressionUpcastRef, ExpressionMethodsDescendants,
    ExpressionRefMutMethodsDescendants, ExpressionRefMethodsDescendants,
};
use super::step::{
    Step, StepRefMut, StepRef, StepStruct, StepStructRefMut, StepStructRef, StepInner,
    StepUpcast, StepUpcastRefMut, StepUpcastRef, StepMethodsDescendants,
    StepRefMutMethodsDescendants, StepRefMethodsDescendants,
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
pub struct CollectExpressionInner {
    pub(super) sup_operator_expression: OperatorExpressionInner,
}
pub trait CollectExpressionStruct
where
    Self: CollectExpressionStructRefMut,
    Self: CollectExpressionStructRef,
    Self: OperatorExpressionStruct,
{}
pub trait CollectExpressionStructRefMut
where
    Self: CollectExpressionStructRef,
    Self: OperatorExpressionStructRefMut,
{}
pub trait CollectExpressionStructRef
where
    Self: OperatorExpressionStructRef,
{}
pub trait CollectExpressionUpcast: CollectExpressionStruct {
    fn into_collect_expression(self) -> CollectExpression;
}
pub trait CollectExpressionUpcastRefMut<'a>: CollectExpressionStructRefMut {
    fn as_collect_expression_ref_mut(self) -> CollectExpressionRefMut<'a>;
}
pub trait CollectExpressionUpcastRef<'a>: CollectExpressionStructRef {
    fn as_collect_expression_ref(self) -> CollectExpressionRef<'a>;
}
impl CollectExpressionStruct for CollectExpressionInner {}
impl CollectExpressionStructRefMut for CollectExpressionInner {}
impl CollectExpressionStructRef for CollectExpressionInner {}
impl OperatorExpressionStruct for CollectExpressionInner {
    fn operator(self) -> String {
        self.sup_operator_expression.operator()
    }
}
impl OperatorExpressionStructRefMut for CollectExpressionInner {
    fn operator_ref_mut(&mut self) -> &mut String {
        self.sup_operator_expression.operator_ref_mut()
    }
}
impl OperatorExpressionStructRef for CollectExpressionInner {
    fn operator_ref(&self) -> &String {
        self.sup_operator_expression.operator_ref()
    }
}
impl InvocationExpressionStruct for CollectExpressionInner {}
impl InvocationExpressionStructRefMut for CollectExpressionInner {}
impl InvocationExpressionStructRef for CollectExpressionInner {}
impl InstantiationExpressionStruct for CollectExpressionInner {}
impl InstantiationExpressionStructRefMut for CollectExpressionInner {}
impl InstantiationExpressionStructRef for CollectExpressionInner {}
impl ExpressionStruct for CollectExpressionInner {}
impl ExpressionStructRefMut for CollectExpressionInner {}
impl ExpressionStructRef for CollectExpressionInner {}
impl StepStruct for CollectExpressionInner {}
impl StepStructRefMut for CollectExpressionInner {}
impl StepStructRef for CollectExpressionInner {}
impl FeatureStruct for CollectExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_operator_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_operator_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_operator_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_operator_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_operator_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_operator_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_operator_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_operator_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_operator_expression.direction()
    }
}
impl FeatureStructRefMut for CollectExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_operator_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for CollectExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_operator_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_operator_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_operator_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_operator_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_operator_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_operator_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_operator_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_operator_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_operator_expression.direction_ref()
    }
}
impl TypeStruct for CollectExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_operator_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_operator_expression.is_sufficient()
    }
}
impl TypeStructRefMut for CollectExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for CollectExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_operator_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_operator_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for CollectExpressionInner {}
impl NamespaceStructRefMut for CollectExpressionInner {}
impl NamespaceStructRef for CollectExpressionInner {}
impl ElementStruct for CollectExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_operator_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_operator_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_operator_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_operator_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_operator_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_operator_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_operator_expression.is_implied_included()
    }
}
impl ElementStructRefMut for CollectExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_operator_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_operator_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_operator_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_operator_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_operator_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_operator_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for CollectExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_operator_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_operator_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_operator_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_operator_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_operator_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_operator_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_operator_expression.is_implied_included_ref()
    }
}
pub enum CollectExpression {
    Itself(CollectExpressionInner),
}
pub enum CollectExpressionRefMut<'a> {
    Itself(&'a mut CollectExpressionInner),
}
pub enum CollectExpressionRef<'a> {
    Itself(&'a CollectExpressionInner),
}
impl CollectExpression {
    pub fn as_ref(&self) -> CollectExpressionRef {
        match self {
            CollectExpression::Itself(inner) => CollectExpressionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> CollectExpressionRefMut {
        match self {
            CollectExpression::Itself(inner) => CollectExpressionRefMut::Itself(inner),
        }
    }
}
impl<'a> CollectExpressionRefMut<'a> {
    pub fn as_ref(self) -> CollectExpressionRef<'a> {
        match self {
            CollectExpressionRefMut::Itself(inner) => CollectExpressionRef::Itself(inner),
        }
    }
}
impl CollectExpressionStruct for CollectExpression {}
impl CollectExpressionStructRefMut for CollectExpression {}
impl CollectExpressionStructRef for CollectExpression {}
impl<'a> CollectExpressionStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> CollectExpressionStructRef for CollectExpressionRefMut<'a> {}
impl<'a> CollectExpressionStructRef for CollectExpressionRef<'a> {}
impl OperatorExpressionStruct for CollectExpression {
    fn operator(self) -> String {
        match self {
            CollectExpression::Itself(x) => x.operator(),
        }
    }
}
impl OperatorExpressionStructRefMut for CollectExpression {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            CollectExpression::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl OperatorExpressionStructRef for CollectExpression {
    fn operator_ref(&self) -> &String {
        match self {
            CollectExpression::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRefMut for CollectExpressionRefMut<'a> {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            CollectExpressionRefMut::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for CollectExpressionRefMut<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            CollectExpressionRefMut::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for CollectExpressionRef<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            CollectExpressionRef::Itself(x) => x.operator_ref(),
        }
    }
}
impl InvocationExpressionStruct for CollectExpression {}
impl InvocationExpressionStructRefMut for CollectExpression {}
impl InvocationExpressionStructRef for CollectExpression {}
impl<'a> InvocationExpressionStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for CollectExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for CollectExpressionRef<'a> {}
impl InstantiationExpressionStruct for CollectExpression {}
impl InstantiationExpressionStructRefMut for CollectExpression {}
impl InstantiationExpressionStructRef for CollectExpression {}
impl<'a> InstantiationExpressionStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for CollectExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for CollectExpressionRef<'a> {}
impl ExpressionStruct for CollectExpression {}
impl ExpressionStructRefMut for CollectExpression {}
impl ExpressionStructRef for CollectExpression {}
impl<'a> ExpressionStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for CollectExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for CollectExpressionRef<'a> {}
impl StepStruct for CollectExpression {}
impl StepStructRefMut for CollectExpression {}
impl StepStructRef for CollectExpression {}
impl<'a> StepStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> StepStructRef for CollectExpressionRefMut<'a> {}
impl<'a> StepStructRef for CollectExpressionRef<'a> {}
impl FeatureStruct for CollectExpression {
    fn is_unique(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_constant(),
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
            CollectExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for CollectExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_constant_ref_mut(),
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
            CollectExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for CollectExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_constant_ref(),
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
            CollectExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for CollectExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            CollectExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for CollectExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            CollectExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for CollectExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_constant_ref(),
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
            CollectExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for CollectExpression {
    fn is_abstract(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for CollectExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for CollectExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for CollectExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for CollectExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for CollectExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for CollectExpression {}
impl NamespaceStructRefMut for CollectExpression {}
impl NamespaceStructRef for CollectExpression {}
impl<'a> NamespaceStructRefMut for CollectExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for CollectExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for CollectExpressionRef<'a> {}
impl ElementStruct for CollectExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            CollectExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            CollectExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            CollectExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for CollectExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            CollectExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            CollectExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            CollectExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for CollectExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CollectExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CollectExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CollectExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for CollectExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            CollectExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            CollectExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for CollectExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CollectExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CollectExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for CollectExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            CollectExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            CollectExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            CollectExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            CollectExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            CollectExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl CollectExpressionUpcast for CollectExpression {
    fn into_collect_expression(self) -> CollectExpression {
        self
    }
}
impl<'a> CollectExpressionUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_collect_expression_ref_mut(self) -> CollectExpressionRefMut<'a> {
        self
    }
}
impl<'a> CollectExpressionUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_collect_expression_ref(self) -> CollectExpressionRef<'a> {
        self
    }
}
impl OperatorExpressionUpcast for CollectExpression {
    fn into_operator_expression(self) -> OperatorExpression {
        OperatorExpression::CollectExpression(self).into_operator_expression()
    }
}
impl<'a> OperatorExpressionUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self)
            .as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_operator_expression_ref()
    }
}
impl InvocationExpressionUpcast for CollectExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        OperatorExpression::CollectExpression(self).into_invocation_expression()
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self)
            .as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_invocation_expression_ref()
    }
}
impl InstantiationExpressionUpcast for CollectExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        OperatorExpression::CollectExpression(self).into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for CollectExpression {
    fn into_expression(self) -> Expression {
        OperatorExpression::CollectExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_expression_ref()
    }
}
impl StepUpcast for CollectExpression {
    fn into_step(self) -> Step {
        OperatorExpression::CollectExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for CollectExpression {
    fn into_feature(self) -> Feature {
        OperatorExpression::CollectExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for CollectExpression {
    fn into_type_(self) -> Type {
        OperatorExpression::CollectExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for CollectExpression {
    fn into_namespace(self) -> Namespace {
        OperatorExpression::CollectExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for CollectExpression {
    fn into_element(self) -> Element {
        OperatorExpression::CollectExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for CollectExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OperatorExpressionRefMut::CollectExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for CollectExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OperatorExpressionRef::CollectExpression(self).as_element_ref()
    }
}
pub trait CollectExpressionDowncast {}
pub trait CollectExpressionDowncastRefMut<'a> {}
pub trait CollectExpressionDowncastRef<'a> {}
impl CollectExpressionDowncast for CollectExpression {}
impl<'a> CollectExpressionDowncastRefMut<'a> for CollectExpressionRefMut<'a> {}
impl<'a> CollectExpressionDowncastRef<'a> for CollectExpressionRef<'a> {}
pub trait CollectExpressionMethodsDescendants
where
    Self: DescendantOf<CollectExpression>,
    Self::Via: CollectExpressionMethods,
    Self: Sized,
{}
pub trait CollectExpressionMethods: Sized {}
impl<T: CollectExpressionMethodsDescendants> CollectExpressionMethods for T
where
    T::Via: CollectExpressionMethods,
{}
impl DescendantOf<OperatorExpression> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl OperatorExpressionMethodsDescendants for CollectExpression {}
impl DescendantOf<InvocationExpression> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InvocationExpressionMethodsDescendants for CollectExpression {}
impl DescendantOf<InstantiationExpression> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for CollectExpression {}
impl DescendantOf<Expression> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ExpressionMethodsDescendants for CollectExpression {}
impl DescendantOf<Step> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl StepMethodsDescendants for CollectExpression {}
impl DescendantOf<Feature> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl FeatureMethodsDescendants for CollectExpression {}
impl DescendantOf<Type> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl TypeMethodsDescendants for CollectExpression {}
impl DescendantOf<Namespace> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl NamespaceMethodsDescendants for CollectExpression {}
impl DescendantOf<Element> for CollectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ElementMethodsDescendants for CollectExpression {}
pub trait CollectExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<CollectExpressionRefMut<'a>>,
    Self::Via: CollectExpressionRefMutMethods,
    Self: Sized,
{}
pub trait CollectExpressionRefMutMethods: Sized {}
impl<'a, T: CollectExpressionRefMutMethodsDescendants<'a>> CollectExpressionRefMutMethods
for T
where
    T::Via: CollectExpressionRefMutMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<InvocationExpressionRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionRefMutMethodsDescendants<'a>
for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>>
for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for CollectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for CollectExpressionRefMut<'a> {}
pub trait CollectExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<CollectExpressionRef<'a>>,
    Self::Via: CollectExpressionRefMethods,
    Self: Sized,
{}
pub trait CollectExpressionRefMethods: Sized {}
impl<'a, T: CollectExpressionRefMethodsDescendants<'a>> CollectExpressionRefMethods for T
where
    T::Via: CollectExpressionRefMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> OperatorExpressionRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<InvocationExpressionRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InvocationExpressionRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for CollectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for CollectExpressionRef<'a> {}

