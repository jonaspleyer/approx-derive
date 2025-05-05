#[derive(Clone)]
pub enum TypeCast {
    CastField,
    CastValue,
}

/// Represents a field in a struct definition
pub struct FieldWithArgs {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub args: FieldArgs,
}

pub struct EnumVariant {
    pub ident: syn::Ident,
    pub discriminant: Option<syn::Expr>,
    pub fields_with_args: Vec<FieldWithArgs>,
}

impl FieldWithArgs {
    pub fn from_field(field: &syn::Field) -> syn::Result<Self> {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        let args = FieldArgs::from_attrs(&field.attrs)?;
        Ok(Self { ident, ty, args })
    }
}

/// All arguments that can be specified and parsed in a field
pub struct FieldArgs {
    pub skip: Option<bool>,
    pub set_equal: Option<bool>,
    pub cast_strategy: Option<TypeCast>,
    pub epsilon_static_value: Option<syn::Expr>,
    pub max_relative_static_value: Option<syn::Expr>,
    pub mapping: Option<syn::Expr>,
    pub epsilon_mapping: Option<syn::Expr>,
    pub max_relative_mapping: Option<syn::Expr>,
    pub use_iterator: Option<bool>,
}

impl FieldArgs {
    pub fn patch_if_not_exists(&mut self, other: &Self) {
        *self = Self {
            skip: self.skip.or(other.skip),
            set_equal: self.set_equal.or(other.set_equal),
            cast_strategy: self.cast_strategy.clone().or(other.cast_strategy.clone()),
            epsilon_static_value: self
                .epsilon_static_value
                .clone()
                .or(other.epsilon_static_value.clone()),
            max_relative_static_value: self
                .max_relative_static_value
                .clone()
                .or(other.max_relative_static_value.clone()),
            mapping: self.mapping.clone().or(other.mapping.clone()),
            epsilon_mapping: self
                .epsilon_mapping
                .clone()
                .or(other.epsilon_mapping.clone()),
            max_relative_mapping: self
                .max_relative_mapping
                .clone()
                .or(other.max_relative_mapping.clone()),
            use_iterator: self.use_iterator.or(other.use_iterator),
        };
    }
}

/// Every value argument specified by `#[approx(value)]`
pub enum FieldValueArg {
    Skip,
    CastStrategy(TypeCast),
    Equal,
    Iter,
}

impl FieldValueArg {
    fn from_ident(ident: &syn::Ident) -> syn::Result<Self> {
        match ident.to_string().as_str() {
            "skip" => Ok(FieldValueArg::Skip),
            "cast_field" => Ok(FieldValueArg::CastStrategy(TypeCast::CastField)),
            "cast_value" => Ok(FieldValueArg::CastStrategy(TypeCast::CastValue)),
            "equal" => Ok(FieldValueArg::Equal),
            "into_iter" => Ok(FieldValueArg::Iter),
            _ => Err(syn::Error::new(ident.span(), "Not a valid value.")),
        }
    }
}

/// Every key-value pair specified by `#[approx(key = value)]`
pub enum FieldKeyValueArg {
    EpsilonStatic(Option<syn::Expr>),
    MaxRelativeStatic(Option<syn::Expr>),
    Mapping(Option<syn::Expr>),
    EpsilonMapping(Option<syn::Expr>),
    MaxRelativeMapping(Option<syn::Expr>),
}

impl FieldKeyValueArg {
    fn parse_value(keyword: &syn::Ident, input: syn::parse::ParseStream) -> syn::Result<Self> {
        match keyword.to_string().as_str() {
            "static_epsilon" => Ok(Self::EpsilonStatic(Some(input.parse()?))),
            "static_max_relative" => Ok(Self::MaxRelativeStatic(Some(input.parse()?))),
            "map" => Ok(Self::Mapping(Some(input.parse()?))),
            "epsilon_map" => Ok(Self::EpsilonMapping(Some(input.parse()?))),
            "max_relative_map" => Ok(Self::MaxRelativeMapping(Some(input.parse()?))),
            _ => Err(syn::Error::new(keyword.span(), "Not a valid keyword")),
        }
    }
}

/// All arguments that can be specified at struct level.
///
/// ```ignore
/// #[derive(PartialEq, Debug, Approx)]
/// #[approx(some_struct_arg)]
/// struct MyStruct {
///     my_value: f64,
/// }
/// ```
pub struct StructArgs {
    pub epsilon_type: Option<syn::Type>,
    pub default_epsilon_value: Option<syn::Expr>,
    pub default_max_relative_value: Option<syn::Expr>,
}

/// Generic Field argument which can be either value or key-value
pub enum FieldArgGeneric {
    Value(FieldValueArg),
    KeyValue(FieldKeyValueArg),
}

