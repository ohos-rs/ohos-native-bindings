use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Ident, LitStr, Token, Type, parse::Parse, parse::ParseStream,
    parse_macro_input,
};

struct ConfigArgs {
    target_type: Ident,
    prefix: LitStr,
    extra_types: Vec<Type>,
}

impl Parse for ConfigArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_type: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let prefix: LitStr = input.parse()?;
        let mut extra_types = Vec::new();
        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            if input.is_empty() {
                break;
            }
            extra_types.push(input.parse()?);
        }
        Ok(ConfigArgs {
            target_type,
            prefix,
            extra_types,
        })
    }
}

fn to_upper_snake_case(s: &str) -> String {
    s.to_case(Case::UpperSnake)
}

fn get_variant_prefix(variant: &syn::Variant, default_prefix: &str) -> String {
    for attr in &variant.attrs {
        if attr.path().is_ident("prefix")
            && let Ok(lit_str) = attr.parse_args::<LitStr>()
        {
            return lit_str.value();
        }
    }
    default_prefix.to_string()
}

fn get_target_variant(variant: &syn::Variant, default_prefix: &str) -> Ident {
    for attr in &variant.attrs {
        if attr.path().is_ident("alias")
            && let Ok(lit_str) = attr.parse_args::<LitStr>()
        {
            return format_ident!("{}", lit_str.value());
        }
    }

    let variant_prefix = get_variant_prefix(variant, default_prefix);
    for attr in &variant.attrs {
        if attr.path().is_ident("suffix")
            && let Ok(lit_str) = attr.parse_args::<LitStr>()
        {
            return format_ident!("{}{}", variant_prefix, lit_str.value());
        }
    }

    let upper_snake_variant = to_upper_snake_case(&variant.ident.to_string());
    format_ident!("{}{}", variant_prefix, upper_snake_variant)
}

#[proc_macro_derive(EnumFrom, attributes(config, prefix, suffix, alias))]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let args = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("config"))
        .map(|attr| attr.parse_args::<ConfigArgs>())
        .expect("config attribute is required")
        .expect("Failed to parse config attribute");

    let target_type = args.target_type;
    let default_prefix = args.prefix.value();
    let extra_types = args.extra_types;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    let from_attribute_type_arms: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            let target_variant = get_target_variant(v, &default_prefix);
            quote! {
                #name::#variant => #target_variant,
            }
        })
        .collect();

    let from_target_type_arms: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            let target_variant = get_target_variant(v, &default_prefix);
            quote! {
                #target_variant => #name::#variant,
            }
        })
        .collect();

    let try_from_target_type_arms: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            let target_variant = get_target_variant(v, &default_prefix);
            quote! {
                #target_variant => ::std::option::Option::Some(#name::#variant),
            }
        })
        .collect();

    let extra_from_attribute_type_impls: Vec<_> = extra_types
        .iter()
        .map(|extra_type| {
            quote! {
                impl From<#name> for #extra_type {
                    fn from(attr: #name) -> Self {
                        let raw: #target_type = attr.into();
                        raw as #extra_type
                    }
                }
            }
        })
        .collect();

    let extra_from_target_type_impls: Vec<_> = extra_types
        .iter()
        .map(|extra_type| {
            quote! {
                impl From<#extra_type> for #name {
                    fn from(attr: #extra_type) -> Self {
                        let raw = attr as #target_type;
                        raw.into()
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl From<#name> for #target_type {
            fn from(attr: #name) -> Self {
                match attr {
                    #(#from_attribute_type_arms)*
                    _ => unreachable!("Invalid attribute value"),
                }
            }
        }

        impl From<#target_type> for #name {
            fn from(attr: #target_type) -> Self {
                match attr {
                    #(#from_target_type_arms)*
                    _ => unreachable!("Invalid attribute value"),
                }
            }
        }

        impl #name {
            pub fn try_from_raw(attr: #target_type) -> ::std::option::Option<Self> {
                match attr {
                    #(#try_from_target_type_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }

        #(#extra_from_attribute_type_impls)*

        #(#extra_from_target_type_impls)*
    };

    TokenStream::from(expanded)
}
