//SPDX-FileCopyrightText: 2024 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2024 OneLiteFeatherNet

//Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
//associated documentation files (the "Software"), to deal in the Software without restriction,
//including without limitation the rights to use, copy, modify, merge, publish, distribute,
//sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice (including the next paragraph) shall be
//included in all copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
//NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
//NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::str::FromStr;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, DeriveInput, GenericArgument, Ident, ItemFn, PathArguments, ReturnType, Type,
};

#[proc_macro_derive(IntoPageRequest)]
pub fn into_page_request_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        impl crate::IntoPageRequest for #ident {
            fn into_page_request(&self) -> rbatis::plugin::page::PageRequest {
                let page_size = if self.page_size > 0 { self.page_size as u64 } else { 20 };
                let page = if self.page_token > 0 { self.page_token as u64 } else { 1 };

                rbatis::plugin::page::PageRequest::new(page, page_size)
            }
        }
    };

    expanded.into()
}

#[derive(Debug, FromMeta)]
struct DynamicCacheOptions {
    key: String,
    ttl: u16,
    #[darling(default)]
    refresh: bool,
}

#[proc_macro_attribute]
pub fn dynamic_cache(arguments: TokenStream, input: TokenStream) -> TokenStream {
    // parse the arguments
    let attributes = match NestedMeta::parse_meta_list(arguments.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let DynamicCacheOptions { key, ttl, refresh } =
        match DynamicCacheOptions::from_list(&attributes) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

    let input = parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = input;

    let return_type = match &sig.output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            return TokenStream::from(quote! {
                compile_error!("Function must have a return type");
            });
        }
    };

    // Check if the return type is a Result and extract the inner type
    let inner_type = if let Type::Path(type_path) = &**return_type {
        if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "Result" {
            if let PathArguments::AngleBracketed(args) = &type_path.path.segments[0].arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    inner_ty.clone()
                } else {
                    return TokenStream::from(quote! {
                        compile_error!("Expected a type argument for Result");
                    });
                }
            } else {
                return TokenStream::from(quote! {
                    compile_error!("Expected angle bracketed arguments for Result");
                });
            }
        } else {
            return TokenStream::from(quote! {
                compile_error!("Expected a Result return type");
            });
        }
    } else {
        return TokenStream::from(quote! {
            compile_error!("Expected a Result return type");
        });
    };

    let args = &sig.inputs;
    let args = args
        .iter()
        .map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    pat_ident.ident.clone()
                } else {
                    panic!("Expected argument to be an identifier pattern");
                }
            } else {
                panic!("Expected typed argument");
            }
        })
        .collect::<Vec<Ident>>();

    let invalidate = format_ident!("invalidate_{}", &sig.ident);
    let wrapper = format_ident!("__{}", &sig.ident);
    let skytable_tls = format_ident!("__{}_skytable_tls", &sig.ident);
    let skytable_tls_static = format_ident!("{}", skytable_tls.to_string().to_uppercase());
    let skytable = format_ident!("__{}_skytable", &sig.ident);
    let skytable_static = format_ident!("{}", skytable.to_string().to_uppercase());
    let memory = format_ident!("__{}_memory", &sig.ident);
    let memory_static = format_ident!("{}", memory.to_string().to_uppercase());

    let mut wrapper_sig = sig.clone();
    wrapper_sig.ident = wrapper.clone();

    let mut skytable_tls_sig = sig.clone();
    skytable_tls_sig.ident = skytable_tls.clone();

    let mut skytable_sig = sig.clone();
    skytable_sig.ident = skytable.clone();

    let mut memory_sig = sig.clone();
    memory_sig.ident = memory.clone();

    // in order to modify the cached proc macros we have to trick quote a bit
    let convert = proc_macro2::TokenStream::from_str(
        format!(
            "\"{{ {} }}\"",
            // TODO: may do this a bit cleaner
            key.to_token_stream()
                .to_string()
                .replace("\"", "")
                .replace("'", "\\\"")
        )
        .as_str(),
    )
    .unwrap();

    let skytable_tls_type = proc_macro2::TokenStream::from_str(
        format!(
            "\"crate::cache::SkytableCache<skytable::pool::ConnectionMgrTls, String, {}>\"",
            inner_type.to_token_stream().to_string()
        )
        .as_str(),
    )
    .unwrap();
    let skytable_tls_create = proc_macro2::TokenStream::from_str(
        format!(
            "\"{{ crate::cache::SkytableTlsCacheBuilder::new(
                    CONFIG.skytable_host().as_ref().unwrap().as_str(),
                    *CONFIG.skytable_port().as_ref().unwrap(),
                    CONFIG.skytable_username().as_ref().unwrap().as_str(),
                    CONFIG.skytable_password().as_ref().unwrap().as_str(),
                )
                .set_certificate(CONFIG.skytable_certificate().as_ref().unwrap().as_str())
                .set_space(CONFIG.skytable_space())
                .set_model(CONFIG.skytable_model())
                .set_refresh({})
                .set_lifetime(std::time::Duration::from_secs({}))
                .build()
                .await
                .unwrap()
            }}\"",
            refresh, ttl
        )
        .as_str(),
    )
    .unwrap();

    let skytable_type = proc_macro2::TokenStream::from_str(
        format!(
            "\"crate::cache::SkytableCache<skytable::pool::ConnectionMgrTcp, String, {}>\"",
            inner_type.to_token_stream().to_string()
        )
        .as_str(),
    )
    .unwrap();
    let skytable_create = proc_macro2::TokenStream::from_str(
        format!(
            "\"{{ crate::cache::SkytableCacheBuilder::new(
                    CONFIG.skytable_host().as_ref().unwrap().as_str(),
                    *CONFIG.skytable_port().as_ref().unwrap(),
                    CONFIG.skytable_username().as_ref().unwrap().as_str(),
                    CONFIG.skytable_password().as_ref().unwrap().as_str(),
                )
                .set_space(CONFIG.skytable_space())
                .set_model(CONFIG.skytable_model())
                .set_refresh({})
                .set_lifetime(std::time::Duration::from_secs({}))
                .build()
                .await
                .unwrap()
            }}\"",
            refresh, ttl
        )
        .as_str(),
    )
    .unwrap();

    let expanded = quote! {
        #vis #sig {
            // check if the user did configure skytable
            #[cfg(feature = "caching-skytable")]
            if CONFIG.skytable_host().is_some() {
                // now check if the user did configure tls
                match CONFIG.skytable_certificate() {
                    None => {
                        // tls is inactive so we do use the raw tcp stream
                        #skytable(#(#args),*).await
                    },
                    Some(_) => {
                        // tls is active so we use the tls cache
                        #skytable_tls(#(#args),*).await
                    }
                }
            } else {
                // otherwise use timed in memory caching
                #memory(#(#args),*).await
            }

            #[cfg(not(feature = "caching-skytable"))]
            #memory(#(#args),*).await
        }

        #vis async fn #invalidate(key: String) -> Result<()> {
            use cached::Cached;
            // check if the user did configure skytable
            #[cfg(feature = "caching-skytable")]
            if CONFIG.skytable_host().is_some() {
                // now check if the user did configure tls
                match CONFIG.skytable_certificate() {
                    None => {
                        // tls is inactive so we do use the raw tcp stream
                        #skytable_static.get().unwrap().cache_remove(&key).await?;
                    },
                    Some(_) => {
                        // tls is active so we use the tls cache
                        #skytable_tls_static.get().unwrap().cache_remove(&key).await?;
                    }
                }
            } else {
                // otherwise use timed in memory caching
                #memory_static.lock().await.cache_remove(&key);
            }

            #[cfg(not(feature = "caching-skytable"))]
            #memory_static.lock().await.cache_remove(&key);


            Ok(())
        }

        // create the wrapper containing the original data logic
        #wrapper_sig #block

        // create the memory function
        #[cached::proc_macro::cached(
            convert = #convert,
            time = 500,
            key = "String",
            result = true
        )]
        #memory_sig {
            #wrapper(#(#args),*).await
        }

        // create the skytable tls function
        #[cfg(feature = "caching-skytable")]
        #[cached::proc_macro::io_cached(
            map_error = r##"|e| FeedbackFusionError::ConfigurationError(format!("{:?}", e))"##,
            ty = #skytable_tls_type,
            create = #skytable_tls_create,
            convert = #convert
        )]
        #skytable_tls_sig {
            #wrapper(#(#args),*).await
        }

        // create the skytable function
        #[cfg(feature = "caching-skytable")]
        #[cached::proc_macro::io_cached(
            map_error = r##"|e| FeedbackFusionError::ConfigurationError(format!("{:?}", e))"##,
            ty = #skytable_type,
            create = #skytable_create,
            convert = #convert
        )]
        #skytable_sig {
            #wrapper(#(#args),*).await
        }
    };

    expanded.into()
}
