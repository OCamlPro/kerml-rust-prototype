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
pub struct SelectExpressionInner {
    pub(super) sup_operator_expression: OperatorExpressionInner,
}
pub trait SelectExpressionStruct
where
    Self: SelectExpressionStructRefMut,
    Self: SelectExpressionStructRef,
    Self: OperatorExpressionStruct,
{}
pub trait SelectExpressionStructRefMut
where
    Self: SelectExpressionStructRef,
    Self: OperatorExpressionStructRefMut,
{}
pub trait SelectExpressionStructRef
where
    Self: OperatorExpressionStructRef,
{}
pub trait SelectExpressionUpcast: SelectExpressionStruct {
    fn into_select_expression(self) -> SelectExpression;
}
pub trait SelectExpressionUpcastRefMut<'a>: SelectExpressionStructRefMut {
    fn as_select_expression_ref_mut(self) -> SelectExpressionRefMut<'a>;
}
pub trait SelectExpressionUpcastRef<'a>: SelectExpressionStructRef {
    fn as_select_expression_ref(self) -> SelectExpressionRef<'a>;
}
impl SelectExpressionStruct for SelectExpressionInner {}
impl SelectExpressionStructRefMut for SelectExpressionInner {}
impl SelectExpressionStructRef for SelectExpressionInner {}
impl OperatorExpressionStruct for SelectExpressionInner {
    fn operator(self) -> String {
        self.sup_operator_expression.operator()
    }
}
impl OperatorExpressionStructRefMut for SelectExpressionInner {
    fn operator_ref_mut(&mut self) -> &mut String {
        self.sup_operator_expression.operator_ref_mut()
    }
}
impl OperatorExpressionStructRef for SelectExpressionInner {
    fn operator_ref(&self) -> &String {
        self.sup_operator_expression.operator_ref()
    }
}
impl InvocationExpressionStruct for SelectExpressionInner {}
impl InvocationExpressionStructRefMut for SelectExpressionInner {}
impl InvocationExpressionStructRef for SelectExpressionInner {}
impl InstantiationExpressionStruct for SelectExpressionInner {}
impl InstantiationExpressionStructRefMut for SelectExpressionInner {}
impl InstantiationExpressionStructRef for SelectExpressionInner {}
impl ExpressionStruct for SelectExpressionInner {}
impl ExpressionStructRefMut for SelectExpressionInner {}
impl ExpressionStructRef for SelectExpressionInner {}
impl StepStruct for SelectExpressionInner {}
impl StepStructRefMut for SelectExpressionInner {}
impl StepStructRef for SelectExpressionInner {}
impl FeatureStruct for SelectExpressionInner {
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
impl FeatureStructRefMut for SelectExpressionInner {
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
impl FeatureStructRef for SelectExpressionInner {
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
impl TypeStruct for SelectExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_operator_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_operator_expression.is_sufficient()
    }
}
impl TypeStructRefMut for SelectExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for SelectExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_operator_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_operator_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for SelectExpressionInner {}
impl NamespaceStructRefMut for SelectExpressionInner {}
impl NamespaceStructRef for SelectExpressionInner {}
impl ElementStruct for SelectExpressionInner {
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
impl ElementStructRefMut for SelectExpressionInner {
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
impl ElementStructRef for SelectExpressionInner {
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
pub enum SelectExpression {
    Itself(SelectExpressionInner),
}
pub enum SelectExpressionRefMut<'a> {
    Itself(&'a mut SelectExpressionInner),
}
pub enum SelectExpressionRef<'a> {
    Itself(&'a SelectExpressionInner),
}
impl SelectExpression {
    pub fn as_ref(&self) -> SelectExpressionRef {
        match self {
            SelectExpression::Itself(inner) => SelectExpressionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> SelectExpressionRefMut {
        match self {
            SelectExpression::Itself(inner) => SelectExpressionRefMut::Itself(inner),
        }
    }
}
impl<'a> SelectExpressionRefMut<'a> {
    pub fn as_ref(self) -> SelectExpressionRef<'a> {
        match self {
            SelectExpressionRefMut::Itself(inner) => SelectExpressionRef::Itself(inner),
        }
    }
}
impl SelectExpressionStruct for SelectExpression {}
impl SelectExpressionStructRefMut for SelectExpression {}
impl SelectExpressionStructRef for SelectExpression {}
impl<'a> SelectExpressionStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> SelectExpressionStructRef for SelectExpressionRefMut<'a> {}
impl<'a> SelectExpressionStructRef for SelectExpressionRef<'a> {}
impl OperatorExpressionStruct for SelectExpression {
    fn operator(self) -> String {
        match self {
            SelectExpression::Itself(x) => x.operator(),
        }
    }
}
impl OperatorExpressionStructRefMut for SelectExpression {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            SelectExpression::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl OperatorExpressionStructRef for SelectExpression {
    fn operator_ref(&self) -> &String {
        match self {
            SelectExpression::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRefMut for SelectExpressionRefMut<'a> {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            SelectExpressionRefMut::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for SelectExpressionRefMut<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            SelectExpressionRefMut::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for SelectExpressionRef<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            SelectExpressionRef::Itself(x) => x.operator_ref(),
        }
    }
}
impl InvocationExpressionStruct for SelectExpression {}
impl InvocationExpressionStructRefMut for SelectExpression {}
impl InvocationExpressionStructRef for SelectExpression {}
impl<'a> InvocationExpressionStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for SelectExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for SelectExpressionRef<'a> {}
impl InstantiationExpressionStruct for SelectExpression {}
impl InstantiationExpressionStructRefMut for SelectExpression {}
impl InstantiationExpressionStructRef for SelectExpression {}
impl<'a> InstantiationExpressionStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for SelectExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for SelectExpressionRef<'a> {}
impl ExpressionStruct for SelectExpression {}
impl ExpressionStructRefMut for SelectExpression {}
impl ExpressionStructRef for SelectExpression {}
impl<'a> ExpressionStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for SelectExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for SelectExpressionRef<'a> {}
impl StepStruct for SelectExpression {}
impl StepStructRefMut for SelectExpression {}
impl StepStructRef for SelectExpression {}
impl<'a> StepStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> StepStructRef for SelectExpressionRefMut<'a> {}
impl<'a> StepStructRef for SelectExpressionRef<'a> {}
impl FeatureStruct for SelectExpression {
    fn is_unique(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_constant(),
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
            SelectExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for SelectExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_constant_ref_mut(),
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
            SelectExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for SelectExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_constant_ref(),
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
            SelectExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for SelectExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            SelectExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for SelectExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            SelectExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for SelectExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_constant_ref(),
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
            SelectExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for SelectExpression {
    fn is_abstract(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for SelectExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for SelectExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for SelectExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for SelectExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for SelectExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for SelectExpression {}
impl NamespaceStructRefMut for SelectExpression {}
impl NamespaceStructRef for SelectExpression {}
impl<'a> NamespaceStructRefMut for SelectExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for SelectExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for SelectExpressionRef<'a> {}
impl ElementStruct for SelectExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            SelectExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            SelectExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            SelectExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for SelectExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SelectExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SelectExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SelectExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for SelectExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SelectExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SelectExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SelectExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SelectExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SelectExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SelectExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SelectExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SelectExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SelectExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SelectExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SelectExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SelectExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SelectExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SelectExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SelectExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl SelectExpressionUpcast for SelectExpression {
    fn into_select_expression(self) -> SelectExpression {
        self
    }
}
impl<'a> SelectExpressionUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_select_expression_ref_mut(self) -> SelectExpressionRefMut<'a> {
        self
    }
}
impl<'a> SelectExpressionUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_select_expression_ref(self) -> SelectExpressionRef<'a> {
        self
    }
}
impl OperatorExpressionUpcast for SelectExpression {
    fn into_operator_expression(self) -> OperatorExpression {
        OperatorExpression::SelectExpression(self).into_operator_expression()
    }
}
impl<'a> OperatorExpressionUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_operator_expression_ref()
    }
}
impl InvocationExpressionUpcast for SelectExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        OperatorExpression::SelectExpression(self).into_invocation_expression()
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self)
            .as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_invocation_expression_ref()
    }
}
impl InstantiationExpressionUpcast for SelectExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        OperatorExpression::SelectExpression(self).into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for SelectExpression {
    fn into_expression(self) -> Expression {
        OperatorExpression::SelectExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_expression_ref()
    }
}
impl StepUpcast for SelectExpression {
    fn into_step(self) -> Step {
        OperatorExpression::SelectExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for SelectExpression {
    fn into_feature(self) -> Feature {
        OperatorExpression::SelectExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for SelectExpression {
    fn into_type_(self) -> Type {
        OperatorExpression::SelectExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for SelectExpression {
    fn into_namespace(self) -> Namespace {
        OperatorExpression::SelectExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for SelectExpression {
    fn into_element(self) -> Element {
        OperatorExpression::SelectExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SelectExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OperatorExpressionRefMut::SelectExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SelectExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OperatorExpressionRef::SelectExpression(self).as_element_ref()
    }
}
pub trait SelectExpressionDowncast {}
pub trait SelectExpressionDowncastRefMut<'a> {}
pub trait SelectExpressionDowncastRef<'a> {}
impl SelectExpressionDowncast for SelectExpression {}
impl<'a> SelectExpressionDowncastRefMut<'a> for SelectExpressionRefMut<'a> {}
impl<'a> SelectExpressionDowncastRef<'a> for SelectExpressionRef<'a> {}
pub trait SelectExpressionMethodsDescendants
where
    Self: DescendantOf<SelectExpression>,
    Self::Via: SelectExpressionMethods,
    Self: Sized,
{}
pub trait SelectExpressionMethods: Sized {}
impl<T: SelectExpressionMethodsDescendants> SelectExpressionMethods for T
where
    T::Via: SelectExpressionMethods,
{}
impl DescendantOf<OperatorExpression> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl OperatorExpressionMethodsDescendants for SelectExpression {}
impl DescendantOf<InvocationExpression> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InvocationExpressionMethodsDescendants for SelectExpression {}
impl DescendantOf<InstantiationExpression> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for SelectExpression {}
impl DescendantOf<Expression> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ExpressionMethodsDescendants for SelectExpression {}
impl DescendantOf<Step> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl StepMethodsDescendants for SelectExpression {}
impl DescendantOf<Feature> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl FeatureMethodsDescendants for SelectExpression {}
impl DescendantOf<Type> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl TypeMethodsDescendants for SelectExpression {}
impl DescendantOf<Namespace> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl NamespaceMethodsDescendants for SelectExpression {}
impl DescendantOf<Element> for SelectExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ElementMethodsDescendants for SelectExpression {}
pub trait SelectExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SelectExpressionRefMut<'a>>,
    Self::Via: SelectExpressionRefMutMethods,
    Self: Sized,
{}
pub trait SelectExpressionRefMutMethods: Sized {}
impl<'a, T: SelectExpressionRefMutMethodsDescendants<'a>> SelectExpressionRefMutMethods
for T
where
    T::Via: SelectExpressionRefMutMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<InvocationExpressionRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionRefMutMethodsDescendants<'a>
for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SelectExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SelectExpressionRefMut<'a> {}
pub trait SelectExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<SelectExpressionRef<'a>>,
    Self::Via: SelectExpressionRefMethods,
    Self: Sized,
{}
pub trait SelectExpressionRefMethods: Sized {}
impl<'a, T: SelectExpressionRefMethodsDescendants<'a>> SelectExpressionRefMethods for T
where
    T::Via: SelectExpressionRefMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> OperatorExpressionRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<InvocationExpressionRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InvocationExpressionRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SelectExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SelectExpressionRef<'a> {}

