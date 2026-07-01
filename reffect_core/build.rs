use shadow_rs::{
    CARGO_MANIFEST_DIR, CARGO_METADATA, CARGO_TREE, COMMITS_SINCE_TAG, GIT_CLEAN, GIT_STATUS_FILE,
    LAST_TAG, ShadowBuilder, TAG,
};

fn main() {
    let deny_const = [
        CARGO_METADATA,
        CARGO_TREE,
        CARGO_MANIFEST_DIR,
        GIT_CLEAN,
        GIT_STATUS_FILE,
        COMMITS_SINCE_TAG,
        TAG,
        LAST_TAG,
    ]
    .into();
    ShadowBuilder::builder()
        .deny_const(deny_const)
        .build()
        .unwrap();
}
