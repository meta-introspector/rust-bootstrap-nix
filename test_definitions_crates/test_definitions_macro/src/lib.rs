pub mod prelude;
use crate::prelude::*;
extern crate syn;
struct TestDefinitionArgs {
    name: Ident,
    path: LitStr,
    mode: LitStr,
    suite: LitStr,
    default: LitBool,
    host: LitBool,
    compare_mode: syn::Expr,
}
impl syn::parse::Parse for TestDefinitionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let mut path = None;
        let mut mode = None;
        let mut suite = None;
        let mut default = None;
        let mut host = None;
        let mut compare_mode = None;
        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<syn::Token![:]>()?;
            match key.to_string().as_str() {
                "path" => path = Some(content.parse()?),
                "mode" => mode = Some(content.parse()?),
                "suite" => suite = Some(content.parse()?),
                "default" => default = Some(content.parse()?),
                "host" => host = Some(content.parse()?),
                "compare_mode" => compare_mode = Some(content.parse()?),
                _ => return Err(content.error(format!("unexpected key: {}", key))),
            }
            if !content.is_empty() {
                content.parse::<syn::Token![,]>()?;
            }
        }
        Ok(TestDefinitionArgs {
            name,
            path: path.ok_or_else(|| content.error("expected `path`"))?,
            mode: mode.ok_or_else(|| content.error("expected `mode`"))?,
            suite: suite.ok_or_else(|| content.error("expected `suite`"))?,
            default: default.ok_or_else(|| content.error("expected `default`"))?,
            host: host.ok_or_else(|| content.error("expected `host`"))?,
            compare_mode: compare_mode
                .ok_or_else(|| content.error("expected `compare_mode`"))?,
        })
    }
}
#[proc_macro]
pub fn test_definitions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as TestDefinitionArgs);
    let name = &args.name;
    let path = &args.path;
    let mode = &args.mode;
    let suite = &args.suite;
    let default = &args.default;
    let host = &args.host;
    let compare_mode = &args.compare_mode;
    let expanded = quote::quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)] pub struct # name { pub compiler :
        Compiler, pub target : TargetSelection, } impl Step for # name { type Output =
        (); const DEFAULT : bool = # default; const ONLY_HOSTS : bool = # host; fn
        should_run(run : ShouldRun) -> ShouldRun { run.suite_path(# path) } fn
        make_run(run : RunConfig) { let compiler = run.builder.compiler(run.builder
        .top_stage, run.builder.build_triple()); run.builder.ensure(# name { compiler,
        target : run.target }); } fn run(self, builder : & Builder) { builder
        .ensure(crate ::compiletest::Compiletest { compiler : self.compiler, target :
        self.target, mode : # mode, suite : # suite, path : # path, compare_mode : #
        compare_mode, }) } }
    };
    expanded.into()
}
