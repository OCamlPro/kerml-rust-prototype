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
pub struct MetadataAccessExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait MetadataAccessExpressionStruct
where
    Self: MetadataAccessExpressionStructRefMut,
    Self: MetadataAccessExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait MetadataAccessExpressionStructRefMut
where
    Self: MetadataAccessExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait MetadataAccessExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait MetadataAccessExpressionUpcast: MetadataAccessExpressionStruct {
    fn into_metadata_access_expression(self) -> MetadataAccessExpression;
}
pub trait MetadataAccessExpressionUpcastRefMut<
    'a,
>: MetadataAccessExpressionStructRefMut {
    fn as_metadata_access_expression_ref_mut(self) -> MetadataAccessExpressionRefMut<'a>;
}
pub trait MetadataAccessExpressionUpcastRef<'a>: MetadataAccessExpressionStructRef {
    fn as_metadata_access_expression_ref(self) -> MetadataAccessExpressionRef<'a>;
}
impl MetadataAccessExpressionStruct for MetadataAccessExpressionInner {}
impl MetadataAccessExpressionStructRefMut for MetadataAccessExpressionInner {}
impl MetadataAccessExpressionStructRef for MetadataAccessExpressionInner {}
impl ExpressionStruct for MetadataAccessExpressionInner {}
impl ExpressionStructRefMut for MetadataAccessExpressionInner {}
impl ExpressionStructRef for MetadataAccessExpressionInner {}
impl StepStruct for MetadataAccessExpressionInner {}
impl StepStructRefMut for MetadataAccessExpressionInner {}
impl StepStructRef for MetadataAccessExpressionInner {}
impl FeatureStruct for MetadataAccessExpressionInner {
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
impl FeatureStructRefMut for MetadataAccessExpressionInner {
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
impl FeatureStructRef for MetadataAccessExpressionInner {
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
impl TypeStruct for MetadataAccessExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for MetadataAccessExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for MetadataAccessExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for MetadataAccessExpressionInner {}
impl NamespaceStructRefMut for MetadataAccessExpressionInner {}
impl NamespaceStructRef for MetadataAccessExpressionInner {}
impl ElementStruct for MetadataAccessExpressionInner {
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
impl ElementStructRefMut for MetadataAccessExpressionInner {
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
impl ElementStructRef for MetadataAccessExpressionInner {
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
pub enum MetadataAccessExpression {
    Itself(MetadataAccessExpressionInner),
}
pub enum MetadataAccessExpressionRefMut<'a> {
    Itself(&'a mut MetadataAccessExpressionInner),
}
pub enum MetadataAccessExpressionRef<'a> {
    Itself(&'a MetadataAccessExpressionInner),
}
impl MetadataAccessExpression {
    pub fn as_ref(&self) -> MetadataAccessExpressionRef {
        match self {
            MetadataAccessExpression::Itself(inner) => {
                MetadataAccessExpressionRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> MetadataAccessExpressionRefMut {
        match self {
            MetadataAccessExpression::Itself(inner) => {
                MetadataAccessExpressionRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> MetadataAccessExpressionRefMut<'a> {
    pub fn as_ref(self) -> MetadataAccessExpressionRef<'a> {
        match self {
            MetadataAccessExpressionRefMut::Itself(inner) => {
                MetadataAccessExpressionRef::Itself(inner)
            }
        }
    }
}
impl MetadataAccessExpressionStruct for MetadataAccessExpression {}
impl MetadataAccessExpressionStructRefMut for MetadataAccessExpression {}
impl MetadataAccessExpressionStructRef for MetadataAccessExpression {}
impl<'a> MetadataAccessExpressionStructRefMut for MetadataAccessExpressionRefMut<'a> {}
impl<'a> MetadataAccessExpressionStructRef for MetadataAccessExpressionRefMut<'a> {}
impl<'a> MetadataAccessExpressionStructRef for MetadataAccessExpressionRef<'a> {}
impl ExpressionStruct for MetadataAccessExpression {}
impl ExpressionStructRefMut for MetadataAccessExpression {}
impl ExpressionStructRef for MetadataAccessExpression {}
impl<'a> ExpressionStructRefMut for MetadataAccessExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for MetadataAccessExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for MetadataAccessExpressionRef<'a> {}
impl StepStruct for MetadataAccessExpression {}
impl StepStructRefMut for MetadataAccessExpression {}
impl StepStructRef for MetadataAccessExpression {}
impl<'a> StepStructRefMut for MetadataAccessExpressionRefMut<'a> {}
impl<'a> StepStructRef for MetadataAccessExpressionRefMut<'a> {}
impl<'a> StepStructRef for MetadataAccessExpressionRef<'a> {}
impl FeatureStruct for MetadataAccessExpression {
    fn is_unique(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_constant(),
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
            MetadataAccessExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for MetadataAccessExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_constant_ref_mut(),
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
            MetadataAccessExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for MetadataAccessExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_constant_ref(),
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
            MetadataAccessExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for MetadataAccessExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            MetadataAccessExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for MetadataAccessExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            MetadataAccessExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for MetadataAccessExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_constant_ref(),
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
            MetadataAccessExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for MetadataAccessExpression {
    fn is_abstract(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for MetadataAccessExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for MetadataAccessExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for MetadataAccessExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for MetadataAccessExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for MetadataAccessExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for MetadataAccessExpression {}
impl NamespaceStructRefMut for MetadataAccessExpression {}
impl NamespaceStructRef for MetadataAccessExpression {}
impl<'a> NamespaceStructRefMut for MetadataAccessExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for MetadataAccessExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for MetadataAccessExpressionRef<'a> {}
impl ElementStruct for MetadataAccessExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            MetadataAccessExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for MetadataAccessExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MetadataAccessExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MetadataAccessExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for MetadataAccessExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataAccessExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataAccessExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MetadataAccessExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MetadataAccessExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MetadataAccessExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MetadataAccessExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl MetadataAccessExpressionUpcast for MetadataAccessExpression {
    fn into_metadata_access_expression(self) -> MetadataAccessExpression {
        self
    }
}
impl<'a> MetadataAccessExpressionUpcastRefMut<'a>
for MetadataAccessExpressionRefMut<'a> {
    fn as_metadata_access_expression_ref_mut(
        self,
    ) -> MetadataAccessExpressionRefMut<'a> {
        self
    }
}
impl<'a> MetadataAccessExpressionUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_metadata_access_expression_ref(self) -> MetadataAccessExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for MetadataAccessExpression {
    fn into_expression(self) -> Expression {
        Expression::MetadataAccessExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_expression_ref()
    }
}
impl StepUpcast for MetadataAccessExpression {
    fn into_step(self) -> Step {
        Expression::MetadataAccessExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for MetadataAccessExpression {
    fn into_feature(self) -> Feature {
        Expression::MetadataAccessExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for MetadataAccessExpression {
    fn into_type_(self) -> Type {
        Expression::MetadataAccessExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for MetadataAccessExpression {
    fn into_namespace(self) -> Namespace {
        Expression::MetadataAccessExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for MetadataAccessExpression {
    fn into_element(self) -> Element {
        Expression::MetadataAccessExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MetadataAccessExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::MetadataAccessExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MetadataAccessExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::MetadataAccessExpression(self).as_element_ref()
    }
}
pub trait MetadataAccessExpressionDowncast {}
pub trait MetadataAccessExpressionDowncastRefMut<'a> {}
pub trait MetadataAccessExpressionDowncastRef<'a> {}
impl MetadataAccessExpressionDowncast for MetadataAccessExpression {}
impl<'a> MetadataAccessExpressionDowncastRefMut<'a>
for MetadataAccessExpressionRefMut<'a> {}
impl<'a> MetadataAccessExpressionDowncastRef<'a> for MetadataAccessExpressionRef<'a> {}
pub trait MetadataAccessExpressionMethodsDescendants
where
    Self: DescendantOf<MetadataAccessExpression>,
    Self::Via: MetadataAccessExpressionMethods,
    Self: Sized,
{}
pub trait MetadataAccessExpressionMethods: Sized {}
impl<T: MetadataAccessExpressionMethodsDescendants> MetadataAccessExpressionMethods for T
where
    T::Via: MetadataAccessExpressionMethods,
{}
impl DescendantOf<Expression> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for MetadataAccessExpression {}
impl DescendantOf<Step> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for MetadataAccessExpression {}
impl DescendantOf<Feature> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for MetadataAccessExpression {}
impl DescendantOf<Type> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for MetadataAccessExpression {}
impl DescendantOf<Namespace> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for MetadataAccessExpression {}
impl DescendantOf<Element> for MetadataAccessExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for MetadataAccessExpression {}
pub trait MetadataAccessExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MetadataAccessExpressionRefMut<'a>>,
    Self::Via: MetadataAccessExpressionRefMutMethods,
    Self: Sized,
{}
pub trait MetadataAccessExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: MetadataAccessExpressionRefMutMethodsDescendants<'a>,
> MetadataAccessExpressionRefMutMethods for T
where
    T::Via: MetadataAccessExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MetadataAccessExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MetadataAccessExpressionRefMut<'a> {}
pub trait MetadataAccessExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<MetadataAccessExpressionRef<'a>>,
    Self::Via: MetadataAccessExpressionRefMethods,
    Self: Sized,
{
    fn metaclass_feature_impl(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::metadata_feature::MetadataFeature>> {
        self.into_via().metaclass_feature()
    }
}
pub trait MetadataAccessExpressionRefMethods: Sized {
    fn metaclass_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::metadata_feature::MetadataFeature>>;
}
impl<
    'a,
    T: MetadataAccessExpressionRefMethodsDescendants<'a>,
> MetadataAccessExpressionRefMethods for T
where
    T::Via: MetadataAccessExpressionRefMethods,
{
    fn metaclass_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<super::metadata_feature::MetadataFeature>> {
        MetadataAccessExpressionRefMethodsDescendants::metaclass_feature_impl(self)
    }
}
impl<'a> DescendantOf<ExpressionRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MetadataAccessExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MetadataAccessExpressionRef<'a> {}

