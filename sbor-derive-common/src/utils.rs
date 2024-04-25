use itertools::Itertools;
use std::collections::BTreeMap;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::*;

#[allow(dead_code)]
pub fn print_generated_code<S: ToString>(kind: &str, code: S) {
    if let Ok(mut proc) = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        {
            let stdin = proc.stdin.as_mut().unwrap();
            stdin.write_all(code.to_string().as_bytes()).unwrap();
        }
        if let Ok(output) = proc.wait_with_output() {
            if output.status.success() {
                println!(
                    "{}\n{}\n{}\n{}\n",
                    "-".repeat(kind.len()),
                    kind,
                    "-".repeat(kind.len()),
                    String::from_utf8(output.stdout).unwrap()
                );
            }
        }
    }
}

pub enum AttributeValue {
    None(Span),
    Path(Path),
    Lit(Lit),
}

impl AttributeValue {
    fn as_string(&self) -> Option<LitStr> {
        match self {
            AttributeValue::Lit(Lit::Str(str)) => Some(str.clone()),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<LitBool> {
        match self {
            AttributeValue::None(_) => Some(LitBool::new(true, self.span())),
            AttributeValue::Lit(Lit::Str(str)) => match str.value().as_str() {
                "true" => Some(LitBool::new(true, self.span())),
                "false" => Some(LitBool::new(false, self.span())),
                _ => None,
            },
            AttributeValue::Lit(Lit::Bool(bool)) => Some(bool.clone()),
            _ => None,
        }
    }

    fn span(&self) -> Span {
        match self {
            AttributeValue::None(span) => *span,
            AttributeValue::Path(path) => path.span(),
            AttributeValue::Lit(lit) => lit.span(),
        }
    }
}

trait AttributeMap {
    fn get_bool_value(&self, name: &str) -> Result<LitBool>;
    fn get_string_value(&self, name: &str) -> Result<Option<LitStr>>;
}

impl AttributeMap for BTreeMap<String, AttributeValue> {
    fn get_bool_value(&self, name: &str) -> Result<LitBool> {
        let Some(value) = self.get(name) else {
            return Ok(LitBool::new(false, Span::call_site()));
        };
        value
            .as_bool()
            .ok_or_else(|| Error::new(value.span(), format!("Expected bool attribute")))
    }

    fn get_string_value(&self, name: &str) -> Result<Option<LitStr>> {
        let Some(value) = self.get(name) else {
            return Ok(None);
        };
        Ok(Some(value.as_string().ok_or_else(|| {
            Error::new(value.span(), format!("Expected string attribute value"))
        })?))
    }
}

/// Permits attribute of the form #[sbor(opt1, opt2 = X, opt3(Y))] for some literal X or some path or literal Y.
pub fn extract_sbor_typed_attributes(
    attributes: &[Attribute],
) -> Result<BTreeMap<String, AttributeValue>> {
    extract_typed_attributes(attributes, "sbor")
}

/// Permits attribute of the form #[{name}(opt1, opt2 = X, opt3(Y))] for some literal X or some path or literal Y.
pub fn extract_typed_attributes(
    attributes: &[Attribute],
    name: &str,
) -> Result<BTreeMap<String, AttributeValue>> {
    let mut fields = BTreeMap::new();
    for attribute in attributes {
        if !attribute.path.is_ident(name) {
            continue;
        }
        let Ok(meta) = attribute.parse_meta() else {
            return Err(Error::new(
                attribute.span(),
                format!("Attribute content is not valid"),
            ));
        };
        let Meta::List(MetaList {
            nested: options, ..
        }) = meta
        else {
            return Err(Error::new(
                attribute.span(),
                format!("Expected list-based attribute as #[{name}(..)]"),
            ));
        };
        let error_message = format!("Expected attribute of the form #[{name}(opt1, opt2 = X, opt3(Y))] for some literal X or some path or literal Y.");
        for option in options.into_iter() {
            match option {
                NestedMeta::Meta(m) => match m {
                    Meta::Path(path) => {
                        if let Some(ident) = path.get_ident() {
                            fields.insert(ident.to_string(), AttributeValue::None(path.span()));
                        } else {
                            return Err(Error::new(path.span(), error_message));
                        }
                    }
                    Meta::NameValue(name_value) => {
                        if let Some(ident) = name_value.path.get_ident() {
                            fields.insert(ident.to_string(), AttributeValue::Lit(name_value.lit));
                        } else {
                            return Err(Error::new(name_value.path.span(), error_message));
                        }
                    }
                    Meta::List(MetaList { nested, path, .. }) => {
                        if let Some(ident) = path.get_ident() {
                            if nested.len() == 1 {
                                match nested.into_iter().next().unwrap() {
                                    NestedMeta::Meta(inner_meta) => match inner_meta {
                                        Meta::Path(path) => {
                                            fields.insert(
                                                ident.to_string(),
                                                AttributeValue::Path(path.clone()),
                                            );
                                        }
                                        _ => {
                                            return Err(Error::new(
                                                inner_meta.span(),
                                                error_message,
                                            ));
                                        }
                                    },
                                    NestedMeta::Lit(lit) => {
                                        fields.insert(
                                            ident.to_string(),
                                            AttributeValue::Lit(lit.clone()),
                                        );
                                    }
                                }
                            } else {
                                return Err(Error::new(nested.span(), error_message));
                            }
                        } else {
                            return Err(Error::new(path.span(), error_message));
                        }
                    }
                },
                _ => {
                    return Err(Error::new(option.span(), error_message));
                }
            }
        }
    }

    Ok(fields)
}

pub(crate) enum SourceVariantData {
    Reachable(VariantData),
    Unreachable(UnreachableVariantData),
}

#[derive(Clone)]
pub(crate) struct UnreachableVariantData {
    pub source_variant: Variant,
    pub fields_data: FieldsData,
}

#[derive(Clone)]
pub(crate) struct VariantData {
    pub source_variant: Variant,
    pub discriminator: Expr,
    pub discriminator_pattern: Pat,
    pub fields_data: FieldsData,
}

pub(crate) struct EnumVariantsData {
    pub source_variants: Vec<SourceVariantData>,
    pub sbor_variants: Vec<VariantData>,
}

pub(crate) fn process_enum_variants(
    enum_attributes: &[Attribute],
    variants: &Punctuated<Variant, Comma>,
) -> Result<EnumVariantsData> {
    if variants.len() > 255 {
        return Err(Error::new(
            Span::call_site(),
            format!("SBOR can only support enums of size <= 255"),
        ));
    }

    let use_repr_discriminators =
        get_sbor_attribute_bool_value(enum_attributes, "use_repr_discriminators")?.value();

    let mut explicit_discriminators_count = 0usize;
    let mut implicit_discriminators_count = 0usize;
    let mut reachable_variants_count = 0usize;

    let source_variants: Vec<SourceVariantData> = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| -> Result<_> {
            let mut variant_attributes = extract_typed_attributes(&variant.attrs, "sbor")?;
            let fields_data = process_fields(&variant.fields)?;
            let source_variant = variant.clone();
            if let Some(_) = variant_attributes.remove("unreachable") {
                return Ok(SourceVariantData::Unreachable(UnreachableVariantData {
                    fields_data,
                    source_variant,
                }));
            }
            reachable_variants_count += 1;
            let discriminator =
                resolve_discriminator(use_repr_discriminators, i, variant, variant_attributes)?;
            if discriminator.implicit {
                implicit_discriminators_count += 1;
            } else {
                explicit_discriminators_count += 1;
            };
            Ok(SourceVariantData::Reachable(VariantData {
                discriminator: discriminator.expression,
                discriminator_pattern: discriminator.pattern,
                source_variant,
                fields_data,
            }))
        })
        .collect::<Result<_>>()?;

    if explicit_discriminators_count > 0 && implicit_discriminators_count > 0 {
        return Err(Error::new(
            Span::call_site(),
            format!("Either all or no variants must be assigned an explicit discriminator. Currently {} of {} variants have one.", explicit_discriminators_count, reachable_variants_count),
        ));
    }

    let sbor_variants = source_variants
        .iter()
        .filter_map(|source_variant| match source_variant {
            SourceVariantData::Reachable(variant_data) => Some(variant_data.clone()),
            SourceVariantData::Unreachable(_) => None,
        })
        .collect();

    Ok(EnumVariantsData {
        source_variants,
        sbor_variants,
    })
}

struct Discriminator {
    expression: Expr,
    pattern: Pat,
    implicit: bool,
}

impl Discriminator {
    fn explicit_u8(value: &LitByte) -> Self {
        Self {
            expression: parse_quote!(#value),
            pattern: parse_quote!(#value),
            implicit: false,
        }
    }

    fn explicit_path(value: &Path) -> Self {
        Self {
            expression: parse_quote!(#value),
            pattern: parse_quote!(#value),
            implicit: false,
        }
    }

    fn explicit(expression: Expr, pattern: Pat) -> Self {
        Self {
            expression,
            pattern,
            implicit: false,
        }
    }

    fn implicit_u8(span: Span, index: usize) -> Option<Self> {
        let value = LitByte::new(u8::try_from(index).ok()?, span);
        Some(Self {
            expression: parse_quote!(#value),
            pattern: parse_quote!(#value),
            implicit: true,
        })
    }
}

fn resolve_discriminator(
    use_repr_discriminators: bool,
    index: usize,
    variant: &Variant,
    mut variant_attributes: BTreeMap<String, AttributeValue>,
) -> Result<Discriminator> {
    if let Some(attribute) = variant_attributes.remove("discriminator") {
        match attribute {
            AttributeValue::None(span) => {
                return Err(Error::new(span, format!("No discriminator was provided")));
            }
            AttributeValue::Path(path) => {
                return Ok(Discriminator::explicit_path(&path));
            }
            AttributeValue::Lit(literal) => {
                if let Some(b) = parse_u8_from_literal(&literal) {
                    return Ok(Discriminator::explicit_u8(&b));
                }
                let expr = parse_expr_from_literal(&literal);
                let pattern = parse_pattern_from_literal(&literal);
                if let Some(expr) = expr {
                    if let Some(pattern) = pattern {
                        return Ok(Discriminator::explicit(expr, pattern));
                    }
                }
                return Err(Error::new(
                    literal.span(),
                    format!("This discriminator is not convertible into a u8; or convertible into both an expression and a pattern. You may want to try using a path to a constant instead for more power."),
                ));
            }
        }
    }

    if use_repr_discriminators {
        if let Some(discriminant) = &variant.discriminant {
            let expression = &discriminant.1;

            let parsed = match expression {
                Expr::Lit(literal_expression) => parse_u8_from_literal(&literal_expression.lit)
                    .map(|b| Discriminator::explicit_u8(&b)),
                Expr::Path(path_expression) => {
                    Some(Discriminator::explicit_path(&path_expression.path))
                }
                _ => None,
            };

            let Some(disc) = parsed else {
                return Err(Error::new(
                    expression.span(),
                    format!("This discriminator is not a u8-convertible value or a path. Add an #[sbor(discriminator(X))] annotation with a u8-compatible literal or path to const/static variable to fix."),
                ));
            };
            return Ok(disc);
        }
    }

    let implicit = Discriminator::implicit_u8(variant.span(), index)
        .ok_or_else(|| Error::new(variant.span(), format!("Too many variants")))?;

    Ok(implicit)
}

fn parse_u8_from_literal(literal: &Lit) -> Option<LitByte> {
    match literal {
        Lit::Byte(byte_literal) => Some(byte_literal.clone()),
        Lit::Int(int_literal) => Some(LitByte::new(
            int_literal.base10_parse::<u8>().ok()?,
            literal.span(),
        )),
        Lit::Str(str_literal) => Some(LitByte::new(
            str_literal.value().parse::<u8>().ok()?,
            literal.span(),
        )),
        _ => None,
    }
}

fn parse_expr_from_literal(literal: &Lit) -> Option<Expr> {
    match literal {
        Lit::Str(str_literal) => str_literal.parse().ok(),
        _ => None,
    }
}

fn parse_pattern_from_literal(literal: &Lit) -> Option<Pat> {
    match literal {
        Lit::Str(str_literal) => str_literal.parse().ok(),
        _ => None,
    }
}

pub fn get_sbor_attribute_string_value(
    attributes: &[Attribute],
    attribute_name: &str,
) -> Result<Option<LitStr>> {
    extract_sbor_typed_attributes(attributes)?.get_string_value(attribute_name)
}

pub fn get_sbor_attribute_bool_value(
    attributes: &[Attribute],
    attribute_name: &str,
) -> Result<LitBool> {
    extract_sbor_typed_attributes(attributes)?.get_bool_value(attribute_name)
}

pub fn is_skipped(f: &Field) -> Result<bool> {
    let attributes = extract_sbor_typed_attributes(&f.attrs)?;
    Ok(attributes.get_bool_value("skip")?.value())
}

pub enum DeriveStrategy {
    Normal,
    Transparent,
    DeriveAs {
        as_type: Type,
        as_ref: TokenStream,
        from_value: TokenStream,
    },
}

pub fn get_derive_strategy(attributes: &[Attribute]) -> Result<DeriveStrategy> {
    let attributes = extract_sbor_typed_attributes(attributes)?;
    let transparent_flag = attributes.get_bool_value("transparent")?;
    let as_type = attributes.get_string_value("as_type")?;
    let as_ref = attributes.get_string_value("as_ref")?;
    let from_value = attributes.get_string_value("from_value")?;
    match (transparent_flag.value(), as_type, as_ref, from_value) {
        (true, None, None, None) => Ok(DeriveStrategy::Transparent),
        (true, _, _, _) => Err(Error::new(
            transparent_flag.span,
            "The `transparent` option cannot be used with `as_type` / `as_ref` / `from_value`",
        )),
        (false, Some(as_type), as_ref, from_value) => {
            let as_type_str = as_type.value();
            let as_type: Type = as_type.parse()?;
            Ok(DeriveStrategy::DeriveAs {
                as_ref: match as_ref {
                    Some(v) => {
                        if v.value().contains("self") {
                            v.parse()?
                        } else {
                            return Err(Error::new(v.span(), format!("The `as_ref` value should be code mapping `self` into a &{as_type_str}")));
                        }
                    }
                    None => quote! { <Self as core::convert::AsRef<#as_type>>::as_ref(self) },
                },
                from_value: match from_value {
                    Some(v) => {
                        if v.value().contains("value") {
                            v.parse()?
                        } else {
                            return Err(Error::new(v.span(), format!("The `from_value` value should be code mapping `value` (of type {as_type_str}) into `Self`")));
                        }
                    }
                    None => quote! { <Self as core::convert::From<#as_type>>::from(value) },
                },
                as_type,
            })
        }
        (false, None, Some(_), Some(_)) => Err(Error::new(
            transparent_flag.span,
            "The `as_ref` or `from_value` options cannot be used without `as_type`",
        )),
        (false, None, Some(_), None) => Err(Error::new(
            transparent_flag.span,
            "The `as_ref` option cannot be used without `as_type`",
        )),
        (false, None, None, Some(_)) => Err(Error::new(
            transparent_flag.span,
            "The `from_value` option cannot be used without `as_type`",
        )),
        (false, None, None, None) => Ok(DeriveStrategy::Normal),
    }
}

pub fn is_transparent(attributes: &[Attribute]) -> Result<bool> {
    let attributes = extract_sbor_typed_attributes(attributes)?;
    Ok(attributes.get_bool_value("transparent")?.value())
}

pub fn get_custom_value_kind(attributes: &[Attribute]) -> Result<Option<LitStr>> {
    get_sbor_attribute_string_value(attributes, "custom_value_kind")
}

pub fn get_custom_type_kind(attributes: &[Attribute]) -> Result<Option<LitStr>> {
    get_sbor_attribute_string_value(attributes, "custom_type_kind")
}

pub fn resolve_type_name(ident: &Ident, attributes: &[Attribute]) -> Result<LitStr> {
    let type_name = get_sbor_attribute_string_value(attributes, "type_name")?
        .unwrap_or(LitStr::new(&ident.to_string(), ident.span()));
    Ok(type_name)
}

pub fn get_generic_types(generics: &Generics) -> Vec<Type> {
    generics
        .type_params()
        .map(|type_param| {
            let ident = &type_param.ident;
            parse_quote!(#ident)
        })
        .collect()
}

pub fn parse_single_type(source_string: &LitStr) -> syn::Result<Type> {
    source_string.parse()
}

pub fn parse_comma_separated_types(source_string: &LitStr) -> syn::Result<Vec<Type>> {
    let span = source_string.span();
    source_string
        .value()
        .split(',')
        .map(|s| s.trim().to_owned())
        .filter(|f| f.len() > 0)
        .map(|s| LitStr::new(&s, span).parse())
        .collect()
}

/// Child types are intended to capture what non-concrete types are embedded in the
/// given type as descendents in the SBOR value model - these types will require explicit bounds for
/// `Encode` / `Decode` / `Describe`.
///
/// By default, like e.g. the default `Clone` impl, we assume that all generic types are
/// child types. But a user can override this with the `#[sbor(child_types = "A,B")]` attribute.
///
/// One of the prime use cases for this is where associated types are used, for example
/// a type `<T> MyStruct(T::MyAssociatedType)` should use `#[sbor(child_types = "T::MyAssociatedType")]`.
fn get_child_types(attributes: &[Attribute], existing_generics: &Generics) -> Result<Vec<Type>> {
    let Some(comma_separated_types) = get_sbor_attribute_string_value(attributes, "child_types")?
    else {
        // If no explicit child_types list is set, we use all pre-existing generic type parameters.
        // This means (eg) that they all have to implement the relevant trait (Encode/Decode/Describe)
        // This is essentially what derived traits such as Clone do: https://github.com/rust-lang/rust/issues/26925
        // It's not perfect - but it's typically good enough!
        return Ok(get_generic_types(existing_generics));
    };

    parse_comma_separated_types(&comma_separated_types)
}

fn get_types_requiring_categorize_bound_for_encode_and_decode(
    attributes: &[Attribute],
) -> Result<Vec<Type>> {
    let comma_separated_types = get_sbor_attribute_string_value(attributes, "categorize_types")?;
    // We need to work out what the default behaviour is if no `categorize_types` are provided.
    //
    // Now, for a given generic parameter T, we have a few cases how it appears in the type:
    // 1. It's embedded as a T into the type, e.g. MyNewType(T)
    // 2. It's wrapped in a collection like a Vec<T> or Map<..>
    //
    // Note that only in case 2 (the rarer case) do we require that T implements Categorize.
    //
    // We can do one of the following options:
    // (A) Assume none
    // (B) Use child_types if they exist
    // (C) Use child_types if they exist, else use the existing generic parameters
    // (D) Use the existing generic parameters
    //
    // - The issue with (C/D) is that generic types such as <T>MyStruct(T) require you to know to include
    //   #[sbor(categorize_types = "")] otherwise they cryptically don't get an Encode/Decode implementation
    //   for something like T = sbor::Value, which is subtle, easy to miss, and perplexing when it happens.
    // - The issue with (B) is the same as (C) - the error is too cryptic. Let's say we define
    //   #[sbor(child_types = "T::MyAssociatedValue")] and the type is <T>MyStruct(T::MyAssociatedValue),
    //   then we lose an encode implementation if T::MyAssociatedValue = sbor::Value.
    // - The issue with (A) is that generic types such as <T>MyStruct(Vec<T>) require you to know to include
    //   #[sbor(categorize_types = "T")] else the implementation doesn't compile. This I think is a slightly
    //   clearer error than (C).
    //
    // We used to use (C), but we have now switched to (A) because we believe it to be clearer, on balance.
    // Also, providing #[sbor(categorize_types = "T")] feels more explicit sometimes than confusingly having
    // to add #[sbor(categorize_types = "")] sometimes.
    if let Some(comma_separated_types) = comma_separated_types {
        parse_comma_separated_types(&comma_separated_types)
    } else {
        Ok(vec![])
    }
}

/// Note - this is only needed for implementing an inherited categorize.
pub fn get_type_requiring_categorize_bound_for_categorize_as(
    as_type: &Type,
    attributes: &[Attribute],
    existing_generics: &Generics,
) -> Result<Option<Type>> {
    let explicit_type = get_sbor_attribute_string_value(attributes, "categorize_as")?;

    if let Some(explicit_type) = explicit_type {
        Ok(Some(parse_single_type(&explicit_type)?))
    } else {
        // We need to work out what the default behaviour is if no `categorize_as` are provided.
        //
        // Thankfully we have a 99% strategy:
        // - If `as_type` _IS_ a child type, then it must be generic, not concrete.
        // - If `as_type` _IS NOT_ a child type, then it's probably a concrete type...
        //
        // Hypothetically we could have <T>OuterWrapper(InnerWrapper(T)) where `as_type = InnerWrapper(T)` but `child_types = T`
        // And we should have a constraint on InnerWrapper(T): Categorize. But that's what the `categorize_as` fallback is for.
        //
        // Or the more likely case might be that the two types are the same type, but syn doesn't match them because of formatting
        // issues of different paths or something, there's not much we can do there.
        //
        // If we miss the constraint then the user will get a compiler error and go digging and find `categorize_as`.

        // IMPORTANT:
        // We convert to string here because type equality is too strict and relies on span equality which won't exist
        // in many cases. But by converting to string we have a looser/better equality check.
        let child_type_strs = get_child_types(attributes, existing_generics)?
            .iter()
            .map(|t| quote!(#t).to_string())
            .collect_vec();
        let as_type_str = quote!(#as_type).to_string();
        if child_type_strs.contains(&as_type_str) {
            Ok(Some(as_type.clone()))
        } else {
            Ok(None)
        }
    }
}

pub fn get_code_hash_const_array_token_stream(input: &TokenStream) -> TokenStream {
    let hash = get_hash_of_code(input);
    quote! {
        [#(#hash),*]
    }
}

pub fn get_hash_of_code(input: &TokenStream) -> [u8; 20] {
    const_sha1::sha1(input.to_string().as_bytes()).as_bytes()
}

pub fn get_unique_types<'a>(types: &[syn::Type]) -> Vec<syn::Type> {
    types.iter().unique().cloned().collect()
}

#[derive(Clone)]
pub(crate) struct FieldsData {
    pub unskipped_field_names: Vec<TokenStream>,
    pub unskipped_field_name_strings: Vec<String>,
    pub unskipped_field_types: Vec<Type>,
    pub skipped_field_names: Vec<TokenStream>,
    pub skipped_field_types: Vec<Type>,
    pub fields_unpacking: TokenStream,
    pub empty_fields_unpacking: TokenStream,
    pub unskipped_unpacked_field_names: Vec<TokenStream>,
    pub unskipped_field_count: Index,
}

pub(crate) fn process_fields(fields: &syn::Fields) -> Result<FieldsData> {
    Ok(match fields {
        Fields::Named(fields) => {
            let mut unskipped_field_names = Vec::new();
            let mut unskipped_field_name_strings = Vec::new();
            let mut unskipped_field_types = Vec::new();
            let mut skipped_field_names = Vec::new();
            let mut skipped_field_types = Vec::new();
            for f in fields.named.iter() {
                let ident = &f.ident;
                if !is_skipped(f)? {
                    unskipped_field_names.push(quote! { #ident });
                    unskipped_field_name_strings
                        .push(ident.as_ref().map(|i| i.to_string()).unwrap_or_default());
                    unskipped_field_types.push(f.ty.clone());
                } else {
                    skipped_field_names.push(quote! { #ident });
                    skipped_field_types.push(f.ty.clone());
                }
            }

            let fields_unpacking = quote! {
                {#(#unskipped_field_names,)* ..}
            };
            let empty_fields_unpacking = quote! {
                { .. }
            };
            let unskipped_unpacked_field_names = unskipped_field_names.clone();

            let unskipped_field_count = Index::from(unskipped_field_names.len());

            FieldsData {
                unskipped_field_names,
                unskipped_field_name_strings,
                unskipped_field_types,
                skipped_field_names,
                skipped_field_types,
                fields_unpacking,
                empty_fields_unpacking,
                unskipped_unpacked_field_names,
                unskipped_field_count,
            }
        }
        Fields::Unnamed(fields) => {
            let mut unskipped_indices = Vec::new();
            let mut unskipped_field_name_strings = Vec::new();
            let mut unskipped_field_types = Vec::new();
            let mut unskipped_unpacked_field_names = Vec::new();
            let mut skipped_indices = Vec::new();
            let mut skipped_field_types = Vec::new();
            let mut unpacking_idents = Vec::new();
            let mut empty_idents = Vec::new();
            for (i, f) in fields.unnamed.iter().enumerate() {
                let index = Index::from(i);
                if !is_skipped(f)? {
                    unskipped_indices.push(quote! { #index });
                    unskipped_field_name_strings.push(i.to_string());
                    unskipped_field_types.push(f.ty.clone());
                    let unpacked_name_ident = format_ident!("a{}", i);
                    unskipped_unpacked_field_names.push(quote! { #unpacked_name_ident });
                    unpacking_idents.push(unpacked_name_ident);
                } else {
                    skipped_indices.push(quote! { #index });
                    skipped_field_types.push(f.ty.clone());
                    unpacking_idents.push(format_ident!("_"));
                }
                empty_idents.push(format_ident!("_"));
            }
            let fields_unpacking = quote! {
                (#(#unpacking_idents),*)
            };
            let empty_fields_unpacking = quote! {
                (#(#empty_idents),*)
            };

            let unskipped_field_count = Index::from(unskipped_indices.len());

            FieldsData {
                unskipped_field_names: unskipped_indices,
                unskipped_field_name_strings,
                unskipped_field_types,
                skipped_field_names: skipped_indices,
                skipped_field_types,
                fields_unpacking,
                empty_fields_unpacking,
                unskipped_unpacked_field_names,
                unskipped_field_count,
            }
        }
        Fields::Unit => FieldsData {
            unskipped_field_names: vec![],
            unskipped_field_name_strings: vec![],
            unskipped_field_types: vec![],
            skipped_field_names: vec![],
            skipped_field_types: vec![],
            fields_unpacking: quote! {},
            empty_fields_unpacking: quote! {},
            unskipped_unpacked_field_names: vec![],
            unskipped_field_count: Index::from(0),
        },
    })
}

pub fn add_where_predicate(
    optional_where: Option<&WhereClause>,
    predicate: WherePredicate,
) -> WhereClause {
    let mut where_clause = optional_where.cloned().unwrap_or(WhereClause {
        where_token: Default::default(),
        predicates: Default::default(),
    });
    where_clause.predicates.push(predicate);
    where_clause
}

pub fn build_decode_generics<'a>(
    original_generics: &'a Generics,
    attributes: &'a [Attribute],
    context_custom_value_kind: Option<&'static str>,
) -> syn::Result<(Generics, TypeGenerics<'a>, Option<WhereClause>, Path, Path)> {
    let custom_value_kind = get_custom_value_kind(&attributes)?;
    let (impl_generics, ty_generics, where_clause) = original_generics.split_for_impl();

    // Extract owned generic to allow mutation
    let mut impl_generics: Generics = parse_quote! { #impl_generics };

    let (custom_value_kind_generic, need_to_add_cvk_generic): (Path, bool) =
        if let Some(path) = custom_value_kind {
            (path.parse()?, false)
        } else if let Some(path) = context_custom_value_kind {
            (parse_str(path)?, false)
        } else {
            let custom_type_label = find_free_generic_name(original_generics, "X")?;
            (parse_str(&custom_type_label)?, true)
        };

    let decoder_label = find_free_generic_name(original_generics, "D")?;
    let decoder_generic: Path = parse_str(&decoder_label)?;

    let child_types = get_child_types(&attributes, &impl_generics)?;
    let categorize_types = get_types_requiring_categorize_bound_for_encode_and_decode(&attributes)?;

    let mut where_clause = where_clause.cloned();
    if child_types.len() > 0 || categorize_types.len() > 0 {
        let mut new_where_clause = where_clause.unwrap_or(WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });
        for child_type in child_types {
            new_where_clause
                .predicates
                .push(parse_quote!(#child_type: sbor::Decode<#custom_value_kind_generic, #decoder_generic>));
        }
        for categorize_type in categorize_types {
            new_where_clause
                .predicates
                .push(parse_quote!(#categorize_type: sbor::Categorize<#custom_value_kind_generic>));
        }
        where_clause = Some(new_where_clause);
    }

    impl_generics
        .params
        .push(parse_quote!(#decoder_generic: sbor::Decoder<#custom_value_kind_generic>));

    if need_to_add_cvk_generic {
        impl_generics
            .params
            .push(parse_quote!(#custom_value_kind_generic: sbor::CustomValueKind));
    }

    Ok((
        impl_generics,
        ty_generics,
        where_clause,
        custom_value_kind_generic,
        decoder_generic,
    ))
}

pub fn build_encode_generics<'a>(
    original_generics: &'a Generics,
    attributes: &'a [Attribute],
    context_custom_value_kind: Option<&'static str>,
) -> syn::Result<(Generics, TypeGenerics<'a>, Option<WhereClause>, Path, Path)> {
    let custom_value_kind = get_custom_value_kind(&attributes)?;
    let (impl_generics, ty_generics, where_clause) = original_generics.split_for_impl();

    // Extract owned generic to allow mutation
    let mut impl_generics: Generics = parse_quote! { #impl_generics };

    let (custom_value_kind_generic, need_to_add_cvk_generic): (Path, bool) =
        if let Some(path) = custom_value_kind {
            (path.parse()?, false)
        } else if let Some(path) = context_custom_value_kind {
            (parse_str(path)?, false)
        } else {
            let custom_type_label = find_free_generic_name(original_generics, "X")?;
            (parse_str(&custom_type_label)?, true)
        };

    let encoder_label = find_free_generic_name(original_generics, "E")?;
    let encoder_generic: Path = parse_str(&encoder_label)?;

    let child_types = get_child_types(&attributes, &impl_generics)?;
    let categorize_types = get_types_requiring_categorize_bound_for_encode_and_decode(&attributes)?;

    let mut where_clause = where_clause.cloned();
    if child_types.len() > 0 || categorize_types.len() > 0 {
        let mut new_where_clause = where_clause.unwrap_or(WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });
        for child_type in child_types {
            new_where_clause
                .predicates
                .push(parse_quote!(#child_type: sbor::Encode<#custom_value_kind_generic, #encoder_generic>));
        }
        for categorize_type in categorize_types {
            new_where_clause
                .predicates
                .push(parse_quote!(#categorize_type: sbor::Categorize<#custom_value_kind_generic>));
        }
        where_clause = Some(new_where_clause);
    }

    impl_generics
        .params
        .push(parse_quote!(#encoder_generic: sbor::Encoder<#custom_value_kind_generic>));

    if need_to_add_cvk_generic {
        impl_generics
            .params
            .push(parse_quote!(#custom_value_kind_generic: sbor::CustomValueKind));
    }

    Ok((
        impl_generics,
        ty_generics,
        where_clause,
        custom_value_kind_generic,
        encoder_generic,
    ))
}

pub fn build_describe_generics<'a>(
    original_generics: &'a Generics,
    attributes: &'a [Attribute],
    context_custom_type_kind: Option<&'static str>,
) -> syn::Result<(Generics, Generics, Option<WhereClause>, Vec<Type>, Path)> {
    let custom_type_kind = get_custom_type_kind(attributes)?;

    let (impl_generics, ty_generics, where_clause) = original_generics.split_for_impl();

    // Extract owned generic to allow mutation
    let mut impl_generics: Generics = parse_quote! { #impl_generics };

    let (custom_type_kind_generic, need_to_add_ctk_generic): (Path, bool) =
        if let Some(path) = custom_type_kind {
            (path.parse()?, false)
        } else if let Some(path) = context_custom_type_kind {
            (parse_str(&path)?, false)
        } else {
            let custom_type_label = find_free_generic_name(original_generics, "C")?;
            (parse_str(&custom_type_label)?, true)
        };

    let child_types = get_child_types(&attributes, &impl_generics)?;

    let mut where_clause = where_clause.cloned();
    if child_types.len() > 0 {
        let mut new_where_clause = where_clause.unwrap_or(WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        });
        for child_type in child_types.iter() {
            new_where_clause
                .predicates
                .push(parse_quote!(#child_type: sbor::Describe<#custom_type_kind_generic>));
        }
        where_clause = Some(new_where_clause);
    }

    if need_to_add_ctk_generic {
        impl_generics
            .params
            .push(parse_quote!(#custom_type_kind_generic: sbor::CustomTypeKind<sbor::RustTypeId>));
    }

    let ty_generics: Generics = parse_quote! { #ty_generics };

    Ok((
        impl_generics,
        ty_generics,
        where_clause,
        child_types,
        custom_type_kind_generic,
    ))
}

pub fn build_categorize_generics<'a>(
    original_generics: &'a Generics,
    attributes: &'a [Attribute],
    context_custom_value_kind: Option<&'static str>,
) -> syn::Result<(Generics, TypeGenerics<'a>, Option<&'a WhereClause>, Path)> {
    let custom_value_kind = get_custom_value_kind(&attributes)?;
    let (impl_generics, ty_generics, where_clause) = original_generics.split_for_impl();

    // Unwrap for mutation
    let mut impl_generics: Generics = parse_quote! { #impl_generics };

    let (custom_value_kind_generic, need_to_add_cvk_generic): (Path, bool) =
        if let Some(path) = custom_value_kind {
            (path.parse()?, false)
        } else if let Some(path) = context_custom_value_kind {
            (parse_str(path)?, false)
        } else {
            let custom_type_label = find_free_generic_name(original_generics, "X")?;
            (parse_str(&custom_type_label)?, true)
        };

    if need_to_add_cvk_generic {
        impl_generics
            .params
            .push(parse_quote!(#custom_value_kind_generic: sbor::CustomValueKind));
    }

    Ok((
        impl_generics,
        ty_generics,
        where_clause,
        custom_value_kind_generic,
    ))
}

fn find_free_generic_name(generics: &Generics, name_prefix: &str) -> syn::Result<String> {
    if !generic_already_exists(generics, name_prefix) {
        return Ok(name_prefix.to_owned());
    }
    for i in 0..100 {
        let name_attempt = format!("{}{}", name_prefix, i);
        if !generic_already_exists(generics, &name_attempt) {
            return Ok(name_attempt);
        }
    }

    return Err(Error::new(
        Span::call_site(),
        format!("Cannot find free generic name with prefix {}!", name_prefix),
    ));
}

fn generic_already_exists(generics: &Generics, name: &str) -> bool {
    generics
        .params
        .iter()
        .any(|p| &get_generic_param_name(p) == name)
}

fn get_generic_param_name(generic_param: &GenericParam) -> String {
    match generic_param {
        GenericParam::Type(type_param) => type_param.ident.to_string(),
        GenericParam::Lifetime(lifetime_param) => lifetime_param.lifetime.to_string(),
        GenericParam::Const(const_param) => const_param.ident.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_attributes() {
        let attr: Attribute = parse_quote! {
            #[sbor(skip, custom_value_kind = "NoCustomValueKind")]
        };
        let attr2: Attribute = parse_quote! {
            #[sbor(skip3)]
        };
        let extracted = extract_typed_attributes(&[attr, attr2], "sbor").unwrap();
        assert_eq!(extracted.get_bool_value("skip").unwrap().value(), true);
        assert_eq!(extracted.get_bool_value("skip2").unwrap().value(), false);
        assert_eq!(extracted.get_bool_value("skip3").unwrap().value(), true);
        assert!(matches!(
            extracted.get_bool_value("custom_value_kind"),
            Err(_)
        ));
        assert_eq!(
            extracted
                .get_string_value("custom_value_kind")
                .unwrap()
                .unwrap()
                .value(),
            "NoCustomValueKind".to_string()
        );
        assert_eq!(
            extracted.get_string_value("custom_value_kind_2").unwrap(),
            None
        );
        assert!(matches!(extracted.get_string_value("skip"), Err(_)));
    }
}
