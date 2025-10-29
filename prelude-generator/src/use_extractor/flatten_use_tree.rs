use syn::UseTree;
use crate::pipeline;

pub fn flatten_use_tree(
    base_path: &mut Vec<String>,
    use_tree: &UseTree,
    flat_uses: &mut Vec<pipeline::UseStatement>,
) {
    match use_tree {
        UseTree::Path(path) => {
            base_path.push(path.ident.to_string());
            flatten_use_tree(base_path, &path.tree, flat_uses);
            base_path.pop();
        }
        UseTree::Name(name) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&name.ident.to_string());
            flat_uses.push(pipeline::UseStatement {
                statement: format!("use {};", full_path),
                error: None,
            });
        }
        UseTree::Rename(rename) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&rename.ident.to_string());
            flat_uses.push(pipeline::UseStatement {
                statement: format!("use {} as {};", full_path, rename.rename.to_string()),
                error: None,
            });
        }
        UseTree::Glob(_glob) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str("* ");
            flat_uses.push(pipeline::UseStatement {
                statement: format!("use {};", full_path),
                error: None,
            });
        }
        UseTree::Group(group) => {
            for tree in group.items.iter() {
                flatten_use_tree(base_path, tree, flat_uses);
            }
        }
    }
}
