#![allow(non_snake_case)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, punctuated::Punctuated, Attribute, ItemEnum, Path, Token, Ident};
use quote::quote;

const MAPPING_ATTR: &'static str = "mapping";

fn attributeIs(attr: &Attribute, name: &str) -> bool {
       attr.path().segments.last()
             .map(|attrName| attrName.ident == name)
             .unwrap_or(false)
}

fn getMapFromVariant(attr: &Attribute) -> Option<Path> {
      attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
            .map(|args| {
                  args.get(0).cloned()
            }).ok().flatten()
}

fn getEnumNameFromPath(path: &Path) -> Option<&Ident> {
       path.segments.get(path.segments.len() - 2).map(|fragment| &fragment.ident)
}

fn generateMatchBody(enumDefinition: ItemEnum, fromEnumName: &Ident) -> TokenStream2 {
      let toEnumName = &enumDefinition.ident;
      // For a enum without #[mapping] annotation, generate a match with From::Variant => To::Variant
      // For a enum with #[mapping] annotations, generate a match with From::$mapping => To::Variant
      // ONLY for variants with #[mapping] annotations
      let variantsWithMappingAttr: Vec<_> = enumDefinition.variants.iter().filter_map(|variant| {
            variant.attrs.iter().find_map(|attr| {
                  if attributeIs(attr, MAPPING_ATTR) {
                        let variantName = &variant.ident;

                        match getMapFromVariant(attr) {
                              Some(path) => (getEnumNameFromPath(&path) == Some(fromEnumName))
                                    .then_some(quote! { #path => #toEnumName::#variantName }),
                              None => Some(quote! {
                                    #fromEnumName::#variantName => #toEnumName::#variantName
                              })
                        }
                  }
                  else {
                        None
                  }
            })
      }).collect();

      if variantsWithMappingAttr.len() != 0 {
           variantsWithMappingAttr.iter().fold(quote! {}, |acc, tree| {
                 quote! {
                       #acc
                       #tree,
                 }
           })
      }
      else {
            enumDefinition.variants.iter().fold(quote! {}, |acc, variant| {
                  let variantName = &variant.ident;

                  quote! {
                        #acc
                        #fromEnumName::#variantName => #toEnumName::#variantName,
                  }
            })
      }
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
      let fromEnum = parse_macro_input!(attrs as Ident);
      let bodyCopy = body.clone();
      let toEnum = parse_macro_input!(bodyCopy as ItemEnum);
      let toEnumName = toEnum.ident.clone();
      let enumDefinition = removeAttributes(&toEnum);

      let matchBody = generateMatchBody(toEnum, &fromEnum);

      quote! {
            #enumDefinition

            impl ::core::convert::From<#fromEnum> for #toEnumName {
                  fn from(other: #fromEnum) -> Self {
                        match other {
                              #matchBody
                        }
                  }
            }
      }.into()
}

