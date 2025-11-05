use std::collections::HashMap;
use split_expanded_lib::Declaration;

pub fn layer_declarations(
    all_declarations: Vec<Declaration>,
) -> HashMap<usize, Vec<Declaration>> {
    let mut layered_decls: HashMap<usize, Vec<Declaration>> = HashMap::new();
    let mut remaining_decls = all_declarations;
    let mut current_layer_num = 0;

    loop {
        if remaining_decls.is_empty() {
            break;
        }

        let mut current_layer_decls = Vec::new();
        let mut next_remaining_decls = Vec::new();
        let mut current_layer_idents = std::collections::HashSet::new();

        // Identify declarations for the current layer
        for decl in remaining_decls.into_iter() {
            let has_unresolved_deps = decl.referenced_types.iter().any(|dep| {
                !current_layer_idents.contains(dep) && !layered_decls.values().flatten().any(|d| d.get_identifier() == *dep)
            });

            if !has_unresolved_deps {
                current_layer_idents.insert(decl.get_identifier());
                current_layer_decls.push(decl);
            } else {
                next_remaining_decls.push(decl);
            }
        }

        if current_layer_decls.is_empty() {
            // No new declarations could be layered, break to prevent infinite loop
            break;
        }

        layered_decls.insert(current_layer_num, current_layer_decls);
        remaining_decls = next_remaining_decls;
        current_layer_num += 1;

        if current_layer_num > 8 { // Stop at layer 8 as per requirement
            break;
        }
    }

    layered_decls
}