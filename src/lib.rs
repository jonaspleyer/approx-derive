//! # Epsilon Type
//! ```
//!
//! ```
//!
//! # Default Epsilon
//!
//! ```
//!
//! ```
//!
//! # Casting
//! ```compile_fail
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStrct {
//!     v1: f32,
//!     v2: f64,
//! }
//! ```

mod args_parsing;
use args_parsing::*;

struct AbsDiffEqParser {
    item_struct: syn::ItemStruct,
    fields_with_args: Vec<FieldWithArgs>,
    struct_args: StructArgs,
}

impl syn::parse::Parse for AbsDiffEqParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_struct: syn::ItemStruct = input.parse()?;
        let struct_args = StructArgs::from_attrs(&item_struct.attrs)?;
        let fields_with_args = item_struct
            .fields
            .iter()
            .map(|field| FieldWithArgs::from_field(field))
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(Self {
            item_struct,
            fields_with_args,
            struct_args,
        })
    }
}

struct FieldFormatted {
    base_type: proc_macro2::TokenStream,
    own_field: proc_macro2::TokenStream,
    other_field: proc_macro2::TokenStream,
    epsilon: proc_macro2::TokenStream,
    max_relative: proc_macro2::TokenStream,
}

impl AbsDiffEqParser {
    fn get_epsilon_type(&self) -> proc_macro2::TokenStream {
        self.struct_args
            .epsilon_type
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| {
                self.fields_with_args.first().and_then(|field| {
                    let eps_type = &field.field.ty;
                    Some(quote::quote!(#eps_type))
                })
            })
            .or_else(|| Some(quote::quote!(f64)))
            .unwrap()
    }

    fn get_epsilon_type_and_default_value(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let epsilon_type = self.get_epsilon_type();
        let epsilon_default_value = self
            .struct_args
            .default_epsilon_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(#epsilon_type::EPSILON)))
            .unwrap();
        (epsilon_type, epsilon_default_value)
    }

    fn get_max_relative_default_value(&self) -> proc_macro2::TokenStream {
        let epsilon_type = self.get_epsilon_type();
        self.struct_args
            .default_max_relative_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(#epsilon_type::EPSILON)))
            .unwrap()
    }

    fn format_field(&self, field_with_args: &FieldWithArgs) -> Option<FieldFormatted> {
        // Determine if this field will be skipped and exit early
        if field_with_args.args.skip {
            return None;
        }

        // Get types for epsilon and max_relative
        let epsilon_type = self.get_epsilon_type();

        // Save field name and type in variables for easy access
        let field_name = &field_with_args.field.ident;
        let field_type = &field_with_args.field.ty;

        // Determine if the field or the value will be casted in any way
        let cast_strategy = &field_with_args.args.cast_strategy;

        // Get static values (if present) for epsilon and max_relative
        let epsilon = &field_with_args
            .args
            .epsilon_static_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(epsilon)))
            .unwrap();
        let max_relative = field_with_args
            .args
            .max_relative_static_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(max_relative)))
            .unwrap();

        // Use the casting strategy
        let (base_type, own_field, other_field, epsilon, max_relative) = match cast_strategy {
            Some(TypeCast::CastField) => (
                quote::quote!(#epsilon_type),
                quote::quote!(&(self.#field_name as #epsilon_type)),
                quote::quote!(&(other.#field_name as #epsilon_type)),
                quote::quote!(#epsilon),
                quote::quote!(#max_relative),
            ),
            Some(TypeCast::CastValue) => (
                quote::quote!(#field_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon as #field_type),
                quote::quote!(#max_relative as #field_type),
            ),
            None => (
                quote::quote!(#epsilon_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon),
                quote::quote!(#max_relative),
            ),
        };

        // Return the fully formatted field
        Some(FieldFormatted {
            base_type,
            own_field,
            other_field,
            epsilon,
            max_relative,
        })
    }

    fn get_abs_diff_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        // We need to extend the where clause for all generics
        let fields = self.fields_with_args.iter().filter_map(|field_with_args| {
            if let Some(FieldFormatted {
                base_type,
                own_field,
                other_field,
                epsilon,
                #[allow(unused)]
                max_relative,
            }) = self.format_field(field_with_args)
            {
                Some(quote::quote!(
                    <#base_type as approx::AbsDiffEq>::abs_diff_eq(
                        #own_field,
                        #other_field,
                        #epsilon
                    ) &&
                ))
            } else {
                None
            }
        });
        fields.collect()
    }

    fn get_rel_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        let fields = self.fields_with_args.iter().filter_map(|field_with_args| {
            if let Some(FieldFormatted {
                base_type,
                own_field,
                other_field,
                epsilon,
                max_relative,
            }) = self.format_field(field_with_args)
            {
                Some(quote::quote!(
                    <#base_type as approx::RelativeEq>::relative_eq(
                        #own_field,
                        #other_field,
                        #epsilon,
                        #max_relative
                    ) &&
                ))
            } else {
                None
            }
        });
        fields.collect()
    }

    fn implement_derive_abs_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.item_struct.ident;
        let (epsilon_type, epsilon_default_value) = self.get_epsilon_type_and_default_value();
        let fields = self.get_abs_diff_eq_fields();
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();

        quote::quote!(
            const _ : () = {
                #[automatically_derived]
                impl #impl_generics approx::AbsDiffEq for #struct_name #ty_generics
                #where_clause
                {
                    type Epsilon = #epsilon_type;

                    fn default_epsilon() -> Self::Epsilon {
                        #epsilon_default_value
                    }

                    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                        #(#fields)*
                        true
                    }
                }
            };
        )
    }

    fn implement_derive_rel_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.item_struct.ident;
        let max_relative_default_value = self.get_max_relative_default_value();
        let fields = self.get_rel_eq_fields();
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();

        quote::quote!(
            const _ : () = {
                #[automatically_derived]
                impl #impl_generics approx::RelativeEq for #struct_name #ty_generics
                #where_clause
                {
                    fn default_max_relative() -> Self::Epsilon {
                        #max_relative_default_value
                    }

                    fn relative_eq(
                        &self,
                        other: &Self,
                        epsilon: Self::Epsilon,
                        max_relative: Self::Epsilon
                    ) -> bool {
                        #(#fields)*
                        true
                    }
                }
            };
        )
    }
}

#[proc_macro_derive(AbsDiffEq, attributes(approx))]
pub fn derive_abs_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    parsed.implement_derive_abs_diff_eq().into()
}

#[proc_macro_derive(RelativeEq, attributes(approx))]
pub fn derive_rel_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    let mut output = quote::quote!();
    output.extend(parsed.implement_derive_abs_diff_eq());
    output.extend(parsed.implement_derive_rel_diff_eq());
    output.into()
}
