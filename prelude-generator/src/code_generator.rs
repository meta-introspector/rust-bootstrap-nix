use quote::quote;

pub fn use_item_to_string(use_item: &syn::ItemUse) -> String {
    quote!(#use_item).to_string()
}
