#![deny(missing_docs)]
//! This crate provides derive macros for the
//! [AbsDiffEq](https://docs.rs/approx/latest/approx/trait.AbsDiffEq.html) and
//! [RelativeEq](https://docs.rs/approx/latest/approx/trait.RelativeEq.html) traits of the
//! [approx](https://docs.rs/approx/latest/approx/) crate.
//!
//! These derive macros only implement both traits with `...<Rhs = Self>`.
//! The macros infer the `EPSILON` type of the [AbsDiffEq] trait by looking
//! at the type of the first struct field or any type specified by the user.
//!
//! This table lists all attributes which can be used to customize the derived traits.
//! They are ordered in descending priority, meaning setting the `#[approx(equal)]` will overwrite
//! any specifications made in the `#[approx(map = ...)]` attribute.
//!
//! | Field Attribute | Functionality |
//! |:--- | --- |
//! | [`#[approx(skip)]`](#skipping-fields) | Skips the field entirely |
//! | [`#[approx(equal)]`](#testing-for-equality) | Checks this field with `==` for Equality |
//! | [`#[approx(cast_field)]`](#casting-fields) | Casts the field with `.. as ..` syntax. |
//! | [`#[approx(map = ..)]`](#mapping-values) | Maps values before comparing them. |
//! | [`#[approx(static_epsilon = ..)]`](#static-values) | Defines a static epsilon value for this particular field. |
//! | | |
//! | **Struct Attribute** | |
//! | [`#[approx(default_epsilon = ...)]`](#default-epsilon) | Sets the default epsilon value |
//! | [`#[approx(default_max_relative = ...)]`](#default-max-relative) | Sets the default `max_relative` value. |
//! | [`#[approx(epsilon_type = ...)]`](#epsilon-type) | Sets the type of the epsilon value |
//!
//! The following example explains a possible use-case.
//!
//! ```
//! use approx_derive::AbsDiffEq;
//!
//! // Define a new type and derive the AbsDiffEq trait
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Position {
//!     x: f64,
//!     y: f64
//! }
//!
//! // Compare if two given positions match
//! // with respect to geiven epsilon.
//! let p1 = Position { x: 1.01, y: 2.36 };
//! let p2 = Position { x: 0.99, y: 2.38 };
//! approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.021);
//! ```
//! In this case, the generated code looks something like this:
//! ```ignore
//! const _ : () =
//! {
//!     #[automatically_derived] impl approx :: AbsDiffEq for Position
//!     {
//!         type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;
//!
//!         fn default_epsilon() -> Self :: Epsilon {
//!             <f64 as approx::AbsDiffEq>::default_epsilon()
//!         }
//!
//!         fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
//!             <f64 as approx::AbsDiffEq>::abs_diff_eq(
//!                 &self.x,
//!                 & other.x,
//!                 epsilon.clone()
//!             ) &&
//!             <f64 as approx::AbsDiffEq>::abs_diff_eq(
//!                 &self.y,
//!                 &other.y,
//!                 epsilon.clone()
//!             ) && true
//!         }
//!     }
//! };
//! ```
//! The [AbsDiffEq] derive macro calls the `abs_diff_eq` method repeatedly on all fields
//! to determine if all are matching.
//!
//! # Field Attributes
//! ## Skipping Fields
//!
//! Sometimes, we only want to compare certain fields and omit others completely.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Player {
//!     hit_points: f32,
//!     pos_x: f32,
//!     pos_y: f32,
//!     #[approx(skip)]
//!     id: (usize, usize),
//! }
//!
//! let player1 = Player {
//!     hit_points: 100.0,
//!     pos_x: 2.0,
//!     pos_y: -650.345,
//!     id: (0, 1),
//! };
//!
//! let player2 = Player {
//!     hit_points: 99.9,
//!     pos_x: 2.001,
//!     pos_y: -649.898,
//!     id: (22, 0),
//! };
//!
//! approx::assert_abs_diff_eq!(player1, player2, epsilon = 0.5);
//! ```
//!
//! ## Testing for [Equality](core::cmp::Eq)
//!
//! When identical equality is desired, we can specify this with the `#[approx(equal)]` attribute.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Prediction {
//!     confidence: f64,
//!     #[approx(equal)]
//!     category: String,
//! }
//! ```
//!
//! Note that in this case, the type of the epsilon value for the implementation of
//! [AbsDiffEq](https://docs.rs/approx/latest/approx/trait.AbsDiffEq.html) is inferred from the
//! first field of the `Prediction` struct.
//! This means if we reorder the arguments of the struct, we need to manually set the epsilon type.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! #[approx(epsilon_type = f64)]
//! struct Prediction {
//!     #[approx(equal)]
//!     category: String,
//!     confidence: f64,
//! }
//! ```
//!
//! ## Casting Fields
//!
//! Structs which consist of multiple fields with different
//! numeric types, can not be derived without additional hints.
//! After all, we should specify how this type mismatch will be handled.
//!
//! ```compile_fail
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct {
//!     v1: f32,
//!     v2: f64,
//! }
//! ```
//!
//! We can use the `#[approx(cast_field)]` and `#[approx(cast_value)]`
//! attributes to achieve this goal.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct {
//!     v1: f32,
//!     #[approx(cast_field)]
//!     v2: f64,
//! }
//! ```
//! Now the second field will be casted to the type of the inferred epsilon value (`f32`).
//! We can check this by testing if a change in the size of `f64::MIN_POSITIVE` would get lost by
//! this procedure.
//! ```
//! # use approx_derive::*;
//! # #[derive(RelativeEq, PartialEq, Debug)]
//! # struct MyStruct {
//! #   v1: f32,
//! #   #[approx(cast_field)]
//! #   v2: f64,
//! # }
//! let ms1 = MyStruct {
//!     v1: 1.0,
//!     v2: 3.0,
//! };
//! let ms2 = MyStruct {
//!     v1: 1.0,
//!     v2: 3.0 + f64::MIN_POSITIVE,
//! };
//! approx::assert_relative_eq!(ms1, ms2);
//! ```
//!
//! ## Mapping Values
//!
//! We can map values before comparing them.
//! By default, we need to return an option of the value in question.
//! This allows to do computations where error can occur.
//! Although this error is not caught, the comparison will fail if any of the two compared objects
//! return a `None` value.
//! ```
//! # use approx_derive::*;
//! # use approx::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Tower {
//!     height_in_meters: f32,
//!     #[approx(map = |x: &f32| Some(x.sqrt()))]
//!     area_in_meters_squared: f32,
//! }
//! # let t1 = Tower {
//! #   height_in_meters: 100.0,
//! #   area_in_meters_squared: 30.1,
//! # };
//! # let t2 = Tower {
//! #   height_in_meters: 100.0,
//! #   area_in_meters_squared: 30.5,
//! # };
//! # approx::assert_abs_diff_ne!(t1, t2, epsilon = 0.03);
//! ```
//!
//! This functionality can also be useful when having more complex datatypes.
//! ```
//! # use approx_derive::*;
//! # use approx::*;
//! #[derive(PartialEq, Debug)]
//! enum Time {
//!     Years(u16),
//!     Months(u16),
//!     Weeks(u16),
//!     Days(u16),
//! }
//!
//! fn time_to_days(time: &Time) -> Option<u16> {
//!     match time {
//!         Time::Years(y) => Some(365 * y),
//!         Time::Months(m) => Some(30 * m),
//!         Time::Weeks(w) => Some(7 * w),
//!         Time::Days(d) => Some(*d),
//!     }
//! }
//!
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! #[approx(epsilon_type = u16)]
//! struct Dog {
//!     #[approx(map = time_to_days)]
//!     age: Time,
//!     #[approx(map = time_to_days)]
//!     next_doctors_appointment: Time,
//! }
//! ```
//!
//! ## Static Values
//! We can force a static `EPSILON` or `max_relative` value for individual fields.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Rectangle {
//!     #[approx(static_epsilon = 5e-2)]
//!     a: f64,
//!     b: f64,
//!     #[approx(static_epsilon = 7e-2)]
//!     c: f64,
//! }
//!
//! let r1 = Rectangle {
//!     a: 100.01,
//!     b: 40.0001,
//!     c: 30.055,
//! };
//! let r2 = Rectangle {
//!     a: 99.97,
//!     b: 40.0005,
//!     c: 30.049,
//! };
//!
//! // This is always true although the epsilon is smaller than the
//! // difference between fields a and b respectively.
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-1);
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-2);
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-3);
//!
//! // Here, the epsilon value has become larger than the difference between the
//! // b field values.
//! approx::assert_abs_diff_ne!(r1, r2, epsilon = 1e-4);
//! ```
//! # Struct Attributes
//! ## Default Epsilon
//! The [AbsDiffEq] trait allows to specify a default value for its `EPSILON` associated type.
//! We can control this value by specifying it on a struct level.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! #[approx(default_epsilon = 10)]
//! struct Benchmark {
//!     cycles: u64,
//!     warm_up: u64,
//! }
//!
//! let benchmark1 = Benchmark {
//!     cycles: 248,
//!     warm_up: 36,
//! };
//! let benchmark2 = Benchmark {
//!     cycles: 239,
//!     warm_up: 28,
//! };
//!
//! // When testing with not additional arguments, the results match
//! approx::assert_abs_diff_eq!(benchmark1, benchmark2);
//! // Once we specify a lower epsilon, the values do not agree anymore.
//! approx::assert_abs_diff_ne!(benchmark1, benchmark2, epsilon = 5);
//! ```
//!
//! ## Default Max Relative
//! Similarly to [Default Epsilon], we can also choose a default max_relative devaition.
//! ```
//! # use approx_derive::*;
//! #[derive(RelativeEq, PartialEq, Debug)]
//! #[approx(default_max_relative = 0.1)]
//! struct Benchmark {
//!     time: f32,
//!     warm_up: f32,
//! }
//!
//! let bench1 = Benchmark {
//!     time: 3.502785781,
//!     warm_up: 0.58039458,
//! };
//! let bench2 = Benchmark {
//!     time: 3.7023458,
//!     warm_up: 0.59015897,
//! };
//!
//! approx::assert_relative_eq!(bench1, bench2);
//! approx::assert_relative_ne!(bench1, bench2, max_relative = 0.05);
//! ```
//! ## Epsilon Type
//! When specifying nothing, the macros will infer the `EPSILON` type from the type of the
//! first struct field.
//! This can be problematic in certain scenarios which is why we can also manually specify this
//! type.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(RelativeEq, PartialEq, Debug)]
//! #[approx(epsilon_type = f32)]
//! struct Car {
//!     #[approx(cast_field)]
//!     produced_year: u32,
//!     horse_power: f32,
//! }
//!
//! let car1 = Car {
//!     produced_year: 1992,
//!     horse_power: 122.87,
//! };
//! let car2 = Car {
//!     produced_year: 2000,
//!     horse_power: 117.45,
//! };
//!
//! approx::assert_relative_eq!(car1, car2, max_relative = 0.05);
//! approx::assert_relative_ne!(car1, car2, max_relative = 0.01);
//! ```

