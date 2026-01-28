use crate::args_parsing::*;
use crate::base_types::{ApproxName, BaseType, FieldFormatted};
use crate::AbsDiffEqParser;

impl syn::parse::Parse for AbsDiffEqParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base_type: BaseType = input.parse()?;
        let struct_args = StructArgs::from_attrs(base_type.attrs())?;
        Ok(Self {
            base_type,
            struct_args,
        })
    }
}

impl AbsDiffEqParser {
    pub fn get_epsilon_parent_type(&self) -> proc_macro2::TokenStream {
        self.struct_args
            .epsilon_type
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| {
                #[allow(unused)]
                match &self.base_type {
                    BaseType::Struct {
                        item_struct,
                        fields_with_args,
                    } => fields_with_args
                        .iter()
                        .find(|f| f.args.skip.is_none_or(|x| !x)),
                    BaseType::Enum {
                        item_enum,
                        variants_with_args,
                    } => variants_with_args
                        .iter()
                        .flat_map(|v| v.fields_with_args.iter())
                        .find(|f| f.args.skip.is_none_or(|x| !x)),
                }
                .map(|field| {
                    let field_type = &field.ty;
                    quote::quote!(#field_type)
                })
            })
            .or_else(|| Some(quote::quote!(f64)))
            .unwrap()
    }

