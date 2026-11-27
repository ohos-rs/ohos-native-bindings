use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr,
    Token, Type,
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

struct FallbackRawArgs {
    raw_expr: proc_macro2::TokenStream,
}

impl Parse for FallbackRawArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "raw" {
            return Err(syn::Error::new_spanned(
                key,
                "expected `raw = <expr>` in `#[fallback(raw = ...)]`",
            ));
        }
        input.parse::<Token![=]>()?;
        let raw_expr: proc_macro2::TokenStream = input.parse()?;
        if raw_expr.is_empty() {
            return Err(input.error("expected expression after `raw =`"));
        }
        Ok(Self { raw_expr })
    }
}

enum FallbackSpec {
    UnitVariant(Ident),
    TupleVariant { ident: Ident, field_ty: Box<Type> },
    ContainerExpr(proc_macro2::TokenStream),
}

fn to_upper_snake_case(s: &str) -> String {
    s.to_case(Case::UpperSnake)
}

fn get_variant_prefix(variant: &syn::Variant, default_prefix: &str) -> String {
    for attr in &variant.attrs {
        if attr.path().is_ident("prefix") {
            if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                return lit_str.value();
            }
        }
    }
    default_prefix.to_string()
}

fn get_target_variant(variant: &syn::Variant, default_prefix: &str) -> Ident {
    for attr in &variant.attrs {
        if attr.path().is_ident("alias") {
            if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                return format_ident!("{}", lit_str.value());
            }
        }
    }

    let variant_prefix = get_variant_prefix(variant, default_prefix);
    for attr in &variant.attrs {
        if attr.path().is_ident("suffix") {
            if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                return format_ident!("{}{}", variant_prefix, lit_str.value());
            }
        }
    }

    let upper_snake_variant = to_upper_snake_case(&variant.ident.to_string());
    format_ident!("{}{}", variant_prefix, upper_snake_variant)
}

fn get_cfg_attributes(variant: &syn::Variant) -> Vec<&syn::Attribute> {
    variant
        .attrs
        .iter()
        .filter(|attribute| {
            attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr")
        })
        .collect()
}

fn has_mapping_override_attr(variant: &syn::Variant) -> bool {
    variant.attrs.iter().any(|attr| {
        attr.path().is_ident("prefix")
            || attr.path().is_ident("suffix")
            || attr.path().is_ident("alias")
    })
}

