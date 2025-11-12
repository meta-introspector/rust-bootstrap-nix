#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use anyhow::Result;
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
pub mod use_statement_types {
    pub mod git_info {
        use std::fmt::Debug;
        pub struct GitInfo {
            pub repo_url: String,
            pub branch: String,
            pub commit_hash: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GitInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GitInfo",
                    "repo_url",
                    &self.repo_url,
                    "branch",
                    &self.branch,
                    "commit_hash",
                    &&self.commit_hash,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GitInfo {
            #[inline]
            fn clone(&self) -> GitInfo {
                GitInfo {
                    repo_url: ::core::clone::Clone::clone(&self.repo_url),
                    branch: ::core::clone::Clone::clone(&self.branch),
                    commit_hash: ::core::clone::Clone::clone(&self.commit_hash),
                }
            }
        }
        pub enum GitDetails {
            Info(GitInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GitDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    GitDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    GitDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    GitDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GitDetails {
            #[inline]
            fn clone(&self) -> GitDetails {
                match self {
                    GitDetails::Info(__self_0) => {
                        GitDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    GitDetails::Error(__self_0) => {
                        GitDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    GitDetails::Unknown => GitDetails::Unknown,
                }
            }
        }
        pub trait GitInfoTrait: Send + Sync + Debug {
            fn git_repo(&self) -> Option<&str>;
            fn git_path(&self) -> Option<&str>;
            fn our_fork_github(&self) -> Option<&str>;
            fn our_branch(&self) -> Option<&str>;
        }
        impl GitInfoTrait for GitDetails {
            fn git_repo(&self) -> Option<&str> {
                match self {
                    GitDetails::Info(info) => Some(&info.repo_url),
                    _ => None,
                }
            }
            fn git_path(&self) -> Option<&str> {
                None
            }
            fn our_fork_github(&self) -> Option<&str> {
                None
            }
            fn our_branch(&self) -> Option<&str> {
                match self {
                    GitDetails::Info(info) => Some(&info.branch),
                    _ => None,
                }
            }
        }
    }
    pub mod nix_info {
        use std::fmt::Debug;
        pub struct NixInfo {
            pub flake_path: String,
            pub output_type: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NixInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "NixInfo",
                    "flake_path",
                    &self.flake_path,
                    "output_type",
                    &&self.output_type,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NixInfo {
            #[inline]
            fn clone(&self) -> NixInfo {
                NixInfo {
                    flake_path: ::core::clone::Clone::clone(&self.flake_path),
                    output_type: ::core::clone::Clone::clone(&self.output_type),
                }
            }
        }
        pub enum NixDetails {
            Info(NixInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NixDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    NixDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    NixDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    NixDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NixDetails {
            #[inline]
            fn clone(&self) -> NixDetails {
                match self {
                    NixDetails::Info(__self_0) => {
                        NixDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    NixDetails::Error(__self_0) => {
                        NixDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    NixDetails::Unknown => NixDetails::Unknown,
                }
            }
        }
        pub trait NixInfoTrait: Send + Sync + Debug {
            fn nix_flake_path(&self) -> Option<&str>;
            fn nix_output_type(&self) -> Option<&str>;
        }
        impl NixInfoTrait for NixDetails {
            fn nix_flake_path(&self) -> Option<&str> {
                match self {
                    NixDetails::Info(info) => Some(&info.flake_path),
                    _ => None,
                }
            }
            fn nix_output_type(&self) -> Option<&str> {
                match self {
                    NixDetails::Info(info) => Some(&info.output_type),
                    _ => None,
                }
            }
        }
    }
    pub mod rust_details_info {
        use std::fmt::Debug;
        pub struct RustDetailsInfo {
            pub version: String,
            pub crate_name: String,
            pub item_path: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RustDetailsInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "RustDetailsInfo",
                    "version",
                    &self.version,
                    "crate_name",
                    &self.crate_name,
                    "item_path",
                    &&self.item_path,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RustDetailsInfo {
            #[inline]
            fn clone(&self) -> RustDetailsInfo {
                RustDetailsInfo {
                    version: ::core::clone::Clone::clone(&self.version),
                    crate_name: ::core::clone::Clone::clone(&self.crate_name),
                    item_path: ::core::clone::Clone::clone(&self.item_path),
                }
            }
        }
        pub enum RustDetails {
            Info(RustDetailsInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RustDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    RustDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    RustDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    RustDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RustDetails {
            #[inline]
            fn clone(&self) -> RustDetails {
                match self {
                    RustDetails::Info(__self_0) => {
                        RustDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    RustDetails::Error(__self_0) => {
                        RustDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    RustDetails::Unknown => RustDetails::Unknown,
                }
            }
        }
        pub trait RustDetailsInfoTrait: Send + Sync + Debug {
            fn version(&self) -> Option<&str>;
            fn crate_name(&self) -> Option<&str>;
            fn item_path(&self) -> Option<&str>;
        }
        impl RustDetailsInfoTrait for RustDetails {
            fn version(&self) -> Option<&str> {
                match self {
                    RustDetails::Info(info) => Some(&info.version),
                    _ => None,
                }
            }
            fn crate_name(&self) -> Option<&str> {
                match self {
                    RustDetails::Info(info) => Some(&info.crate_name),
                    _ => None,
                }
            }
            fn item_path(&self) -> Option<&str> {
                match self {
                    RustDetails::Info(info) => Some(&info.item_path),
                    _ => None,
                }
            }
        }
    }
    pub mod cargo_info {
        use std::fmt::Debug;
        pub struct CargoInfo {
            pub package_name: String,
            pub version: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CargoInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CargoInfo",
                    "package_name",
                    &self.package_name,
                    "version",
                    &&self.version,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CargoInfo {
            #[inline]
            fn clone(&self) -> CargoInfo {
                CargoInfo {
                    package_name: ::core::clone::Clone::clone(&self.package_name),
                    version: ::core::clone::Clone::clone(&self.version),
                }
            }
        }
        pub enum CargoDetails {
            Info(CargoInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CargoDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CargoDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    CargoDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    CargoDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CargoDetails {
            #[inline]
            fn clone(&self) -> CargoDetails {
                match self {
                    CargoDetails::Info(__self_0) => {
                        CargoDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    CargoDetails::Error(__self_0) => {
                        CargoDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    CargoDetails::Unknown => CargoDetails::Unknown,
                }
            }
        }
        pub trait CargoInfoTrait: Send + Sync + Debug {
            fn package_name(&self) -> Option<&str>;
            fn version(&self) -> Option<&str>;
        }
        impl CargoInfoTrait for CargoDetails {
            fn package_name(&self) -> Option<&str> {
                match self {
                    CargoDetails::Info(info) => Some(&info.package_name),
                    _ => None,
                }
            }
            fn version(&self) -> Option<&str> {
                match self {
                    CargoDetails::Info(info) => Some(&info.version),
                    _ => None,
                }
            }
        }
    }
    pub mod syn_info {
        use std::fmt::Debug;
        pub struct SynInfo {
            pub parsed_type: String,
            pub version: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SynInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SynInfo",
                    "parsed_type",
                    &self.parsed_type,
                    "version",
                    &&self.version,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SynInfo {
            #[inline]
            fn clone(&self) -> SynInfo {
                SynInfo {
                    parsed_type: ::core::clone::Clone::clone(&self.parsed_type),
                    version: ::core::clone::Clone::clone(&self.version),
                }
            }
        }
        pub enum SynDetails {
            Info(SynInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SynDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    SynDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    SynDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    SynDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SynDetails {
            #[inline]
            fn clone(&self) -> SynDetails {
                match self {
                    SynDetails::Info(__self_0) => {
                        SynDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    SynDetails::Error(__self_0) => {
                        SynDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    SynDetails::Unknown => SynDetails::Unknown,
                }
            }
        }
        pub trait SynInfoTrait: Send + Sync + Debug {
            fn parsed_type(&self) -> Option<&str>;
            fn version(&self) -> Option<&str>;
        }
        impl SynInfoTrait for SynDetails {
            fn parsed_type(&self) -> Option<&str> {
                match self {
                    SynDetails::Info(info) => Some(&info.parsed_type),
                    _ => None,
                }
            }
            fn version(&self) -> Option<&str> {
                match self {
                    SynDetails::Info(info) => Some(&info.version),
                    _ => None,
                }
            }
        }
    }
    pub mod llvm_info {
        use std::fmt::Debug;
        pub struct LlvmInfo {
            pub ir_version: String,
            pub target_triple: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LlvmInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "LlvmInfo",
                    "ir_version",
                    &self.ir_version,
                    "target_triple",
                    &&self.target_triple,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LlvmInfo {
            #[inline]
            fn clone(&self) -> LlvmInfo {
                LlvmInfo {
                    ir_version: ::core::clone::Clone::clone(&self.ir_version),
                    target_triple: ::core::clone::Clone::clone(&self.target_triple),
                }
            }
        }
        pub enum LlvmDetails {
            Info(LlvmInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LlvmDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LlvmDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    LlvmDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    LlvmDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LlvmDetails {
            #[inline]
            fn clone(&self) -> LlvmDetails {
                match self {
                    LlvmDetails::Info(__self_0) => {
                        LlvmDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    LlvmDetails::Error(__self_0) => {
                        LlvmDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    LlvmDetails::Unknown => LlvmDetails::Unknown,
                }
            }
        }
        pub trait LlvmInfoTrait: Send + Sync + Debug {
            fn ir_version(&self) -> Option<&str>;
            fn target_triple(&self) -> Option<&str>;
        }
        impl LlvmInfoTrait for LlvmDetails {
            fn ir_version(&self) -> Option<&str> {
                match self {
                    LlvmDetails::Info(info) => Some(&info.ir_version),
                    _ => None,
                }
            }
            fn target_triple(&self) -> Option<&str> {
                match self {
                    LlvmDetails::Info(info) => Some(&info.target_triple),
                    _ => None,
                }
            }
        }
    }
    pub mod linux_info {
        use std::fmt::Debug;
        pub struct LinuxInfo {
            pub kernel_version: String,
            pub architecture: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LinuxInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "LinuxInfo",
                    "kernel_version",
                    &self.kernel_version,
                    "architecture",
                    &&self.architecture,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LinuxInfo {
            #[inline]
            fn clone(&self) -> LinuxInfo {
                LinuxInfo {
                    kernel_version: ::core::clone::Clone::clone(&self.kernel_version),
                    architecture: ::core::clone::Clone::clone(&self.architecture),
                }
            }
        }
        pub enum LinuxDetails {
            Info(LinuxInfo),
            Error(String),
            Unknown,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LinuxDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LinuxDetails::Info(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Info",
                            &__self_0,
                        )
                    }
                    LinuxDetails::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Error",
                            &__self_0,
                        )
                    }
                    LinuxDetails::Unknown => {
                        ::core::fmt::Formatter::write_str(f, "Unknown")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LinuxDetails {
            #[inline]
            fn clone(&self) -> LinuxDetails {
                match self {
                    LinuxDetails::Info(__self_0) => {
                        LinuxDetails::Info(::core::clone::Clone::clone(__self_0))
                    }
                    LinuxDetails::Error(__self_0) => {
                        LinuxDetails::Error(::core::clone::Clone::clone(__self_0))
                    }
                    LinuxDetails::Unknown => LinuxDetails::Unknown,
                }
            }
        }
        pub trait LinuxInfoTrait: Send + Sync + Debug {
            fn kernel_version(&self) -> Option<&str>;
            fn architecture(&self) -> Option<&str>;
        }
        impl LinuxInfoTrait for LinuxDetails {
            fn kernel_version(&self) -> Option<&str> {
                match self {
                    LinuxDetails::Info(info) => Some(&info.kernel_version),
                    _ => None,
                }
            }
            fn architecture(&self) -> Option<&str> {
                match self {
                    LinuxDetails::Info(info) => Some(&info.architecture),
                    _ => None,
                }
            }
        }
    }
    pub use git_info::{GitInfo, GitDetails, GitInfoTrait};
    pub use nix_info::{NixInfo, NixDetails, NixInfoTrait};
    pub use rust_details_info::{RustDetailsInfo, RustDetails, RustDetailsInfoTrait};
    pub use cargo_info::{CargoInfo, CargoDetails, CargoInfoTrait};
    pub use syn_info::{SynInfo, SynDetails, SynInfoTrait};
    pub use llvm_info::{LlvmInfo, LlvmDetails, LlvmInfoTrait};
    pub use linux_info::{LinuxInfo, LinuxDetails, LinuxInfoTrait};
}
pub use use_statement_types::{
    GitDetails, GitInfo, GitInfoTrait, NixDetails, NixInfo, NixInfoTrait, RustDetails,
    RustDetailsInfo, RustDetailsInfoTrait, CargoDetails, CargoInfo, CargoInfoTrait,
    SynDetails, SynInfo, SynInfoTrait, LlvmDetails, LlvmInfo, LlvmInfoTrait,
    LinuxDetails, LinuxInfo, LinuxInfoTrait,
};
pub struct RawFile(pub String, pub String);
#[automatically_derived]
impl ::core::fmt::Debug for RawFile {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "RawFile",
            &self.0,
            &&self.1,
        )
    }
}
pub struct ParsedFile(pub String, pub PathBuf);
#[automatically_derived]
impl ::core::clone::Clone for ParsedFile {
    #[inline]
    fn clone(&self) -> ParsedFile {
        ParsedFile(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
pub struct UseStatements(pub Vec<String>);
#[automatically_derived]
impl ::core::fmt::Debug for UseStatements {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UseStatements", &&self.0)
    }
}
pub struct ClassifiedUseStatements(
    pub Vec<UseStatement>,
    pub HashMap<String, Vec<String>>,
);
#[automatically_derived]
impl ::core::fmt::Debug for ClassifiedUseStatements {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "ClassifiedUseStatements",
            &self.0,
            &&self.1,
        )
    }
}
pub struct ValidatedFile(pub String, pub PathBuf);
#[automatically_derived]
impl ::core::fmt::Debug for ValidatedFile {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "ValidatedFile",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ValidatedFile {
    #[inline]
    fn clone(&self) -> ValidatedFile {
        ValidatedFile(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
pub trait PipelineFunctor<Input: Send + 'static, Output: Send + 'static> {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: Input,
    ) -> Pin<Box<dyn Future<Output = Result<Output>> + Send + 'writer>>;
}
pub struct UseStatement {
    pub statement: String,
    pub error: Option<String>,
    pub git_details: Option<GitDetails>,
    pub nix_details: Option<NixDetails>,
    pub rust_details: Option<RustDetails>,
    pub cargo_details: Option<CargoDetails>,
    pub syn_details: Option<SynDetails>,
    pub llvm_details: Option<LlvmDetails>,
    pub linux_details: Option<LinuxDetails>,
}
#[automatically_derived]
impl ::core::fmt::Debug for UseStatement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "statement",
            "error",
            "git_details",
            "nix_details",
            "rust_details",
            "cargo_details",
            "syn_details",
            "llvm_details",
            "linux_details",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.statement,
            &self.error,
            &self.git_details,
            &self.nix_details,
            &self.rust_details,
            &self.cargo_details,
            &self.syn_details,
            &self.llvm_details,
            &&self.linux_details,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "UseStatement",
            names,
            values,
        )
    }
}
/// Information about a variable found in the AST
pub struct VariableInfo {
    pub name: String,
    pub type_name: String,
    pub is_mutable: bool,
    pub scope: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for VariableInfo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "VariableInfo",
            "name",
            &self.name,
            "type_name",
            &self.type_name,
            "is_mutable",
            &self.is_mutable,
            "scope",
            &&self.scope,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for VariableInfo {
    #[inline]
    fn clone(&self) -> VariableInfo {
        VariableInfo {
            name: ::core::clone::Clone::clone(&self.name),
            type_name: ::core::clone::Clone::clone(&self.type_name),
            is_mutable: ::core::clone::Clone::clone(&self.is_mutable),
            scope: ::core::clone::Clone::clone(&self.scope),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for VariableInfo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "VariableInfo",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "type_name",
                &self.type_name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_mutable",
                &self.is_mutable,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "scope",
                &self.scope,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for VariableInfo {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private228::Ok(__Field::__field0),
                        "type_name" => _serde::__private228::Ok(__Field::__field1),
                        "is_mutable" => _serde::__private228::Ok(__Field::__field2),
                        "scope" => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private228::Ok(__Field::__field0),
                        b"type_name" => _serde::__private228::Ok(__Field::__field1),
                        b"is_mutable" => _serde::__private228::Ok(__Field::__field2),
                        b"scope" => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<VariableInfo>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VariableInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct VariableInfo",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct VariableInfo with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct VariableInfo with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct VariableInfo with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct VariableInfo with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(VariableInfo {
                        name: __field0,
                        type_name: __field1,
                        is_mutable: __field2,
                        scope: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<bool> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<String> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "type_name",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_mutable",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("scope"),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("type_name")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("is_mutable")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("scope")?
                        }
                    };
                    _serde::__private228::Ok(VariableInfo {
                        name: __field0,
                        type_name: __field1,
                        is_mutable: __field2,
                        scope: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "name",
                "type_name",
                "is_mutable",
                "scope",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "VariableInfo",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<VariableInfo>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
/// Information about a function found in the AST
pub struct FunctionInfo {
    pub name: String,
    pub visibility: String,
    pub arg_count: u32,
    pub arg_types: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_const: bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for FunctionInfo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "name",
            "visibility",
            "arg_count",
            "arg_types",
            "return_type",
            "is_async",
            "is_unsafe",
            "is_const",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.name,
            &self.visibility,
            &self.arg_count,
            &self.arg_types,
            &self.return_type,
            &self.is_async,
            &self.is_unsafe,
            &&self.is_const,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "FunctionInfo",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FunctionInfo {
    #[inline]
    fn clone(&self) -> FunctionInfo {
        FunctionInfo {
            name: ::core::clone::Clone::clone(&self.name),
            visibility: ::core::clone::Clone::clone(&self.visibility),
            arg_count: ::core::clone::Clone::clone(&self.arg_count),
            arg_types: ::core::clone::Clone::clone(&self.arg_types),
            return_type: ::core::clone::Clone::clone(&self.return_type),
            is_async: ::core::clone::Clone::clone(&self.is_async),
            is_unsafe: ::core::clone::Clone::clone(&self.is_unsafe),
            is_const: ::core::clone::Clone::clone(&self.is_const),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for FunctionInfo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "FunctionInfo",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "visibility",
                &self.visibility,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "arg_count",
                &self.arg_count,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "arg_types",
                &self.arg_types,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "return_type",
                &self.return_type,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_async",
                &self.is_async,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_unsafe",
                &self.is_unsafe,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_const",
                &self.is_const,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FunctionInfo {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        4u64 => _serde::__private228::Ok(__Field::__field4),
                        5u64 => _serde::__private228::Ok(__Field::__field5),
                        6u64 => _serde::__private228::Ok(__Field::__field6),
                        7u64 => _serde::__private228::Ok(__Field::__field7),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private228::Ok(__Field::__field0),
                        "visibility" => _serde::__private228::Ok(__Field::__field1),
                        "arg_count" => _serde::__private228::Ok(__Field::__field2),
                        "arg_types" => _serde::__private228::Ok(__Field::__field3),
                        "return_type" => _serde::__private228::Ok(__Field::__field4),
                        "is_async" => _serde::__private228::Ok(__Field::__field5),
                        "is_unsafe" => _serde::__private228::Ok(__Field::__field6),
                        "is_const" => _serde::__private228::Ok(__Field::__field7),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private228::Ok(__Field::__field0),
                        b"visibility" => _serde::__private228::Ok(__Field::__field1),
                        b"arg_count" => _serde::__private228::Ok(__Field::__field2),
                        b"arg_types" => _serde::__private228::Ok(__Field::__field3),
                        b"return_type" => _serde::__private228::Ok(__Field::__field4),
                        b"is_async" => _serde::__private228::Ok(__Field::__field5),
                        b"is_unsafe" => _serde::__private228::Ok(__Field::__field6),
                        b"is_const" => _serde::__private228::Ok(__Field::__field7),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<FunctionInfo>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FunctionInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct FunctionInfo",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        u32,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Vec<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field5 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field6 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    let __field7 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct FunctionInfo with 8 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(FunctionInfo {
                        name: __field0,
                        visibility: __field1,
                        arg_count: __field2,
                        arg_types: __field3,
                        return_type: __field4,
                        is_async: __field5,
                        is_unsafe: __field6,
                        is_const: __field7,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<u32> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<Vec<String>> = _serde::__private228::None;
                    let mut __field4: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field5: _serde::__private228::Option<bool> = _serde::__private228::None;
                    let mut __field6: _serde::__private228::Option<bool> = _serde::__private228::None;
                    let mut __field7: _serde::__private228::Option<bool> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "visibility",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "arg_count",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "arg_types",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private228::Option::is_some(&__field4) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "return_type",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private228::Option::is_some(&__field5) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_async",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                );
                            }
                            __Field::__field6 => {
                                if _serde::__private228::Option::is_some(&__field6) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_unsafe",
                                        ),
                                    );
                                }
                                __field6 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                );
                            }
                            __Field::__field7 => {
                                if _serde::__private228::Option::is_some(&__field7) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_const",
                                        ),
                                    );
                                }
                                __field7 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("visibility")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("arg_count")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("arg_types")?
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private228::Some(__field4) => __field4,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("return_type")?
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private228::Some(__field5) => __field5,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("is_async")?
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private228::Some(__field6) => __field6,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("is_unsafe")?
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private228::Some(__field7) => __field7,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("is_const")?
                        }
                    };
                    _serde::__private228::Ok(FunctionInfo {
                        name: __field0,
                        visibility: __field1,
                        arg_count: __field2,
                        arg_types: __field3,
                        return_type: __field4,
                        is_async: __field5,
                        is_unsafe: __field6,
                        is_const: __field7,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "name",
                "visibility",
                "arg_count",
                "arg_types",
                "return_type",
                "is_async",
                "is_unsafe",
                "is_const",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "FunctionInfo",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<FunctionInfo>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
/// Information about an import statement found in the AST
pub struct ImportInfo {
    pub path: String,
    pub alias: Option<String>,
    pub is_external: bool,
    pub source_crate: Option<String>,
    pub git_source_url: Option<String>,
    pub git_branch: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ImportInfo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "path",
            "alias",
            "is_external",
            "source_crate",
            "git_source_url",
            "git_branch",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.path,
            &self.alias,
            &self.is_external,
            &self.source_crate,
            &self.git_source_url,
            &&self.git_branch,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "ImportInfo",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ImportInfo {
    #[inline]
    fn clone(&self) -> ImportInfo {
        ImportInfo {
            path: ::core::clone::Clone::clone(&self.path),
            alias: ::core::clone::Clone::clone(&self.alias),
            is_external: ::core::clone::Clone::clone(&self.is_external),
            source_crate: ::core::clone::Clone::clone(&self.source_crate),
            git_source_url: ::core::clone::Clone::clone(&self.git_source_url),
            git_branch: ::core::clone::Clone::clone(&self.git_branch),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ImportInfo {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ImportInfo",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "path",
                &self.path,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "alias",
                &self.alias,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_external",
                &self.is_external,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "source_crate",
                &self.source_crate,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "git_source_url",
                &self.git_source_url,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "git_branch",
                &self.git_branch,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ImportInfo {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        4u64 => _serde::__private228::Ok(__Field::__field4),
                        5u64 => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "path" => _serde::__private228::Ok(__Field::__field0),
                        "alias" => _serde::__private228::Ok(__Field::__field1),
                        "is_external" => _serde::__private228::Ok(__Field::__field2),
                        "source_crate" => _serde::__private228::Ok(__Field::__field3),
                        "git_source_url" => _serde::__private228::Ok(__Field::__field4),
                        "git_branch" => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"path" => _serde::__private228::Ok(__Field::__field0),
                        b"alias" => _serde::__private228::Ok(__Field::__field1),
                        b"is_external" => _serde::__private228::Ok(__Field::__field2),
                        b"source_crate" => _serde::__private228::Ok(__Field::__field3),
                        b"git_source_url" => _serde::__private228::Ok(__Field::__field4),
                        b"git_branch" => _serde::__private228::Ok(__Field::__field5),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<ImportInfo>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ImportInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct ImportInfo",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        bool,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    let __field5 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct ImportInfo with 6 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(ImportInfo {
                        path: __field0,
                        alias: __field1,
                        is_external: __field2,
                        source_crate: __field3,
                        git_source_url: __field4,
                        git_branch: __field5,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<bool> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    let mut __field4: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    let mut __field5: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("path"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("alias"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_external",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "source_crate",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private228::Option::is_some(&__field4) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "git_source_url",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private228::Option::is_some(&__field5) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "git_branch",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("path")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("alias")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("is_external")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("source_crate")?
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private228::Some(__field4) => __field4,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("git_source_url")?
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private228::Some(__field5) => __field5,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("git_branch")?
                        }
                    };
                    _serde::__private228::Ok(ImportInfo {
                        path: __field0,
                        alias: __field1,
                        is_external: __field2,
                        source_crate: __field3,
                        git_source_url: __field4,
                        git_branch: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "path",
                "alias",
                "is_external",
                "source_crate",
                "git_source_url",
                "git_branch",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ImportInfo",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<ImportInfo>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
/// Comprehensive AST analysis data for a Rust project
pub struct AstStatistics {
    pub node_type_counts: HashMap<String, u32>,
    pub variable_declarations: Vec<VariableInfo>,
    pub function_definitions: Vec<FunctionInfo>,
    pub import_statements: Vec<ImportInfo>,
}
#[automatically_derived]
impl ::core::fmt::Debug for AstStatistics {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "AstStatistics",
            "node_type_counts",
            &self.node_type_counts,
            "variable_declarations",
            &self.variable_declarations,
            "function_definitions",
            &self.function_definitions,
            "import_statements",
            &&self.import_statements,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for AstStatistics {
    #[inline]
    fn clone(&self) -> AstStatistics {
        AstStatistics {
            node_type_counts: ::core::clone::Clone::clone(&self.node_type_counts),
            variable_declarations: ::core::clone::Clone::clone(
                &self.variable_declarations,
            ),
            function_definitions: ::core::clone::Clone::clone(
                &self.function_definitions,
            ),
            import_statements: ::core::clone::Clone::clone(&self.import_statements),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for AstStatistics {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "AstStatistics",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "node_type_counts",
                &self.node_type_counts,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "variable_declarations",
                &self.variable_declarations,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "function_definitions",
                &self.function_definitions,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "import_statements",
                &self.import_statements,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AstStatistics {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "node_type_counts" => _serde::__private228::Ok(__Field::__field0),
                        "variable_declarations" => {
                            _serde::__private228::Ok(__Field::__field1)
                        }
                        "function_definitions" => {
                            _serde::__private228::Ok(__Field::__field2)
                        }
                        "import_statements" => {
                            _serde::__private228::Ok(__Field::__field3)
                        }
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"node_type_counts" => {
                            _serde::__private228::Ok(__Field::__field0)
                        }
                        b"variable_declarations" => {
                            _serde::__private228::Ok(__Field::__field1)
                        }
                        b"function_definitions" => {
                            _serde::__private228::Ok(__Field::__field2)
                        }
                        b"import_statements" => {
                            _serde::__private228::Ok(__Field::__field3)
                        }
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<AstStatistics>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AstStatistics;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct AstStatistics",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        HashMap<String, u32>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct AstStatistics with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<VariableInfo>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct AstStatistics with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Vec<FunctionInfo>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct AstStatistics with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Vec<ImportInfo>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct AstStatistics with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(AstStatistics {
                        node_type_counts: __field0,
                        variable_declarations: __field1,
                        function_definitions: __field2,
                        import_statements: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<
                        HashMap<String, u32>,
                    > = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<Vec<VariableInfo>> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<Vec<FunctionInfo>> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<Vec<ImportInfo>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "node_type_counts",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        HashMap<String, u32>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variable_declarations",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<VariableInfo>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "function_definitions",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<FunctionInfo>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "import_statements",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<ImportInfo>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("node_type_counts")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field(
                                "variable_declarations",
                            )?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field(
                                "function_definitions",
                            )?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("import_statements")?
                        }
                    };
                    _serde::__private228::Ok(AstStatistics {
                        node_type_counts: __field0,
                        variable_declarations: __field1,
                        function_definitions: __field2,
                        import_statements: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "node_type_counts",
                "variable_declarations",
                "function_definitions",
                "import_statements",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AstStatistics",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<AstStatistics>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::default::Default for AstStatistics {
    #[inline]
    fn default() -> AstStatistics {
        AstStatistics {
            node_type_counts: ::core::default::Default::default(),
            variable_declarations: ::core::default::Default::default(),
            function_definitions: ::core::default::Default::default(),
            import_statements: ::core::default::Default::default(),
        }
    }
}
