use syn::UseTree;
use pipeline_traits::UseStatement;

pub fn flatten_use_tree(
    base_path: &mut Vec<String>,
    use_tree: &UseTree,
    flat_uses: &mut Vec<UseStatement>,
) {
    match use_tree {
        UseTree::Path(path) => {
            base_path.push(path.ident.to_string());
            flatten_use_tree(base_path, &path.tree, flat_uses);
            base_path.pop();
        }
        UseTree::Name(_name) => {
            let full_path = base_path.join("::");
            flat_uses.push(UseStatement {
                statement: format!("use {};", full_path),
                error: None,
                git_details: None,
                nix_details: None,
                rust_details: None,
                cargo_details: None,
                syn_details: None,
                llvm_details: None,
                linux_details: None,
            });
        }
        UseTree::Rename(rename) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            flat_uses.push(UseStatement {
                statement: format!("use {} as {};", full_path, rename.rename.to_string()),
                error: None,
                git_details: None,
                nix_details: None,
                rust_details: None,
                cargo_details: None,
                syn_details: None,
                llvm_details: None,
                linux_details: None,
            });
        }
        UseTree::Glob(_glob) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str("* ");
            flat_uses.push(UseStatement {
                statement: format!("use {};", full_path),
                error: None,
                git_details: None,
                nix_details: None,
                rust_details: None,
                cargo_details: None,
                syn_details: None,
                llvm_details: None,
                linux_details: None,
            });
        }
        UseTree::Group(group) => {
            for tree in group.items.iter() {
                flatten_use_tree(base_path, tree, flat_uses);
            }
        }
    }
}
