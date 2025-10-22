use crate::parsed_config::ParsedConfig;
use crate::local_toml_config::LocalTomlConfig;
use crate::config_applicator::ConfigApplicator;

pub struct RustChannelGitHashConfigApplicator;

impl ConfigApplicator for RustChannelGitHashConfigApplicator {
    fn apply_to_config(&self, config: &mut ParsedConfig, toml: &LocalTomlConfig) {
        let is_user_configured_rust_channel =
            if let Some(channel) = toml.rust.as_ref().and_then(|r| r.channel.clone()) {
                config.channel = Some(channel);
                true
            } else {
                false
            };

        config.omit_git_hash = toml.rust.as_ref().and_then(|r| r.omit_git_hash).unwrap_or(config.channel.as_deref() == Some("dev"));
        // GitInfo assignments will be handled by the processor crate
    }
}
