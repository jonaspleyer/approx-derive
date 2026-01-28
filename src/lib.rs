#![deny(missing_docs)]
//! This crate provides derive macros for the
//! [AbsDiffEq](https://docs.rs/approx/latest/approx/trait.AbsDiffEq.html) and
//! [RelativeEq](https://docs.rs/approx/latest/approx/trait.RelativeEq.html) traits of the
//! [approx](https://docs.rs/approx/latest/approx/) crate.
//!
//! These derive macros only implement both traits with `...<Rhs = Self>`.
//! The macros infer the `EPSILON` type of the [AbsDiffEq] trait by looking
//! at the type of the first struct or enum field or any type specified by the user.
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
//! | [`#[approx(cast_value)]`](#casting-fields) | Casts the epsilon value with `.. as ..` syntax. |
//! | [`#[approx(map = ..)]`](#mapping-values) | Maps values before comparing them. |
//! | [`#[approx(epsilon_map = ..)]`](#mapping-epsilon-values) | Maps epsilon values before using them. |
//! | [`#[approx(static_epsilon = ..)]`](#static-values) | Defines a static epsilon value for this particular field. |
//! | [`#[approx(into_iter)]`](#into-iterator) | Tries to use the `into_iterator` method to compare fields. |
//! | | |
//! | **Object Attribute** | |
//! | [`#[approx(default_epsilon = ...)]`](#default-epsilon) | Sets the default epsilon value |
//! | [`#[approx(default_max_relative = ...)]`](#default-max-relative) | Sets the default `max_relative` value. |
//! | [`#[approx(epsilon_type = ...)]`](#epsilon-type) | Sets the type of the epsilon value |
//!
//! # Usage
//!
//! ```
//! # use approx::*;
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
//! assert_abs_diff_eq!(p1, p2, epsilon = 0.021);
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
//! ## Enums
//! Since `approx-derive` supports enums since `0.2`
//!
//! ```
//! # use approx::*;
//! use approx_derive::AbsDiffEq;
//!
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! enum Position {
//!     Smooth { x: f32, y: f32, },
//!     #[approx(cast_value)]
//!     Lattice { x: isize, y: isize },
//! }
//!
//! let p1 = Position::Smooth { x: 1.0, y: 1.1 };
//! let p2 = Position::Smooth { x: 1.1, y: 1.0};
//! let p3 = Position::Lattice { x: 1, y: 1 };
//!
//! assert_abs_diff_eq!(p1, p2, epsilon=0.2);
//! ```
//!
//! ```should_panic
//! # use approx::*;
//! # use approx_derive::AbsDiffEq;
//! # #[derive(AbsDiffEq, PartialEq, Debug)]
//! # enum Position {
//! #     Smooth { x: f32, y: f32, },
//! #     #[approx(cast_value)]
//! #     Lattice { x: isize, y: isize },
//! # }
//! # let p1 = Position::Smooth { x: 1.0, y: 1.1 };
//! # let p3 = Position::Lattice { x: 1, y: 1 };
//! // Note! Different enum variants can never be equal!
//! assert_abs_diff_eq!(p1, p3, epsilon = 1000.0);
//! ```
//!
//!
//! # Field Attributes
//! ## Skipping Fields
//!
//! Sometimes, we only want to compare certain fields and omit others completely.
//! ```
//! # use approx::*;
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
//! assert_abs_diff_eq!(player1, player2, epsilon = 0.5);
//! ```
//!
//! ## Testing for [Equality](core::cmp::Eq)
//!
//! When identical equality is desired, we can specify this with the `#[approx(equal)]` attribute.
//!
//! ```
//! # use approx::*;
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
//! # use approx::*;
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
//!
//! ### Example 1
//! Here, the second field will be casted to the type of the inferred epsilon value (`f32`).
//! We can check this by testing if a change in the size of `f64::MIN_POSITIVE` would get lost by
//! this procedure.
//! ```
//! # use approx::*;
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
//! assert_relative_eq!(ms1, ms2);
//! ```
//!
//! ### Example 2
//! In this example, we cast the `f64` type to `isize`.
//! ```
//! # use approx::*;
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct {
//!     v1: isize,
//!     #[approx(cast_field)]
//!     v2: f64,
//! }
//! let ms1 = MyStruct { v1: 1, v2: 2.0 };
//! let ms2 = MyStruct { v1: 1, v2: 2.1 };
//! assert_abs_diff_eq!(ms1, ms2);
//!
//! // The underlying generated code performs
//! assert!(isize::abs_diff_eq(
//!     &(ms1.v2 as isize),
//!     &(ms2.v2 as isize),
//!     0,
//! ));
//! ```
//! When we use the `#[approx(cast_value)]` syntax, we get a different result.
//! ```
//! # use approx::*;
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct2 {
//!     v1: isize,
//!     #[approx(cast_value)]
//!     v2: f64,
//! }
//! let ms1 = MyStruct2 { v1: 1, v2: 2.0 };
//! let ms2 = MyStruct2 { v1: 1, v2: 2.1 };
//! assert_abs_diff_ne!(ms1, ms2);
//!
//! // Here, the epsilon value for isize is casted to f64
//! assert!(!f64::abs_diff_eq(
//!     &ms1.v2,
//!     &ms2.v2,
//!     0isize as f64
//! ));
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
//! # assert_abs_diff_ne!(t1, t2, epsilon = 0.03);
//! ```
//!
//! This functionality can also be useful when having more complex datatypes.
//! ```
//! # use approx::*;
//! # use approx_derive::*;
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
//! ## Mapping Epsilon Values
//!
//! We can also map `epsilon` values before using them. This is usefull i.e. for tuples or arrays.
//!
//! ```
//! # use approx::*;
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct DifferentialEvolution {
//!     recombination: f32,
//!     #[approx(epsilon_map = |x| (x, x))]
//!     mutation: (f32, f32),
//! }
//! # let d1 = DifferentialEvolution {
//! #   recombination: 0.7,
//! #   mutation: (0.5, 1.5),
//! # };
//! # let d2 = DifferentialEvolution {
//! #   recombination: 0.7001,
//! #   mutation: (0.501, 1.499),
//! # };
//! # assert_abs_diff_eq!(d1, d2, epsilon = 0.02);
//! ```
//!
//! ## Static Values
//! We can force a static `EPSILON` or `max_relative` value for individual fields.
//! ```
//! # use approx::*;
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
//! assert_abs_diff_eq!(r1, r2, epsilon = 1e-1);
//! assert_abs_diff_eq!(r1, r2, epsilon = 1e-2);
//! assert_abs_diff_eq!(r1, r2, epsilon = 1e-3);
//!
//! // Here, the epsilon value has become larger than the difference between the
//! // b field values.
//! assert_abs_diff_ne!(r1, r2, epsilon = 1e-4);
//! ```
//! # Object Attributes
//! ## Default Epsilon
//! The [AbsDiffEq] trait allows to specify a default value for its `EPSILON` associated type.
//! We can control this value by specifying it on an object level.
//!
//! ```
//! # use approx::*;
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
//! assert_abs_diff_eq!(benchmark1, benchmark2);
//! // Once we specify a lower epsilon, the values do not agree anymore.
//! assert_abs_diff_ne!(benchmark1, benchmark2, epsilon = 5);
//! ```
//!
//! ## Default Max Relative
//! Similarly to [Default Epsilon], we can also choose a default max_relative devaition.
//! ```
//! # use approx_derive::*;
//! # use approx::*;
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
//! assert_relative_eq!(bench1, bench2);
//! assert_relative_ne!(bench1, bench2, max_relative = 0.05);
//! ```
//! ## Epsilon Type
//! When specifying nothing, the macros will infer the `EPSILON` type from the type of the
//! first struct/enum field (the order in which it is parsed).
//! This can be problematic in certain scenarios which is why we can also manually specify this
//! type.
//!
//! ```
//! # use approx::*;
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
//! assert_relative_eq!(car1, car2, max_relative = 0.05);
//! assert_relative_ne!(car1, car2, max_relative = 0.01);
//! ```
//!
//! # Into Iterator
//! To compare two fields which consist of a iterable list of values, we can use the
//! `#[approx(into_iter)]` field attribute.
//!
//! ```
//! # use approx::*;
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Parameter {
//!     value: f32,
//!     #[approx(into_iter)]
//!     bounds: [f32; 2],
//! }
//! let p1 = Parameter { value: 3.144, bounds: [0.0, 10.0] };
//! let p2 = Parameter { value: 3.145, bounds: [0.1, 10.2] };
//!
//! assert_abs_diff_ne!(p1, p2);
//! assert_abs_diff_eq!(p1, p2, epsilon = 0.21);
//! ```
//! It has to be noted that whenever both iterator are not of the same length, that the comparison
//! will fail.
//!
//! ```should_panic
//! # use approx::*;
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! #[approx(epsilon_type = f64)]
//! struct Polynomial {
//!     #[approx(into_iter)]
//!     coefficients: Vec<f64>,
//! }
//! let poly1 = Polynomial { coefficients: vec![1.0, 0.5] };
//! let poly2 = Polynomial { coefficients: vec![1.0, 0.5, 1.0/6.0] };
//! assert_abs_diff_eq!(poly1, poly2);
//! ```

mod abs_diff_eq;
mod args_parsing;
mod base_types;
mod rel_diff_eq;

use args_parsing::*;
use base_types::*;

struct AbsDiffEqParser {
    pub base_type: BaseType,
    pub struct_args: StructArgs,
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