    pub fn get_derived_epsilon_type(&self) -> proc_macro2::TokenStream {
        let parent = self.get_epsilon_parent_type();
        quote::quote!(<#parent as #ApproxName::AbsDiffEq>::Epsilon)
    }

    pub fn get_epsilon_type_and_default_value(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let parent = self.get_epsilon_parent_type();
        let epsilon_type = self.get_derived_epsilon_type();
        let epsilon_default_value = self
            .struct_args
            .default_epsilon_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| Some(quote::quote!(<#parent as #ApproxName::AbsDiffEq>::default_epsilon())))
            .unwrap();
        (epsilon_type, epsilon_default_value)
    }

    pub fn generics_involved(&self) -> bool {
        let parent = self.get_epsilon_parent_type();
        self.base_type
            .generics()
            .params
            .iter()
            .any(|param| quote::quote!(#param).to_string() == parent.to_string())
    }

    pub fn get_max_relative_default_value(&self) -> proc_macro2::TokenStream {
        let epsilon_type = self.get_epsilon_parent_type();
        self.struct_args
            .default_max_relative_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| {
                Some(quote::quote!(<#epsilon_type as #ApproxName::RelativeEq>::default_max_relative()))
            })
            .unwrap()
    }

    pub fn format_nth_field(
        &self,
        n: usize,
        field_with_args: &FieldWithArgs,
        idents: Option<(syn::Ident, syn::Ident)>,
    ) -> Option<FieldFormatted> {
        // Determine if this field will be skipped and exit early
        if let Some(true) = field_with_args.args.skip {
            return None;
        }

        // Get types for epsilon and max_relative
        let parent_type = self.get_epsilon_parent_type();

        // Save field name and type in variables for easy access
        use core::str::FromStr;
        let (field_name1, field_name2) = match (&field_with_args.ident, idents) {
            (Some(id), None) => (quote::quote!(self.#id), quote::quote!(other.#id)),
            (None, None) => {
                let field_number = proc_macro2::TokenStream::from_str(&format!("{}", n)).unwrap();
                (
                    quote::quote!(self.#field_number),
                    quote::quote!(other.#field_number),
                )
            }
            (_, Some((id1, id2))) => (quote::quote!(#id1), quote::quote!(#id2)),
        };
        let field_type = &field_with_args.ty;

        // Determine if the field or the value will be casted in any way
        let cast_strategy = &field_with_args.args.cast_strategy;

        // Get static values (if present) for epsilon and max_relative
        let epsilon = &field_with_args
            .args
            .epsilon_static_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| Some(quote::quote!(epsilon)))
            .unwrap();
        let max_relative = field_with_args
            .args
            .max_relative_static_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| Some(quote::quote!(max_relative)))
            .unwrap();

        // Use the casting strategy
        let (base_type, own_field, other_field, mut epsilon, mut max_relative) = match cast_strategy
        {
            Some(TypeCast::CastField) => (
                quote::quote!(#parent_type),
                quote::quote!(&(#field_name1.clone() as #parent_type)),
                quote::quote!(&(#field_name2.clone() as #parent_type)),
                quote::quote!(#epsilon.clone()),
                quote::quote!(#max_relative.clone()),
            ),
            Some(TypeCast::CastValue) => (
                quote::quote!(#field_type),
                quote::quote!(&#field_name1),
                quote::quote!(&#field_name2),
                quote::quote!((#epsilon.clone() as #field_type)),
                quote::quote!((#max_relative.clone() as #field_type)),
            ),
            None => (
                quote::quote!(#field_type),
                quote::quote!(&#field_name1),
                quote::quote!(&#field_name2),
                quote::quote!(#epsilon.clone()),
                quote::quote!(#max_relative.clone()),
            ),
        };
        if let Some(eps_map) = &field_with_args.args.epsilon_mapping {
            epsilon = quote::quote!((#eps_map)(#epsilon));
        };
        if let Some(max_rel_map) = &field_with_args.args.max_relative_mapping {
            max_relative = quote::quote!((#max_rel_map)(#max_relative));
        };

        let mapping = field_with_args
            .args
            .mapping
            .clone()
            .map(|expr| quote::quote!(#expr));

        // Return the fully formatted field
        Some(FieldFormatted {
            base_type,
            own_field,
            other_field,
            epsilon,
            max_relative,
            set_equal: field_with_args.args.set_equal.unwrap_or(false),
            mapping,
            use_iterator: field_with_args.args.use_iterator.unwrap_or(false),
        })
    }

    pub fn get_abs_diff_eq_struct_fields(
        &self,
        fields_with_args: &[FieldWithArgs],
    ) -> Vec<proc_macro2::TokenStream> {
        // We need to extend the where clause for all generics
        let fields = fields_with_args
            .iter()
            .enumerate()
            .filter_map(|(n, field_with_args)| {
                if let Some(FieldFormatted {
                    base_type,
                    own_field,
                    other_field,
                    epsilon,
                    #[allow(unused)]
                    max_relative,
                    set_equal,
                    mapping,
                    use_iterator,
                }) = self.format_nth_field(n, field_with_args, None)
                {
                    if set_equal {
                        Some(quote::quote!(#own_field == #other_field &&))
                    } else if let Some(map) = mapping {
                        Some(quote::quote!(
                            (if let ((Some(a), Some(b))) = (
                                (#map)(#own_field),
                                (#map)(#other_field)
                            ) {
                                #ApproxName::AbsDiffEq::abs_diff_eq(&a, &b, #epsilon)
                            } else {
                                false
                            }) &&
                        ))
                    } else if use_iterator {
                        Some(quote::quote!(({
                            let mut iter1 = core::iter::IntoIterator::into_iter(#own_field);
                            let mut iter2 = core::iter::IntoIterator::into_iter(#other_field);
                            let mut res = true;
                            loop {
                                match (iter1.next(), iter2.next()) {
                                    (None, None) => break,
                                    (Some(a), Some(b)) => {
                                        if !#ApproxName::AbsDiffEq::abs_diff_eq(a, b, #epsilon) {
                                            res = false;
                                            break;
                                        }
                                    },
                                    _ => {
                                        res = false;
                                        break;
                                    }
                                }
                            }
                            res
                        }) &&))
                    } else {
                        Some(quote::quote!(
                            <#base_type as #ApproxName::AbsDiffEq>::abs_diff_eq(
                                #own_field,
                                #other_field,
                                #epsilon
                            ) &&
                        ))
                    }
                } else {
                    None
                }
            });
        fields.collect()
    }

    pub fn get_abs_diff_eq_enum_variants(
        &self,
        variants_with_args: &[EnumVariant],
    ) -> Vec<proc_macro2::TokenStream> {
        variants_with_args
            .iter()
            .map(|variant_with_args| {
                let variant = &variant_with_args.ident;
                use syn::spanned::Spanned;

                let gen_field_names = |var: &str| -> Vec<syn::Ident> {
                    variant_with_args
                        .fields_with_args
                        .iter()
                        .enumerate()
                        .map(|(n, field)| syn::Ident::new(&format!("{var}{n}"), field.ident.span()))
                        .collect()
                };
                if variant_with_args
                    .fields_with_args
                    .first()
                    .and_then(|f| f.ident.clone())
                    .is_some()
                {
                    let field_placeholders1 = gen_field_names("x");
                    let field_placeholders2 = gen_field_names("y");
                    let gen_combos = |iterator: Vec<syn::Ident>| {
                        iterator
                            .iter()
                            .zip(&variant_with_args.fields_with_args)
                            .map(|(fph, fwa)| {
                                let id = &fwa.ident;
                                quote::quote!(#id: #fph)
                            })
                            .collect::<Vec<_>>()
                    };
                    let comps: Vec<_> = field_placeholders1
                        .iter()
                        .zip(field_placeholders2.iter())
                        .zip(variant_with_args.fields_with_args.iter())
                        .map(|((xi, yi), field)| {
                            self.get_abs_diff_eq_single_field(xi.clone(), yi.clone(), field)
                        })
                        .collect();
                    let field_name_placeholder_combos1 = gen_combos(field_placeholders1);
                    let field_name_placeholder_combos2 = gen_combos(field_placeholders2);
                    quote::quote!(
                        (
                            Self:: #variant {
                                #(#field_name_placeholder_combos1),*
                            },
                            Self:: #variant {
                                #(#field_name_placeholder_combos2),*
                            }
                        ) => #(#comps) &&*,
                    )
                } else if !variant_with_args.fields_with_args.is_empty() {
                    let field_names1 = gen_field_names("x");
                    let field_names2 = gen_field_names("y");
                    let comps: Vec<_> = field_names1
                        .iter()
                        .zip(field_names2.iter())
                        .zip(variant_with_args.fields_with_args.iter())
                        .map(|((xi, yi), field)| {
                            self.get_abs_diff_eq_single_field(xi.clone(), yi.clone(), field)
                        })
                        .collect();
                    quote::quote!(
                        (
                            Self:: #variant (#(#field_names1),*),
                            Self:: #variant (#(#field_names2),*)
                        ) => {#(#comps) &&*},
                    )
                } else {
                    quote::quote!(
                        (Self:: #variant, Self:: #variant) => true,
                    )
                }
            })
            .collect()
    }

    pub fn get_abs_diff_eq_single_field(
        &self,
        xi: syn::Ident,
        yi: syn::Ident,
        field_with_args: &FieldWithArgs,
    ) -> Option<proc_macro2::TokenStream> {
        if let Some(FieldFormatted {
            base_type,
            own_field,
            other_field,
            epsilon,
            #[allow(unused)]
            max_relative,
            set_equal,
            mapping,
            use_iterator,
        }) = self.format_nth_field(0, field_with_args, Some((xi, yi)))
        {
            if set_equal {
                Some(quote::quote!(#own_field == #other_field))
            } else if let Some(map) = mapping {
                Some(quote::quote!(
                    (if let ((Some(a), Some(b))) = (
                        (#map)(#own_field),
                        (#map)(#other_field)
                    ) {
                        #ApproxName::AbsDiffEq::abs_diff_eq(&a, &b, #epsilon)
                    } else {
                        false
                    })
                ))
            } else if use_iterator {
                Some(quote::quote!({
                    let mut iter1 = core::iter::IntoIterator::into_iter(*#own_field);
                    let mut iter2 = core::iter::IntoIterator::into_iter(*#other_field);
                    let mut res = true;
                    loop {
                        match (iter1.next(), iter2.next()) {
                            (None, None) => break,
                            (Some(a), Some(b)) => {
                                if !#ApproxName::AbsDiffEq::abs_diff_eq(a, b, #epsilon) {
                                    res = false;
                                    break;
                                }
                            },
                            _ => {
                                res = false;
                                break;
                            }
                        }
                    }
                    res
                }))
            } else {
                Some(quote::quote!(
                    <#base_type as #ApproxName::AbsDiffEq>::abs_diff_eq(
                        &#own_field,
                        &#other_field,
                        #epsilon
                    )
                ))
            }
        } else {
            None
        }
    }

    pub fn generate_where_clause(&self, abs_diff_eq: bool) -> proc_macro2::TokenStream {
        let (epsilon_type, _) = self.get_epsilon_type_and_default_value();
        let (_, _, where_clause) = self.base_type.generics().split_for_impl();
        let trait_bound = match abs_diff_eq {
            true => quote::quote!(#ApproxName::AbsDiffEq),
            false => quote::quote!(#ApproxName::RelativeEq),
        };
        if self.generics_involved() {
            let parent = self.get_epsilon_parent_type();
            match where_clause {
                Some(clause) => quote::quote!(
                    #clause
                        #parent: #trait_bound,
                        #parent: PartialEq,
                        #epsilon_type: Clone,
                ),
                None => quote::quote!(
                where
                    #parent: #trait_bound,
                    #parent: PartialEq,
                    #epsilon_type: Clone,
                ),
            }
        } else {
            quote::quote!(#where_clause)
        }
    }

    pub fn implement_derive_abs_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.base_type.ident();
        let (epsilon_type, epsilon_default_value) = self.get_epsilon_type_and_default_value();

        let (impl_generics, ty_generics, _) = self.base_type.generics().split_for_impl();
        let where_clause = self.generate_where_clause(true);

        match &self.base_type {
            #[allow(unused)]
            BaseType::Struct {
                item_struct,
                fields_with_args,
            } => {
                let fields = self.get_abs_diff_eq_struct_fields(fields_with_args);

                quote::quote!(
                    const _ : () = {
                        #[automatically_derived]
                        impl #impl_generics #ApproxName::AbsDiffEq for #struct_name #ty_generics
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
            #[allow(unused)]
            BaseType::Enum {
                item_enum,
                variants_with_args,
            } => {
                let variants = self.get_abs_diff_eq_enum_variants(variants_with_args);
                quote::quote!(
                    const _: () = {
                        #[automatically_derived]
                        impl #impl_generics #ApproxName::AbsDiffEq for #struct_name #ty_generics
                        #where_clause
                        {
                            type Epsilon = #epsilon_type;

                            fn default_epsilon() -> Self::Epsilon {
                                #epsilon_default_value
                            }

                            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                                match (self, other) {
                                    #(#variants)*
                                    _ => false,
                                }
                            }
                        }
                    };
                )
            }
        }
    }
}