#[proc_macro_derive(EnumFrom, attributes(config, prefix, suffix, alias, fallback))]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_enum_from(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_enum_from(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let config_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("config"))
        .ok_or_else(|| syn::Error::new_spanned(name, "`#[config(...)]` attribute is required"))?;
    let args = config_attr.parse_args::<ConfigArgs>()?;

    let target_type = args.target_type;
    let default_prefix = args.prefix.value();
    let extra_types = args.extra_types;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "`EnumFrom` can only be derived for enums",
            ));
        }
    };

    let mut fallback_spec: Option<FallbackSpec> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("fallback") {
            if fallback_spec.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "duplicate `#[fallback(...)]` attribute on enum",
                ));
            }
            let expr: proc_macro2::TokenStream = attr.parse_args()?;
            if expr.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected fallback function or closure in `#[fallback(...)]`",
                ));
            }
            fallback_spec = Some(FallbackSpec::ContainerExpr(expr));
        }
    }

    let mut from_attribute_type_arms = Vec::new();
    let mut from_target_type_arms = Vec::new();
    let mut try_from_target_type_arms = Vec::new();

    for v in variants {
        let variant = &v.ident;
        let cfg_attributes = get_cfg_attributes(v);
        let fallback_attrs: Vec<_> = v
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("fallback"))
            .collect();

        if fallback_attrs.len() > 1 {
            return Err(syn::Error::new_spanned(
                fallback_attrs[1],
                "duplicate `#[fallback]` attribute on variant",
            ));
        }

        if let Some(fb_attr) = fallback_attrs.first() {
            if fallback_spec.is_some() {
                return Err(syn::Error::new_spanned(
                    fb_attr,
                    "only one fallback (either enum-level `#[fallback(...)]` or a single variant `#[fallback]`) is allowed",
                ));
            }
            if !cfg_attributes.is_empty() {
                return Err(syn::Error::new_spanned(
                    v,
                    "`#[fallback]` variant cannot be conditionally compiled with `#[cfg]`",
                ));
            }

            match &v.fields {
                Fields::Unit => {
                    if matches!(fb_attr.meta, syn::Meta::Path(_)) {
                        let target_variant = get_target_variant(v, &default_prefix);
                        from_attribute_type_arms.push(quote! {
                            #name::#variant => #target_variant,
                        });
                        from_target_type_arms.push(quote! {
                            #target_variant => #name::#variant,
                        });
                        try_from_target_type_arms.push(quote! {
                            #target_variant => ::std::option::Option::Some(#name::#variant),
                        });
                        fallback_spec = Some(FallbackSpec::UnitVariant(variant.clone()));
                    } else {
                        if has_mapping_override_attr(v) {
                            return Err(syn::Error::new_spanned(
                                v,
                                "`#[fallback(raw = ...)]` cannot be combined with `#[prefix]`, `#[suffix]`, or `#[alias]`",
                            ));
                        }
                        let raw_args = fb_attr.parse_args::<FallbackRawArgs>()?;
                        let raw_expr = &raw_args.raw_expr;
                        from_attribute_type_arms.push(quote! {
                            #name::#variant => (#raw_expr) as #target_type,
                        });
                        fallback_spec = Some(FallbackSpec::UnitVariant(variant.clone()));
                    }
                }
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    if !matches!(fb_attr.meta, syn::Meta::Path(_)) {
                        return Err(syn::Error::new_spanned(
                            fb_attr,
                            "tuple fallback variants take plain `#[fallback]` without arguments",
                        ));
                    }
                    if has_mapping_override_attr(v) {
                        return Err(syn::Error::new_spanned(
                            v,
                            "tuple `#[fallback]` variant cannot use `#[prefix]`, `#[suffix]`, or `#[alias]`",
                        ));
                    }
                    let field_ty = fields.unnamed[0].ty.clone();
                    from_attribute_type_arms.push(quote! {
                        #name::#variant(raw) => raw as #target_type,
                    });
                    fallback_spec = Some(FallbackSpec::TupleVariant {
                        ident: variant.clone(),
                        field_ty: Box::new(field_ty),
                    });
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        v,
                        "`#[fallback]` only supports unit variants or single-field tuple variants `Other(Raw)`",
                    ));
                }
            }
        } else {
            if !matches!(v.fields, Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    v,
                    "non-unit variants in `EnumFrom` must be marked with `#[fallback]` and have a single raw field",
                ));
            }
            let target_variant = get_target_variant(v, &default_prefix);
            from_attribute_type_arms.push(quote! {
                #(#cfg_attributes)*
                #name::#variant => #target_variant,
            });
            from_target_type_arms.push(quote! {
                #(#cfg_attributes)*
                #target_variant => #name::#variant,
            });
            try_from_target_type_arms.push(quote! {
                #(#cfg_attributes)*
                #target_variant => ::std::option::Option::Some(#name::#variant),
            });
        }
    }

    let from_target_fallback_arm = match &fallback_spec {
        None => quote! {
            _ => unreachable!("Invalid attribute value"),
        },
        Some(FallbackSpec::UnitVariant(ident)) => quote! {
            _ => #name::#ident,
        },
        Some(FallbackSpec::TupleVariant { ident, field_ty }) => quote! {
            raw => #name::#ident(raw as #field_ty),
        },
        Some(FallbackSpec::ContainerExpr(expr)) => quote! {
            raw => (#expr)(raw),
        },
    };

    let extra_from_attribute_type_impls: Vec<_> = extra_types
        .iter()
        .map(|extra_type| {
            quote! {
                impl From<#name> for #extra_type {
                    #[allow(clippy::unnecessary_cast)]
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
                    #[allow(clippy::unnecessary_cast)]
                    fn from(attr: #extra_type) -> Self {
                        let raw = attr as #target_type;
                        raw.into()
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        impl From<#name> for #target_type {
            #[allow(unreachable_patterns, clippy::unnecessary_cast)]
            fn from(attr: #name) -> Self {
                match attr {
                    #(#from_attribute_type_arms)*
                    _ => unreachable!("Invalid attribute value"),
                }
            }
        }

        impl From<#target_type> for #name {
            #[allow(clippy::unnecessary_cast)]
            fn from(attr: #target_type) -> Self {
                match attr {
                    #(#from_target_type_arms)*
                    #from_target_fallback_arm
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
    })
}
