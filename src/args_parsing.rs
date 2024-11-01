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
    pub skip: bool,
    pub set_equal: bool,
    pub cast_strategy: Option<TypeCast>,
    pub epsilon_static_value: Option<syn::Expr>,
    pub max_relative_static_value: Option<syn::Expr>,
    pub mapping: Option<syn::Expr>,
}

/// Every value argument specified by `#[approx(value)]`
pub enum FieldValueArg {
    Skip,
    CastStrategy(TypeCast),
    Equal,
}

impl FieldValueArg {
    fn from_ident(ident: &syn::Ident) -> syn::Result<Self> {
        match ident.to_string().as_str() {
            "skip" => Ok(FieldValueArg::Skip),
            "cast_field" => Ok(FieldValueArg::CastStrategy(TypeCast::CastField)),
            "cast_value" => Ok(FieldValueArg::CastStrategy(TypeCast::CastValue)),
            "equal" => Ok(FieldValueArg::Equal),
            _ => Err(syn::Error::new(ident.span(), "Not a valid value.")),
        }
    }
}

/// Every key-value pair specified by `#[approx(key = value)]`
pub enum FieldKeyValueArg {
    EpsilonStatic(Option<syn::Expr>),
    MaxRelativeStatic(Option<syn::Expr>),
    Mapping(Option<syn::Expr>),
}

impl FieldKeyValueArg {
    fn parse_value(keyword: &syn::Ident, input: syn::parse::ParseStream) -> syn::Result<Self> {
        match keyword.to_string().as_str() {
            "static_epsilon" => Ok(Self::EpsilonStatic(Some(input.parse()?))),
            "static_max_relative" => Ok(Self::MaxRelativeStatic(Some(input.parse()?))),
            "map" => Ok(Self::Mapping(Some(input.parse()?))),
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
    pub fn from_ident(ident: &syn::Ident) -> syn::Result<Self> {
        match ident.to_string().as_str() {
            _ => Ok(Self::None),
            // _ => Err(syn::Error::new(ident.span(), "Not a valid value")),
        }
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
    pub fn from_attrs(attributes: &Vec<syn::Attribute>) -> syn::Result<Self> {
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
    fn from_attrs(attributes: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut skip = false;
        let mut set_equal = false;
        let mut mapping = None;
        let mut cast_strategy = None;
        let mut epsilon_static_value = None;
        let mut max_relative_static_value = None;
        for attribute in attributes.iter() {
            // Only do anything if approx is specified
            if attribute.path().is_ident("approx") {
                let arg: FieldArgGeneric = attribute.parse_args()?;
                match arg {
                    FieldArgGeneric::Value(FieldValueArg::Skip) => skip = true,
                    FieldArgGeneric::Value(FieldValueArg::CastStrategy(strategy)) => {
                        cast_strategy = Some(strategy)
                    }
                    FieldArgGeneric::Value(FieldValueArg::Equal) => set_equal = true,
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::EpsilonStatic(epsilon_static)) => {
                        epsilon_static_value = epsilon_static;
                    }
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::MaxRelativeStatic(
                        max_rel_static,
                    )) => {
                        max_relative_static_value = max_rel_static;
                    }
                    FieldArgGeneric::KeyValue(FieldKeyValueArg::Mapping(expr)) => mapping = expr,
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
        })
    }
}
