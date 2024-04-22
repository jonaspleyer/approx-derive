use quote::ToTokens;
struct Parser(syn::ItemStruct);

struct ApproxDeriveArgs {
    skip: bool,
}

impl ApproxDeriveArgs {
    fn extract(attributes: &Vec<syn::Attribute>) -> Self {
        let mut skip = false;
        attributes.iter().for_each(|attribute| {
            let arg_expr: syn::Ident = attribute.parse_args().unwrap();
            if arg_expr == "skip" {
                skip = true;
            }
        });
        Self {
            skip,
        }
    }
}

impl syn::parse::Parse for Parser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parser {
    fn implement_derive_abs_diff_eq_f64(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.0.ident;
        let fields = self.0.fields.clone().into_iter().filter_map(|field| {
            let args = ApproxDeriveArgs::extract(&field.attrs);
            if args.skip {
                None
            } else {
                // TODO add options to specify epsilon here as well
                Some(field.ident)
            }
        });

        // We need to extend the where clause for all generics
        let (impl_generics, ty_generics, where_clause) = self.0.generics.split_for_impl();

        let res = quote::quote!(
            const _ : () = {
                #[automatically_derived]
                impl #impl_generics approx::AbsDiffEq for #struct_name #ty_generics
                #where_clause
                {
                    type Epsilon = f64;

                    fn default_epsilon() -> Self::Epsilon {
                        f64::EPSILON
                    }

                    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                        #(
                            self.#fields.abs_diff_eq(&other.#fields, epsilon) &&
                        )*
                        true
                    }
                }
            };
        );
        // println!("{}", res);
        res
    }
}

#[proc_macro_derive(AbsDiffEq_f64, attributes(approx))]
pub fn derive_abs_diff_eq_f64(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as Parser);
    parsed.implement_derive_abs_diff_eq_f64().into()
}
