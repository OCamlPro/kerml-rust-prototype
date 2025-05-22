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
pub struct IndexExpressionInner {
    pub(super) sup_operator_expression: OperatorExpressionInner,
}
pub trait IndexExpressionStruct
where
    Self: IndexExpressionStructRefMut,
    Self: IndexExpressionStructRef,
    Self: OperatorExpressionStruct,
{}
pub trait IndexExpressionStructRefMut
where
    Self: IndexExpressionStructRef,
    Self: OperatorExpressionStructRefMut,
{}
pub trait IndexExpressionStructRef
where
    Self: OperatorExpressionStructRef,
{}
pub trait IndexExpressionUpcast: IndexExpressionStruct {
    fn into_index_expression(self) -> IndexExpression;
}
pub trait IndexExpressionUpcastRefMut<'a>: IndexExpressionStructRefMut {
    fn as_index_expression_ref_mut(self) -> IndexExpressionRefMut<'a>;
}
pub trait IndexExpressionUpcastRef<'a>: IndexExpressionStructRef {
    fn as_index_expression_ref(self) -> IndexExpressionRef<'a>;
}
impl IndexExpressionStruct for IndexExpressionInner {}
impl IndexExpressionStructRefMut for IndexExpressionInner {}
impl IndexExpressionStructRef for IndexExpressionInner {}
impl OperatorExpressionStruct for IndexExpressionInner {
    fn operator(self) -> String {
        self.sup_operator_expression.operator()
    }
}
impl OperatorExpressionStructRefMut for IndexExpressionInner {
    fn operator_ref_mut(&mut self) -> &mut String {
        self.sup_operator_expression.operator_ref_mut()
    }
}
impl OperatorExpressionStructRef for IndexExpressionInner {
    fn operator_ref(&self) -> &String {
        self.sup_operator_expression.operator_ref()
    }
}
impl InvocationExpressionStruct for IndexExpressionInner {}
impl InvocationExpressionStructRefMut for IndexExpressionInner {}
impl InvocationExpressionStructRef for IndexExpressionInner {}
impl InstantiationExpressionStruct for IndexExpressionInner {}
impl InstantiationExpressionStructRefMut for IndexExpressionInner {}
impl InstantiationExpressionStructRef for IndexExpressionInner {}
impl ExpressionStruct for IndexExpressionInner {}
impl ExpressionStructRefMut for IndexExpressionInner {}
impl ExpressionStructRef for IndexExpressionInner {}
impl StepStruct for IndexExpressionInner {}
impl StepStructRefMut for IndexExpressionInner {}
impl StepStructRef for IndexExpressionInner {}
impl FeatureStruct for IndexExpressionInner {
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
impl FeatureStructRefMut for IndexExpressionInner {
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
impl FeatureStructRef for IndexExpressionInner {
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
impl TypeStruct for IndexExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_operator_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_operator_expression.is_sufficient()
    }
}
impl TypeStructRefMut for IndexExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_operator_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for IndexExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_operator_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_operator_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for IndexExpressionInner {}
impl NamespaceStructRefMut for IndexExpressionInner {}
impl NamespaceStructRef for IndexExpressionInner {}
impl ElementStruct for IndexExpressionInner {
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
impl ElementStructRefMut for IndexExpressionInner {
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
impl ElementStructRef for IndexExpressionInner {
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
pub enum IndexExpression {
    Itself(IndexExpressionInner),
}
pub enum IndexExpressionRefMut<'a> {
    Itself(&'a mut IndexExpressionInner),
}
pub enum IndexExpressionRef<'a> {
    Itself(&'a IndexExpressionInner),
}
impl IndexExpression {
    pub fn as_ref(&self) -> IndexExpressionRef {
        match self {
            IndexExpression::Itself(inner) => IndexExpressionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> IndexExpressionRefMut {
        match self {
            IndexExpression::Itself(inner) => IndexExpressionRefMut::Itself(inner),
        }
    }
}
impl<'a> IndexExpressionRefMut<'a> {
    pub fn as_ref(self) -> IndexExpressionRef<'a> {
        match self {
            IndexExpressionRefMut::Itself(inner) => IndexExpressionRef::Itself(inner),
        }
    }
}
impl IndexExpressionStruct for IndexExpression {}
impl IndexExpressionStructRefMut for IndexExpression {}
impl IndexExpressionStructRef for IndexExpression {}
impl<'a> IndexExpressionStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> IndexExpressionStructRef for IndexExpressionRefMut<'a> {}
impl<'a> IndexExpressionStructRef for IndexExpressionRef<'a> {}
impl OperatorExpressionStruct for IndexExpression {
    fn operator(self) -> String {
        match self {
            IndexExpression::Itself(x) => x.operator(),
        }
    }
}
impl OperatorExpressionStructRefMut for IndexExpression {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            IndexExpression::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl OperatorExpressionStructRef for IndexExpression {
    fn operator_ref(&self) -> &String {
        match self {
            IndexExpression::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRefMut for IndexExpressionRefMut<'a> {
    fn operator_ref_mut(&mut self) -> &mut String {
        match self {
            IndexExpressionRefMut::Itself(x) => x.operator_ref_mut(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for IndexExpressionRefMut<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            IndexExpressionRefMut::Itself(x) => x.operator_ref(),
        }
    }
}
impl<'a> OperatorExpressionStructRef for IndexExpressionRef<'a> {
    fn operator_ref(&self) -> &String {
        match self {
            IndexExpressionRef::Itself(x) => x.operator_ref(),
        }
    }
}
impl InvocationExpressionStruct for IndexExpression {}
impl InvocationExpressionStructRefMut for IndexExpression {}
impl InvocationExpressionStructRef for IndexExpression {}
impl<'a> InvocationExpressionStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for IndexExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for IndexExpressionRef<'a> {}
impl InstantiationExpressionStruct for IndexExpression {}
impl InstantiationExpressionStructRefMut for IndexExpression {}
impl InstantiationExpressionStructRef for IndexExpression {}
impl<'a> InstantiationExpressionStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for IndexExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for IndexExpressionRef<'a> {}
impl ExpressionStruct for IndexExpression {}
impl ExpressionStructRefMut for IndexExpression {}
impl ExpressionStructRef for IndexExpression {}
impl<'a> ExpressionStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for IndexExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for IndexExpressionRef<'a> {}
impl StepStruct for IndexExpression {}
impl StepStructRefMut for IndexExpression {}
impl StepStructRef for IndexExpression {}
impl<'a> StepStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> StepStructRef for IndexExpressionRefMut<'a> {}
impl<'a> StepStructRef for IndexExpressionRef<'a> {}
impl FeatureStruct for IndexExpression {
    fn is_unique(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_constant(),
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
            IndexExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for IndexExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_constant_ref_mut(),
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
            IndexExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for IndexExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_constant_ref(),
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
            IndexExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for IndexExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            IndexExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for IndexExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            IndexExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for IndexExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_constant_ref(),
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
            IndexExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for IndexExpression {
    fn is_abstract(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for IndexExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for IndexExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for IndexExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for IndexExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for IndexExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for IndexExpression {}
impl NamespaceStructRefMut for IndexExpression {}
impl NamespaceStructRef for IndexExpression {}
impl<'a> NamespaceStructRefMut for IndexExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for IndexExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for IndexExpressionRef<'a> {}
impl ElementStruct for IndexExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            IndexExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            IndexExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            IndexExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for IndexExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            IndexExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            IndexExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            IndexExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for IndexExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            IndexExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            IndexExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            IndexExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for IndexExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            IndexExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            IndexExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for IndexExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            IndexExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            IndexExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for IndexExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            IndexExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            IndexExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            IndexExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            IndexExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            IndexExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl IndexExpressionUpcast for IndexExpression {
    fn into_index_expression(self) -> IndexExpression {
        self
    }
}
impl<'a> IndexExpressionUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_index_expression_ref_mut(self) -> IndexExpressionRefMut<'a> {
        self
    }
}
impl<'a> IndexExpressionUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_index_expression_ref(self) -> IndexExpressionRef<'a> {
        self
    }
}
impl OperatorExpressionUpcast for IndexExpression {
    fn into_operator_expression(self) -> OperatorExpression {
        OperatorExpression::IndexExpression(self).into_operator_expression()
    }
}
impl<'a> OperatorExpressionUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_operator_expression_ref_mut(self) -> OperatorExpressionRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_operator_expression_ref(self) -> OperatorExpressionRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_operator_expression_ref()
    }
}
impl InvocationExpressionUpcast for IndexExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        OperatorExpression::IndexExpression(self).into_invocation_expression()
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self)
            .as_invocation_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_invocation_expression_ref()
    }
}
impl InstantiationExpressionUpcast for IndexExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        OperatorExpression::IndexExpression(self).into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for IndexExpression {
    fn into_expression(self) -> Expression {
        OperatorExpression::IndexExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_expression_ref()
    }
}
impl StepUpcast for IndexExpression {
    fn into_step(self) -> Step {
        OperatorExpression::IndexExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for IndexExpression {
    fn into_feature(self) -> Feature {
        OperatorExpression::IndexExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for IndexExpression {
    fn into_type_(self) -> Type {
        OperatorExpression::IndexExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for IndexExpression {
    fn into_namespace(self) -> Namespace {
        OperatorExpression::IndexExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for IndexExpression {
    fn into_element(self) -> Element {
        OperatorExpression::IndexExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for IndexExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        OperatorExpressionRefMut::IndexExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for IndexExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        OperatorExpressionRef::IndexExpression(self).as_element_ref()
    }
}
pub trait IndexExpressionDowncast {}
pub trait IndexExpressionDowncastRefMut<'a> {}
pub trait IndexExpressionDowncastRef<'a> {}
impl IndexExpressionDowncast for IndexExpression {}
impl<'a> IndexExpressionDowncastRefMut<'a> for IndexExpressionRefMut<'a> {}
impl<'a> IndexExpressionDowncastRef<'a> for IndexExpressionRef<'a> {}
pub trait IndexExpressionMethodsDescendants
where
    Self: DescendantOf<IndexExpression>,
    Self::Via: IndexExpressionMethods,
    Self: Sized,
{}
pub trait IndexExpressionMethods: Sized {}
impl<T: IndexExpressionMethodsDescendants> IndexExpressionMethods for T
where
    T::Via: IndexExpressionMethods,
{}
impl DescendantOf<OperatorExpression> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl OperatorExpressionMethodsDescendants for IndexExpression {}
impl DescendantOf<InvocationExpression> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InvocationExpressionMethodsDescendants for IndexExpression {}
impl DescendantOf<InstantiationExpression> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for IndexExpression {}
impl DescendantOf<Expression> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ExpressionMethodsDescendants for IndexExpression {}
impl DescendantOf<Step> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl StepMethodsDescendants for IndexExpression {}
impl DescendantOf<Feature> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl FeatureMethodsDescendants for IndexExpression {}
impl DescendantOf<Type> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl TypeMethodsDescendants for IndexExpression {}
impl DescendantOf<Namespace> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl NamespaceMethodsDescendants for IndexExpression {}
impl DescendantOf<Element> for IndexExpression {
    type Via = OperatorExpression;
    fn into_via(self) -> Self::Via {
        self.into_operator_expression()
    }
}
impl ElementMethodsDescendants for IndexExpression {}
pub trait IndexExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<IndexExpressionRefMut<'a>>,
    Self::Via: IndexExpressionRefMutMethods,
    Self: Sized,
{}
pub trait IndexExpressionRefMutMethods: Sized {}
impl<'a, T: IndexExpressionRefMutMethodsDescendants<'a>> IndexExpressionRefMutMethods
for T
where
    T::Via: IndexExpressionRefMutMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> OperatorExpressionRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<InvocationExpressionRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InvocationExpressionRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for IndexExpressionRefMut<'a> {
    type Via = OperatorExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for IndexExpressionRefMut<'a> {}
pub trait IndexExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<IndexExpressionRef<'a>>,
    Self::Via: IndexExpressionRefMethods,
    Self: Sized,
{}
pub trait IndexExpressionRefMethods: Sized {}
impl<'a, T: IndexExpressionRefMethodsDescendants<'a>> IndexExpressionRefMethods for T
where
    T::Via: IndexExpressionRefMethods,
{}
impl<'a> DescendantOf<OperatorExpressionRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> OperatorExpressionRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<InvocationExpressionRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InvocationExpressionRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for IndexExpressionRef<'a> {
    type Via = OperatorExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_operator_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for IndexExpressionRef<'a> {}

