#![allow(unused)]
use super::utils::DescendantOf;
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
use super::invariant::{Invariant, InvariantRefMut, InvariantRef};
pub struct BooleanExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait BooleanExpressionStruct
where
    Self: BooleanExpressionStructRefMut,
    Self: BooleanExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait BooleanExpressionStructRefMut
where
    Self: BooleanExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait BooleanExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait BooleanExpressionUpcast: BooleanExpressionStruct {
    fn into_boolean_expression(self) -> BooleanExpression;
}
pub trait BooleanExpressionUpcastRefMut<'a>: BooleanExpressionStructRefMut {
    fn as_boolean_expression_ref_mut(self) -> BooleanExpressionRefMut<'a>;
}
pub trait BooleanExpressionUpcastRef<'a>: BooleanExpressionStructRef {
    fn as_boolean_expression_ref(self) -> BooleanExpressionRef<'a>;
}
impl BooleanExpressionStruct for BooleanExpressionInner {}
impl BooleanExpressionStructRefMut for BooleanExpressionInner {}
impl BooleanExpressionStructRef for BooleanExpressionInner {}
impl ExpressionStruct for BooleanExpressionInner {}
impl ExpressionStructRefMut for BooleanExpressionInner {}
impl ExpressionStructRef for BooleanExpressionInner {}
impl StepStruct for BooleanExpressionInner {}
impl StepStructRefMut for BooleanExpressionInner {}
impl StepStructRef for BooleanExpressionInner {}
impl FeatureStruct for BooleanExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction()
    }
}
impl FeatureStructRefMut for BooleanExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for BooleanExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction_ref()
    }
}
impl TypeStruct for BooleanExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for BooleanExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for BooleanExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for BooleanExpressionInner {}
impl NamespaceStructRefMut for BooleanExpressionInner {}
impl NamespaceStructRef for BooleanExpressionInner {}
impl ElementStruct for BooleanExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_expression.is_implied_included()
    }
}
impl ElementStructRefMut for BooleanExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for BooleanExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_expression.is_implied_included_ref()
    }
}
pub enum BooleanExpression {
    Itself(BooleanExpressionInner),
    Invariant(Invariant),
}
pub enum BooleanExpressionRefMut<'a> {
    Itself(&'a mut BooleanExpressionInner),
    Invariant(InvariantRefMut<'a>),
}
pub enum BooleanExpressionRef<'a> {
    Itself(&'a BooleanExpressionInner),
    Invariant(InvariantRef<'a>),
}
impl BooleanExpression {
    pub fn as_ref(&self) -> BooleanExpressionRef {
        match self {
            BooleanExpression::Itself(inner) => BooleanExpressionRef::Itself(&inner),
            BooleanExpression::Invariant(inner) => {
                BooleanExpressionRef::Invariant(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> BooleanExpressionRefMut {
        match self {
            BooleanExpression::Itself(inner) => BooleanExpressionRefMut::Itself(inner),
            BooleanExpression::Invariant(inner) => {
                BooleanExpressionRefMut::Invariant(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> BooleanExpressionRefMut<'a> {
    pub fn as_ref(self) -> BooleanExpressionRef<'a> {
        match self {
            BooleanExpressionRefMut::Itself(inner) => BooleanExpressionRef::Itself(inner),
            BooleanExpressionRefMut::Invariant(inner) => {
                BooleanExpressionRef::Invariant(inner.as_ref())
            }
        }
    }
}
impl BooleanExpressionStruct for BooleanExpression {}
impl BooleanExpressionStructRefMut for BooleanExpression {}
impl BooleanExpressionStructRef for BooleanExpression {}
impl<'a> BooleanExpressionStructRefMut for BooleanExpressionRefMut<'a> {}
impl<'a> BooleanExpressionStructRef for BooleanExpressionRefMut<'a> {}
impl<'a> BooleanExpressionStructRef for BooleanExpressionRef<'a> {}
impl ExpressionStruct for BooleanExpression {}
impl ExpressionStructRefMut for BooleanExpression {}
impl ExpressionStructRef for BooleanExpression {}
impl<'a> ExpressionStructRefMut for BooleanExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for BooleanExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for BooleanExpressionRef<'a> {}
impl StepStruct for BooleanExpression {}
impl StepStructRefMut for BooleanExpression {}
impl StepStructRef for BooleanExpression {}
impl<'a> StepStructRefMut for BooleanExpressionRefMut<'a> {}
impl<'a> StepStructRef for BooleanExpressionRefMut<'a> {}
impl<'a> StepStructRef for BooleanExpressionRef<'a> {}
impl FeatureStruct for BooleanExpression {
    fn is_unique(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_unique(),
            BooleanExpression::Invariant(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_ordered(),
            BooleanExpression::Invariant(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_composite(),
            BooleanExpression::Invariant(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_end(),
            BooleanExpression::Invariant(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_derived(),
            BooleanExpression::Invariant(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_portion(),
            BooleanExpression::Invariant(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_variable(),
            BooleanExpression::Invariant(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_constant(),
            BooleanExpression::Invariant(x) => x.is_constant(),
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
            BooleanExpression::Itself(x) => x.direction(),
            BooleanExpression::Invariant(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for BooleanExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_unique_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_ordered_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_composite_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_end_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_derived_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_portion_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_variable_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_constant_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_constant_ref_mut(),
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
            BooleanExpression::Itself(x) => x.direction_ref_mut(),
            BooleanExpression::Invariant(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for BooleanExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_unique_ref(),
            BooleanExpression::Invariant(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_ordered_ref(),
            BooleanExpression::Invariant(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_composite_ref(),
            BooleanExpression::Invariant(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_end_ref(),
            BooleanExpression::Invariant(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_derived_ref(),
            BooleanExpression::Invariant(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_portion_ref(),
            BooleanExpression::Invariant(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_variable_ref(),
            BooleanExpression::Invariant(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_constant_ref(),
            BooleanExpression::Invariant(x) => x.is_constant_ref(),
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
            BooleanExpression::Itself(x) => x.direction_ref(),
            BooleanExpression::Invariant(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for BooleanExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_constant_ref_mut(),
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
            BooleanExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for BooleanExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_unique_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_composite_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_end_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_derived_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_portion_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_variable_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_constant_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_constant_ref(),
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
            BooleanExpressionRefMut::Itself(x) => x.direction_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for BooleanExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_unique_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_ordered_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_composite_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_end_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_derived_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_portion_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_variable_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_constant_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_constant_ref(),
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
            BooleanExpressionRef::Itself(x) => x.direction_ref(),
            BooleanExpressionRef::Invariant(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for BooleanExpression {
    fn is_abstract(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_abstract(),
            BooleanExpression::Invariant(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_sufficient(),
            BooleanExpression::Invariant(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for BooleanExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_abstract_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_sufficient_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for BooleanExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_abstract_ref(),
            BooleanExpression::Invariant(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_sufficient_ref(),
            BooleanExpression::Invariant(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for BooleanExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for BooleanExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for BooleanExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_abstract_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_sufficient_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for BooleanExpression {}
impl NamespaceStructRefMut for BooleanExpression {}
impl NamespaceStructRef for BooleanExpression {}
impl<'a> NamespaceStructRefMut for BooleanExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for BooleanExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for BooleanExpressionRef<'a> {}
impl ElementStruct for BooleanExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpression::Itself(x) => x.owned_relationship(),
            BooleanExpression::Invariant(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpression::Itself(x) => x.owning_relationship(),
            BooleanExpression::Invariant(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            BooleanExpression::Itself(x) => x.element_id(),
            BooleanExpression::Invariant(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            BooleanExpression::Itself(x) => x.alias_ids(),
            BooleanExpression::Invariant(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_short_name(),
            BooleanExpression::Invariant(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_name(),
            BooleanExpression::Invariant(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            BooleanExpression::Itself(x) => x.is_implied_included(),
            BooleanExpression::Invariant(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for BooleanExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpression::Itself(x) => x.owned_relationship_ref_mut(),
            BooleanExpression::Invariant(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            BooleanExpression::Itself(x) => x.owning_relationship_ref_mut(),
            BooleanExpression::Invariant(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            BooleanExpression::Itself(x) => x.element_id_ref_mut(),
            BooleanExpression::Invariant(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            BooleanExpression::Itself(x) => x.alias_ids_ref_mut(),
            BooleanExpression::Invariant(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_short_name_ref_mut(),
            BooleanExpression::Invariant(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_name_ref_mut(),
            BooleanExpression::Invariant(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpression::Itself(x) => x.is_implied_included_ref_mut(),
            BooleanExpression::Invariant(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for BooleanExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpression::Itself(x) => x.owned_relationship_ref(),
            BooleanExpression::Invariant(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpression::Itself(x) => x.owning_relationship_ref(),
            BooleanExpression::Invariant(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BooleanExpression::Itself(x) => x.element_id_ref(),
            BooleanExpression::Invariant(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BooleanExpression::Itself(x) => x.alias_ids_ref(),
            BooleanExpression::Invariant(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_short_name_ref(),
            BooleanExpression::Invariant(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpression::Itself(x) => x.declared_name_ref(),
            BooleanExpression::Invariant(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BooleanExpression::Itself(x) => x.is_implied_included_ref(),
            BooleanExpression::Invariant(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for BooleanExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            BooleanExpressionRefMut::Invariant(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for BooleanExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.element_id_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.declared_name_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BooleanExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            BooleanExpressionRefMut::Invariant(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for BooleanExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpressionRef::Itself(x) => x.owned_relationship_ref(),
            BooleanExpressionRef::Invariant(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BooleanExpressionRef::Itself(x) => x.owning_relationship_ref(),
            BooleanExpressionRef::Invariant(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BooleanExpressionRef::Itself(x) => x.element_id_ref(),
            BooleanExpressionRef::Invariant(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BooleanExpressionRef::Itself(x) => x.alias_ids_ref(),
            BooleanExpressionRef::Invariant(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpressionRef::Itself(x) => x.declared_short_name_ref(),
            BooleanExpressionRef::Invariant(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BooleanExpressionRef::Itself(x) => x.declared_name_ref(),
            BooleanExpressionRef::Invariant(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BooleanExpressionRef::Itself(x) => x.is_implied_included_ref(),
            BooleanExpressionRef::Invariant(x) => x.is_implied_included_ref(),
        }
    }
}
impl BooleanExpressionUpcast for BooleanExpression {
    fn into_boolean_expression(self) -> BooleanExpression {
        self
    }
}
impl<'a> BooleanExpressionUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_boolean_expression_ref_mut(self) -> BooleanExpressionRefMut<'a> {
        self
    }
}
impl<'a> BooleanExpressionUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_boolean_expression_ref(self) -> BooleanExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for BooleanExpression {
    fn into_expression(self) -> Expression {
        Expression::BooleanExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::BooleanExpression(self).as_expression_ref()
    }
}
impl StepUpcast for BooleanExpression {
    fn into_step(self) -> Step {
        Expression::BooleanExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::BooleanExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for BooleanExpression {
    fn into_feature(self) -> Feature {
        Expression::BooleanExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::BooleanExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for BooleanExpression {
    fn into_type_(self) -> Type {
        Expression::BooleanExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::BooleanExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for BooleanExpression {
    fn into_namespace(self) -> Namespace {
        Expression::BooleanExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::BooleanExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for BooleanExpression {
    fn into_element(self) -> Element {
        Expression::BooleanExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::BooleanExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for BooleanExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::BooleanExpression(self).as_element_ref()
    }
}
pub trait BooleanExpressionDowncast {
    fn try_into_invariant(self) -> Result<Invariant, String>;
}
pub trait BooleanExpressionDowncastRefMut<'a> {
    fn try_as_invariant_ref_mut(self) -> Result<InvariantRefMut<'a>, String>;
}
pub trait BooleanExpressionDowncastRef<'a> {
    fn try_as_invariant_ref(self) -> Result<InvariantRef<'a>, String>;
}
impl BooleanExpressionDowncast for BooleanExpression {
    fn try_into_invariant(self) -> Result<Invariant, String> {
        match self {
            BooleanExpression::Invariant(e) => Ok(e),
            _ => Err("Not a Invariant".into()),
        }
    }
}
impl<'a> BooleanExpressionDowncastRefMut<'a> for BooleanExpressionRefMut<'a> {
    fn try_as_invariant_ref_mut(self) -> Result<InvariantRefMut<'a>, String> {
        match self {
            BooleanExpressionRefMut::Invariant(e) => Ok(e),
            _ => Err("Not a Invariant".into()),
        }
    }
}
impl<'a> BooleanExpressionDowncastRef<'a> for BooleanExpressionRef<'a> {
    fn try_as_invariant_ref(self) -> Result<InvariantRef<'a>, String> {
        match self {
            BooleanExpressionRef::Invariant(e) => Ok(e),
            _ => Err("Not a Invariant".into()),
        }
    }
}
pub trait BooleanExpressionMethodsDescendants
where
    Self: DescendantOf<BooleanExpression>,
    Self::Via: BooleanExpressionMethods,
    Self: Sized,
{}
pub trait BooleanExpressionMethods: Sized {}
impl<T: BooleanExpressionMethodsDescendants> BooleanExpressionMethods for T
where
    T::Via: BooleanExpressionMethods,
{}
impl DescendantOf<Expression> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for BooleanExpression {}
impl DescendantOf<Step> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for BooleanExpression {}
impl DescendantOf<Feature> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for BooleanExpression {}
impl DescendantOf<Type> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for BooleanExpression {}
impl DescendantOf<Namespace> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for BooleanExpression {}
impl DescendantOf<Element> for BooleanExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for BooleanExpression {}
pub trait BooleanExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<BooleanExpressionRefMut<'a>>,
    Self::Via: BooleanExpressionRefMutMethods,
    Self: Sized,
{}
pub trait BooleanExpressionRefMutMethods: Sized {}
impl<'a, T: BooleanExpressionRefMutMethodsDescendants<'a>> BooleanExpressionRefMutMethods
for T
where
    T::Via: BooleanExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for BooleanExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for BooleanExpressionRefMut<'a> {}
pub trait BooleanExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<BooleanExpressionRef<'a>>,
    Self::Via: BooleanExpressionRefMethods,
    Self: Sized,
{}
pub trait BooleanExpressionRefMethods: Sized {}
impl<'a, T: BooleanExpressionRefMethodsDescendants<'a>> BooleanExpressionRefMethods for T
where
    T::Via: BooleanExpressionRefMethods,
{}
impl<'a> DescendantOf<ExpressionRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for BooleanExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for BooleanExpressionRef<'a> {}

