pub use crate::generated::feature_typing::{
    FeatureTyping, FeatureTypingMethods, FeatureTypingRef, FeatureTypingRefMethods,
    FeatureTypingRefMut, FeatureTypingRefMutMethods,
};

impl FeatureTypingMethods for FeatureTyping {}

impl FeatureTypingRefMutMethods for FeatureTypingRefMut<'_> {}

impl FeatureTypingRefMethods for FeatureTypingRef<'_> {}
