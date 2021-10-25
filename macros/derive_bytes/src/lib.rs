use proc_macro2;
use quote::quote;

/// Given a struct, a procedural macro `ToBytes` to generate `to_bytes` funtion on the struct.
#[proc_macro_derive(ToBytes)]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let tokens_item = tokens.clone();
  let item = syn::parse_macro_input!(tokens_item as syn::Item);
  let expanded = to_bytes(item);
  expanded.into()
}

/// expands `syn::Item` and matches
fn to_bytes(item: syn::Item) -> proc_macro2::TokenStream {
  match item {
    syn::Item::Struct(item_struct) => expand_item_struct(item_struct),
    _ => unimplemented!(),
  }
}


// expands `syn::ItemStruct` to generate the complete implementation of `ToBytes`
fn expand_item_struct(item_struct: syn::ItemStruct) -> proc_macro2::TokenStream {
  let name = &item_struct.ident;
  let fn_lines = expand_fields(&item_struct.fields);

  quote! {
    impl #name {
      fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        #fn_lines
        bytes
      }
    }
  }
}

/// expand `syn::Fields` and generate the appropriate line for the `to_bytes` function.
fn expand_fields(fields: &syn::Fields) -> proc_macro2::TokenStream {
  match fields {
    syn::Fields::Named(ref named) => expand_fields_named(named),
    _ => unimplemented!(),
  }
}

/// expand `FieldNamed` to generate the right statement for `bytes.extend_from_slice`
/// based on the type of the field.
fn expand_fields_named(fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
  let lines = fields.named.iter().map(|field| expand_field(field));
  quote! {
    #(#lines)*
  }
}

/// expand `syn::Field` and generate the appropriate line for the `to_bytes` function.
fn expand_field(field: &syn::Field) -> proc_macro2::TokenStream {
  let field_name = &field.ident;
  let field_type = &field.ty;

  let fn_line = match field_type {
    syn::Type::Array(syn::TypeArray { elem, .. }) => expand_array(field_name, elem),
    syn::Type::Path(type_path, ..) if check_path(&type_path.path) => expand_path(field_name, type_path),
    _ => unimplemented!(),
  };

  quote! {
    #fn_line
  }
}

/// expand `syn::Type::Array` if the array contains only unsigned integers.
fn expand_array(field_name: &Option<syn::Ident>, elem: &Box<syn::Type>) -> proc_macro2::TokenStream {
  let ref_type = elem.as_ref();
  match ref_type {
    syn::Type::Path(type_path, ..) if check_type_path(type_path) => expand_array_path(field_name),
    _ => unimplemented!("Supported types for an array are all unsigned integers"),
  }
}

/// expand `syn::Type::Path` directly for the field `syn::FieldNamed` if the type is any of the unsigned integer.
fn expand_path(field_name: &Option<syn::Ident>, type_path: &syn::TypePath) -> proc_macro2::TokenStream {
  quote! {
    bytes.extend_from_slice(&(self.#field_name as #type_path).to_be_bytes().to_vec());
  }
}

/// expand `syn::Type::Path` inside `syn::Type::Array` if the type is any of the unsigned integer.
fn expand_array_path(field_name: &Option<syn::Ident>) -> proc_macro2::TokenStream {
  quote! {
    bytes.extend_from_slice(&(self.#field_name));
  }
}

/// Checks if the provided `syn::Path` has valid ident
fn check_path(path: &syn::Path) -> bool {
  path.clone().is_ident("u8")
    || path.clone().is_ident("u16")
    || path.clone().is_ident("u32")
    || path.clone().is_ident("u64")
    || path.clone().is_ident("u128")
    || path.clone().is_ident("usize")
    || path.clone().is_ident("usize")
}

// check if the conditions are met for `syn::TypePath`
fn check_type_path(type_path: &syn::TypePath) -> bool {
  type_path.qself.is_none()
    && type_path.path.clone().leading_colon.is_none()
    && type_path.path.clone().segments.len() == 1
    && check_path(&type_path.path)
}
