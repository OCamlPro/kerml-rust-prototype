#![allow(unused)]
use super::utils::DescendantOf;
use super::literal_expression::{
    LiteralExpression, LiteralExpressionRefMut, LiteralExpressionRef,
    LiteralExpressionStruct, LiteralExpressionStructRefMut, LiteralExpressionStructRef,
    LiteralExpressionInner, LiteralExpressionUpcast, LiteralExpressionUpcastRefMut,
    LiteralExpressionUpcastRef, LiteralExpressionMethodsDescendants,
    LiteralExpressionRefMutMethodsDescendants, LiteralExpressionRefMethodsDescendants,
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
pub struct LiteralInfinityInner {
    pub(super) sup_literal_expression: LiteralExpressionInner,
}
pub trait LiteralInfinityStruct
where
    Self: LiteralInfinityStructRefMut,
    Self: LiteralInfinityStructRef,
    Self: LiteralExpressionStruct,
{}
pub trait LiteralInfinityStructRefMut
where
    Self: LiteralInfinityStructRef,
    Self: LiteralExpressionStructRefMut,
{}
pub trait LiteralInfinityStructRef
where
    Self: LiteralExpressionStructRef,
{}
pub trait LiteralInfinityUpcast: LiteralInfinityStruct {
    fn into_literal_infinity(self) -> LiteralInfinity;
}
pub trait LiteralInfinityUpcastRefMut<'a>: LiteralInfinityStructRefMut {
    fn as_literal_infinity_ref_mut(self) -> LiteralInfinityRefMut<'a>;
}
pub trait LiteralInfinityUpcastRef<'a>: LiteralInfinityStructRef {
    fn as_literal_infinity_ref(self) -> LiteralInfinityRef<'a>;
}
impl LiteralInfinityStruct for LiteralInfinityInner {}
impl LiteralInfinityStructRefMut for LiteralInfinityInner {}
impl LiteralInfinityStructRef for LiteralInfinityInner {}
impl LiteralExpressionStruct for LiteralInfinityInner {}
impl LiteralExpressionStructRefMut for LiteralInfinityInner {}
impl LiteralExpressionStructRef for LiteralInfinityInner {}
impl ExpressionStruct for LiteralInfinityInner {}
impl ExpressionStructRefMut for LiteralInfinityInner {}
impl ExpressionStructRef for LiteralInfinityInner {}
impl StepStruct for LiteralInfinityInner {}
impl StepStructRefMut for LiteralInfinityInner {}
impl StepStructRef for LiteralInfinityInner {}
impl FeatureStruct for LiteralInfinityInner {
    fn is_unique(self) -> bool {
        self.sup_literal_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_literal_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_literal_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_literal_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_literal_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_literal_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_literal_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_literal_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction()
    }
}
impl FeatureStructRefMut for LiteralInfinityInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for LiteralInfinityInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_literal_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_literal_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_literal_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_literal_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_literal_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_literal_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_literal_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_literal_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction_ref()
    }
}
impl TypeStruct for LiteralInfinityInner {
    fn is_abstract(self) -> bool {
        self.sup_literal_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_literal_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralInfinityInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralInfinityInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_literal_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_literal_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralInfinityInner {}
impl NamespaceStructRefMut for LiteralInfinityInner {}
impl NamespaceStructRef for LiteralInfinityInner {}
impl ElementStruct for LiteralInfinityInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_literal_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_literal_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_literal_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_literal_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_literal_expression.is_implied_included()
    }
}
impl ElementStructRefMut for LiteralInfinityInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_literal_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_literal_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_literal_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_literal_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_literal_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for LiteralInfinityInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_literal_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_literal_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_literal_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_literal_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_literal_expression.is_implied_included_ref()
    }
}
pub enum LiteralInfinity {
    Itself(LiteralInfinityInner),
}
pub enum LiteralInfinityRefMut<'a> {
    Itself(&'a mut LiteralInfinityInner),
}
pub enum LiteralInfinityRef<'a> {
    Itself(&'a LiteralInfinityInner),
}
impl LiteralInfinity {
    pub fn as_ref(&self) -> LiteralInfinityRef {
        match self {
            LiteralInfinity::Itself(inner) => LiteralInfinityRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralInfinityRefMut {
        match self {
            LiteralInfinity::Itself(inner) => LiteralInfinityRefMut::Itself(inner),
        }
    }
}
impl<'a> LiteralInfinityRefMut<'a> {
    pub fn as_ref(self) -> LiteralInfinityRef<'a> {
        match self {
            LiteralInfinityRefMut::Itself(inner) => LiteralInfinityRef::Itself(inner),
        }
    }
}
impl LiteralInfinityStruct for LiteralInfinity {}
impl LiteralInfinityStructRefMut for LiteralInfinity {}
impl LiteralInfinityStructRef for LiteralInfinity {}
impl<'a> LiteralInfinityStructRefMut for LiteralInfinityRefMut<'a> {}
impl<'a> LiteralInfinityStructRef for LiteralInfinityRefMut<'a> {}
impl<'a> LiteralInfinityStructRef for LiteralInfinityRef<'a> {}
impl LiteralExpressionStruct for LiteralInfinity {}
impl LiteralExpressionStructRefMut for LiteralInfinity {}
impl LiteralExpressionStructRef for LiteralInfinity {}
impl<'a> LiteralExpressionStructRefMut for LiteralInfinityRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralInfinityRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralInfinityRef<'a> {}
impl ExpressionStruct for LiteralInfinity {}
impl ExpressionStructRefMut for LiteralInfinity {}
impl ExpressionStructRef for LiteralInfinity {}
impl<'a> ExpressionStructRefMut for LiteralInfinityRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralInfinityRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralInfinityRef<'a> {}
impl StepStruct for LiteralInfinity {}
impl StepStructRefMut for LiteralInfinity {}
impl StepStructRef for LiteralInfinity {}
impl<'a> StepStructRefMut for LiteralInfinityRefMut<'a> {}
impl<'a> StepStructRef for LiteralInfinityRefMut<'a> {}
impl<'a> StepStructRef for LiteralInfinityRef<'a> {}
impl FeatureStruct for LiteralInfinity {
    fn is_unique(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_constant(),
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
            LiteralInfinity::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralInfinity {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralInfinity::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralInfinity {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_constant_ref(),
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
            LiteralInfinity::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralInfinityRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralInfinityRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralInfinityRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_constant_ref(),
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
            LiteralInfinityRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralInfinityRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_constant_ref(),
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
            LiteralInfinityRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralInfinity {
    fn is_abstract(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralInfinity {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralInfinity {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralInfinityRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralInfinityRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralInfinityRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralInfinity {}
impl NamespaceStructRefMut for LiteralInfinity {}
impl NamespaceStructRef for LiteralInfinity {}
impl<'a> NamespaceStructRefMut for LiteralInfinityRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralInfinityRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralInfinityRef<'a> {}
impl ElementStruct for LiteralInfinity {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinity::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinity::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralInfinity::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralInfinity::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralInfinity {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinity::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralInfinity::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralInfinity::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralInfinity::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralInfinity {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinity::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinity::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralInfinity::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralInfinity::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinity::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralInfinity::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralInfinityRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LiteralInfinityRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralInfinityRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralInfinityRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinityRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInfinityRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralInfinityRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralInfinityRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinityRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInfinityRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralInfinityRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralInfinityUpcast for LiteralInfinity {
    fn into_literal_infinity(self) -> LiteralInfinity {
        self
    }
}
impl<'a> LiteralInfinityUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_literal_infinity_ref_mut(self) -> LiteralInfinityRefMut<'a> {
        self
    }
}
impl<'a> LiteralInfinityUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_literal_infinity_ref(self) -> LiteralInfinityRef<'a> {
        self
    }
}
impl LiteralExpressionUpcast for LiteralInfinity {
    fn into_literal_expression(self) -> LiteralExpression {
        LiteralExpression::LiteralInfinity(self).into_literal_expression()
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_literal_expression_ref()
    }
}
impl ExpressionUpcast for LiteralInfinity {
    fn into_expression(self) -> Expression {
        LiteralExpression::LiteralInfinity(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralInfinity {
    fn into_step(self) -> Step {
        LiteralExpression::LiteralInfinity(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralInfinity {
    fn into_feature(self) -> Feature {
        LiteralExpression::LiteralInfinity(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralInfinity {
    fn into_type_(self) -> Type {
        LiteralExpression::LiteralInfinity(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralInfinity {
    fn into_namespace(self) -> Namespace {
        LiteralExpression::LiteralInfinity(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralInfinity {
    fn into_element(self) -> Element {
        LiteralExpression::LiteralInfinity(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralInfinityRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        LiteralExpressionRefMut::LiteralInfinity(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralInfinityRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        LiteralExpressionRef::LiteralInfinity(self).as_element_ref()
    }
}
pub trait LiteralInfinityDowncast {}
pub trait LiteralInfinityDowncastRefMut<'a> {}
pub trait LiteralInfinityDowncastRef<'a> {}
impl LiteralInfinityDowncast for LiteralInfinity {}
impl<'a> LiteralInfinityDowncastRefMut<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> LiteralInfinityDowncastRef<'a> for LiteralInfinityRef<'a> {}
pub trait LiteralInfinityMethodsDescendants
where
    Self: DescendantOf<LiteralInfinity>,
    Self::Via: LiteralInfinityMethods,
    Self: Sized,
{}
pub trait LiteralInfinityMethods: Sized {}
impl<T: LiteralInfinityMethodsDescendants> LiteralInfinityMethods for T
where
    T::Via: LiteralInfinityMethods,
{}
impl DescendantOf<LiteralExpression> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl LiteralExpressionMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Expression> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Step> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl StepMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Feature> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl FeatureMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Type> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl TypeMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Namespace> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralInfinity {}
impl DescendantOf<Element> for LiteralInfinity {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ElementMethodsDescendants for LiteralInfinity {}
pub trait LiteralInfinityRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralInfinityRefMut<'a>>,
    Self::Via: LiteralInfinityRefMutMethods,
    Self: Sized,
{}
pub trait LiteralInfinityRefMutMethods: Sized {}
impl<'a, T: LiteralInfinityRefMutMethodsDescendants<'a>> LiteralInfinityRefMutMethods
for T
where
    T::Via: LiteralInfinityRefMutMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralInfinityRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralInfinityRefMut<'a> {}
pub trait LiteralInfinityRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralInfinityRef<'a>>,
    Self::Via: LiteralInfinityRefMethods,
    Self: Sized,
{}
pub trait LiteralInfinityRefMethods: Sized {}
impl<'a, T: LiteralInfinityRefMethodsDescendants<'a>> LiteralInfinityRefMethods for T
where
    T::Via: LiteralInfinityRefMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> LiteralExpressionRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralInfinityRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralInfinityRef<'a> {}

