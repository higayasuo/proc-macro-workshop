use syn::{
    parse::ParseStream, Attribute, Error, Field, GenericArgument, Ident, LitStr, PathArguments,
    Result, Token, Type,
};

/// Check if `ty` is a syntax tree representing a generic type with a
/// single argument -- `Target< T >`. If so, return the argument
/// type T. Otherwise, return None.
pub fn single_arg_generic_type<'t>(ty: &'t Type, target: &'static str) -> Option<&'t Type> {
    if let Type::Path(type_path) = ty {
        if type_path.path.segments.len() == 1 {
            if let Some(seg) = type_path.path.segments.first() {
                if seg.ident == target {
                    if let PathArguments::AngleBracketed(gen_args) = &seg.arguments {
                        if gen_args.args.len() == 1 {
                            if let Some(GenericArgument::Type(tp)) = gen_args.args.first() {
                                return Some(tp);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Check if the field has an inert attribute with "builder" path like below:
/// #[builder(...)]
pub fn has_builder_attr(field: &Field) -> Option<&Attribute> {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
}

/// Parse argument to `builder` attribute, expecting the following syntax:
/// #[builder(each="XYZ")]
///           ^^^^^^^^^^  <- parse this input
/// Return the string literal "XYZ" if input is valid.
pub fn parse_builder_attr_arg(input: ParseStream) -> Result<LitStr> {
    let each_token: Ident = input.parse()?;
    if each_token != "each" {
        return Err(Error::new(each_token.span(), "expected `each`"));
    }
    input.parse::<Token![=]>()?;
    let s: LitStr = input.parse()?;
    Ok(s)
}
