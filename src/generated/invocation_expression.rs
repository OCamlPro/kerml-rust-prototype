#![allow(unused)]
use super::utils::DescendantOf;
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
use super::operator_expression::{
    OperatorExpression, OperatorExpressionRefMut, OperatorExpressionRef,
};
pub struct InvocationExpressionInner {
    pub(super) sup_instantiation_expression: InstantiationExpressionInner,
}
pub trait InvocationExpressionStruct
where
    Self: InvocationExpressionStructRefMut,
    Self: InvocationExpressionStructRef,
    Self: InstantiationExpressionStruct,
{}
pub trait InvocationExpressionStructRefMut
where
    Self: InvocationExpressionStructRef,
    Self: InstantiationExpressionStructRefMut,
{}
pub trait InvocationExpressionStructRef
where
    Self: InstantiationExpressionStructRef,
{}
pub trait InvocationExpressionUpcast: InvocationExpressionStruct {
    fn into_invocation_expression(self) -> InvocationExpression;
}
pub trait InvocationExpressionUpcastRefMut<'a>: InvocationExpressionStructRefMut {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a>;
}
pub trait InvocationExpressionUpcastRef<'a>: InvocationExpressionStructRef {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a>;
}
impl InvocationExpressionStruct for InvocationExpressionInner {}
impl InvocationExpressionStructRefMut for InvocationExpressionInner {}
impl InvocationExpressionStructRef for InvocationExpressionInner {}
impl InstantiationExpressionStruct for InvocationExpressionInner {}
impl InstantiationExpressionStructRefMut for InvocationExpressionInner {}
impl InstantiationExpressionStructRef for InvocationExpressionInner {}
impl ExpressionStruct for InvocationExpressionInner {}
impl ExpressionStructRefMut for InvocationExpressionInner {}
impl ExpressionStructRef for InvocationExpressionInner {}
impl StepStruct for InvocationExpressionInner {}
impl StepStructRefMut for InvocationExpressionInner {}
impl StepStructRef for InvocationExpressionInner {}
impl FeatureStruct for InvocationExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_instantiation_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_instantiation_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_instantiation_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_instantiation_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_instantiation_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_instantiation_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_instantiation_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_instantiation_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_instantiation_expression.direction()
    }
}
impl FeatureStructRefMut for InvocationExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_instantiation_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for InvocationExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_instantiation_expression.direction_ref()
    }
}
impl TypeStruct for InvocationExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_instantiation_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_instantiation_expression.is_sufficient()
    }
}
impl TypeStructRefMut for InvocationExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for InvocationExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for InvocationExpressionInner {}
impl NamespaceStructRefMut for InvocationExpressionInner {}
impl NamespaceStructRef for InvocationExpressionInner {}
impl ElementStruct for InvocationExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_instantiation_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_instantiation_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_instantiation_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_instantiation_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_instantiation_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_instantiation_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_instantiation_expression.is_implied_included()
    }
}
impl ElementStructRefMut for InvocationExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_instantiation_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_instantiation_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_instantiation_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_instantiation_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_instantiation_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_instantiation_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for InvocationExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_instantiation_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_instantiation_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_instantiation_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_instantiation_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_instantiation_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_instantiation_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_implied_included_ref()
    }
}
pub enum InvocationExpression {
    Itself(InvocationExpressionInner),
    OperatorExpression(OperatorExpression),
}
pub enum InvocationExpressionRefMut<'a> {
    Itself(&'a mut InvocationExpressionInner),
    OperatorExpression(OperatorExpressionRefMut<'a>),
}
pub enum InvocationExpressionRef<'a> {
    Itself(&'a InvocationExpressionInner),
    OperatorExpression(OperatorExpressionRef<'a>),
}
impl InvocationExpression {
    pub fn as_ref(&self) -> InvocationExpressionRef {
        match self {
            InvocationExpression::Itself(inner) => {
                InvocationExpressionRef::Itself(&inner)
            }
            InvocationExpression::OperatorExpression(inner) => {
                InvocationExpressionRef::OperatorExpression(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> InvocationExpressionRefMut {
        match self {
            InvocationExpression::Itself(inner) => {
                InvocationExpressionRefMut::Itself(inner)
            }
            InvocationExpression::OperatorExpression(inner) => {
                InvocationExpressionRefMut::OperatorExpression(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> InvocationExpressionRefMut<'a> {
    pub fn as_ref(self) -> InvocationExpressionRef<'a> {
        match self {
            InvocationExpressionRefMut::Itself(inner) => {
                InvocationExpressionRef::Itself(inner)
            }
            InvocationExpressionRefMut::OperatorExpression(inner) => {
                InvocationExpressionRef::OperatorExpression(inner.as_ref())
            }
        }
    }
}
impl InvocationExpressionStruct for InvocationExpression {}
impl InvocationExpressionStructRefMut for InvocationExpression {}
impl InvocationExpressionStructRef for InvocationExpression {}
impl<'a> InvocationExpressionStructRefMut for InvocationExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for InvocationExpressionRefMut<'a> {}
impl<'a> InvocationExpressionStructRef for InvocationExpressionRef<'a> {}
impl InstantiationExpressionStruct for InvocationExpression {}
impl InstantiationExpressionStructRefMut for InvocationExpression {}
impl InstantiationExpressionStructRef for InvocationExpression {}
impl<'a> InstantiationExpressionStructRefMut for InvocationExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for InvocationExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for InvocationExpressionRef<'a> {}
impl ExpressionStruct for InvocationExpression {}
impl ExpressionStructRefMut for InvocationExpression {}
impl ExpressionStructRef for InvocationExpression {}
impl<'a> ExpressionStructRefMut for InvocationExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for InvocationExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for InvocationExpressionRef<'a> {}
impl StepStruct for InvocationExpression {}
impl StepStructRefMut for InvocationExpression {}
impl StepStructRef for InvocationExpression {}
impl<'a> StepStructRefMut for InvocationExpressionRefMut<'a> {}
impl<'a> StepStructRef for InvocationExpressionRefMut<'a> {}
impl<'a> StepStructRef for InvocationExpressionRef<'a> {}
impl FeatureStruct for InvocationExpression {
    fn is_unique(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_unique(),
            InvocationExpression::OperatorExpression(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_ordered(),
            InvocationExpression::OperatorExpression(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_composite(),
            InvocationExpression::OperatorExpression(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_end(),
            InvocationExpression::OperatorExpression(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_derived(),
            InvocationExpression::OperatorExpression(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_portion(),
            InvocationExpression::OperatorExpression(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_variable(),
            InvocationExpression::OperatorExpression(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_constant(),
            InvocationExpression::OperatorExpression(x) => x.is_constant(),
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
            InvocationExpression::Itself(x) => x.direction(),
            InvocationExpression::OperatorExpression(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for InvocationExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_unique_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_ordered_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_composite_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_end_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_derived_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_portion_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_variable_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_constant_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_constant_ref_mut(),
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
            InvocationExpression::Itself(x) => x.direction_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for InvocationExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_unique_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_ordered_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_composite_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_end_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_derived_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_portion_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_variable_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_constant_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_constant_ref(),
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
            InvocationExpression::Itself(x) => x.direction_ref(),
            InvocationExpression::OperatorExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for InvocationExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_constant_ref_mut(),
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
            InvocationExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for InvocationExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_unique_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_composite_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_end_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_derived_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_portion_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_variable_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_constant_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_constant_ref(),
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
            InvocationExpressionRefMut::Itself(x) => x.direction_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for InvocationExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_unique_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_ordered_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_composite_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_end_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_derived_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_portion_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_variable_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_constant_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_constant_ref(),
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
            InvocationExpressionRef::Itself(x) => x.direction_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for InvocationExpression {
    fn is_abstract(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_abstract(),
            InvocationExpression::OperatorExpression(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_sufficient(),
            InvocationExpression::OperatorExpression(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for InvocationExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_abstract_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_sufficient_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for InvocationExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_abstract_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_sufficient_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for InvocationExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.is_sufficient_ref_mut()
            }
        }
    }
}
impl<'a> TypeStructRef for InvocationExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for InvocationExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_abstract_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_sufficient_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for InvocationExpression {}
impl NamespaceStructRefMut for InvocationExpression {}
impl NamespaceStructRef for InvocationExpression {}
impl<'a> NamespaceStructRefMut for InvocationExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for InvocationExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for InvocationExpressionRef<'a> {}
impl ElementStruct for InvocationExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpression::Itself(x) => x.owned_relationship(),
            InvocationExpression::OperatorExpression(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpression::Itself(x) => x.owning_relationship(),
            InvocationExpression::OperatorExpression(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            InvocationExpression::Itself(x) => x.element_id(),
            InvocationExpression::OperatorExpression(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            InvocationExpression::Itself(x) => x.alias_ids(),
            InvocationExpression::OperatorExpression(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_short_name(),
            InvocationExpression::OperatorExpression(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_name(),
            InvocationExpression::OperatorExpression(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            InvocationExpression::Itself(x) => x.is_implied_included(),
            InvocationExpression::OperatorExpression(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for InvocationExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpression::Itself(x) => x.owned_relationship_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            InvocationExpression::Itself(x) => x.owning_relationship_ref_mut(),
            InvocationExpression::OperatorExpression(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InvocationExpression::Itself(x) => x.element_id_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InvocationExpression::Itself(x) => x.alias_ids_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_short_name_ref_mut(),
            InvocationExpression::OperatorExpression(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_name_ref_mut(),
            InvocationExpression::OperatorExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpression::Itself(x) => x.is_implied_included_ref_mut(),
            InvocationExpression::OperatorExpression(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl ElementStructRef for InvocationExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpression::Itself(x) => x.owned_relationship_ref(),
            InvocationExpression::OperatorExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpression::Itself(x) => x.owning_relationship_ref(),
            InvocationExpression::OperatorExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InvocationExpression::Itself(x) => x.element_id_ref(),
            InvocationExpression::OperatorExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InvocationExpression::Itself(x) => x.alias_ids_ref(),
            InvocationExpression::OperatorExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_short_name_ref(),
            InvocationExpression::OperatorExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpression::Itself(x) => x.declared_name_ref(),
            InvocationExpression::OperatorExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InvocationExpression::Itself(x) => x.is_implied_included_ref(),
            InvocationExpression::OperatorExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for InvocationExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
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
            InvocationExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.declared_name_ref_mut()
            }
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for InvocationExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.element_id_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.declared_name_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InvocationExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            InvocationExpressionRefMut::OperatorExpression(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRef for InvocationExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpressionRef::Itself(x) => x.owned_relationship_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvocationExpressionRef::Itself(x) => x.owning_relationship_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InvocationExpressionRef::Itself(x) => x.element_id_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InvocationExpressionRef::Itself(x) => x.alias_ids_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpressionRef::Itself(x) => x.declared_short_name_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InvocationExpressionRef::Itself(x) => x.declared_name_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InvocationExpressionRef::Itself(x) => x.is_implied_included_ref(),
            InvocationExpressionRef::OperatorExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl InvocationExpressionUpcast for InvocationExpression {
    fn into_invocation_expression(self) -> InvocationExpression {
        self
    }
}
impl<'a> InvocationExpressionUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_invocation_expression_ref_mut(self) -> InvocationExpressionRefMut<'a> {
        self
    }
}
impl<'a> InvocationExpressionUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_invocation_expression_ref(self) -> InvocationExpressionRef<'a> {
        self
    }
}
impl InstantiationExpressionUpcast for InvocationExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        InstantiationExpression::InvocationExpression(self)
            .into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self)
            .as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for InvocationExpression {
    fn into_expression(self) -> Expression {
        InstantiationExpression::InvocationExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_expression_ref()
    }
}
impl StepUpcast for InvocationExpression {
    fn into_step(self) -> Step {
        InstantiationExpression::InvocationExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for InvocationExpression {
    fn into_feature(self) -> Feature {
        InstantiationExpression::InvocationExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for InvocationExpression {
    fn into_type_(self) -> Type {
        InstantiationExpression::InvocationExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for InvocationExpression {
    fn into_namespace(self) -> Namespace {
        InstantiationExpression::InvocationExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for InvocationExpression {
    fn into_element(self) -> Element {
        InstantiationExpression::InvocationExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        InstantiationExpressionRefMut::InvocationExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for InvocationExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        InstantiationExpressionRef::InvocationExpression(self).as_element_ref()
    }
}
pub trait InvocationExpressionDowncast {
    fn try_into_operator_expression(self) -> Result<OperatorExpression, String>;
}
pub trait InvocationExpressionDowncastRefMut<'a> {
    fn try_as_operator_expression_ref_mut(
        self,
    ) -> Result<OperatorExpressionRefMut<'a>, String>;
}
pub trait InvocationExpressionDowncastRef<'a> {
    fn try_as_operator_expression_ref(self) -> Result<OperatorExpressionRef<'a>, String>;
}
impl InvocationExpressionDowncast for InvocationExpression {
    fn try_into_operator_expression(self) -> Result<OperatorExpression, String> {
        match self {
            InvocationExpression::OperatorExpression(e) => Ok(e),
            _ => Err("Not a OperatorExpression".into()),
        }
    }
}
impl<'a> InvocationExpressionDowncastRefMut<'a> for InvocationExpressionRefMut<'a> {
    fn try_as_operator_expression_ref_mut(
        self,
    ) -> Result<OperatorExpressionRefMut<'a>, String> {
        match self {
            InvocationExpressionRefMut::OperatorExpression(e) => Ok(e),
            _ => Err("Not a OperatorExpression".into()),
        }
    }
}
impl<'a> InvocationExpressionDowncastRef<'a> for InvocationExpressionRef<'a> {
    fn try_as_operator_expression_ref(
        self,
    ) -> Result<OperatorExpressionRef<'a>, String> {
        match self {
            InvocationExpressionRef::OperatorExpression(e) => Ok(e),
            _ => Err("Not a OperatorExpression".into()),
        }
    }
}
pub trait InvocationExpressionMethodsDescendants
where
    Self: DescendantOf<InvocationExpression>,
    Self::Via: InvocationExpressionMethods,
    Self: Sized,
{}
pub trait InvocationExpressionMethods: Sized {}
impl<T: InvocationExpressionMethodsDescendants> InvocationExpressionMethods for T
where
    T::Via: InvocationExpressionMethods,
{}
impl DescendantOf<InstantiationExpression> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for InvocationExpression {}
impl DescendantOf<Expression> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl ExpressionMethodsDescendants for InvocationExpression {}
impl DescendantOf<Step> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl StepMethodsDescendants for InvocationExpression {}
impl DescendantOf<Feature> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl FeatureMethodsDescendants for InvocationExpression {}
impl DescendantOf<Type> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl TypeMethodsDescendants for InvocationExpression {}
impl DescendantOf<Namespace> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl NamespaceMethodsDescendants for InvocationExpression {}
impl DescendantOf<Element> for InvocationExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl ElementMethodsDescendants for InvocationExpression {}
pub trait InvocationExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<InvocationExpressionRefMut<'a>>,
    Self::Via: InvocationExpressionRefMutMethods,
    Self: Sized,
{}
pub trait InvocationExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: InvocationExpressionRefMutMethodsDescendants<'a>,
> InvocationExpressionRefMutMethods for T
where
    T::Via: InvocationExpressionRefMutMethods,
{}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>>
for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for InvocationExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for InvocationExpressionRefMut<'a> {}
pub trait InvocationExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<InvocationExpressionRef<'a>>,
    Self::Via: InvocationExpressionRefMethods,
    Self: Sized,
{}
pub trait InvocationExpressionRefMethods: Sized {}
impl<'a, T: InvocationExpressionRefMethodsDescendants<'a>> InvocationExpressionRefMethods
for T
where
    T::Via: InvocationExpressionRefMethods,
{}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a>
for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for InvocationExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for InvocationExpressionRef<'a> {}

