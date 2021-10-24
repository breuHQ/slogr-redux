use proc_macro::TokenStream;
use quote::{quote, ToTokens};

// #[proc_macro_derive(ToBytes)]
// pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let ast = parse_macro_input!(input as DeriveInput);
//   let name = &ast.ident;

//   eprintln!("{:#?}", ast);

//   let fields = match &ast.data {
//     syn::Data::Struct( syn::DataStruct { fields: syn::Fields::Named(fields), .. } ) => &fields.named,
//     // syn::Data::Enum(_) => todo!(),
//     // syn::Data::Union(_) => todo!(),
//     _ => todo!(),
//   };

//   // let field_names = fields.iter().map(|field| &field.ident);
//   // let field_types = fields.iter().map(|field| &field.ty);

//   let expanded = is_struct(name, fields);
//   expanded.into()

//   // let expanded = quote! {
//   //   impl #name {
//   //     fn to_bytes(&self) -> Vec<u8> {
//   //       let mut bytes: Vec<u8> = Vec::new();
//   //       bytes
//   //     }
//   //   }

//   //   struct X {
//   //     #fields
//   //   }
//   // };
// }

// fn is_struct(name: &syn::Ident, fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>) -> proc_macro2::TokenStream {
//   let field_names = fields.iter().map(|field| &field.ident);
//   let field_types = fields.iter().map(|field| &field.ty);

//   quote! {
//     impl #name {
//       fn to_bytes(&self) -> Vec<u8> {
//         let mut bytes: Vec<u8> = Vec::new();
//         #(
//           bytes.extend_from_slice(&self.#field_names);
//         )*
//         bytes
//       }
//     }
//   }
// }

#[proc_macro_derive(ToBytes)]
pub fn derive(tokens: TokenStream) -> TokenStream {
  let tokens_item = tokens.clone();
  let tokens_struct = tokens.clone();
  let items = syn::parse_macro_input!(tokens_item as syn::Item);
  let m = match items {
    syn::Item::Struct(item) => {
      let name = &item.ident;
      let statements = match &item.fields {
        syn::Fields::Named(ref fields) => {
          // eprint!("{:#?}", field);
          let vary_by_type = fields.named.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let statement = match field_type {
              syn::Type::Array(syn::TypeArray { elem, .. }) => {
                let ty = elem.as_ref();
                match ty {
                  syn::Type::Path(typepath)
                    if typepath.qself.is_none()
                      && typepath.path.leading_colon.is_none()
                      && typepath.path.segments.len() == 1 && typepath.path.is_ident("u8") =>
                  {
                    quote! {
                      bytes.extend_from_slice(&self.#field_name);
                    }
                  },
                  _ => todo!(),
                }
              }
              syn::Type::Path(ty) if ty.path.clone().is_ident("u32") => {
                quote! {
                  bytes.extend_from_slice(&(self.#field_name as u32).to_be_bytes().to_vec());
                }
              },
              _ => todo!(),
            };
            statement
          });
          vary_by_type
        }
        _ => todo!(),
      };
      quote! {
        impl #name {
          fn to_bytes(&self) -> Vec<u8> {
            let mut bytes: Vec<u8> = Vec::new();
            #(
              #statements
            )*
            bytes
          }
        }
      }
    }
    _ => todo!(),
  };
  m.into()
  // let s = syn::parse_macro_input!(tokens_struct as syn::ItemStruct);
  // let n = &s.ident;
  // let expanded = quote! {
  //   impl #n {
  //     fn to_bytes(&self) -> Vec<u8> {
  //       let mut bytes: Vec<u8> = Vec::new();
  //       bytes
  //     }
  //   }
  // };
  // expanded.into()
}
