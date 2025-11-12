/// Describes how to handle conflicts in merging two [`TomlConfig`]
#[derive(Copy, Clone, Debug)]
enum ReplaceOpt {
    /// Silently ignore a duplicated value
    IgnoreDuplicate,
    /// Override the current value, even if it's `Some`
    Override,
    /// Exit with an error on duplicate values
    ErrorOnDuplicate,
}
