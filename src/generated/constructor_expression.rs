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
pub struct ConstructorExpressionInner {
    pub(super) sup_instantiation_expression: InstantiationExpressionInner,
}
pub trait ConstructorExpressionStruct
where
    Self: ConstructorExpressionStructRefMut,
    Self: ConstructorExpressionStructRef,
    Self: InstantiationExpressionStruct,
{}
pub trait ConstructorExpressionStructRefMut
where
    Self: ConstructorExpressionStructRef,
    Self: InstantiationExpressionStructRefMut,
{}
pub trait ConstructorExpressionStructRef
where
    Self: InstantiationExpressionStructRef,
{}
pub trait ConstructorExpressionUpcast: ConstructorExpressionStruct {
    fn into_constructor_expression(self) -> ConstructorExpression;
}
pub trait ConstructorExpressionUpcastRefMut<'a>: ConstructorExpressionStructRefMut {
    fn as_constructor_expression_ref_mut(self) -> ConstructorExpressionRefMut<'a>;
}
pub trait ConstructorExpressionUpcastRef<'a>: ConstructorExpressionStructRef {
    fn as_constructor_expression_ref(self) -> ConstructorExpressionRef<'a>;
}
impl ConstructorExpressionStruct for ConstructorExpressionInner {}
impl ConstructorExpressionStructRefMut for ConstructorExpressionInner {}
impl ConstructorExpressionStructRef for ConstructorExpressionInner {}
impl InstantiationExpressionStruct for ConstructorExpressionInner {}
impl InstantiationExpressionStructRefMut for ConstructorExpressionInner {}
impl InstantiationExpressionStructRef for ConstructorExpressionInner {}
impl ExpressionStruct for ConstructorExpressionInner {}
impl ExpressionStructRefMut for ConstructorExpressionInner {}
impl ExpressionStructRef for ConstructorExpressionInner {}
impl StepStruct for ConstructorExpressionInner {}
impl StepStructRefMut for ConstructorExpressionInner {}
impl StepStructRef for ConstructorExpressionInner {}
impl FeatureStruct for ConstructorExpressionInner {
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
impl FeatureStructRefMut for ConstructorExpressionInner {
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
impl FeatureStructRef for ConstructorExpressionInner {
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
impl TypeStruct for ConstructorExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_instantiation_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_instantiation_expression.is_sufficient()
    }
}
impl TypeStructRefMut for ConstructorExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_instantiation_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for ConstructorExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_instantiation_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for ConstructorExpressionInner {}
impl NamespaceStructRefMut for ConstructorExpressionInner {}
impl NamespaceStructRef for ConstructorExpressionInner {}
impl ElementStruct for ConstructorExpressionInner {
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
impl ElementStructRefMut for ConstructorExpressionInner {
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
impl ElementStructRef for ConstructorExpressionInner {
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
pub enum ConstructorExpression {
    Itself(ConstructorExpressionInner),
}
pub enum ConstructorExpressionRefMut<'a> {
    Itself(&'a mut ConstructorExpressionInner),
}
pub enum ConstructorExpressionRef<'a> {
    Itself(&'a ConstructorExpressionInner),
}
impl ConstructorExpression {
    pub fn as_ref(&self) -> ConstructorExpressionRef {
        match self {
            ConstructorExpression::Itself(inner) => {
                ConstructorExpressionRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ConstructorExpressionRefMut {
        match self {
            ConstructorExpression::Itself(inner) => {
                ConstructorExpressionRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> ConstructorExpressionRefMut<'a> {
    pub fn as_ref(self) -> ConstructorExpressionRef<'a> {
        match self {
            ConstructorExpressionRefMut::Itself(inner) => {
                ConstructorExpressionRef::Itself(inner)
            }
        }
    }
}
impl ConstructorExpressionStruct for ConstructorExpression {}
impl ConstructorExpressionStructRefMut for ConstructorExpression {}
impl ConstructorExpressionStructRef for ConstructorExpression {}
impl<'a> ConstructorExpressionStructRefMut for ConstructorExpressionRefMut<'a> {}
impl<'a> ConstructorExpressionStructRef for ConstructorExpressionRefMut<'a> {}
impl<'a> ConstructorExpressionStructRef for ConstructorExpressionRef<'a> {}
impl InstantiationExpressionStruct for ConstructorExpression {}
impl InstantiationExpressionStructRefMut for ConstructorExpression {}
impl InstantiationExpressionStructRef for ConstructorExpression {}
impl<'a> InstantiationExpressionStructRefMut for ConstructorExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for ConstructorExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for ConstructorExpressionRef<'a> {}
impl ExpressionStruct for ConstructorExpression {}
impl ExpressionStructRefMut for ConstructorExpression {}
impl ExpressionStructRef for ConstructorExpression {}
impl<'a> ExpressionStructRefMut for ConstructorExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for ConstructorExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for ConstructorExpressionRef<'a> {}
impl StepStruct for ConstructorExpression {}
impl StepStructRefMut for ConstructorExpression {}
impl StepStructRef for ConstructorExpression {}
impl<'a> StepStructRefMut for ConstructorExpressionRefMut<'a> {}
impl<'a> StepStructRef for ConstructorExpressionRefMut<'a> {}
impl<'a> StepStructRef for ConstructorExpressionRef<'a> {}
impl FeatureStruct for ConstructorExpression {
    fn is_unique(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_constant(),
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
            ConstructorExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for ConstructorExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_constant_ref_mut(),
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
            ConstructorExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for ConstructorExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_constant_ref(),
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
            ConstructorExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for ConstructorExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            ConstructorExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for ConstructorExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            ConstructorExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for ConstructorExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_constant_ref(),
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
            ConstructorExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for ConstructorExpression {
    fn is_abstract(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for ConstructorExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for ConstructorExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for ConstructorExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for ConstructorExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for ConstructorExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for ConstructorExpression {}
impl NamespaceStructRefMut for ConstructorExpression {}
impl NamespaceStructRef for ConstructorExpression {}
impl<'a> NamespaceStructRefMut for ConstructorExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for ConstructorExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for ConstructorExpressionRef<'a> {}
impl ElementStruct for ConstructorExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            ConstructorExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            ConstructorExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for ConstructorExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ConstructorExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ConstructorExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ConstructorExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for ConstructorExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConstructorExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConstructorExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConstructorExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ConstructorExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ConstructorExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ConstructorExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConstructorExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConstructorExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConstructorExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConstructorExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConstructorExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl ConstructorExpressionUpcast for ConstructorExpression {
    fn into_constructor_expression(self) -> ConstructorExpression {
        self
    }
}
impl<'a> ConstructorExpressionUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_constructor_expression_ref_mut(self) -> ConstructorExpressionRefMut<'a> {
        self
    }
}
impl<'a> ConstructorExpressionUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_constructor_expression_ref(self) -> ConstructorExpressionRef<'a> {
        self
    }
}
impl InstantiationExpressionUpcast for ConstructorExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        InstantiationExpression::ConstructorExpression(self)
            .into_instantiation_expression()
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self)
            .as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self)
            .as_instantiation_expression_ref()
    }
}
impl ExpressionUpcast for ConstructorExpression {
    fn into_expression(self) -> Expression {
        InstantiationExpression::ConstructorExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self)
            .as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_expression_ref()
    }
}
impl StepUpcast for ConstructorExpression {
    fn into_step(self) -> Step {
        InstantiationExpression::ConstructorExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for ConstructorExpression {
    fn into_feature(self) -> Feature {
        InstantiationExpression::ConstructorExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for ConstructorExpression {
    fn into_type_(self) -> Type {
        InstantiationExpression::ConstructorExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for ConstructorExpression {
    fn into_namespace(self) -> Namespace {
        InstantiationExpression::ConstructorExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for ConstructorExpression {
    fn into_element(self) -> Element {
        InstantiationExpression::ConstructorExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ConstructorExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        InstantiationExpressionRefMut::ConstructorExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ConstructorExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        InstantiationExpressionRef::ConstructorExpression(self).as_element_ref()
    }
}
pub trait ConstructorExpressionDowncast {}
pub trait ConstructorExpressionDowncastRefMut<'a> {}
pub trait ConstructorExpressionDowncastRef<'a> {}
impl ConstructorExpressionDowncast for ConstructorExpression {}
impl<'a> ConstructorExpressionDowncastRefMut<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> ConstructorExpressionDowncastRef<'a> for ConstructorExpressionRef<'a> {}
pub trait ConstructorExpressionMethodsDescendants
where
    Self: DescendantOf<ConstructorExpression>,
    Self::Via: ConstructorExpressionMethods,
    Self: Sized,
{}
pub trait ConstructorExpressionMethods: Sized {}
impl<T: ConstructorExpressionMethodsDescendants> ConstructorExpressionMethods for T
where
    T::Via: ConstructorExpressionMethods,
{}
impl DescendantOf<InstantiationExpression> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl InstantiationExpressionMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Expression> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl ExpressionMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Step> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl StepMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Feature> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl FeatureMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Type> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl TypeMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Namespace> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl NamespaceMethodsDescendants for ConstructorExpression {}
impl DescendantOf<Element> for ConstructorExpression {
    type Via = InstantiationExpression;
    fn into_via(self) -> Self::Via {
        self.into_instantiation_expression()
    }
}
impl ElementMethodsDescendants for ConstructorExpression {}
pub trait ConstructorExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ConstructorExpressionRefMut<'a>>,
    Self::Via: ConstructorExpressionRefMutMethods,
    Self: Sized,
{}
pub trait ConstructorExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: ConstructorExpressionRefMutMethodsDescendants<'a>,
> ConstructorExpressionRefMutMethods for T
where
    T::Via: ConstructorExpressionRefMutMethods,
{}
impl<'a> DescendantOf<InstantiationExpressionRefMut<'a>>
for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> InstantiationExpressionRefMutMethodsDescendants<'a>
for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ConstructorExpressionRefMut<'a> {
    type Via = InstantiationExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ConstructorExpressionRefMut<'a> {}
pub trait ConstructorExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<ConstructorExpressionRef<'a>>,
    Self::Via: ConstructorExpressionRefMethods,
    Self: Sized,
{}
pub trait ConstructorExpressionRefMethods: Sized {}
impl<
    'a,
    T: ConstructorExpressionRefMethodsDescendants<'a>,
> ConstructorExpressionRefMethods for T
where
    T::Via: ConstructorExpressionRefMethods,
{}
impl<'a> DescendantOf<InstantiationExpressionRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> InstantiationExpressionRefMethodsDescendants<'a>
for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ConstructorExpressionRef<'a> {
    type Via = InstantiationExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_instantiation_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ConstructorExpressionRef<'a> {}

