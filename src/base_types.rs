use crate::args_parsing::*;

pub struct ApproxName;

impl quote::ToTokens for ApproxName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if cfg!(feature = "infer_name") {
            if std::env::var("CARGO_PKG_NAME").is_ok_and(|x| x == "approx-derive") {
                tokens.extend(quote::quote!(approxim));
                return;
            }
            let found_name =
                proc_macro_crate::crate_name("approxim").expect("could-not-find-rename");
            match found_name {
                proc_macro_crate::FoundCrate::Itself => quote::quote!(approxim).to_tokens(tokens),
                proc_macro_crate::FoundCrate::Name(name) => {
                    let name = match name.as_str() {
                        "approx-derive" => "approx",
                        other => other,
                    };
                    quote::format_ident!("{name}").to_tokens(tokens)
                }
            };
        } else {
            tokens.extend(quote::quote!(approx));
        }
    }
}

pub enum BaseType {
    Struct {
        item_struct: syn::ItemStruct,
        fields_with_args: Vec<FieldWithArgs>,
    },
    Enum {
        item_enum: syn::ItemEnum,
        variants_with_args: Vec<EnumVariant>,
    },
}

impl syn::parse::Parse for BaseType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.fork().parse::<syn::ItemStruct>().is_ok() {
            use syn::spanned::Spanned;
            let item_struct: syn::ItemStruct = input.parse()?;
            let fields_with_args = match item_struct.fields.clone() {
                syn::Fields::Named(named_fields) => named_fields
                    .named
                    .iter()
                    .map(FieldWithArgs::from_field)
                    .collect::<syn::Result<Vec<_>>>(),
                syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
                    .unnamed
                    .iter()
                    .map(FieldWithArgs::from_field)
                    .collect::<syn::Result<Vec<_>>>(),
                syn::Fields::Unit => Err(syn::Error::new(
                    item_struct.span(),
                    "cannot derive from unit struct",
                )),
            }?;
            Ok(BaseType::Struct {
                item_struct,
                fields_with_args,
            })
        } else if let Ok(item_enum) = input.parse::<syn::ItemEnum>() {
            // let item_enum: syn::ItemEnum = input.parse()?;
            let variants_with_args = item_enum
                .variants
                .iter()
                .map(|v| {
                    let args = FieldArgs::from_attrs(&v.attrs)?;
                    let fields_with_args = v
                        .fields
                        .iter()
                        .map(|f| {
                            let mut fwa = FieldWithArgs::from_field(f)?;
                            fwa.args.patch_if_not_exists(&args);
                            Ok(fwa)
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    Ok(EnumVariant {
                        fields_with_args,
                        ident: v.ident.clone(),
                        discriminant: v.discriminant.clone().map(|x| x.1),
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(BaseType::Enum {
                item_enum,
                variants_with_args,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Could not parse enum or struct",
            ))
        }
    }
}

impl BaseType {
    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        match self {
            #[allow(unused)]
            BaseType::Struct {
                item_struct,
                fields_with_args,
            } => &item_struct.attrs,
            #[allow(unused)]
            BaseType::Enum {
                item_enum,
                variants_with_args,
            } => &item_enum.attrs,
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            #[allow(unused)]
            BaseType::Struct {
                item_struct,
                fields_with_args,
            } => &item_struct.generics,
            #[allow(unused)]
            BaseType::Enum {
                item_enum,
                variants_with_args,
            } => &item_enum.generics,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            #[allow(unused)]
            BaseType::Struct {
                item_struct,
                fields_with_args,
            } => &item_struct.ident,
            #[allow(unused)]
            BaseType::Enum {
                item_enum,
                variants_with_args,
            } => &item_enum.ident,
        }
    }
}

#[derive(Debug)]
pub struct FieldFormatted {
    pub base_type: proc_macro2::TokenStream,
    pub own_field: proc_macro2::TokenStream,
    pub other_field: proc_macro2::TokenStream,
    pub epsilon: proc_macro2::TokenStream,
    pub max_relative: proc_macro2::TokenStream,
    pub mapping: Option<proc_macro2::TokenStream>,
    pub set_equal: bool,
    // If this is Some type, we should be matching for this type
    pub use_iterator: bool,
}
