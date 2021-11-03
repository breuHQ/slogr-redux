//! Quickly generate `to_bytes()` and `from_bytes` methods on a struct to for easy data conversions.
//!
//! We have made the following assumptions about the fields of the struct:
//!
//! - The fields are public.
//! - The fields will have the following types
//!   - `u8`
//!   - `u16`
//!   - `u32`
//!   - `u64`
//!   - `u128`
//!   - `usize`
//!   - array of `u8`
//!   - an enum
//! - For enum, we must attach a `#[byte_me($size)]` attribute, where size is any of the positive integer types.
//! - The enum declration must `#[derive(FromPrimitive)]` from the `num-derive` crate.
//! 
//! The `num-derive` crate is required to generate the `FromPrimitive` trait for enums. Having said that, the same 
//! functionality can be achieved using `num-enum` crate. It provides furthur control over the enum data types,
//! and might prove handy. here is the discussion on the topic.
//! 
//! https://github.com/illicitonion/num_enum/issues/61#issuecomment-955804109

use crate::models::{ByteMeField, ByteMeStruct};
mod models;
mod utils;

/// Quickly generate `to_bytes()` and `from_bytes` methods on a struct to for easy data conversions.
#[proc_macro_derive(ByteMe, attributes(byte_me))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let strukt = syn::parse_macro_input!(tokens as ByteMeStruct);

  let fn_lines_to_bytes = strukt.fields.iter().clone().map(|field| to_bytes_fn_factory(field));

  let count = core::cell::Cell::new(0_usize);
  let count_ = &count;
  let fn_line_from_bytes = strukt
    .fields
    .iter()
    .clone()
    .map(|field| from_bytes_fn_factory(field, count_));

  let name = &strukt.ident;
  let size: usize = strukt.fields.clone().iter().clone().map(|field| field.size).sum();
  let fields = strukt.fields.iter().clone().map(|field| get_field_name(field));
  let processed = quote::quote! {
    impl #name {
      /// Size of the struct in bytes
      pub const SIZE: usize = #size;

      /// Convert the struct to a byte array.
      pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        #(#fn_lines_to_bytes)*
        bytes
      }

      /// Convert the byte array to a struct.
      pub fn from_bytes(bytes: Vec<u8>) -> Self {
        // This import from the `num-traits` crate is required to use the `FromPrimitive` trait.
        // If the field is an enum, or any of the positive integer types, we quickly get the required
        // value from a [u8] array.
        use num_traits::FromPrimitive;
        // Converting `[u8]` to fields based on their length.
        #(#fn_line_from_bytes)*
        // Returning the struct.
        Self {
          #(#fields)*
        }
      }

      /// Gets the delimiter as a vector of bytes.
      pub fn get_delimiter(self) -> Vec<u8> {
        (Self::SIZE as u16).to_be_bytes().to_vec()
      }
    }
  };

  processed.into()
}

/// Creates a line for `to_bytes` function for a single field depending on the data_type
fn to_bytes_fn_factory(field: &ByteMeField) -> proc_macro2::TokenStream {
  let name = &field.ident;
  let data_type = &field.data_type;

  if field.is_array {
    quote::quote! {
      bytes.extend_from_slice(&(self.#name));
    }
  } else {
    quote::quote! {
      bytes.extend_from_slice(&(self.#name as #data_type).to_be_bytes().to_vec());
    }
  }
}

/// Creates a line for `from_bytes` function for a single field depending on the data_type
fn from_bytes_fn_factory(field: &ByteMeField, count: &core::cell::Cell<usize>) -> proc_macro2::TokenStream {
  let name = &field.ident;
  let size = &field.size;
  let data_type = &field.data_type;

  let start = count.get();
  let end = start + field.size;
  count.set(end);

  // The first line is the same for all data types
  let lines = quote::quote! {
    let #name: [u8; #size] = bytes[#start .. #end].try_into().unwrap();
  };

  // If the field is not a `[u8]` and if it doesn't have an atrribute, we can safely assume it is a positive integer.
  // In this case, we would have to return the value as the type on the field.
  let lines = if !field.is_array && field.attribute_for.is_none() {
    quote::quote! {
      #lines
      let #name = #data_type::from_be_bytes(#name);
    }
  } else {
    quote::quote! {
      #lines
    }
  };

  // If the field is not an array
  let lines = if !field.is_array && field.attribute_for.is_some() {
    let enum_name = &field.attribute_for.clone();
    let enum_data_type = data_type.to_string();
    let from_enum_data_type = syn::Ident::new(format!("from_{}", enum_data_type).as_str(), proc_macro2::Span::call_site());
    quote::quote! {
      #lines
      let #name = #data_type::from_be_bytes(#name);
      let #name = #enum_name::#from_enum_data_type(#name).unwrap();
    }
  } else {
    quote::quote! {
      #lines
    }
  };

  lines
}

/// Given `ByteMeField`, returns a TokenStream with only name of the field
fn get_field_name(field: &ByteMeField) -> proc_macro2::TokenStream {
  let name = &field.ident;
  quote::quote! {#name,}
}