impl syn::parse::Parse for FieldArgGeneric {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if input.peek(syn::Token![=]) {
            let keyword = ident;
            let _: syn::Token![=] = input.parse()?;
            let approx_key_value_arg = FieldKeyValueArg::parse_value(&keyword, input)?;
            return Ok(Self::KeyValue(approx_key_value_arg));
        }
        Ok(Self::Value(FieldValueArg::from_ident(&ident)?))
    }
}

pub enum StructValueArg {
    None,
}

impl StructValueArg {
    pub fn from_ident(_ident: &syn::Ident) -> syn::Result<Self> {
        // match ident.to_string().as_str() {
        //     _ => Ok(Self::None),
        //     // _ => Err(syn::Error::new(ident.span(), "Not a valid value")),
        // }
        Ok(Self::None)
    }
}

pub enum StructKeyValueArg {
    EpsilonType(syn::Type),
    DefaultEpsilon(syn::Expr),
    DefaultMaxRelative(syn::Expr),
}

impl StructKeyValueArg {
    pub fn parse_value(keyword: &syn::Ident, input: syn::parse::ParseStream) -> syn::Result<Self> {
        match keyword.to_string().as_str() {
            "epsilon_type" => Ok(Self::EpsilonType(input.parse()?)),
            "default_epsilon" => Ok(Self::DefaultEpsilon(input.parse()?)),
            "default_max_relative" => Ok(Self::DefaultMaxRelative(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Not a valid keyword")),
        }
    }
}

pub enum StructArgGeneric {
    Value(StructValueArg),
    KeyValue(StructKeyValueArg),
}

impl syn::parse::Parse for StructArgGeneric {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if input.peek(syn::Token![=]) {
            // Now we know that this will be a key-value pair
            let keyword = ident;
            let _: syn::Token![=] = input.parse()?;
            let key_value_arg = StructKeyValueArg::parse_value(&keyword, input)?;
            return Ok(Self::KeyValue(key_value_arg));
        }
        // Otherwise we know that it is a value
        Ok(Self::Value(StructValueArg::from_ident(&ident)?))
    }
}

impl StructArgs {
    pub fn from_attrs(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        let mut epsilon_type = None;
        let mut default_epsilon_value = None;
        let mut default_max_relative_value = None;
        for attribute in attributes.iter() {
            match attribute.parse_args() {
                Ok(StructArgGeneric::Value(StructValueArg::None)) => (),
                Ok(StructArgGeneric::KeyValue(StructKeyValueArg::EpsilonType(epsilon_ty))) => {
                    epsilon_type = Some(epsilon_ty)
                }
                Ok(StructArgGeneric::KeyValue(StructKeyValueArg::DefaultEpsilon(default_eps))) => {
                    default_epsilon_value = Some(default_eps)
                }
                Ok(StructArgGeneric::KeyValue(StructKeyValueArg::DefaultMaxRelative(
                    default_max_rel,
                ))) => {
                    default_max_relative_value = Some(default_max_rel);
                }
                Err(_) => {}
            }
        }
        Ok(Self {
            epsilon_type,
            default_epsilon_value,
            default_max_relative_value,
        })
    }
}

impl FieldArgs {
    pub fn from_attrs(attributes: &[syn::Attribute]) -> syn::Result<Self> {
        let mut skip = None;
        let mut set_equal = None;
        let mut mapping = None;
        let mut epsilon_mapping = None;
        let mut max_relative_mapping = None;
        let mut cast_strategy = None;
        let mut epsilon_static_value = None;
        let mut max_relative_static_value = None;
        let mut iter = None;
        for attribute in attributes.iter() {
            // Only do anything if approx is specified
            if attribute.path().is_ident("approx") {
                let arg: FieldArgGeneric = attribute.parse_args()?;
                match arg {
                    FieldArgGeneric::Value(FieldValueArg::Skip) => skip = Some(true),
                    FieldArgGeneric::Value(FieldValueArg::CastStrategy(strategy)) => {
                        cast_strategy = Some(strategy)
                    }
                    FieldArgGeneric::Value(FieldValueArg::Equal) => set_equal = Some(true),
                    FieldArgGeneric::Value(FieldValueArg::Iter) => iter = Some(true),
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::EpsilonStatic(epsilon_static)) => {
                        epsilon_static_value = epsilon_static;
                    }
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::MaxRelativeStatic(
                        max_rel_static,
                    )) => {
                        max_relative_static_value = max_rel_static;
                    }
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::Mapping(expr)) => mapping = expr,
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::EpsilonMapping(expr)) => {
                        epsilon_mapping = expr
                    }
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::MaxRelativeMapping(expr)) => {
                        max_relative_mapping = expr
                    }
                }
            }
        }
        Ok(Self {
            skip,
            set_equal,
            cast_strategy,
            epsilon_static_value,
            max_relative_static_value,
            mapping,
            epsilon_mapping,
            max_relative_mapping,
            use_iterator: iter,
        })
    }
}
