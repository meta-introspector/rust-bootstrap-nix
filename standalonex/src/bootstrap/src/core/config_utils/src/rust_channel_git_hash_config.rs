use crate::ParsedConfig;
use crate::LocalTomlConfig;
use crate::ConfigApplicator;

pub struct RustChannelGitHashConfigApplicator;

impl ConfigApplicator for RustChannelGitHashConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let is_user_configured_rust_channel =
            if let Some(channel) = toml.rust.as_ref().and_then(|r| r.channel.clone()) {
                config.channel = channel;
                true
            } else {
                false
            };

        config.omit_git_hash = toml.rust.as_ref().and_then(|r| r.omit_git_hash).unwrap_or(config.channel == "dev");
        // GitInfo assignments will be handled by the processor crate
    }
}
