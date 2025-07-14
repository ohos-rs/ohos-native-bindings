use convert::convert_case;
use convert_case::Case;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Data, DeriveInput, Ident, LitStr, Token,
};

mod convert;

struct MacroArgs {
    target_type: Ident,
    prefix: LitStr,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_type: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let prefix: LitStr = input.parse()?;
        Ok(MacroArgs {
            target_type,
            prefix,
        })
    }
}

fn to_pascal_case(s: &str) -> String {
    convert_case(s, Case::UpperSnake)
}

fn get_variant_prefix(variant: &syn::Variant, default_prefix: &str) -> String {
    // 查找 enum_prefix 属性
    for attr in &variant.attrs {
        if attr.path().is_ident("enum_prefix") {
            if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                return lit_str.value();
            }
        }
    }
    default_prefix.to_string()
}

#[proc_macro_derive(EnumFrom, attributes(enum_from_config, enum_prefix))]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let args = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("enum_from_config"))
        .map(|attr| attr.parse_args::<MacroArgs>())
        .expect("enum_from_config attribute is required")
        .expect("Failed to parse enum_from_config attribute");

    let target_type = args.target_type;
    let default_prefix = args.prefix.value();

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("AttributeConversion can only be derived for enums"),
    };

    let from_attribute_type_arms = variants.iter().map(|v| {
        let variant = &v.ident;
        let variant_prefix = get_variant_prefix(v, &default_prefix);
        let pascal_case_variant = to_pascal_case(&variant.to_string());
        let target_variant = format_ident!("{}{}", variant_prefix, pascal_case_variant);
        quote! {
            #name::#variant => #target_variant,
        }
    });

    let from_target_type_arms = variants.iter().map(|v| {
        let variant = &v.ident;
        let variant_prefix = get_variant_prefix(v, &default_prefix);
        let pascal_case_variant = to_pascal_case(&variant.to_string());
        let target_variant = format_ident!("{}{}", variant_prefix, pascal_case_variant);
        quote! {
            #target_variant => #name::#variant,
        }
    });

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
    };

    TokenStream::from(expanded)
}