mod args_parsing;
use args_parsing::*;

enum BaseType {
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
    fn attrs(&self) -> &Vec<syn::Attribute> {
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

    fn generics(&self) -> &syn::Generics {
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

    fn ident(&self) -> &syn::Ident {
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

struct AbsDiffEqParser {
    base_type: BaseType,
    struct_args: StructArgs,
}

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

#[derive(Debug)]
struct FieldFormatted {
    base_type: proc_macro2::TokenStream,
    own_field: proc_macro2::TokenStream,
    other_field: proc_macro2::TokenStream,
    epsilon: proc_macro2::TokenStream,
    max_relative: proc_macro2::TokenStream,
    mapping: Option<proc_macro2::TokenStream>,
    set_equal: bool,
}

impl AbsDiffEqParser {
    fn get_epsilon_parent_type(&self) -> proc_macro2::TokenStream {
        self.struct_args
            .epsilon_type
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| {
                self.fields_with_args
                    .iter()
                    .find(|field| !field.args.skip)
                    .map(|field| {
                        let field_type = &field.ty;
                        quote::quote!(#field_type)
                    })
            })
            .or_else(|| Some(quote::quote!(f64)))
            .unwrap()
    }

    fn get_derived_epsilon_type(&self) -> proc_macro2::TokenStream {
        let parent = self.get_epsilon_parent_type();
        quote::quote!(<#parent as approx::AbsDiffEq>::Epsilon)
    }

    fn get_epsilon_type_and_default_value(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let parent = self.get_epsilon_parent_type();
        let epsilon_type = self.get_derived_epsilon_type();
        let epsilon_default_value = self
            .struct_args
            .default_epsilon_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| Some(quote::quote!(<#parent as approx::AbsDiffEq>::default_epsilon())))
            .unwrap();
        (epsilon_type, epsilon_default_value)
    }

    fn generics_involved(&self) -> bool {
        let parent = self.get_epsilon_parent_type();
        self.item_struct
            .generics
            .params
            .iter()
            .any(|param| quote::quote!(#param).to_string() == parent.to_string())
    }

    fn get_max_relative_default_value(&self) -> proc_macro2::TokenStream {
        let epsilon_type = self.get_epsilon_parent_type();
        self.struct_args
            .default_max_relative_value
            .clone()
            .map(|x| quote::quote!(#x))
            .or_else(|| {
                Some(quote::quote!(<#epsilon_type as approx::RelativeEq>::default_max_relative()))
            })
            .unwrap()
    }

    fn format_nth_field(
        &self,
        n: usize,
        field_with_args: &FieldWithArgs,
    ) -> Option<FieldFormatted> {
        // Determine if this field will be skipped and exit early
        if field_with_args.args.skip {
            return None;
        }

        // Get types for epsilon and max_relative
        let parent_type = self.get_epsilon_parent_type();

        // Save field name and type in variables for easy access
        use std::str::FromStr;
        let field_name = match &field_with_args.ident {
            Some(id) => quote::quote!(#id),
            None => proc_macro2::TokenStream::from_str(&format!("{}", n)).unwrap(),
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
        let (base_type, own_field, other_field, epsilon, max_relative) = match cast_strategy {
            Some(TypeCast::CastField) => (
                quote::quote!(#parent_type),
                quote::quote!(&(self.#field_name as #parent_type)),
                quote::quote!(&(other.#field_name as #parent_type)),
                quote::quote!(#epsilon.clone()),
                quote::quote!(#max_relative.clone()),
            ),
            Some(TypeCast::CastValue) => (
                quote::quote!(#field_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon.clone() as #field_type),
                quote::quote!(#max_relative.clone() as #field_type),
            ),
            None => (
                quote::quote!(#parent_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon.clone()),
                quote::quote!(#max_relative.clone()),
            ),
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
            set_equal: field_with_args.args.set_equal,
            mapping,
        })
    }

    fn get_abs_diff_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        // We need to extend the where clause for all generics
        let fields = self
            .fields_with_args
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
                }) = self.format_nth_field(n, field_with_args)
                {
                    if set_equal {
                        Some(quote::quote!(#own_field == #other_field &&))
                    } else if let Some(map) = mapping {
                        Some(quote::quote!(
                            (if let ((Some(a), Some(b))) = (
                                (#map)(#own_field),
                                (#map)(#other_field)
                            ) {
                                approx::AbsDiffEq::abs_diff_eq(&a, &b, #epsilon)
                            } else {
                                false
                            }) &&
                        ))
                    } else {
                        Some(quote::quote!(
                            <#base_type as approx::AbsDiffEq>::abs_diff_eq(
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

    fn get_rel_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        let fields = self
            .fields_with_args
            .iter()
            .enumerate()
            .filter_map(|(n, field_with_args)| {
                if let Some(FieldFormatted {
                    base_type,
                    own_field,
                    other_field,
                    epsilon,
                    max_relative,
                    set_equal,
                    mapping,
                }) = self.format_nth_field(n, field_with_args)
                {
                    if set_equal {
                        Some(quote::quote!(#own_field == #other_field &&))
                    } else if let Some(map) = mapping {
                        Some(quote::quote!(
                            (if let ((Some(a), Some(b))) = (
                                (#map)(#own_field),
                                (#map)(#other_field)
                            ) {
                                approx::RelativeEq::relative_eq(&a, &b, #epsilon, #max_relative)
                            } else {
                                false
                            }) &&
                        ))
                    } else {
                        Some(quote::quote!(
                            <#base_type as approx::RelativeEq>::relative_eq(
                                #own_field,
                                #other_field,
                                #epsilon,
                                #max_relative
                            ) &&
                        ))
                    }
                } else {
                    None
                }
            });
        fields.collect()
    }

    fn generate_where_clause(&self, abs_diff_eq: bool) -> proc_macro2::TokenStream {
        let (epsilon_type, _) = self.get_epsilon_type_and_default_value();
        let (_, _, where_clause) = self.item_struct.generics.split_for_impl();
        let trait_bound = match abs_diff_eq {
            true => quote::quote!(approx::AbsDiffEq),
            false => quote::quote!(approx::RelativeEq),
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

    fn implement_derive_abs_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.item_struct.ident;
        let (epsilon_type, epsilon_default_value) = self.get_epsilon_type_and_default_value();
        let fields = self.get_abs_diff_eq_fields();
        let (impl_generics, ty_generics, _) = self.item_struct.generics.split_for_impl();
        let where_clause = self.generate_where_clause(true);

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
        let (impl_generics, ty_generics, _) = self.item_struct.generics.split_for_impl();
        let where_clause = self.generate_where_clause(false);

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

/// See the [crate] level documentation for a guide.
#[proc_macro_derive(AbsDiffEq, attributes(approx))]
pub fn derive_abs_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    parsed.implement_derive_abs_diff_eq().into()
}

/// See the [crate] level documentation for a guide.
#[proc_macro_derive(RelativeEq, attributes(approx))]
pub fn derive_rel_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    let mut output = quote::quote!();
    output.extend(parsed.implement_derive_abs_diff_eq());
    output.extend(parsed.implement_derive_rel_diff_eq());
    output.into()
}
