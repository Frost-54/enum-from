#![allow(non_snake_case)]

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use syn::{parse_macro_input, punctuated::Punctuated, Attribute, Token, ItemEnum, Meta, Path};
use quote::{quote, ToTokens};

const MAPPING_ATTR: &'static str = "mapping";

fn attributeIs(attr: &Attribute, name: &str) -> bool {
       attr.path().segments.last()
             .map(|attrName| attrName.ident == name)
             .unwrap_or(false)
}

fn generateMatchBody(enumDefinition: ItemEnum, fromEnumName: &TokenTree2) -> TokenStream2 {
      let toEnumName = &enumDefinition.ident;

      enumDefinition.variants.iter().fold(quote! {}, |acc, variant| {
            let variantName = &variant.ident;

            let from = variant.attrs.iter().filter_map(|attr| {
                  match &attr.meta {
                        Meta::List(ref list) if attributeIs(attr, MAPPING_ATTR) => {
                              list.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                                    .map(|args| {
                                          if args.len() != 1 {
                                                panic!("Expect exactly one enum variant in {} to map from", fromEnumName.to_string());
                                          }
                                          args[0].clone().into_token_stream()
                                    }).ok()
                        },
                        _ => None
                  }
            }).next().unwrap_or_else(|| quote! { #fromEnumName::#variantName });

            quote! {
                  #acc
                  #from => #toEnumName::#variantName,
            }
      })
}

fn removeAttributes(enumDefinition: &ItemEnum) -> TokenStream2 {
      let body= enumDefinition.variants.iter()
            .fold(quote! {}, |acc, variant| {
                  let mut variant = variant.clone();
                  
                  variant.attrs = variant.attrs.iter()
                        .filter(|attr| !attributeIs(attr, MAPPING_ATTR))
                        .cloned()
                        .collect();

                  quote! {
                        #acc
                        #variant,
                  }
      });
      let visibility = &enumDefinition.vis;
      let attrs = &enumDefinition.attrs;
      let enumName = &enumDefinition.ident;
      let generics = &enumDefinition.generics;

      quote! {
            #(#attrs)*
            #visibility enum #enumName #generics {
                  #body
            }
      }
}

#[proc_macro_attribute]
pub fn from(attrs: TokenStream, body: TokenStream) -> TokenStream {
      let fromEnum = TokenStream2::from(attrs).into_iter()
            .next()
            .expect("Expect target enum");
      let bodyCopy = body.clone();
      let toEnum = parse_macro_input!(bodyCopy as ItemEnum);
      let toEnumName = toEnum.ident.clone();
      let enumDefinition = removeAttributes(&toEnum);

      let matchBody = generateMatchBody(toEnum, &fromEnum);

      quote! {
            #enumDefinition

            impl From<#fromEnum> for #toEnumName {
                  fn from(other: #fromEnum) -> Self {
                        match other {
                              #matchBody
                        }
                  }
            }
      }.into()
}

#[proc_macro_attribute]
pub fn mapping(_attrs: TokenStream, tokens: TokenStream) -> TokenStream {
      tokens
}
