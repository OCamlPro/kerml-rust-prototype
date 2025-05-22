#![allow(unused)]
use super::utils::DescendantOf;
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
use super::select_expression::{
    SelectExpression, SelectExpressionRefMut, SelectExpressionRef,
};
use super::collect_expression::{
    CollectExpression, CollectExpressionRefMut, CollectExpressionRef,
};
use super::feature_chain_expression::{
    FeatureChainExpression, FeatureChainExpressionRefMut, FeatureChainExpressionRef,
};
use super::index_expression::{
    IndexExpression, IndexExpressionRefMut, IndexExpressionRef,
};
pub struct OperatorExpressionInner {
    pub(super) sup_invocation_expression: InvocationExpressionInner,
    pub(super) operator: String,
}
pub trait OperatorExpressionStruct
where
    Self: OperatorExpressionStructRefMut,
    Self: OperatorExpressionStructRef,
    Self: InvocationExpressionStruct,
{
    fn operator(self) -> String;
}
pub trait OperatorExpressionStructRefMut
where
    Self: OperatorExpressionStructRef,
    Self: InvocationExpressionStructRefMut,
{
    fn operator_ref_mut(&mut self) -> &mut String;
}
pub trait OperatorExpressionStructRef
where
    Self: InvocationExpressionStructRef,
{
    fn operator_ref(&self) -> &String;
}
pub trait OperatorExpressionUpcast: OperatorExpressionStruct {
    fn into_operator_expression(self) -> OperatorExpression;
}
pub trait OperatorExpressionUpcastRefMut<'a>: OperatorExpressionStructRefMut {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a>;
}
pub trait OperatorExpressionUpcastRef<'a>: OperatorExpressionStructRef {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a>;
}
impl OperatorExpressionStruct for OperatorExpressionInner {
    fn operator(self) -> String {
        self.operator
    }
}
impl OperatorExpressionStructRefMut for OperatorExpressionInner {
    fn operator_ref_mut(&mut self) -> &mut String {
        &mut self.operator
    }
}
impl OperatorExpressionStructRef for OperatorExpressionInner {
    fn operator_ref(&self) -> &String {
        &self.operator
    }
}
impl InvocationExpressionStruct for OperatorExpressionInner {}
impl InvocationExpressionStructRefMut for OperatorExpressionInner {}
impl InvocationExpressionStructRef for OperatorExpressionInner {}
impl InstantiationExpressionStruct for OperatorExpressionInner {}
impl InstantiationExpressionStructRefMut for OperatorExpressionInner {}
impl InstantiationExpressionStructRef for OperatorExpressionInner {}
impl ExpressionStruct for OperatorExpressionInner {}
impl ExpressionStructRefMut for OperatorExpressionInner {}
impl ExpressionStructRef for OperatorExpressionInner {}
impl StepStruct for OperatorExpressionInner {}
impl StepStructRefMut for OperatorExpressionInner {}
impl StepStructRef for OperatorExpressionInner {}
impl FeatureStruct for OperatorExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_invocation_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_invocation_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_invocation_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_invocation_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_invocation_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_invocation_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_invocation_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_invocation_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_invocation_expression.direction()
    }
}
impl FeatureStructRefMut for OperatorExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_invocation_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for OperatorExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_invocation_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_invocation_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_invocation_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_invocation_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_invocation_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_invocation_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_invocation_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_invocation_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_invocation_expression.direction_ref()
    }
}
impl TypeStruct for OperatorExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_invocation_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_invocation_expression.is_sufficient()
    }
}
impl TypeStructRefMut for OperatorExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for OperatorExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_invocation_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_invocation_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for OperatorExpressionInner {}
impl NamespaceStructRefMut for OperatorExpressionInner {}
impl NamespaceStructRef for OperatorExpressionInner {}
impl ElementStruct for OperatorExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_invocation_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_invocation_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_invocation_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_invocation_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_invocation_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_invocation_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_invocation_expression.is_implied_included()
    }
}
impl ElementStructRefMut for OperatorExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_invocation_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_invocation_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_invocation_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_invocation_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_invocation_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_invocation_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_invocation_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for OperatorExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_invocation_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_invocation_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_invocation_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_invocation_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_invocation_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_invocation_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_invocation_expression.is_implied_included_ref()
    }
}
pub enum OperatorExpression {
    Itself(OperatorExpressionInner),
    SelectExpression(SelectExpression),
    CollectExpression(CollectExpression),
    FeatureChainExpression(FeatureChainExpression),
    IndexExpression(IndexExpression),
}
pub enum OperatorExpressionRefMut<'a> {
    Itself(&'a mut OperatorExpressionInner),
    SelectExpression(SelectExpressionRefMut<'a>),
    CollectExpression(CollectExpressionRefMut<'a>),
    FeatureChainExpression(FeatureChainExpressionRefMut<'a>),
    IndexExpression(IndexExpressionRefMut<'a>),
}
pub enum OperatorExpressionRef<'a> {
    Itself(&'a OperatorExpressionInner),
    SelectExpression(SelectExpressionRef<'a>),
    CollectExpression(CollectExpressionRef<'a>),
    FeatureChainExpression(FeatureChainExpressionRef<'a>),
    IndexExpression(IndexExpressionRef<'a>),
}
impl OperatorExpression {
    pub fn as_ref(&self) -> OperatorExpressionRef {
        match self {
            OperatorExpression::Itself(inner) => OperatorExpressionRef::Itself(&inner),
            OperatorExpression::SelectExpression(inner) => {
                OperatorExpressionRef::SelectExpression(inner.as_ref())
            }
            OperatorExpression::CollectExpression(inner) => {
                OperatorExpressionRef::CollectExpression(inner.as_ref())
            }
            OperatorExpression::FeatureChainExpression(inner) => {
                OperatorExpressionRef::FeatureChainExpression(inner.as_ref())
            }
            OperatorExpression::IndexExpression(inner) => {
                OperatorExpressionRef::IndexExpression(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> OperatorExpressionRefMut {
        match self {
            OperatorExpression::Itself(inner) => OperatorExpressionRefMut::Itself(inner),
            OperatorExpression::SelectExpression(inner) => {
                OperatorExpressionRefMut::SelectExpression(inner.as_ref_mut())
            }
            OperatorExpression::CollectExpression(inner) => {
                OperatorExpressionRefMut::CollectExpression(inner.as_ref_mut())
            }
            OperatorExpression::FeatureChainExpression(inner) => {
                OperatorExpressionRefMut::FeatureChainExpression(inner.as_ref_mut())
            }
            OperatorExpression::IndexExpression(inner) => {
                OperatorExpressionRefMut::IndexExpression(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> OperatorExpressionRefMut<'a> {
    pub fn as_ref(self) -> OperatorExpressionRef<'a> {
        match self {
            OperatorExpressionRefMut::Itself(inner) => {
                OperatorExpressionRef::Itself(inner)
            }
            OperatorExpressionRefMut::SelectExpression(inner) => {
                OperatorExpressionRef::SelectExpression(inner.as_ref())
            }
            OperatorExpressionRefMut::CollectExpression(inner) => {
                OperatorExpressionRef::CollectExpression(inner.as_ref())
            }
            OperatorExpressionRefMut::FeatureChainExpression(inner) => {
                OperatorExpressionRef::FeatureChainExpression(inner.as_ref())
            }
            OperatorExpressionRefMut::IndexExpression(inner) => {
                OperatorExpressionRef::IndexExpression(inner.as_ref())
            }
        }
    }
}
impl OperatorExpressionStruct for OperatorExpression {
    fn operator(self) -> String {
        match self {
            OperatorExpression::Itself(x) => x.operator(),
            OperatorExpression::SelectExpression(x) => x.operator(),
            OperatorExpression::CollectExpression(x) => x.operator(),
            OperatorExpression::FeatureChainExpression(x) => x.operator(),
            OperatorExpression::IndexExpression(x) => x.operator(),
        }
    }
}
impl OperatorExpressionStructRefMut for OperatorExpression {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            OperatorExpression::Itself(x) => x.operator_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.operator_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.operator_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.operator_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.operator_ref_mut(),
        }
    }
}
impl OperatorExpressionStructRef for OperatorExpression {
    fn operator_ref(&self) -> &String {
        match self {
            OperatorExpression::Itself(x) => x.operator_ref(),
            OperatorExpression::SelectExpression(x) => x.operator_ref(),
            OperatorExpression::CollectExpression(x) => x.operator_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.operator_ref(),
            OperatorExpression::IndexExpression(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRefMut for OperatorExpressionRefMut<'a> {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.operator_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.operator_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.operator_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.operator_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.operator_ref_mut(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for OperatorExpressionRefMut<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.operator_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.operator_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.operator_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.operator_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for OperatorExpressionRef<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            OperatorExpressionRef::Itself(x) => x.operator_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.operator_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.operator_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.operator_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.operator_ref(),
        }
    }
}
impl InvocationExpressionStruct for OperatorExpression {}
impl InvocationExpressionStructRefMut for OperatorExpression {}
impl InvocationExpressionStructRef for OperatorExpression {}
impl<'a> InvocationExpressionStructRefMut for OperatorExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for OperatorExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for OperatorExpressionRef<'a> {}
impl InstantiationExpressionStruct for OperatorExpression {}
impl InstantiationExpressionStructRefMut for OperatorExpression {}
impl InstantiationExpressionStructRef for OperatorExpression {}
impl<'a> InstantiationExpressionStructRefMut for OperatorExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for OperatorExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for OperatorExpressionRef<'a> {}
impl ExpressionStruct for OperatorExpression {}
impl ExpressionStructRefMut for OperatorExpression {}
impl ExpressionStructRef for OperatorExpression {}
impl<'a> ExpressionStructRefMut for OperatorExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for OperatorExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for OperatorExpressionRef<'a> {}
impl StepStruct for OperatorExpression {}
impl StepStructRefMut for OperatorExpression {}
impl StepStructRef for OperatorExpression {}
impl<'a> StepStructRefMut for OperatorExpressionRefMut<'a> {}
impl<'a> StepStructRef for OperatorExpressionRefMut<'a> {}
impl<'a> StepStructRef for OperatorExpressionRef<'a> {}
impl FeatureStruct for OperatorExpression {
    fn is_unique(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_unique(),
            OperatorExpression::SelectExpression(x) => x.is_unique(),
            OperatorExpression::CollectExpression(x) => x.is_unique(),
            OperatorExpression::FeatureChainExpression(x) => x.is_unique(),
            OperatorExpression::IndexExpression(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_ordered(),
            OperatorExpression::SelectExpression(x) => x.is_ordered(),
            OperatorExpression::CollectExpression(x) => x.is_ordered(),
            OperatorExpression::FeatureChainExpression(x) => x.is_ordered(),
            OperatorExpression::IndexExpression(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_composite(),
            OperatorExpression::SelectExpression(x) => x.is_composite(),
            OperatorExpression::CollectExpression(x) => x.is_composite(),
            OperatorExpression::FeatureChainExpression(x) => x.is_composite(),
            OperatorExpression::IndexExpression(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_end(),
            OperatorExpression::SelectExpression(x) => x.is_end(),
            OperatorExpression::CollectExpression(x) => x.is_end(),
            OperatorExpression::FeatureChainExpression(x) => x.is_end(),
            OperatorExpression::IndexExpression(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_derived(),
            OperatorExpression::SelectExpression(x) => x.is_derived(),
            OperatorExpression::CollectExpression(x) => x.is_derived(),
            OperatorExpression::FeatureChainExpression(x) => x.is_derived(),
            OperatorExpression::IndexExpression(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_portion(),
            OperatorExpression::SelectExpression(x) => x.is_portion(),
            OperatorExpression::CollectExpression(x) => x.is_portion(),
            OperatorExpression::FeatureChainExpression(x) => x.is_portion(),
            OperatorExpression::IndexExpression(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_variable(),
            OperatorExpression::SelectExpression(x) => x.is_variable(),
            OperatorExpression::CollectExpression(x) => x.is_variable(),
            OperatorExpression::FeatureChainExpression(x) => x.is_variable(),
            OperatorExpression::IndexExpression(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_constant(),
            OperatorExpression::SelectExpression(x) => x.is_constant(),
            OperatorExpression::CollectExpression(x) => x.is_constant(),
            OperatorExpression::FeatureChainExpression(x) => x.is_constant(),
            OperatorExpression::IndexExpression(x) => x.is_constant(),
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
            OperatorExpression::Itself(x) => x.direction(),
            OperatorExpression::SelectExpression(x) => x.direction(),
            OperatorExpression::CollectExpression(x) => x.direction(),
            OperatorExpression::FeatureChainExpression(x) => x.direction(),
            OperatorExpression::IndexExpression(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for OperatorExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_unique_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_unique_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_unique_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_unique_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_ordered_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_composite_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_composite_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_composite_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_composite_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_end_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_end_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_end_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_end_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_derived_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_derived_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_derived_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_derived_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_portion_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_portion_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_portion_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_portion_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_variable_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_variable_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_variable_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_variable_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_constant_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_constant_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_constant_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_constant_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_constant_ref_mut(),
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
            OperatorExpression::Itself(x) => x.direction_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.direction_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.direction_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.direction_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for OperatorExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_unique_ref(),
            OperatorExpression::SelectExpression(x) => x.is_unique_ref(),
            OperatorExpression::CollectExpression(x) => x.is_unique_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_unique_ref(),
            OperatorExpression::IndexExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_ordered_ref(),
            OperatorExpression::SelectExpression(x) => x.is_ordered_ref(),
            OperatorExpression::CollectExpression(x) => x.is_ordered_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_ordered_ref(),
            OperatorExpression::IndexExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_composite_ref(),
            OperatorExpression::SelectExpression(x) => x.is_composite_ref(),
            OperatorExpression::CollectExpression(x) => x.is_composite_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_composite_ref(),
            OperatorExpression::IndexExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_end_ref(),
            OperatorExpression::SelectExpression(x) => x.is_end_ref(),
            OperatorExpression::CollectExpression(x) => x.is_end_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_end_ref(),
            OperatorExpression::IndexExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_derived_ref(),
            OperatorExpression::SelectExpression(x) => x.is_derived_ref(),
            OperatorExpression::CollectExpression(x) => x.is_derived_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_derived_ref(),
            OperatorExpression::IndexExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_portion_ref(),
            OperatorExpression::SelectExpression(x) => x.is_portion_ref(),
            OperatorExpression::CollectExpression(x) => x.is_portion_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_portion_ref(),
            OperatorExpression::IndexExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_variable_ref(),
            OperatorExpression::SelectExpression(x) => x.is_variable_ref(),
            OperatorExpression::CollectExpression(x) => x.is_variable_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_variable_ref(),
            OperatorExpression::IndexExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_constant_ref(),
            OperatorExpression::SelectExpression(x) => x.is_constant_ref(),
            OperatorExpression::CollectExpression(x) => x.is_constant_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_constant_ref(),
            OperatorExpression::IndexExpression(x) => x.is_constant_ref(),
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
            OperatorExpression::Itself(x) => x.direction_ref(),
            OperatorExpression::SelectExpression(x) => x.direction_ref(),
            OperatorExpression::CollectExpression(x) => x.direction_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.direction_ref(),
            OperatorExpression::IndexExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for OperatorExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_unique_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_unique_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_unique_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_ordered_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_composite_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_composite_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_composite_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_end_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_end_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_end_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_derived_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_derived_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_derived_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_portion_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_portion_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_portion_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_variable_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_variable_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_variable_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_constant_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_constant_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_constant_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_constant_ref_mut(),
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
            OperatorExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.direction_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.direction_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.direction_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for OperatorExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_unique_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_unique_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_unique_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_unique_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_composite_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_composite_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_composite_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_composite_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_end_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_end_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_end_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_end_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_derived_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_derived_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_derived_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_derived_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_portion_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_portion_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_portion_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_portion_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_variable_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_variable_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_variable_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_variable_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_constant_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_constant_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_constant_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_constant_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_constant_ref(),
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
            OperatorExpressionRefMut::Itself(x) => x.direction_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.direction_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.direction_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.direction_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for OperatorExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_unique_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_unique_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_unique_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_unique_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_ordered_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_ordered_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_composite_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_composite_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_composite_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_composite_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_end_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_end_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_end_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_end_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_derived_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_derived_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_derived_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_derived_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_portion_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_portion_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_portion_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_portion_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_variable_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_variable_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_variable_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_variable_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_constant_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_constant_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_constant_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_constant_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_constant_ref(),
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
            OperatorExpressionRef::Itself(x) => x.direction_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.direction_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.direction_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.direction_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for OperatorExpression {
    fn is_abstract(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_abstract(),
            OperatorExpression::SelectExpression(x) => x.is_abstract(),
            OperatorExpression::CollectExpression(x) => x.is_abstract(),
            OperatorExpression::FeatureChainExpression(x) => x.is_abstract(),
            OperatorExpression::IndexExpression(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_sufficient(),
            OperatorExpression::SelectExpression(x) => x.is_sufficient(),
            OperatorExpression::CollectExpression(x) => x.is_sufficient(),
            OperatorExpression::FeatureChainExpression(x) => x.is_sufficient(),
            OperatorExpression::IndexExpression(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for OperatorExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_abstract_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_abstract_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_abstract_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_abstract_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_sufficient_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_sufficient_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_sufficient_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.is_sufficient_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for OperatorExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_abstract_ref(),
            OperatorExpression::SelectExpression(x) => x.is_abstract_ref(),
            OperatorExpression::CollectExpression(x) => x.is_abstract_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_abstract_ref(),
            OperatorExpression::IndexExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_sufficient_ref(),
            OperatorExpression::SelectExpression(x) => x.is_sufficient_ref(),
            OperatorExpression::CollectExpression(x) => x.is_sufficient_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_sufficient_ref(),
            OperatorExpression::IndexExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for OperatorExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_abstract_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_abstract_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_abstract_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_sufficient_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_sufficient_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_sufficient_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for OperatorExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for OperatorExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_abstract_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_abstract_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_sufficient_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.is_sufficient_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for OperatorExpression {}
impl NamespaceStructRefMut for OperatorExpression {}
impl NamespaceStructRef for OperatorExpression {}
impl<'a> NamespaceStructRefMut for OperatorExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for OperatorExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for OperatorExpressionRef<'a> {}
impl ElementStruct for OperatorExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpression::Itself(x) => x.owned_relationship(),
            OperatorExpression::SelectExpression(x) => x.owned_relationship(),
            OperatorExpression::CollectExpression(x) => x.owned_relationship(),
            OperatorExpression::FeatureChainExpression(x) => x.owned_relationship(),
            OperatorExpression::IndexExpression(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpression::Itself(x) => x.owning_relationship(),
            OperatorExpression::SelectExpression(x) => x.owning_relationship(),
            OperatorExpression::CollectExpression(x) => x.owning_relationship(),
            OperatorExpression::FeatureChainExpression(x) => x.owning_relationship(),
            OperatorExpression::IndexExpression(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            OperatorExpression::Itself(x) => x.element_id(),
            OperatorExpression::SelectExpression(x) => x.element_id(),
            OperatorExpression::CollectExpression(x) => x.element_id(),
            OperatorExpression::FeatureChainExpression(x) => x.element_id(),
            OperatorExpression::IndexExpression(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            OperatorExpression::Itself(x) => x.alias_ids(),
            OperatorExpression::SelectExpression(x) => x.alias_ids(),
            OperatorExpression::CollectExpression(x) => x.alias_ids(),
            OperatorExpression::FeatureChainExpression(x) => x.alias_ids(),
            OperatorExpression::IndexExpression(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_short_name(),
            OperatorExpression::SelectExpression(x) => x.declared_short_name(),
            OperatorExpression::CollectExpression(x) => x.declared_short_name(),
            OperatorExpression::FeatureChainExpression(x) => x.declared_short_name(),
            OperatorExpression::IndexExpression(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_name(),
            OperatorExpression::SelectExpression(x) => x.declared_name(),
            OperatorExpression::CollectExpression(x) => x.declared_name(),
            OperatorExpression::FeatureChainExpression(x) => x.declared_name(),
            OperatorExpression::IndexExpression(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            OperatorExpression::Itself(x) => x.is_implied_included(),
            OperatorExpression::SelectExpression(x) => x.is_implied_included(),
            OperatorExpression::CollectExpression(x) => x.is_implied_included(),
            OperatorExpression::FeatureChainExpression(x) => x.is_implied_included(),
            OperatorExpression::IndexExpression(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for OperatorExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpression::Itself(x) => x.owned_relationship_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.owned_relationship_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.owned_relationship_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            OperatorExpression::IndexExpression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            OperatorExpression::Itself(x) => x.owning_relationship_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.owning_relationship_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.owning_relationship_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            OperatorExpression::IndexExpression(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            OperatorExpression::Itself(x) => x.element_id_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.element_id_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.element_id_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.element_id_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            OperatorExpression::Itself(x) => x.alias_ids_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_short_name_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.declared_short_name_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.declared_short_name_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            OperatorExpression::IndexExpression(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_name_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.declared_name_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.declared_name_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => x.declared_name_ref_mut(),
            OperatorExpression::IndexExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpression::Itself(x) => x.is_implied_included_ref_mut(),
            OperatorExpression::SelectExpression(x) => x.is_implied_included_ref_mut(),
            OperatorExpression::CollectExpression(x) => x.is_implied_included_ref_mut(),
            OperatorExpression::FeatureChainExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            OperatorExpression::IndexExpression(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for OperatorExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpression::Itself(x) => x.owned_relationship_ref(),
            OperatorExpression::SelectExpression(x) => x.owned_relationship_ref(),
            OperatorExpression::CollectExpression(x) => x.owned_relationship_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.owned_relationship_ref(),
            OperatorExpression::IndexExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpression::Itself(x) => x.owning_relationship_ref(),
            OperatorExpression::SelectExpression(x) => x.owning_relationship_ref(),
            OperatorExpression::CollectExpression(x) => x.owning_relationship_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.owning_relationship_ref(),
            OperatorExpression::IndexExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OperatorExpression::Itself(x) => x.element_id_ref(),
            OperatorExpression::SelectExpression(x) => x.element_id_ref(),
            OperatorExpression::CollectExpression(x) => x.element_id_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.element_id_ref(),
            OperatorExpression::IndexExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OperatorExpression::Itself(x) => x.alias_ids_ref(),
            OperatorExpression::SelectExpression(x) => x.alias_ids_ref(),
            OperatorExpression::CollectExpression(x) => x.alias_ids_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.alias_ids_ref(),
            OperatorExpression::IndexExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_short_name_ref(),
            OperatorExpression::SelectExpression(x) => x.declared_short_name_ref(),
            OperatorExpression::CollectExpression(x) => x.declared_short_name_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.declared_short_name_ref(),
            OperatorExpression::IndexExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpression::Itself(x) => x.declared_name_ref(),
            OperatorExpression::SelectExpression(x) => x.declared_name_ref(),
            OperatorExpression::CollectExpression(x) => x.declared_name_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.declared_name_ref(),
            OperatorExpression::IndexExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OperatorExpression::Itself(x) => x.is_implied_included_ref(),
            OperatorExpression::SelectExpression(x) => x.is_implied_included_ref(),
            OperatorExpression::CollectExpression(x) => x.is_implied_included_ref(),
            OperatorExpression::FeatureChainExpression(x) => x.is_implied_included_ref(),
            OperatorExpression::IndexExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for OperatorExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            OperatorExpressionRefMut::CollectExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => {
                x.owned_relationship_ref_mut()
            }
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            OperatorExpressionRefMut::CollectExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.element_id_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.element_id_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.element_id_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.alias_ids_ref_mut(),
            OperatorExpressionRefMut::IndexExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            OperatorExpressionRefMut::CollectExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => x.declared_name_ref_mut(),
            OperatorExpressionRefMut::CollectExpression(x) => x.declared_name_ref_mut(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.declared_name_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            OperatorExpressionRefMut::SelectExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            OperatorExpressionRefMut::CollectExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            OperatorExpressionRefMut::IndexExpression(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for OperatorExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.owned_relationship_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.owned_relationship_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.owned_relationship_ref()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.owning_relationship_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.owning_relationship_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.owning_relationship_ref()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.element_id_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.element_id_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.element_id_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.element_id_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.declared_short_name_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.declared_short_name_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.declared_short_name_ref()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.declared_name_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.declared_name_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.declared_name_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => x.declared_name_ref(),
            OperatorExpressionRefMut::IndexExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OperatorExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            OperatorExpressionRefMut::SelectExpression(x) => x.is_implied_included_ref(),
            OperatorExpressionRefMut::CollectExpression(x) => x.is_implied_included_ref(),
            OperatorExpressionRefMut::FeatureChainExpression(x) => {
                x.is_implied_included_ref()
            }
            OperatorExpressionRefMut::IndexExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for OperatorExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpressionRef::Itself(x) => x.owned_relationship_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.owned_relationship_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.owned_relationship_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => {
                x.owned_relationship_ref()
            }
            OperatorExpressionRef::IndexExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            OperatorExpressionRef::Itself(x) => x.owning_relationship_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.owning_relationship_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.owning_relationship_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => {
                x.owning_relationship_ref()
            }
            OperatorExpressionRef::IndexExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            OperatorExpressionRef::Itself(x) => x.element_id_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.element_id_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.element_id_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.element_id_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            OperatorExpressionRef::Itself(x) => x.alias_ids_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.alias_ids_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpressionRef::Itself(x) => x.declared_short_name_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.declared_short_name_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.declared_short_name_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => {
                x.declared_short_name_ref()
            }
            OperatorExpressionRef::IndexExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            OperatorExpressionRef::Itself(x) => x.declared_name_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.declared_name_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.declared_name_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => x.declared_name_ref(),
            OperatorExpressionRef::IndexExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            OperatorExpressionRef::Itself(x) => x.is_implied_included_ref(),
            OperatorExpressionRef::SelectExpression(x) => x.is_implied_included_ref(),
            OperatorExpressionRef::CollectExpression(x) => x.is_implied_included_ref(),
            OperatorExpressionRef::FeatureChainExpression(x) => {
                x.is_implied_included_ref()
            }
            OperatorExpressionRef::IndexExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl OperatorExpressionUpcast for OperatorExpression {
    fn into_operator_expression(self) -> OperatorExpression {
        self
    }
}
impl<'a> OperatorExpressionUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a> {
        self
    }
}
impl<'a> OperatorExpressionUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a> {
        self
    }
}
impl InvocationExpressionUpcast for OperatorExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        InvocationExpression::OperatorExpression(self).into_invocation_expression()
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self)
            .as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_invocation_expression_ref()
    }
}
impl InstantiationExpressionUpcast for OperatorExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        InvocationExpression::OperatorExpression(self).into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        InvocationExpressionRef::OperatorExpression(self)
            .as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for OperatorExpression {
    fn into_expression(self) -> Expression {
        InvocationExpression::OperatorExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_expression_ref()
    }
}
impl StepUpcast for OperatorExpression {
    fn into_step(self) -> Step {
        InvocationExpression::OperatorExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for OperatorExpression {
    fn into_feature(self) -> Feature {
        InvocationExpression::OperatorExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for OperatorExpression {
    fn into_type_(self) -> Type {
        InvocationExpression::OperatorExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for OperatorExpression {
    fn into_namespace(self) -> Namespace {
        InvocationExpression::OperatorExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for OperatorExpression {
    fn into_element(self) -> Element {
        InvocationExpression::OperatorExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        InvocationExpressionRefMut::OperatorExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for OperatorExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        InvocationExpressionRef::OperatorExpression(self).as_element_ref()
    }
}
pub trait OperatorExpressionDowncast {
    fn try_into_select_expression(self) -> Result<SelectExpression, String>;
    fn try_into_collect_expression(self) -> Result<CollectExpression, String>;
    fn try_into_feature_chain_expression(self) -> Result<FeatureChainExpression, String>;
    fn try_into_index_expression(self) -> Result<IndexExpression, String>;
}
pub trait OperatorExpressionDowncastRefMut<'a> {
    fn try_as_select_expression_ref_mut(
        self,
    ) -> Result<SelectExpressionRefMut<'a>, String>;
    fn try_as_collect_expression_ref_mut(
        self,
    ) -> Result<CollectExpressionRefMut<'a>, String>;
    fn try_as_feature_chain_expression_ref_mut(
        self,
    ) -> Result<FeatureChainExpressionRefMut<'a>, String>;
    fn try_as_index_expression_ref_mut(
        self,
    ) -> Result<IndexExpressionRefMut<'a>, String>;
}
pub trait OperatorExpressionDowncastRef<'a> {
    fn try_as_select_expression_ref(self) -> Result<SelectExpressionRef<'a>, String>;
    fn try_as_collect_expression_ref(self) -> Result<CollectExpressionRef<'a>, String>;
    fn try_as_feature_chain_expression_ref(
        self,
    ) -> Result<FeatureChainExpressionRef<'a>, String>;
    fn try_as_index_expression_ref(self) -> Result<IndexExpressionRef<'a>, String>;
}
impl OperatorExpressionDowncast for OperatorExpression {
    fn try_into_select_expression(self) -> Result<SelectExpression, String> {
        match self {
            OperatorExpression::SelectExpression(e) => Ok(e),
            _ => Err("Not a SelectExpression".into()),
        }
    }
    fn try_into_collect_expression(self) -> Result<CollectExpression, String> {
        match self {
            OperatorExpression::CollectExpression(e) => Ok(e),
            _ => Err("Not a CollectExpression".into()),
        }
    }
    fn try_into_feature_chain_expression(
        self,
    ) -> Result<FeatureChainExpression, String> {
        match self {
            OperatorExpression::FeatureChainExpression(e) => Ok(e),
            _ => Err("Not a FeatureChainExpression".into()),
        }
    }
    fn try_into_index_expression(self) -> Result<IndexExpression, String> {
        match self {
            OperatorExpression::IndexExpression(e) => Ok(e),
            _ => Err("Not a IndexExpression".into()),
        }
    }
}
impl<'a> OperatorExpressionDowncastRefMut<'a> for OperatorExpressionRefMut<'a> {
    fn try_as_select_expression_ref_mut(
        self,
    ) -> Result<SelectExpressionRefMut<'a>, String> {
        match self {
            OperatorExpressionRefMut::SelectExpression(e) => Ok(e),
            _ => Err("Not a SelectExpression".into()),
        }
    }
    fn try_as_collect_expression_ref_mut(
        self,
    ) -> Result<CollectExpressionRefMut<'a>, String> {
        match self {
            OperatorExpressionRefMut::CollectExpression(e) => Ok(e),
            _ => Err("Not a CollectExpression".into()),
        }
    }
    fn try_as_feature_chain_expression_ref_mut(
        self,
    ) -> Result<FeatureChainExpressionRefMut<'a>, String> {
        match self {
            OperatorExpressionRefMut::FeatureChainExpression(e) => Ok(e),
            _ => Err("Not a FeatureChainExpression".into()),
        }
    }
    fn try_as_index_expression_ref_mut(
        self,
    ) -> Result<IndexExpressionRefMut<'a>, String> {
        match self {
            OperatorExpressionRefMut::IndexExpression(e) => Ok(e),
            _ => Err("Not a IndexExpression".into()),
        }
    }
}
impl<'a> OperatorExpressionDowncastRef<'a> for OperatorExpressionRef<'a> {
    fn try_as_select_expression_ref(self) -> Result<SelectExpressionRef<'a>, String> {
        match self {
            OperatorExpressionRef::SelectExpression(e) => Ok(e),
            _ => Err("Not a SelectExpression".into()),
        }
    }
    fn try_as_collect_expression_ref(self) -> Result<CollectExpressionRef<'a>, String> {
        match self {
            OperatorExpressionRef::CollectExpression(e) => Ok(e),
            _ => Err("Not a CollectExpression".into()),
        }
    }
    fn try_as_feature_chain_expression_ref(
        self,
    ) -> Result<FeatureChainExpressionRef<'a>, String> {
        match self {
            OperatorExpressionRef::FeatureChainExpression(e) => Ok(e),
            _ => Err("Not a FeatureChainExpression".into()),
        }
    }
    fn try_as_index_expression_ref(self) -> Result<IndexExpressionRef<'a>, String> {
        match self {
            OperatorExpressionRef::IndexExpression(e) => Ok(e),
            _ => Err("Not a IndexExpression".into()),
        }
    }
}
pub trait OperatorExpressionMethodsDescendants
where
    Self: DescendantOf<OperatorExpression>,
    Self::Via: OperatorExpressionMethods,
    Self: Sized,
{}
pub trait OperatorExpressionMethods: Sized {}
impl<T: OperatorExpressionMethodsDescendants> OperatorExpressionMethods for T
where
    T::Via: OperatorExpressionMethods,
{}
impl DescendantOf<InvocationExpression> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl InvocationExpressionMethodsDescendants for OperatorExpression {}
impl DescendantOf<InstantiationExpression> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for OperatorExpression {}
impl DescendantOf<Expression> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl ExpressionMethodsDescendants for OperatorExpression {}
impl DescendantOf<Step> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl StepMethodsDescendants for OperatorExpression {}
impl DescendantOf<Feature> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl FeatureMethodsDescendants for OperatorExpression {}
impl DescendantOf<Type> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl TypeMethodsDescendants for OperatorExpression {}
impl DescendantOf<Namespace> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl NamespaceMethodsDescendants for OperatorExpression {}
impl DescendantOf<Element> for OperatorExpression {
    type Via = InvocationExpression;
    fn into_via(self) -> Self::Via {
        self.into_invocation_expression()
    }
}
impl ElementMethodsDescendants for OperatorExpression {}
pub trait OperatorExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<OperatorExpressionRefMut<'a>>,
    Self::Via: OperatorExpressionRefMutMethods,
    Self: Sized,
{}
pub trait OperatorExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: OperatorExpressionRefMutMethodsDescendants<'a>,
> OperatorExpressionRefMutMethods for T
where
    T::Via: OperatorExpressionRefMutMethods,
{}
impl<'a> DescendantOf<InvocationExpressionRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionRefMutMethodsDescendants<'a>
for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>>
for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for OperatorExpressionRefMut<'a> {
    type Via = InvocationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for OperatorExpressionRefMut<'a> {}
pub trait OperatorExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<OperatorExpressionRef<'a>>,
    Self::Via: OperatorExpressionRefMethods,
    Self: Sized,
{}
pub trait OperatorExpressionRefMethods: Sized {}
impl<'a, T: OperatorExpressionRefMethodsDescendants<'a>> OperatorExpressionRefMethods
for T
where
    T::Via: OperatorExpressionRefMethods,
{}
impl<'a> DescendantOf<InvocationExpressionRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> InvocationExpressionRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for OperatorExpressionRef<'a> {
    type Via = InvocationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_invocation_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for OperatorExpressionRef<'a> {}

