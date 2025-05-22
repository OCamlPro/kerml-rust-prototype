pub mod annotating_element;
pub mod annotation;
pub mod association;
pub mod association_structure;
pub mod behavior;
pub mod binding_connector;
pub mod boolean_expression;
pub mod classifier;
pub mod class;
pub mod collect_expression;
pub mod comment;
pub mod conjugation;
pub mod connector;
pub mod constructor_expression;
pub mod cross_subsetting;
pub mod data_type;
pub mod dependency;
pub mod differencing;
pub mod disjoining;
pub mod documentation;
pub mod element_filter_membership;
pub mod element;
pub mod end_feature_membership;
pub mod expression;
pub mod feature_chain_expression;
pub mod feature_chaining;
pub mod feature_inverting;
pub mod feature_membership;
pub mod feature_reference_expression;
pub mod feature;
pub mod feature_typing;
pub mod feature_value;
pub mod flow_end;
pub mod flow;
pub mod function;
pub mod import;
pub mod index_expression;
pub mod instantiation_expression;
pub mod interaction;
pub mod intersecting;
pub mod invariant;
pub mod invocation_expression;
pub mod library_package;
pub mod literal_boolean;
pub mod literal_expression;
pub mod literal_infinity;
pub mod literal_integer;
pub mod literal_rational;
pub mod literal_string;
pub mod membership_import;
pub mod membership;
pub mod metaclass;
pub mod metadata_access_expression;
pub mod metadata_feature;
pub mod multiplicity_range;
pub mod multiplicity;
pub mod namespace_import;
pub mod namespace;
pub mod null_expression;
pub mod operator_expression;
pub mod owning_membership;
pub mod package;
pub mod parameter_membership;
pub mod payload_feature;
pub mod predicate;
pub mod redefinition;
pub mod reference_subsetting;
pub mod relationship;
pub mod result_expression_membership;
pub mod return_parameter_membership;
pub mod select_expression;
pub mod specialization;
pub mod step;
pub mod structure;
pub mod subclassification;
pub mod subsetting;
pub mod succession_flow;
pub mod succession;
pub mod textual_representation;
pub mod type_featuring;
pub mod type_;
pub mod unioning;
pub mod utils;

pub mod visibility_kind {
    #[derive(Default)]
    pub enum VisibilityKind {
        Public,
        Protected,
        #[default]
        Private,
    }
}

pub mod feature_direction_kind {
    #[derive(Default)]
    pub enum FeatureDirectionKind {
        In,
        #[default]
        InOut,
        Out,
    }
}
