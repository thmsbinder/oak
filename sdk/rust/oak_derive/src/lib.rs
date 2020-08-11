//
// Copyright 2020 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, Fields, Ident};

/// Automatically derives the `HandleVisit` trait for
/// structs and enums generated by prost.
#[proc_macro_derive(HandleVisit)]
pub fn handle_visit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    match &ast.data {
        Data::Struct(data) => struct_impls(name, data),
        Data::Enum(data) => enum_impls(name, data),
        Data::Union(_) => panic!("HandleVisit cannot be derived for unions"),
    }
    .into()
}

fn struct_impls(name: &Ident, data: &syn::DataStruct) -> TokenStream {
    let accessors: Vec<TokenStream> = match &data.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .flat_map(|f| f.ident.clone())
            .map(|i| quote!(self.#i))
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let index = syn::Index::from(i);
                quote!(self.#index)
            })
            .collect(),
        Fields::Unit => Vec::new(),
    };
    let body = accessors_visit(&accessors);

    quote! {
        impl ::oak::handle::HandleVisit for #name {
            fn visit<F: FnMut(&mut ::oak::Handle)>(&mut self, visitor: F) -> F {
                #body
            }
        }
    }
}

fn enum_impls(name: &Ident, data: &syn::DataEnum) -> TokenStream {
    let variants: Vec<TokenStream> = data.variants.iter().map(variant_impl).collect();
    quote! {
        impl ::oak::handle::HandleVisit for #name {
            fn visit<F: FnMut(&mut ::oak::Handle)>(&mut self, visitor: F) -> F {
                match self {
                    #(
                        #name::#variants,
                    )*
                }
            }
        }
    }
}

fn variant_impl(variant: &syn::Variant) -> TokenStream {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Named(fields) => {
            let fields: Vec<Ident> = fields.named.iter().flat_map(|f| f.ident.clone()).collect();
            let body = accessors_visit(&fields);
            quote! {
                #variant_ident { #( #fields ),* } => {
                    #body
                }
            }
        }
        Fields::Unnamed(fields) => {
            // Name the fields _0, _1, ...
            let accessors: Vec<Ident> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("_{}", i))
                .collect();
            let body = accessors_visit(&accessors);
            quote! {
                #variant_ident( #( #accessors ),* ) => {
                    #body
                }
            }
        }
        Fields::Unit => quote! {
            #variant => visitor
        },
    }
}

fn accessors_visit<T: ToTokens>(accessors: &[T]) -> TokenStream {
    quote! {
        let mut _v = visitor;
        #(
            _v  = #accessors.visit(_v);
        )*
        _v
    }
}
