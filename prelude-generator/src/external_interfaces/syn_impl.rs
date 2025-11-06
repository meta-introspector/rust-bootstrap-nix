use anyhow::Result;
use crate::external_interfaces::SynInterface;

pub struct SynInterfaceImpl;

impl SynInterface for SynInterfaceImpl {
    fn parse_file(&self, content: &str) -> Result<syn::File> {
        syn::parse_file(content).map_err(|e| anyhow::anyhow!("Failed to parse file with syn: {}", e))
    }

    fn parse_str<T: syn::parse::Parse>(&self, s: &str) -> Result<T> {
        syn::parse_str(s).map_err(|e| anyhow::anyhow!("Failed to parse string with syn: {}", e))
    }
}
