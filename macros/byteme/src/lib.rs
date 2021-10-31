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

use quote::ToTokens;

/// Quickly generate `to_bytes()` and `from_bytes` methods on a struct to for easy data conversions.
#[proc_macro_derive(ByteMe, attributes(byte_me))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let strukt = syn::parse_macro_input!(tokens as ByteMeStruct);

  let fn_lines_to_bytes = strukt.fields.iter().clone().map(|field| to_bytes_fn_factory(field));

  let count = core::cell::Cell::new(0 as usize);
  let count_ = &count;
  let fn_line_from_bytes = strukt
    .fields
    .iter()
    .clone()
    .map(|field| from_bytes_fn_factory(field, count_));

  let name = &strukt.ident;
  let size: usize = strukt.fields.clone().iter().clone().map(|field| field.size).sum();
  let fields = strukt.fields.iter().clone().map(|field| get_field_name(&field));
  let processed = quote::quote! {
    impl #name {
      const SIZE: usize = #size;

      pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        #(#fn_lines_to_bytes)*
        bytes
      }

      pub fn from_bytes(bytes: Vec<u8>) -> Self {
        #(#fn_line_from_bytes)*
        Self {
          #(#fields)*
        }
      }
    }
  };

  processed.into()
}

/// Creates a line for `to_bytes` function for a single field depending on the data_type
fn to_bytes_fn_factory(field: &ByteMeField) -> proc_macro2::TokenStream {
  let name = &field.ident;
  let data_type = &field.data_type;

  if field.is_array == true {
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
fn from_bytes_fn_factory(field: &ByteMeField, count_: &core::cell::Cell<usize>) -> proc_macro2::TokenStream {
  let name = &field.ident;
  let size = &field.size;
  let data_type = &field.data_type;

  let start = count_.get();
  let end = start + field.size;
  count_.set(end);

  // The first line is the same for all data types
  let lines = quote::quote! {
    let #name: [u8; #size] = bytes[#start .. #end].try_into().unwrap();
  };

  // If it is not an array and if it doesn't have an atrribute, we can safely assume it is a positive integer.
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

  let lines = if !field.is_array && field.attribute_for.is_some() {
    let attribute_for = &field.attribute_for.clone();
    let prepend = data_type.to_string();
    let prepend = syn::Ident::new(format!("from_{}", prepend).as_str(), proc_macro2::Span::call_site());
    quote::quote! {
      #lines
      let #name = #data_type::from_be_bytes(#name);
      let #name = #attribute_for::#prepend(#name).unwrap();
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

/// Summary of the proc_macro::TokenStream
#[derive(Debug, Clone)]
struct ByteMeStruct {
  ident: syn::Ident,
  fields: Vec<ByteMeField>,
}

/// Represents a single field in the struct
#[derive(Debug, Clone)]
struct ByteMeField {
  /// Represents the name of the fields
  ident: syn::Ident,
  /// Represents the number of bytes that the field takes up.
  size: usize,
  /// Represents the positive integer type of the field.
  data_type: syn::Ident,
  /// Represents if the field is an array
  is_array: bool,
  /// Name of the enum for which the `#[byte_me($size)]` attribute is attached.
  attribute_for: Option<syn::Ident>,
}

/// Implements parser for ByteMeStruct
impl syn::parse::Parse for ByteMeStruct {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let strukt = input.parse::<syn::ItemStruct>()?;
    let ident = strukt.ident.clone();
    let mut fields: Vec<ByteMeField> = Vec::new();
    for f in strukt.fields {
      let field = ByteMeField::try_from(&f)?;
      fields.push(field);
    }
    Ok(Self { ident, fields })
  }
}

/// Implements TryFrom for ByteMeField
impl TryFrom<&syn::Field> for ByteMeField {
  type Error = syn::Error;

  /// Parses a field and returns a ByteMeField
  fn try_from(field: &syn::Field) -> Result<Self, Self::Error> {
    let ident = field
      .ident
      .clone()
      .ok_or_else(|| {
        syn::Error::new_spanned(
          field.into_token_stream(),
          "`ByteMe` only works for a struct with a named field",
        )
      })
      .unwrap();

    let attrs_ref = field.attrs.clone();

    let attribute = attrs_ref.iter().find(|attr| attr.path.is_ident("byte_me"));

    let attribute = if attribute.is_some() {
      let meta: syn::Meta = attribute.clone().unwrap().parse_args().unwrap();
      let segments = meta.path().clone().segments.clone();
      if segments.len() != 1 {
        return Err(syn::Error::new_spanned(
          field.into_token_stream(),
          "`byte_me` attribute can only have one argument",
        ));
      } else {
        Some(segments.clone().into_iter().next().unwrap().ident)
      }
    } else {
      None
    };

    let field_type = field.ty.clone();

    match field_type {
        // Check if have an array and the type is any of the unsigned integer.
        syn::Type::Array(syn::TypeArray {elem, len, ..}) => {
          let elem = elem.as_ref();
          let elem: syn::TypePath = syn::parse_quote!(#elem);
          let size: syn::LitInt = syn::parse_quote!(#len);
          let size = size.base10_parse::<usize>().unwrap();
          // Currently we only support u8 arrays
          if is_u8(&elem.path) == true {
            Ok(Self {
              ident,
              size,
              data_type: syn::Ident::new("u8", proc_macro2::Span::call_site()),
              is_array: true,
              attribute_for: None,
            })
          } else {
            Err(syn::Error::new_spanned(field.into_token_stream(), "`u8` is the only supported type for arrays"))
          }
        },
        // Check if the type is any of the unsigned integer.
        syn::Type::Path(syn::TypePath {path,..}) if is_positive_integer(&path) => {
          let data_type = Some(path.clone().segments.clone().into_iter().next().unwrap().ident.clone());
          let size: usize = get_byte_size_from_integer_type(data_type.clone().unwrap()).unwrap();
          Ok(Self {
            ident,
            size,
            data_type: data_type.unwrap(),
            is_array: false,
            attribute_for: None,
          })
        },
        // Check for the custom struct with a `#[byte_me]` attribute defining any of the unsigned integer.
        syn::Type::Path(syn::TypePath {path, ..}) if attribute.is_some() => {
          let attribute_for = Some(path.clone().segments.clone().into_iter().next().unwrap().ident.clone());
          let data_type: syn::Ident = syn::parse_quote!(#attribute);
          let size: usize = get_byte_size_from_integer_type(data_type.clone()).unwrap();
          Ok(Self {
            ident,
            size,
            data_type,
            is_array: false,
            attribute_for,
          })
        },
        // Raise Error if the conditions are not met.
        _ => Err(syn::Error::new_spanned(field.into_token_stream(), "Unsupported field. Try adding `byte_me($type)` attribute to the field where $type is any of the `u8`, `u16`, `u32`, `u64`, `u128` or `usize`."))
    }
  }
}

/// checks if `syn::Path` is of type `u8`
fn is_u8(path: &syn::Path) -> bool {
  path.clone().is_ident("u8")
}

/// checks if `syn::Path` is any of type `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
fn is_positive_integer(path: &syn::Path) -> bool {
  path.clone().is_ident("u8")
    || path.clone().is_ident("u16")
    || path.clone().is_ident("u32")
    || path.clone().is_ident("u64")
    || path.clone().is_ident("u128")
    || path.clone().is_ident("usize")
}

/// Returns the size of the field in bytes given the data type as a `syn::Ident`.
fn get_byte_size_from_integer_type(ident: syn::Ident) -> Result<usize, syn::Error> {
  match ident.to_string().as_str() {
    "u8" => Ok(1),
    "u16" => Ok(2),
    "u32" => Ok(4),
    "u64" => Ok(8),
    "u128" => Ok(16),
    "usize" => Ok(8),
    _ => Err(syn::Error::new_spanned(
      ident,
      "Unsupported type. We can only process positive integers or Enums",
    )),
  }
}
