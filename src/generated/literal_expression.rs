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
use super::literal_infinity::{
    LiteralInfinity, LiteralInfinityRefMut, LiteralInfinityRef,
};
use super::literal_boolean::{LiteralBoolean, LiteralBooleanRefMut, LiteralBooleanRef};
use super::literal_string::{LiteralString, LiteralStringRefMut, LiteralStringRef};
use super::literal_integer::{LiteralInteger, LiteralIntegerRefMut, LiteralIntegerRef};
use super::literal_rational::{
    LiteralRational, LiteralRationalRefMut, LiteralRationalRef,
};
pub struct LiteralExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait LiteralExpressionStruct
where
    Self: LiteralExpressionStructRefMut,
    Self: LiteralExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait LiteralExpressionStructRefMut
where
    Self: LiteralExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait LiteralExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait LiteralExpressionUpcast: LiteralExpressionStruct {
    fn into_literal_expression(self) -> LiteralExpression;
}
pub trait LiteralExpressionUpcastRefMut<'a>: LiteralExpressionStructRefMut {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a>;
}
pub trait LiteralExpressionUpcastRef<'a>: LiteralExpressionStructRef {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a>;
}
impl LiteralExpressionStruct for LiteralExpressionInner {}
impl LiteralExpressionStructRefMut for LiteralExpressionInner {}
impl LiteralExpressionStructRef for LiteralExpressionInner {}
impl ExpressionStruct for LiteralExpressionInner {}
impl ExpressionStructRefMut for LiteralExpressionInner {}
impl ExpressionStructRef for LiteralExpressionInner {}
impl StepStruct for LiteralExpressionInner {}
impl StepStructRefMut for LiteralExpressionInner {}
impl StepStructRef for LiteralExpressionInner {}
impl FeatureStruct for LiteralExpressionInner {
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
impl FeatureStructRefMut for LiteralExpressionInner {
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
impl FeatureStructRef for LiteralExpressionInner {
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
impl TypeStruct for LiteralExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralExpressionInner {}
impl NamespaceStructRefMut for LiteralExpressionInner {}
impl NamespaceStructRef for LiteralExpressionInner {}
impl ElementStruct for LiteralExpressionInner {
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
impl ElementStructRefMut for LiteralExpressionInner {
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
impl ElementStructRef for LiteralExpressionInner {
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
pub enum LiteralExpression {
    Itself(LiteralExpressionInner),
    LiteralInfinity(LiteralInfinity),
    LiteralBoolean(LiteralBoolean),
    LiteralString(LiteralString),
    LiteralInteger(LiteralInteger),
    LiteralRational(LiteralRational),
}
pub enum LiteralExpressionRefMut<'a> {
    Itself(&'a mut LiteralExpressionInner),
    LiteralInfinity(LiteralInfinityRefMut<'a>),
    LiteralBoolean(LiteralBooleanRefMut<'a>),
    LiteralString(LiteralStringRefMut<'a>),
    LiteralInteger(LiteralIntegerRefMut<'a>),
    LiteralRational(LiteralRationalRefMut<'a>),
}
pub enum LiteralExpressionRef<'a> {
    Itself(&'a LiteralExpressionInner),
    LiteralInfinity(LiteralInfinityRef<'a>),
    LiteralBoolean(LiteralBooleanRef<'a>),
    LiteralString(LiteralStringRef<'a>),
    LiteralInteger(LiteralIntegerRef<'a>),
    LiteralRational(LiteralRationalRef<'a>),
}
impl LiteralExpression {
    pub fn as_ref(&self) -> LiteralExpressionRef {
        match self {
            LiteralExpression::Itself(inner) => LiteralExpressionRef::Itself(&inner),
            LiteralExpression::LiteralInfinity(inner) => {
                LiteralExpressionRef::LiteralInfinity(inner.as_ref())
            }
            LiteralExpression::LiteralBoolean(inner) => {
                LiteralExpressionRef::LiteralBoolean(inner.as_ref())
            }
            LiteralExpression::LiteralString(inner) => {
                LiteralExpressionRef::LiteralString(inner.as_ref())
            }
            LiteralExpression::LiteralInteger(inner) => {
                LiteralExpressionRef::LiteralInteger(inner.as_ref())
            }
            LiteralExpression::LiteralRational(inner) => {
                LiteralExpressionRef::LiteralRational(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralExpressionRefMut {
        match self {
            LiteralExpression::Itself(inner) => LiteralExpressionRefMut::Itself(inner),
            LiteralExpression::LiteralInfinity(inner) => {
                LiteralExpressionRefMut::LiteralInfinity(inner.as_ref_mut())
            }
            LiteralExpression::LiteralBoolean(inner) => {
                LiteralExpressionRefMut::LiteralBoolean(inner.as_ref_mut())
            }
            LiteralExpression::LiteralString(inner) => {
                LiteralExpressionRefMut::LiteralString(inner.as_ref_mut())
            }
            LiteralExpression::LiteralInteger(inner) => {
                LiteralExpressionRefMut::LiteralInteger(inner.as_ref_mut())
            }
            LiteralExpression::LiteralRational(inner) => {
                LiteralExpressionRefMut::LiteralRational(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> LiteralExpressionRefMut<'a> {
    pub fn as_ref(self) -> LiteralExpressionRef<'a> {
        match self {
            LiteralExpressionRefMut::Itself(inner) => LiteralExpressionRef::Itself(inner),
            LiteralExpressionRefMut::LiteralInfinity(inner) => {
                LiteralExpressionRef::LiteralInfinity(inner.as_ref())
            }
            LiteralExpressionRefMut::LiteralBoolean(inner) => {
                LiteralExpressionRef::LiteralBoolean(inner.as_ref())
            }
            LiteralExpressionRefMut::LiteralString(inner) => {
                LiteralExpressionRef::LiteralString(inner.as_ref())
            }
            LiteralExpressionRefMut::LiteralInteger(inner) => {
                LiteralExpressionRef::LiteralInteger(inner.as_ref())
            }
            LiteralExpressionRefMut::LiteralRational(inner) => {
                LiteralExpressionRef::LiteralRational(inner.as_ref())
            }
        }
    }
}
impl LiteralExpressionStruct for LiteralExpression {}
impl LiteralExpressionStructRefMut for LiteralExpression {}
impl LiteralExpressionStructRef for LiteralExpression {}
impl<'a> LiteralExpressionStructRefMut for LiteralExpressionRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralExpressionRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralExpressionRef<'a> {}
impl ExpressionStruct for LiteralExpression {}
impl ExpressionStructRefMut for LiteralExpression {}
impl ExpressionStructRef for LiteralExpression {}
impl<'a> ExpressionStructRefMut for LiteralExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralExpressionRef<'a> {}
impl StepStruct for LiteralExpression {}
impl StepStructRefMut for LiteralExpression {}
impl StepStructRef for LiteralExpression {}
impl<'a> StepStructRefMut for LiteralExpressionRefMut<'a> {}
impl<'a> StepStructRef for LiteralExpressionRefMut<'a> {}
impl<'a> StepStructRef for LiteralExpressionRef<'a> {}
impl FeatureStruct for LiteralExpression {
    fn is_unique(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_unique(),
            LiteralExpression::LiteralInfinity(x) => x.is_unique(),
            LiteralExpression::LiteralBoolean(x) => x.is_unique(),
            LiteralExpression::LiteralString(x) => x.is_unique(),
            LiteralExpression::LiteralInteger(x) => x.is_unique(),
            LiteralExpression::LiteralRational(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_ordered(),
            LiteralExpression::LiteralInfinity(x) => x.is_ordered(),
            LiteralExpression::LiteralBoolean(x) => x.is_ordered(),
            LiteralExpression::LiteralString(x) => x.is_ordered(),
            LiteralExpression::LiteralInteger(x) => x.is_ordered(),
            LiteralExpression::LiteralRational(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_composite(),
            LiteralExpression::LiteralInfinity(x) => x.is_composite(),
            LiteralExpression::LiteralBoolean(x) => x.is_composite(),
            LiteralExpression::LiteralString(x) => x.is_composite(),
            LiteralExpression::LiteralInteger(x) => x.is_composite(),
            LiteralExpression::LiteralRational(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_end(),
            LiteralExpression::LiteralInfinity(x) => x.is_end(),
            LiteralExpression::LiteralBoolean(x) => x.is_end(),
            LiteralExpression::LiteralString(x) => x.is_end(),
            LiteralExpression::LiteralInteger(x) => x.is_end(),
            LiteralExpression::LiteralRational(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_derived(),
            LiteralExpression::LiteralInfinity(x) => x.is_derived(),
            LiteralExpression::LiteralBoolean(x) => x.is_derived(),
            LiteralExpression::LiteralString(x) => x.is_derived(),
            LiteralExpression::LiteralInteger(x) => x.is_derived(),
            LiteralExpression::LiteralRational(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_portion(),
            LiteralExpression::LiteralInfinity(x) => x.is_portion(),
            LiteralExpression::LiteralBoolean(x) => x.is_portion(),
            LiteralExpression::LiteralString(x) => x.is_portion(),
            LiteralExpression::LiteralInteger(x) => x.is_portion(),
            LiteralExpression::LiteralRational(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_variable(),
            LiteralExpression::LiteralInfinity(x) => x.is_variable(),
            LiteralExpression::LiteralBoolean(x) => x.is_variable(),
            LiteralExpression::LiteralString(x) => x.is_variable(),
            LiteralExpression::LiteralInteger(x) => x.is_variable(),
            LiteralExpression::LiteralRational(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_constant(),
            LiteralExpression::LiteralInfinity(x) => x.is_constant(),
            LiteralExpression::LiteralBoolean(x) => x.is_constant(),
            LiteralExpression::LiteralString(x) => x.is_constant(),
            LiteralExpression::LiteralInteger(x) => x.is_constant(),
            LiteralExpression::LiteralRational(x) => x.is_constant(),
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
            LiteralExpression::Itself(x) => x.direction(),
            LiteralExpression::LiteralInfinity(x) => x.direction(),
            LiteralExpression::LiteralBoolean(x) => x.direction(),
            LiteralExpression::LiteralString(x) => x.direction(),
            LiteralExpression::LiteralInteger(x) => x.direction(),
            LiteralExpression::LiteralRational(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_unique_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_unique_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_unique_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_unique_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_unique_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_ordered_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_ordered_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_ordered_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_ordered_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_ordered_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_composite_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_composite_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_composite_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_composite_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_composite_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_end_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_end_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_end_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_end_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_end_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_derived_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_derived_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_derived_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_derived_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_derived_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_portion_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_portion_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_portion_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_portion_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_portion_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_variable_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_variable_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_variable_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_variable_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_variable_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_constant_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_constant_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_constant_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_constant_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_constant_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_constant_ref_mut(),
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
            LiteralExpression::Itself(x) => x.direction_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.direction_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.direction_ref_mut(),
            LiteralExpression::LiteralString(x) => x.direction_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.direction_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_unique_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_unique_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_unique_ref(),
            LiteralExpression::LiteralString(x) => x.is_unique_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_unique_ref(),
            LiteralExpression::LiteralRational(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_ordered_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_ordered_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_ordered_ref(),
            LiteralExpression::LiteralString(x) => x.is_ordered_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_ordered_ref(),
            LiteralExpression::LiteralRational(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_composite_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_composite_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_composite_ref(),
            LiteralExpression::LiteralString(x) => x.is_composite_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_composite_ref(),
            LiteralExpression::LiteralRational(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_end_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_end_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_end_ref(),
            LiteralExpression::LiteralString(x) => x.is_end_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_end_ref(),
            LiteralExpression::LiteralRational(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_derived_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_derived_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_derived_ref(),
            LiteralExpression::LiteralString(x) => x.is_derived_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_derived_ref(),
            LiteralExpression::LiteralRational(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_portion_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_portion_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_portion_ref(),
            LiteralExpression::LiteralString(x) => x.is_portion_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_portion_ref(),
            LiteralExpression::LiteralRational(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_variable_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_variable_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_variable_ref(),
            LiteralExpression::LiteralString(x) => x.is_variable_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_variable_ref(),
            LiteralExpression::LiteralRational(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_constant_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_constant_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_constant_ref(),
            LiteralExpression::LiteralString(x) => x.is_constant_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_constant_ref(),
            LiteralExpression::LiteralRational(x) => x.is_constant_ref(),
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
            LiteralExpression::Itself(x) => x.direction_ref(),
            LiteralExpression::LiteralInfinity(x) => x.direction_ref(),
            LiteralExpression::LiteralBoolean(x) => x.direction_ref(),
            LiteralExpression::LiteralString(x) => x.direction_ref(),
            LiteralExpression::LiteralInteger(x) => x.direction_ref(),
            LiteralExpression::LiteralRational(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_unique_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_unique_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_unique_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_unique_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_ordered_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_ordered_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_ordered_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_ordered_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_composite_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_composite_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_composite_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_composite_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_end_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_end_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_end_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_end_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_derived_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_derived_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_derived_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_derived_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_portion_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_portion_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_portion_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_portion_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_variable_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_variable_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_variable_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_variable_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_constant_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_constant_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_constant_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_constant_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_constant_ref_mut(),
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
            LiteralExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.direction_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.direction_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.direction_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.direction_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_unique_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_unique_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_unique_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_unique_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_unique_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_ordered_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_ordered_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_ordered_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_ordered_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_composite_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_composite_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_composite_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_composite_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_composite_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_end_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_end_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_end_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_end_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_end_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_derived_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_derived_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_derived_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_derived_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_derived_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_portion_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_portion_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_portion_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_portion_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_portion_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_variable_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_variable_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_variable_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_variable_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_variable_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_constant_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_constant_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_constant_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_constant_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_constant_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_constant_ref(),
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
            LiteralExpressionRefMut::Itself(x) => x.direction_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.direction_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.direction_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.direction_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.direction_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_unique_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_unique_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_unique_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_unique_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_unique_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_ordered_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_ordered_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_ordered_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_ordered_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_ordered_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_composite_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_composite_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_composite_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_composite_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_composite_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_end_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_end_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_end_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_end_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_end_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_derived_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_derived_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_derived_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_derived_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_derived_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_portion_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_portion_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_portion_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_portion_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_portion_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_variable_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_variable_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_variable_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_variable_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_variable_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_constant_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_constant_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_constant_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_constant_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_constant_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_constant_ref(),
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
            LiteralExpressionRef::Itself(x) => x.direction_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.direction_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.direction_ref(),
            LiteralExpressionRef::LiteralString(x) => x.direction_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.direction_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralExpression {
    fn is_abstract(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_abstract(),
            LiteralExpression::LiteralInfinity(x) => x.is_abstract(),
            LiteralExpression::LiteralBoolean(x) => x.is_abstract(),
            LiteralExpression::LiteralString(x) => x.is_abstract(),
            LiteralExpression::LiteralInteger(x) => x.is_abstract(),
            LiteralExpression::LiteralRational(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_sufficient(),
            LiteralExpression::LiteralInfinity(x) => x.is_sufficient(),
            LiteralExpression::LiteralBoolean(x) => x.is_sufficient(),
            LiteralExpression::LiteralString(x) => x.is_sufficient(),
            LiteralExpression::LiteralInteger(x) => x.is_sufficient(),
            LiteralExpression::LiteralRational(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_abstract_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_abstract_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_abstract_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_abstract_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_abstract_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_sufficient_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_sufficient_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_sufficient_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_sufficient_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_sufficient_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_abstract_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_abstract_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_abstract_ref(),
            LiteralExpression::LiteralString(x) => x.is_abstract_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_abstract_ref(),
            LiteralExpression::LiteralRational(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_sufficient_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_sufficient_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_sufficient_ref(),
            LiteralExpression::LiteralString(x) => x.is_sufficient_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_sufficient_ref(),
            LiteralExpression::LiteralRational(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_abstract_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_abstract_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_abstract_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_abstract_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_sufficient_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_sufficient_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_sufficient_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_sufficient_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_abstract_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_abstract_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_abstract_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_abstract_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_sufficient_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_sufficient_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_sufficient_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_sufficient_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_abstract_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_abstract_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_abstract_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_abstract_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_abstract_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_sufficient_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_sufficient_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_sufficient_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_sufficient_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_sufficient_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralExpression {}
impl NamespaceStructRefMut for LiteralExpression {}
impl NamespaceStructRef for LiteralExpression {}
impl<'a> NamespaceStructRefMut for LiteralExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralExpressionRef<'a> {}
impl ElementStruct for LiteralExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpression::Itself(x) => x.owned_relationship(),
            LiteralExpression::LiteralInfinity(x) => x.owned_relationship(),
            LiteralExpression::LiteralBoolean(x) => x.owned_relationship(),
            LiteralExpression::LiteralString(x) => x.owned_relationship(),
            LiteralExpression::LiteralInteger(x) => x.owned_relationship(),
            LiteralExpression::LiteralRational(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpression::Itself(x) => x.owning_relationship(),
            LiteralExpression::LiteralInfinity(x) => x.owning_relationship(),
            LiteralExpression::LiteralBoolean(x) => x.owning_relationship(),
            LiteralExpression::LiteralString(x) => x.owning_relationship(),
            LiteralExpression::LiteralInteger(x) => x.owning_relationship(),
            LiteralExpression::LiteralRational(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralExpression::Itself(x) => x.element_id(),
            LiteralExpression::LiteralInfinity(x) => x.element_id(),
            LiteralExpression::LiteralBoolean(x) => x.element_id(),
            LiteralExpression::LiteralString(x) => x.element_id(),
            LiteralExpression::LiteralInteger(x) => x.element_id(),
            LiteralExpression::LiteralRational(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralExpression::Itself(x) => x.alias_ids(),
            LiteralExpression::LiteralInfinity(x) => x.alias_ids(),
            LiteralExpression::LiteralBoolean(x) => x.alias_ids(),
            LiteralExpression::LiteralString(x) => x.alias_ids(),
            LiteralExpression::LiteralInteger(x) => x.alias_ids(),
            LiteralExpression::LiteralRational(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_short_name(),
            LiteralExpression::LiteralInfinity(x) => x.declared_short_name(),
            LiteralExpression::LiteralBoolean(x) => x.declared_short_name(),
            LiteralExpression::LiteralString(x) => x.declared_short_name(),
            LiteralExpression::LiteralInteger(x) => x.declared_short_name(),
            LiteralExpression::LiteralRational(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_name(),
            LiteralExpression::LiteralInfinity(x) => x.declared_name(),
            LiteralExpression::LiteralBoolean(x) => x.declared_name(),
            LiteralExpression::LiteralString(x) => x.declared_name(),
            LiteralExpression::LiteralInteger(x) => x.declared_name(),
            LiteralExpression::LiteralRational(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralExpression::Itself(x) => x.is_implied_included(),
            LiteralExpression::LiteralInfinity(x) => x.is_implied_included(),
            LiteralExpression::LiteralBoolean(x) => x.is_implied_included(),
            LiteralExpression::LiteralString(x) => x.is_implied_included(),
            LiteralExpression::LiteralInteger(x) => x.is_implied_included(),
            LiteralExpression::LiteralRational(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpression::Itself(x) => x.owned_relationship_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.owned_relationship_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.owned_relationship_ref_mut(),
            LiteralExpression::LiteralString(x) => x.owned_relationship_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.owned_relationship_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralExpression::Itself(x) => x.owning_relationship_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.owning_relationship_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.owning_relationship_ref_mut(),
            LiteralExpression::LiteralString(x) => x.owning_relationship_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.owning_relationship_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralExpression::Itself(x) => x.element_id_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.element_id_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.element_id_ref_mut(),
            LiteralExpression::LiteralString(x) => x.element_id_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.element_id_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralExpression::Itself(x) => x.alias_ids_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.alias_ids_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.alias_ids_ref_mut(),
            LiteralExpression::LiteralString(x) => x.alias_ids_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.alias_ids_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_short_name_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.declared_short_name_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.declared_short_name_ref_mut(),
            LiteralExpression::LiteralString(x) => x.declared_short_name_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.declared_short_name_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_name_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.declared_name_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.declared_name_ref_mut(),
            LiteralExpression::LiteralString(x) => x.declared_name_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.declared_name_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpression::Itself(x) => x.is_implied_included_ref_mut(),
            LiteralExpression::LiteralInfinity(x) => x.is_implied_included_ref_mut(),
            LiteralExpression::LiteralBoolean(x) => x.is_implied_included_ref_mut(),
            LiteralExpression::LiteralString(x) => x.is_implied_included_ref_mut(),
            LiteralExpression::LiteralInteger(x) => x.is_implied_included_ref_mut(),
            LiteralExpression::LiteralRational(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpression::Itself(x) => x.owned_relationship_ref(),
            LiteralExpression::LiteralInfinity(x) => x.owned_relationship_ref(),
            LiteralExpression::LiteralBoolean(x) => x.owned_relationship_ref(),
            LiteralExpression::LiteralString(x) => x.owned_relationship_ref(),
            LiteralExpression::LiteralInteger(x) => x.owned_relationship_ref(),
            LiteralExpression::LiteralRational(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpression::Itself(x) => x.owning_relationship_ref(),
            LiteralExpression::LiteralInfinity(x) => x.owning_relationship_ref(),
            LiteralExpression::LiteralBoolean(x) => x.owning_relationship_ref(),
            LiteralExpression::LiteralString(x) => x.owning_relationship_ref(),
            LiteralExpression::LiteralInteger(x) => x.owning_relationship_ref(),
            LiteralExpression::LiteralRational(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralExpression::Itself(x) => x.element_id_ref(),
            LiteralExpression::LiteralInfinity(x) => x.element_id_ref(),
            LiteralExpression::LiteralBoolean(x) => x.element_id_ref(),
            LiteralExpression::LiteralString(x) => x.element_id_ref(),
            LiteralExpression::LiteralInteger(x) => x.element_id_ref(),
            LiteralExpression::LiteralRational(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralExpression::Itself(x) => x.alias_ids_ref(),
            LiteralExpression::LiteralInfinity(x) => x.alias_ids_ref(),
            LiteralExpression::LiteralBoolean(x) => x.alias_ids_ref(),
            LiteralExpression::LiteralString(x) => x.alias_ids_ref(),
            LiteralExpression::LiteralInteger(x) => x.alias_ids_ref(),
            LiteralExpression::LiteralRational(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_short_name_ref(),
            LiteralExpression::LiteralInfinity(x) => x.declared_short_name_ref(),
            LiteralExpression::LiteralBoolean(x) => x.declared_short_name_ref(),
            LiteralExpression::LiteralString(x) => x.declared_short_name_ref(),
            LiteralExpression::LiteralInteger(x) => x.declared_short_name_ref(),
            LiteralExpression::LiteralRational(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpression::Itself(x) => x.declared_name_ref(),
            LiteralExpression::LiteralInfinity(x) => x.declared_name_ref(),
            LiteralExpression::LiteralBoolean(x) => x.declared_name_ref(),
            LiteralExpression::LiteralString(x) => x.declared_name_ref(),
            LiteralExpression::LiteralInteger(x) => x.declared_name_ref(),
            LiteralExpression::LiteralRational(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralExpression::Itself(x) => x.is_implied_included_ref(),
            LiteralExpression::LiteralInfinity(x) => x.is_implied_included_ref(),
            LiteralExpression::LiteralBoolean(x) => x.is_implied_included_ref(),
            LiteralExpression::LiteralString(x) => x.is_implied_included_ref(),
            LiteralExpression::LiteralInteger(x) => x.is_implied_included_ref(),
            LiteralExpression::LiteralRational(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.owned_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.owned_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.owned_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.owned_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => {
                x.owning_relationship_ref_mut()
            }
            LiteralExpressionRefMut::LiteralBoolean(x) => x.owning_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.owning_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.owning_relationship_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.element_id_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.element_id_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.element_id_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.element_id_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.alias_ids_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.alias_ids_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.alias_ids_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.alias_ids_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => {
                x.declared_short_name_ref_mut()
            }
            LiteralExpressionRefMut::LiteralBoolean(x) => x.declared_short_name_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.declared_short_name_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.declared_short_name_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.declared_name_ref_mut(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.declared_name_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.declared_name_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.declared_name_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            LiteralExpressionRefMut::LiteralInfinity(x) => {
                x.is_implied_included_ref_mut()
            }
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_implied_included_ref_mut(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_implied_included_ref_mut(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_implied_included_ref_mut(),
            LiteralExpressionRefMut::LiteralRational(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for LiteralExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.owned_relationship_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.owned_relationship_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.owned_relationship_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.owned_relationship_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.owning_relationship_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.owning_relationship_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.owning_relationship_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.owning_relationship_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.element_id_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.element_id_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.element_id_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.element_id_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.element_id_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.alias_ids_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.alias_ids_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.alias_ids_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.alias_ids_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.declared_short_name_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.declared_short_name_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.declared_short_name_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.declared_short_name_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.declared_name_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.declared_name_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.declared_name_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.declared_name_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.declared_name_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            LiteralExpressionRefMut::LiteralInfinity(x) => x.is_implied_included_ref(),
            LiteralExpressionRefMut::LiteralBoolean(x) => x.is_implied_included_ref(),
            LiteralExpressionRefMut::LiteralString(x) => x.is_implied_included_ref(),
            LiteralExpressionRefMut::LiteralInteger(x) => x.is_implied_included_ref(),
            LiteralExpressionRefMut::LiteralRational(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpressionRef::Itself(x) => x.owned_relationship_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.owned_relationship_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.owned_relationship_ref(),
            LiteralExpressionRef::LiteralString(x) => x.owned_relationship_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.owned_relationship_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralExpressionRef::Itself(x) => x.owning_relationship_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.owning_relationship_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.owning_relationship_ref(),
            LiteralExpressionRef::LiteralString(x) => x.owning_relationship_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.owning_relationship_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralExpressionRef::Itself(x) => x.element_id_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.element_id_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.element_id_ref(),
            LiteralExpressionRef::LiteralString(x) => x.element_id_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.element_id_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralExpressionRef::Itself(x) => x.alias_ids_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.alias_ids_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.alias_ids_ref(),
            LiteralExpressionRef::LiteralString(x) => x.alias_ids_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.alias_ids_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpressionRef::Itself(x) => x.declared_short_name_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.declared_short_name_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.declared_short_name_ref(),
            LiteralExpressionRef::LiteralString(x) => x.declared_short_name_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.declared_short_name_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralExpressionRef::Itself(x) => x.declared_name_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.declared_name_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.declared_name_ref(),
            LiteralExpressionRef::LiteralString(x) => x.declared_name_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.declared_name_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralExpressionRef::Itself(x) => x.is_implied_included_ref(),
            LiteralExpressionRef::LiteralInfinity(x) => x.is_implied_included_ref(),
            LiteralExpressionRef::LiteralBoolean(x) => x.is_implied_included_ref(),
            LiteralExpressionRef::LiteralString(x) => x.is_implied_included_ref(),
            LiteralExpressionRef::LiteralInteger(x) => x.is_implied_included_ref(),
            LiteralExpressionRef::LiteralRational(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralExpressionUpcast for LiteralExpression {
    fn into_literal_expression(self) -> LiteralExpression {
        self
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        self
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for LiteralExpression {
    fn into_expression(self) -> Expression {
        Expression::LiteralExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::LiteralExpression(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralExpression {
    fn into_step(self) -> Step {
        Expression::LiteralExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::LiteralExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralExpression {
    fn into_feature(self) -> Feature {
        Expression::LiteralExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::LiteralExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralExpression {
    fn into_type_(self) -> Type {
        Expression::LiteralExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::LiteralExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralExpression {
    fn into_namespace(self) -> Namespace {
        Expression::LiteralExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::LiteralExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralExpression {
    fn into_element(self) -> Element {
        Expression::LiteralExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::LiteralExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::LiteralExpression(self).as_element_ref()
    }
}
pub trait LiteralExpressionDowncast {
    fn try_into_literal_infinity(self) -> Result<LiteralInfinity, String>;
    fn try_into_literal_boolean(self) -> Result<LiteralBoolean, String>;
    fn try_into_literal_string(self) -> Result<LiteralString, String>;
    fn try_into_literal_integer(self) -> Result<LiteralInteger, String>;
    fn try_into_literal_rational(self) -> Result<LiteralRational, String>;
}
pub trait LiteralExpressionDowncastRefMut<'a> {
    fn try_as_literal_infinity_ref_mut(
        self,
    ) -> Result<LiteralInfinityRefMut<'a>, String>;
    fn try_as_literal_boolean_ref_mut(self) -> Result<LiteralBooleanRefMut<'a>, String>;
    fn try_as_literal_string_ref_mut(self) -> Result<LiteralStringRefMut<'a>, String>;
    fn try_as_literal_integer_ref_mut(self) -> Result<LiteralIntegerRefMut<'a>, String>;
    fn try_as_literal_rational_ref_mut(
        self,
    ) -> Result<LiteralRationalRefMut<'a>, String>;
}
pub trait LiteralExpressionDowncastRef<'a> {
    fn try_as_literal_infinity_ref(self) -> Result<LiteralInfinityRef<'a>, String>;
    fn try_as_literal_boolean_ref(self) -> Result<LiteralBooleanRef<'a>, String>;
    fn try_as_literal_string_ref(self) -> Result<LiteralStringRef<'a>, String>;
    fn try_as_literal_integer_ref(self) -> Result<LiteralIntegerRef<'a>, String>;
    fn try_as_literal_rational_ref(self) -> Result<LiteralRationalRef<'a>, String>;
}
impl LiteralExpressionDowncast for LiteralExpression {
    fn try_into_literal_infinity(self) -> Result<LiteralInfinity, String> {
        match self {
            LiteralExpression::LiteralInfinity(e) => Ok(e),
            _ => Err("Not a LiteralInfinity".into()),
        }
    }
    fn try_into_literal_boolean(self) -> Result<LiteralBoolean, String> {
        match self {
            LiteralExpression::LiteralBoolean(e) => Ok(e),
            _ => Err("Not a LiteralBoolean".into()),
        }
    }
    fn try_into_literal_string(self) -> Result<LiteralString, String> {
        match self {
            LiteralExpression::LiteralString(e) => Ok(e),
            _ => Err("Not a LiteralString".into()),
        }
    }
    fn try_into_literal_integer(self) -> Result<LiteralInteger, String> {
        match self {
            LiteralExpression::LiteralInteger(e) => Ok(e),
            _ => Err("Not a LiteralInteger".into()),
        }
    }
    fn try_into_literal_rational(self) -> Result<LiteralRational, String> {
        match self {
            LiteralExpression::LiteralRational(e) => Ok(e),
            _ => Err("Not a LiteralRational".into()),
        }
    }
}
impl<'a> LiteralExpressionDowncastRefMut<'a> for LiteralExpressionRefMut<'a> {
    fn try_as_literal_infinity_ref_mut(
        self,
    ) -> Result<LiteralInfinityRefMut<'a>, String> {
        match self {
            LiteralExpressionRefMut::LiteralInfinity(e) => Ok(e),
            _ => Err("Not a LiteralInfinity".into()),
        }
    }
    fn try_as_literal_boolean_ref_mut(self) -> Result<LiteralBooleanRefMut<'a>, String> {
        match self {
            LiteralExpressionRefMut::LiteralBoolean(e) => Ok(e),
            _ => Err("Not a LiteralBoolean".into()),
        }
    }
    fn try_as_literal_string_ref_mut(self) -> Result<LiteralStringRefMut<'a>, String> {
        match self {
            LiteralExpressionRefMut::LiteralString(e) => Ok(e),
            _ => Err("Not a LiteralString".into()),
        }
    }
    fn try_as_literal_integer_ref_mut(self) -> Result<LiteralIntegerRefMut<'a>, String> {
        match self {
            LiteralExpressionRefMut::LiteralInteger(e) => Ok(e),
            _ => Err("Not a LiteralInteger".into()),
        }
    }
    fn try_as_literal_rational_ref_mut(
        self,
    ) -> Result<LiteralRationalRefMut<'a>, String> {
        match self {
            LiteralExpressionRefMut::LiteralRational(e) => Ok(e),
            _ => Err("Not a LiteralRational".into()),
        }
    }
}
impl<'a> LiteralExpressionDowncastRef<'a> for LiteralExpressionRef<'a> {
    fn try_as_literal_infinity_ref(self) -> Result<LiteralInfinityRef<'a>, String> {
        match self {
            LiteralExpressionRef::LiteralInfinity(e) => Ok(e),
            _ => Err("Not a LiteralInfinity".into()),
        }
    }
    fn try_as_literal_boolean_ref(self) -> Result<LiteralBooleanRef<'a>, String> {
        match self {
            LiteralExpressionRef::LiteralBoolean(e) => Ok(e),
            _ => Err("Not a LiteralBoolean".into()),
        }
    }
    fn try_as_literal_string_ref(self) -> Result<LiteralStringRef<'a>, String> {
        match self {
            LiteralExpressionRef::LiteralString(e) => Ok(e),
            _ => Err("Not a LiteralString".into()),
        }
    }
    fn try_as_literal_integer_ref(self) -> Result<LiteralIntegerRef<'a>, String> {
        match self {
            LiteralExpressionRef::LiteralInteger(e) => Ok(e),
            _ => Err("Not a LiteralInteger".into()),
        }
    }
    fn try_as_literal_rational_ref(self) -> Result<LiteralRationalRef<'a>, String> {
        match self {
            LiteralExpressionRef::LiteralRational(e) => Ok(e),
            _ => Err("Not a LiteralRational".into()),
        }
    }
}
pub trait LiteralExpressionMethodsDescendants
where
    Self: DescendantOf<LiteralExpression>,
    Self::Via: LiteralExpressionMethods,
    Self: Sized,
{}
pub trait LiteralExpressionMethods: Sized {}
impl<T: LiteralExpressionMethodsDescendants> LiteralExpressionMethods for T
where
    T::Via: LiteralExpressionMethods,
{}
impl DescendantOf<Expression> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralExpression {}
impl DescendantOf<Step> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for LiteralExpression {}
impl DescendantOf<Feature> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for LiteralExpression {}
impl DescendantOf<Type> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for LiteralExpression {}
impl DescendantOf<Namespace> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralExpression {}
impl DescendantOf<Element> for LiteralExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for LiteralExpression {}
pub trait LiteralExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralExpressionRefMut<'a>>,
    Self::Via: LiteralExpressionRefMutMethods,
    Self: Sized,
{}
pub trait LiteralExpressionRefMutMethods: Sized {}
impl<'a, T: LiteralExpressionRefMutMethodsDescendants<'a>> LiteralExpressionRefMutMethods
for T
where
    T::Via: LiteralExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralExpressionRefMut<'a> {}
pub trait LiteralExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralExpressionRef<'a>>,
    Self::Via: LiteralExpressionRefMethods,
    Self: Sized,
{}
pub trait LiteralExpressionRefMethods: Sized {}
impl<'a, T: LiteralExpressionRefMethodsDescendants<'a>> LiteralExpressionRefMethods for T
where
    T::Via: LiteralExpressionRefMethods,
{}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralExpressionRef<'a> {}

