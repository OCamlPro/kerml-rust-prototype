pub use crate::generated::comment::{
    Comment, CommentMethods, CommentRef, CommentRefMethods, CommentRefMut, CommentRefMutMethods,
};

impl CommentMethods for Comment {}

impl CommentRefMethods for CommentRef<'_> {}

impl CommentRefMutMethods for CommentRefMut<'_> {}
