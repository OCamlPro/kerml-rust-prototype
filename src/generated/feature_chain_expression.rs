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
pub struct FeatureChainExpressionInner {
    pub(super) sup_operator_expression: OperatorExpressionInner,
}
pub trait FeatureChainExpressionStruct
where
    Self: FeatureChainExpressionStructRefMut,
    Self: FeatureChainExpressionStructRef,
    Self: OperatorExpressionStruct,
{}
pub trait FeatureChainExpressionStructRefMut
where
    Self: FeatureChainExpressionStructRef,
    Self: OperatorExpressionStructRefMut,
{}
pub trait FeatureChainExpressionStructRef
where
    Self: OperatorExpressionStructRef,
{}
pub trait FeatureChainExpressionUpcast: FeatureChainExpressionStruct {
    fn into_feature_chain_expression(self) -> FeatureChainExpression;
}
pub trait FeatureChainExpressionUpcastRefMut<'a>: FeatureChainExpressionStructRefMut {
    fn as_feature_chain_expression_ref_mut(self) -> FeatureChainExpressionRefMut<'a>;
}
pub trait FeatureChainExpressionUpcastRef<'a>: FeatureChainExpressionStructRef {
    fn as_feature_chain_expression_ref(self) -> FeatureChainExpressionRef<'a>;
}
impl FeatureChainExpressionStruct for FeatureChainExpressionInner {}
impl FeatureChainExpressionStructRefMut for FeatureChainExpressionInner {}
impl FeatureChainExpressionStructRef for FeatureChainExpressionInner {}
impl OperatorExpressionStruct for FeatureChainExpressionInner {
    fn operator(self) -> String {
        self.sup_operator_expression.operator()
    }
}
impl OperatorExpressionStructRefMut for FeatureChainExpressionInner {
    fn operator_ref_mut(&mut self) -> &mut String {
        self.sup_operator_expression.operator_ref_mut()
    }
}
impl OperatorExpressionStructRef for FeatureChainExpressionInner {
    fn operator_ref(&self) -> &String {
        self.sup_operator_expression.operator_ref()
    }
}
impl InvocationExpressionStruct for FeatureChainExpressionInner {}
impl InvocationExpressionStructRefMut for FeatureChainExpressionInner {}
impl InvocationExpressionStructRef for FeatureChainExpressionInner {}
impl InstantiationExpressionStruct for FeatureChainExpressionInner {}
impl InstantiationExpressionStructRefMut for FeatureChainExpressionInner {}
impl InstantiationExpressionStructRef for FeatureChainExpressionInner {}
impl ExpressionStruct for FeatureChainExpressionInner {}
impl ExpressionStructRefMut for FeatureChainExpressionInner {}
impl ExpressionStructRef for FeatureChainExpressionInner {}
impl StepStruct for FeatureChainExpressionInner {}
impl StepStructRefMut for FeatureChainExpressionInner {}
impl StepStructRef for FeatureChainExpressionInner {}
impl FeatureStruct for FeatureChainExpressionInner {
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
impl FeatureStructRefMut for FeatureChainExpressionInner {
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
impl FeatureStructRef for FeatureChainExpressionInner {
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
impl TypeStruct for FeatureChainExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_operator_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_operator_expression.is_sufficient()
    }
}
impl TypeStructRefMut for FeatureChainExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FeatureChainExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_operator_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_operator_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for FeatureChainExpressionInner {}
impl NamespaceStructRefMut for FeatureChainExpressionInner {}
impl NamespaceStructRef for FeatureChainExpressionInner {}
impl ElementStruct for FeatureChainExpressionInner {
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
impl ElementStructRefMut for FeatureChainExpressionInner {
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
impl ElementStructRef for FeatureChainExpressionInner {
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
pub enum FeatureChainExpression {
    Itself(FeatureChainExpressionInner),
}
pub enum FeatureChainExpressionRefMut<'a> {
    Itself(&'a mut FeatureChainExpressionInner),
}
pub enum FeatureChainExpressionRef<'a> {
    Itself(&'a FeatureChainExpressionInner),
}
impl FeatureChainExpression {
    pub fn as_ref(&self) -> FeatureChainExpressionRef {
        match self {
            FeatureChainExpression::Itself(inner) => {
                FeatureChainExpressionRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureChainExpressionRefMut {
        match self {
            FeatureChainExpression::Itself(inner) => {
                FeatureChainExpressionRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> FeatureChainExpressionRefMut<'a> {
    pub fn as_ref(self) -> FeatureChainExpressionRef<'a> {
        match self {
            FeatureChainExpressionRefMut::Itself(inner) => {
                FeatureChainExpressionRef::Itself(inner)
            }
        }
    }
}
impl FeatureChainExpressionStruct for FeatureChainExpression {}
impl FeatureChainExpressionStructRefMut for FeatureChainExpression {}
impl FeatureChainExpressionStructRef for FeatureChainExpression {}
impl<'a> FeatureChainExpressionStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> FeatureChainExpressionStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> FeatureChainExpressionStructRef for FeatureChainExpressionRef<'a> {}
impl OperatorExpressionStruct for FeatureChainExpression {
    fn operator(self) -> String {
        match self {
            FeatureChainExpression::Itself(x) => x.operator(),
        }
    }
}
impl OperatorExpressionStructRefMut for FeatureChainExpression {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChainExpression::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl OperatorExpressionStructRef for FeatureChainExpression {
    fn operator_ref(&self) -> &String {
        match self {
            FeatureChainExpression::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRefMut for FeatureChainExpressionRefMut<'a> {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for FeatureChainExpressionRefMut<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for FeatureChainExpressionRef<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.operator_ref(),
        }
    }
}
impl InvocationExpressionStruct for FeatureChainExpression {}
impl InvocationExpressionStructRefMut for FeatureChainExpression {}
impl InvocationExpressionStructRef for FeatureChainExpression {}
impl<'a> InvocationExpressionStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for FeatureChainExpressionRef<'a> {}
impl InstantiationExpressionStruct for FeatureChainExpression {}
impl InstantiationExpressionStructRefMut for FeatureChainExpression {}
impl InstantiationExpressionStructRef for FeatureChainExpression {}
impl<'a> InstantiationExpressionStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for FeatureChainExpressionRef<'a> {}
impl ExpressionStruct for FeatureChainExpression {}
impl ExpressionStructRefMut for FeatureChainExpression {}
impl ExpressionStructRef for FeatureChainExpression {}
impl<'a> ExpressionStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for FeatureChainExpressionRef<'a> {}
impl StepStruct for FeatureChainExpression {}
impl StepStructRefMut for FeatureChainExpression {}
impl StepStructRef for FeatureChainExpression {}
impl<'a> StepStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> StepStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> StepStructRef for FeatureChainExpressionRef<'a> {}
impl FeatureStruct for FeatureChainExpression {
    fn is_unique(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_constant(),
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
            FeatureChainExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for FeatureChainExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_constant_ref_mut(),
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
            FeatureChainExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for FeatureChainExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_constant_ref(),
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
            FeatureChainExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for FeatureChainExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            FeatureChainExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureChainExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            FeatureChainExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureChainExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_constant_ref(),
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
            FeatureChainExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for FeatureChainExpression {
    fn is_abstract(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for FeatureChainExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for FeatureChainExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FeatureChainExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FeatureChainExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FeatureChainExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for FeatureChainExpression {}
impl NamespaceStructRefMut for FeatureChainExpression {}
impl NamespaceStructRef for FeatureChainExpression {}
impl<'a> NamespaceStructRefMut for FeatureChainExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureChainExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureChainExpressionRef<'a> {}
impl ElementStruct for FeatureChainExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureChainExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureChainExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureChainExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChainExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureChainExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChainExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChainExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureChainExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureChainExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureChainExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureChainExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureChainExpressionUpcast for FeatureChainExpression {
    fn into_feature_chain_expression(self) -> FeatureChainExpression {
        self
    }
}
impl<'a> FeatureChainExpressionUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_feature_chain_expression_ref_mut(self) -> FeatureChainExpressionRefMut<'a> {
        self
    }
}
impl<'a> FeatureChainExpressionUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_feature_chain_expression_ref(self) -> FeatureChainExpressionRef<'a> {
        self
    }
}
impl OperatorExpressionUpcast for FeatureChainExpression {
    fn into_operator_expression(self) -> OperatorExpression {
        OperatorExpression::FeatureChainExpression(self).into_operator_expression()
    }
}
impl<'a> OperatorExpressionUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self)
            .as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_operator_expression_ref()
    }
}
impl InvocationExpressionUpcast for FeatureChainExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        OperatorExpression::FeatureChainExpression(self).into_invocation_expression()
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self)
            .as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self)
            .as_invocation_expression_ref()
    }
}
impl InstantiationExpressionUpcast for FeatureChainExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        OperatorExpression::FeatureChainExpression(self).into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self)
            .as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for FeatureChainExpression {
    fn into_expression(self) -> Expression {
        OperatorExpression::FeatureChainExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_expression_ref()
    }
}
impl StepUpcast for FeatureChainExpression {
    fn into_step(self) -> Step {
        OperatorExpression::FeatureChainExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for FeatureChainExpression {
    fn into_feature(self) -> Feature {
        OperatorExpression::FeatureChainExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for FeatureChainExpression {
    fn into_type_(self) -> Type {
        OperatorExpression::FeatureChainExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for FeatureChainExpression {
    fn into_namespace(self) -> Namespace {
        OperatorExpression::FeatureChainExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for FeatureChainExpression {
    fn into_element(self) -> Element {
        OperatorExpression::FeatureChainExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureChainExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OperatorExpressionRefMut::FeatureChainExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureChainExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OperatorExpressionRef::FeatureChainExpression(self).as_element_ref()
    }
}
pub trait FeatureChainExpressionDowncast {}
pub trait FeatureChainExpressionDowncastRefMut<'a> {}
pub trait FeatureChainExpressionDowncastRef<'a> {}
impl FeatureChainExpressionDowncast for FeatureChainExpression {}
impl<'a> FeatureChainExpressionDowncastRefMut<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> FeatureChainExpressionDowncastRef<'a> for FeatureChainExpressionRef<'a> {}
pub trait FeatureChainExpressionMethodsDescendants
where
    Self: DescendantOf<FeatureChainExpression>,
    Self::Via: FeatureChainExpressionMethods,
    Self: Sized,
{}
pub trait FeatureChainExpressionMethods: Sized {}
impl<T: FeatureChainExpressionMethodsDescendants> FeatureChainExpressionMethods for T
where
    T::Via: FeatureChainExpressionMethods,
{}
impl DescendantOf<OperatorExpression> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl OperatorExpressionMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<InvocationExpression> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InvocationExpressionMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<InstantiationExpression> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Expression> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ExpressionMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Step> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl StepMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Feature> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl FeatureMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Type> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl TypeMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Namespace> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl NamespaceMethodsDescendants for FeatureChainExpression {}
impl DescendantOf<Element> for FeatureChainExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ElementMethodsDescendants for FeatureChainExpression {}
pub trait FeatureChainExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureChainExpressionRefMut<'a>>,
    Self::Via: FeatureChainExpressionRefMutMethods,
    Self: Sized,
{}
pub trait FeatureChainExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: FeatureChainExpressionRefMutMethodsDescendants<'a>,
> FeatureChainExpressionRefMutMethods for T
where
    T::Via: FeatureChainExpressionRefMutMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRefMut<'a>>
for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionRefMutMethodsDescendants<'a>
for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<InvocationExpressionRefMut<'a>>
for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionRefMutMethodsDescendants<'a>
for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>>
for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureChainExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureChainExpressionRefMut<'a> {}
pub trait FeatureChainExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureChainExpressionRef<'a>>,
    Self::Via: FeatureChainExpressionRefMethods,
    Self: Sized,
{
    fn source_target_feature_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().source_target_feature()
    }
}
pub trait FeatureChainExpressionRefMethods: Sized {
    fn source_target_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
}
impl<
    'a,
    T: FeatureChainExpressionRefMethodsDescendants<'a>,
> FeatureChainExpressionRefMethods for T
where
    T::Via: FeatureChainExpressionRefMethods,
{
    fn source_target_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        FeatureChainExpressionRefMethodsDescendants::source_target_feature_impl(self)
    }
}
impl<'a> DescendantOf<OperatorExpressionRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> OperatorExpressionRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<InvocationExpressionRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InvocationExpressionRefMethodsDescendants<'a>
for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a>
for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureChainExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureChainExpressionRef<'a> {}

