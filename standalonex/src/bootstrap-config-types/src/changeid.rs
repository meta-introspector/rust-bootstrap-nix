use serde::Deserialize;
/// Since we use `#[serde(deny_unknown_fields)]` on `TomlConfig`, we need a wrapper type
/// for the "change-id" field to parse it even if other fields are invalid. This ensures
/// that if deserialization fails due to other fields, we can still provide the changelogs
/// to allow developers to potentially find the reason for the failure in the logs..
#[derive(Deserialize, Default)]
pub struct ChangeIdWrapper {
    #[serde(alias = "change-id")]
    pub(crate) inner: Option<usize>,
}
