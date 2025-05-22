#![allow(unused)]
use super::utils::DescendantOf;
use super::boolean_expression::{
    BooleanExpression, BooleanExpressionRefMut, BooleanExpressionRef,
    BooleanExpressionStruct, BooleanExpressionStructRefMut, BooleanExpressionStructRef,
    BooleanExpressionInner, BooleanExpressionUpcast, BooleanExpressionUpcastRefMut,
    BooleanExpressionUpcastRef, BooleanExpressionMethodsDescendants,
    BooleanExpressionRefMutMethodsDescendants, BooleanExpressionRefMethodsDescendants,
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
pub struct InvariantInner {
    pub(super) sup_boolean_expression: BooleanExpressionInner,
    pub(super) is_negated: bool,
}
pub trait InvariantStruct
where
    Self: InvariantStructRefMut,
    Self: InvariantStructRef,
    Self: BooleanExpressionStruct,
{
    fn is_negated(self) -> bool;
}
pub trait InvariantStructRefMut
where
    Self: InvariantStructRef,
    Self: BooleanExpressionStructRefMut,
{
    fn is_negated_ref_mut(&mut self) -> &mut bool;
}
pub trait InvariantStructRef
where
    Self: BooleanExpressionStructRef,
{
    fn is_negated_ref(&self) -> &bool;
}
pub trait InvariantUpcast: InvariantStruct {
    fn into_invariant(self) -> Invariant;
}
pub trait InvariantUpcastRefMut<'a>: InvariantStructRefMut {
    fn as_invariant_ref_mut(self) -> InvariantRefMut<'a>;
}
pub trait InvariantUpcastRef<'a>: InvariantStructRef {
    fn as_invariant_ref(self) -> InvariantRef<'a>;
}
impl InvariantStruct for InvariantInner {
    fn is_negated(self) -> bool {
        self.is_negated
    }
}
impl InvariantStructRefMut for InvariantInner {
    fn is_negated_ref_mut(&mut self) -> &mut bool {
        &mut self.is_negated
    }
}
impl InvariantStructRef for InvariantInner {
    fn is_negated_ref(&self) -> &bool {
        &self.is_negated
    }
}
impl BooleanExpressionStruct for InvariantInner {}
impl BooleanExpressionStructRefMut for InvariantInner {}
impl BooleanExpressionStructRef for InvariantInner {}
impl ExpressionStruct for InvariantInner {}
impl ExpressionStructRefMut for InvariantInner {}
impl ExpressionStructRef for InvariantInner {}
impl StepStruct for InvariantInner {}
impl StepStructRefMut for InvariantInner {}
impl StepStructRef for InvariantInner {}
impl FeatureStruct for InvariantInner {
    fn is_unique(self) -> bool {
        self.sup_boolean_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_boolean_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_boolean_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_boolean_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_boolean_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_boolean_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_boolean_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_boolean_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_boolean_expression.direction()
    }
}
impl FeatureStructRefMut for InvariantInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_boolean_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for InvariantInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_boolean_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_boolean_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_boolean_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_boolean_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_boolean_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_boolean_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_boolean_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_boolean_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_boolean_expression.direction_ref()
    }
}
impl TypeStruct for InvariantInner {
    fn is_abstract(self) -> bool {
        self.sup_boolean_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_boolean_expression.is_sufficient()
    }
}
impl TypeStructRefMut for InvariantInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for InvariantInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_boolean_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_boolean_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for InvariantInner {}
impl NamespaceStructRefMut for InvariantInner {}
impl NamespaceStructRef for InvariantInner {}
impl ElementStruct for InvariantInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_boolean_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_boolean_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_boolean_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_boolean_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_boolean_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_boolean_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_boolean_expression.is_implied_included()
    }
}
impl ElementStructRefMut for InvariantInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_boolean_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_boolean_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_boolean_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_boolean_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_boolean_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_boolean_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_boolean_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for InvariantInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_boolean_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_boolean_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_boolean_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_boolean_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_boolean_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_boolean_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_boolean_expression.is_implied_included_ref()
    }
}
pub enum Invariant {
    Itself(InvariantInner),
}
pub enum InvariantRefMut<'a> {
    Itself(&'a mut InvariantInner),
}
pub enum InvariantRef<'a> {
    Itself(&'a InvariantInner),
}
impl Invariant {
    pub fn as_ref(&self) -> InvariantRef {
        match self {
            Invariant::Itself(inner) => InvariantRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> InvariantRefMut {
        match self {
            Invariant::Itself(inner) => InvariantRefMut::Itself(inner),
        }
    }
}
impl<'a> InvariantRefMut<'a> {
    pub fn as_ref(self) -> InvariantRef<'a> {
        match self {
            InvariantRefMut::Itself(inner) => InvariantRef::Itself(inner),
        }
    }
}
impl InvariantStruct for Invariant {
    fn is_negated(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_negated(),
        }
    }
}
impl InvariantStructRefMut for Invariant {
    fn is_negated_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_negated_ref_mut(),
        }
    }
}
impl InvariantStructRef for Invariant {
    fn is_negated_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_negated_ref(),
        }
    }
}
impl<'a> InvariantStructRefMut for InvariantRefMut<'a> {
    fn is_negated_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_negated_ref_mut(),
        }
    }
}
impl<'a> InvariantStructRef for InvariantRefMut<'a> {
    fn is_negated_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_negated_ref(),
        }
    }
}
impl<'a> InvariantStructRef for InvariantRef<'a> {
    fn is_negated_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_negated_ref(),
        }
    }
}
impl BooleanExpressionStruct for Invariant {}
impl BooleanExpressionStructRefMut for Invariant {}
impl BooleanExpressionStructRef for Invariant {}
impl<'a> BooleanExpressionStructRefMut for InvariantRefMut<'a> {}
impl<'a> BooleanExpressionStructRef for InvariantRefMut<'a> {}
impl<'a> BooleanExpressionStructRef for InvariantRef<'a> {}
impl ExpressionStruct for Invariant {}
impl ExpressionStructRefMut for Invariant {}
impl ExpressionStructRef for Invariant {}
impl<'a> ExpressionStructRefMut for InvariantRefMut<'a> {}
impl<'a> ExpressionStructRef for InvariantRefMut<'a> {}
impl<'a> ExpressionStructRef for InvariantRef<'a> {}
impl StepStruct for Invariant {}
impl StepStructRefMut for Invariant {}
impl StepStructRef for Invariant {}
impl<'a> StepStructRefMut for InvariantRefMut<'a> {}
impl<'a> StepStructRef for InvariantRefMut<'a> {}
impl<'a> StepStructRef for InvariantRef<'a> {}
impl FeatureStruct for Invariant {
    fn is_unique(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_constant(),
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
            Invariant::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Invariant {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_constant_ref_mut(),
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
            Invariant::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Invariant {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_constant_ref(),
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
            Invariant::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for InvariantRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            InvariantRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for InvariantRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_constant_ref(),
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
            InvariantRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for InvariantRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_constant_ref(),
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
            InvariantRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Invariant {
    fn is_abstract(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Invariant {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Invariant {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for InvariantRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for InvariantRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for InvariantRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Invariant {}
impl NamespaceStructRefMut for Invariant {}
impl NamespaceStructRef for Invariant {}
impl<'a> NamespaceStructRefMut for InvariantRefMut<'a> {}
impl<'a> NamespaceStructRef for InvariantRefMut<'a> {}
impl<'a> NamespaceStructRef for InvariantRef<'a> {}
impl ElementStruct for Invariant {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Invariant::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Invariant::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Invariant::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Invariant::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Invariant::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Invariant {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Invariant::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Invariant::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Invariant::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Invariant::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Invariant::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Invariant {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Invariant::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Invariant::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Invariant::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Invariant::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Invariant::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Invariant::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for InvariantRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvariantRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            InvariantRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InvariantRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InvariantRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvariantRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InvariantRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for InvariantRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvariantRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvariantRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InvariantRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InvariantRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InvariantRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InvariantRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InvariantRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for InvariantRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvariantRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InvariantRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InvariantRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InvariantRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InvariantRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InvariantRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InvariantRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl InvariantUpcast for Invariant {
    fn into_invariant(self) -> Invariant {
        self
    }
}
impl<'a> InvariantUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_invariant_ref_mut(self) -> InvariantRefMut<'a> {
        self
    }
}
impl<'a> InvariantUpcastRef<'a> for InvariantRef<'a> {
    fn as_invariant_ref(self) -> InvariantRef<'a> {
        self
    }
}
impl BooleanExpressionUpcast for Invariant {
    fn into_boolean_expression(self) -> BooleanExpression {
        BooleanExpression::Invariant(self).into_boolean_expression()
    }
}
impl<'a> BooleanExpressionUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_boolean_expression_ref_mut(self) -> BooleanExpressionRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_boolean_expression_ref_mut()
    }
}
impl<'a> BooleanExpressionUpcastRef<'a> for InvariantRef<'a> {
    fn as_boolean_expression_ref(self) -> BooleanExpressionRef<'a> {
        BooleanExpressionRef::Invariant(self).as_boolean_expression_ref()
    }
}
impl ExpressionUpcast for Invariant {
    fn into_expression(self) -> Expression {
        BooleanExpression::Invariant(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for InvariantRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        BooleanExpressionRef::Invariant(self).as_expression_ref()
    }
}
impl StepUpcast for Invariant {
    fn into_step(self) -> Step {
        BooleanExpression::Invariant(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for InvariantRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        BooleanExpressionRef::Invariant(self).as_step_ref()
    }
}
impl FeatureUpcast for Invariant {
    fn into_feature(self) -> Feature {
        BooleanExpression::Invariant(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for InvariantRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        BooleanExpressionRef::Invariant(self).as_feature_ref()
    }
}
impl TypeUpcast for Invariant {
    fn into_type_(self) -> Type {
        BooleanExpression::Invariant(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for InvariantRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        BooleanExpressionRef::Invariant(self).as_type_ref()
    }
}
impl NamespaceUpcast for Invariant {
    fn into_namespace(self) -> Namespace {
        BooleanExpression::Invariant(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for InvariantRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        BooleanExpressionRef::Invariant(self).as_namespace_ref()
    }
}
impl ElementUpcast for Invariant {
    fn into_element(self) -> Element {
        BooleanExpression::Invariant(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for InvariantRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        BooleanExpressionRefMut::Invariant(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for InvariantRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        BooleanExpressionRef::Invariant(self).as_element_ref()
    }
}
pub trait InvariantDowncast {}
pub trait InvariantDowncastRefMut<'a> {}
pub trait InvariantDowncastRef<'a> {}
impl InvariantDowncast for Invariant {}
impl<'a> InvariantDowncastRefMut<'a> for InvariantRefMut<'a> {}
impl<'a> InvariantDowncastRef<'a> for InvariantRef<'a> {}
pub trait InvariantMethodsDescendants
where
    Self: DescendantOf<Invariant>,
    Self::Via: InvariantMethods,
    Self: Sized,
{}
pub trait InvariantMethods: Sized {}
impl<T: InvariantMethodsDescendants> InvariantMethods for T
where
    T::Via: InvariantMethods,
{}
impl DescendantOf<BooleanExpression> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl BooleanExpressionMethodsDescendants for Invariant {}
impl DescendantOf<Expression> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl ExpressionMethodsDescendants for Invariant {}
impl DescendantOf<Step> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl StepMethodsDescendants for Invariant {}
impl DescendantOf<Feature> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl FeatureMethodsDescendants for Invariant {}
impl DescendantOf<Type> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl TypeMethodsDescendants for Invariant {}
impl DescendantOf<Namespace> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl NamespaceMethodsDescendants for Invariant {}
impl DescendantOf<Element> for Invariant {
    type Via = BooleanExpression;
    fn into_via(self) -> Self::Via {
        self.into_boolean_expression()
    }
}
impl ElementMethodsDescendants for Invariant {}
pub trait InvariantRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<InvariantRefMut<'a>>,
    Self::Via: InvariantRefMutMethods,
    Self: Sized,
{}
pub trait InvariantRefMutMethods: Sized {}
impl<'a, T: InvariantRefMutMethodsDescendants<'a>> InvariantRefMutMethods for T
where
    T::Via: InvariantRefMutMethods,
{}
impl<'a> DescendantOf<BooleanExpressionRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> BooleanExpressionRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for InvariantRefMut<'a> {
    type Via = BooleanExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for InvariantRefMut<'a> {}
pub trait InvariantRefMethodsDescendants<'a>
where
    Self: DescendantOf<InvariantRef<'a>>,
    Self::Via: InvariantRefMethods,
    Self: Sized,
{}
pub trait InvariantRefMethods: Sized {}
impl<'a, T: InvariantRefMethodsDescendants<'a>> InvariantRefMethods for T
where
    T::Via: InvariantRefMethods,
{}
impl<'a> DescendantOf<BooleanExpressionRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> BooleanExpressionRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for InvariantRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for InvariantRef<'a> {
    type Via = BooleanExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_boolean_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for InvariantRef<'a> {}

