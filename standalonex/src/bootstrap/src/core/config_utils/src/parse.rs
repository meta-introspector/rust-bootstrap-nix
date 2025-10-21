use bootstrap::Config;
use bootstrap::Flags;
use crate::get_toml;

pub fn parse(flags: Flags) -> Config {
    // Assuming parse_inner will also be moved and called as a standalone function
    // For now, I'll keep it as Config::parse_inner and fix it later when parse_inner is moved.
    Config::parse_inner(flags, get_toml::get_toml)
}