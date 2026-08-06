//! # lit-ui-macros — declarative derives that eliminate DSL boilerplate.
//!
//! Three derives; each has one job.
//!
//! ## `#[derive(AttrEnum)]`
//! Generates `pub fn as_str(self) -> &'static str` from `#[attr("wire")]`
//! on each variant. Optional `#[attr_enum(default)]` on a variant marks it
//! as `Default::default()`.
//!
//! ```ignore
//! #[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum Variant {
//!     #[attr("primary")]   #[attr_enum(default)] Primary,
//!     #[attr("secondary")] Secondary,
//! }
//! ```
//!
//! ## `#[derive(UiComponent)]`
//! Given a struct annotated with `#[ui(tag = "ui-button")]`, generates:
//! * a `Default` impl (respecting per-field `default = "..."`),
//! * a free constructor function (name = struct name in snake_case),
//! * one setter per field (returning `Self` for chaining),
//! * `impl Component` that produces the exact `<tag ...>body</tag>` string.
//!
//! Per-field attributes (all under `#[ui(...)]`):
//! * `attr = "wire-name"`  — render as key/value HTML attribute
//! * `flag = "wire-name"`  — bool field → HTML boolean attribute
//! * `slot`                — string field → escaped default-slot text
//! * `children`            — `Vec<Child>` → rendered children body; also
//!   auto-derives `.add()` / `.children()` chain methods
//! * `default = "..."`     — expression used in Default and constructor
//! * `skip`                — internal field, not rendered, no setter
//! * `enum_attr = "wire"`  — like `attr` but calls `.as_str()` on the value
//!   (use with any type implementing `as_str(self) -> &'static str`, e.g.
//!   an `AttrEnum` enum)
//!
//! Optional-typed fields (`Option<T>`) are automatically omitted when `None`.
//!
//! This trio collapses ~120 lines of ceremony per primitive into ~20.

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, Fields, GenericArgument, Ident, Lit,
    Meta, PathArguments, Type,
};

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// Convert a `PascalCase` identifier into `snake_case` for the generated
/// free constructor function (e.g. `Button` → `button`).
fn snake_case(pascal: &str) -> String {
    let mut out = String::with_capacity(pascal.len() + 4);
    for (i, ch) in pascal.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 { out.push('_'); }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// If `ty` is `Option<T>` return `Some(T)`, else `None`.
fn option_inner(ty: &Type) -> Option<Type> {
    let Type::Path(p) = ty else { return None; };
    let last = p.path.segments.last()?;
    if last.ident != "Option" { return None; }
    let PathArguments::AngleBracketed(a) = &last.arguments else { return None; };
    for arg in &a.args {
        if let GenericArgument::Type(t) = arg { return Some(t.clone()); }
    }
    None
}

/// True if the type prints as exactly `"bool"`.
fn is_bool(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.is_ident("bool"))
}

/// True if the type prints as `String` (used to decide slot-escaping).
fn is_string(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.is_ident("String"))
}

/// Extract the single `"literal"` inside an attribute like `#[attr("primary")]`.
fn parse_str_arg(attr: &Attribute) -> Option<String> {
    match &attr.meta {
        Meta::List(list) => {
            let mut out = None;
            let _ = list.parse_nested_meta(|meta| {
                if let Ok(Lit::Str(s)) = meta.value().and_then(|v| v.parse::<Lit>()) {
                    out = Some(s.value());
                }
                Ok(())
            });
            // Fallback: `#[attr("literal")]` — one direct string literal.
            if out.is_none() {
                if let Ok(lit) = list.parse_args::<syn::LitStr>() {
                    out = Some(lit.value());
                }
            }
            out
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// #[derive(AttrEnum)]
// ---------------------------------------------------------------------------

#[proc_macro_derive(AttrEnum, attributes(attr, attr_enum))]
pub fn derive_attr_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(en) = &input.data else {
        return syn::Error::new_spanned(name, "AttrEnum can only be derived on enums")
            .to_compile_error()
            .into();
    };

    let mut arms = Vec::new();
    let mut default_variant: Option<Ident> = None;

    for v in &en.variants {
        let ident = &v.ident;
        // The wire string.
        let wire = v
            .attrs
            .iter()
            .find(|a| a.path().is_ident("attr"))
            .and_then(parse_str_arg)
            .unwrap_or_else(|| snake_case(&ident.to_string()));

        arms.push(quote! { Self::#ident => #wire });

        // #[attr_enum(default)]
        for a in &v.attrs {
            if a.path().is_ident("attr_enum") {
                let _ = a.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") { default_variant = Some(ident.clone()); }
                    Ok(())
                });
            }
        }
    }

    let default_impl = default_variant.map(|d| {
        quote! {
            impl ::core::default::Default for #name {
                fn default() -> Self { Self::#d }
            }
        }
    });

    let expanded = quote! {
        impl #name {
            /// The HTML attribute string this variant serializes to.
            pub fn as_str(self) -> &'static str {
                match self { #(#arms),* }
            }
        }
        #default_impl
    };

    expanded.into()
}

// ---------------------------------------------------------------------------
// #[derive(UiComponent)]
// ---------------------------------------------------------------------------

/// Per-field configuration parsed out of `#[ui(...)]` on a struct field.
#[derive(Default)]
struct FieldCfg {
    /// Render as a key/value HTML attribute.
    attr: Option<String>,
    /// Same as `attr`, but the value is `.as_str()`-ed first.
    enum_attr: Option<String>,
    /// Render as a boolean HTML attribute (only when the field is `true`).
    flag: Option<String>,
    /// Escape and inject into the element body (before children).
    slot: bool,
    /// Vec<Child> body; auto-derives `.add()` and `.children()`.
    children: bool,
    /// Do not render, do not generate a setter.
    skip: bool,
    /// Expression used in Default / constructor.
    default: Option<Expr>,
    /// Skip rendering when this expression is true. The field is bound as
    /// `self.<name>` inside the expression, so callers write
    /// `skip_if = "self.tone == Tone::Neutral"`.
    skip_if: Option<Expr>,
}

fn parse_field_cfg(attrs: &[Attribute]) -> FieldCfg {
    let mut cfg = FieldCfg::default();
    for a in attrs {
        if !a.path().is_ident("ui") { continue; }
        let _ = a.parse_nested_meta(|meta| {
            let key = meta.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            match key.as_str() {
                "attr"      => { if let Ok(v) = meta.value()?.parse::<syn::LitStr>() { cfg.attr = Some(v.value()); } }
                "enum_attr" => { if let Ok(v) = meta.value()?.parse::<syn::LitStr>() { cfg.enum_attr = Some(v.value()); } }
                "flag"      => { if let Ok(v) = meta.value()?.parse::<syn::LitStr>() { cfg.flag = Some(v.value()); } }
                "slot"      => cfg.slot     = true,
                "children"  => cfg.children = true,
                "skip"      => cfg.skip     = true,
                "default"   => {
                    // Accept `default = "expr"` — the string is parsed as an
                    // expression so callers can write `default = "Variant::Primary"`.
                    if let Ok(v) = meta.value()?.parse::<syn::LitStr>() {
                        if let Ok(expr) = syn::parse_str::<Expr>(&v.value()) { cfg.default = Some(expr); }
                    }
                }
                "skip_if"   => {
                    if let Ok(v) = meta.value()?.parse::<syn::LitStr>() {
                        if let Ok(expr) = syn::parse_str::<Expr>(&v.value()) { cfg.skip_if = Some(expr); }
                    }
                }
                _ => {}
            }
            Ok(())
        });
    }
    cfg
}

/// Struct-level `#[ui(...)]` options.
#[derive(Default)]
struct StructCfg {
    /// The HTML tag name to render (`ui-button`, `ui-badge`, ...).
    tag: Option<String>,
    /// If set, skip generating the free `pub fn <name>() -> Self`
    /// constructor. Use when the module wants a custom constructor with a
    /// different signature (e.g. `badge(label)` takes the label directly).
    no_ctor: bool,
}

fn parse_struct_cfg(attrs: &[Attribute]) -> StructCfg {
    let mut cfg = StructCfg::default();
    for a in attrs {
        if !a.path().is_ident("ui") { continue; }
        let _ = a.parse_nested_meta(|meta| {
            let key = meta.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            match key.as_str() {
                "tag" => {
                    if let Ok(v) = meta.value()?.parse::<syn::LitStr>() { cfg.tag = Some(v.value()); }
                }
                "no_ctor" => cfg.no_ctor = true,
                _ => {}
            }
            Ok(())
        });
    }
    cfg
}

#[proc_macro_derive(UiComponent, attributes(ui))]
pub fn derive_ui_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let ctor = format_ident!("{}", snake_case(&name.to_string()));

    let struct_cfg = parse_struct_cfg(&input.attrs);
    let tag = match struct_cfg.tag.clone() {
        Some(t) => t,
        None => {
            return syn::Error::new_spanned(name, r#"missing #[ui(tag = "…")] on struct"#)
                .to_compile_error()
                .into();
        }
    };

    let Data::Struct(strct) = &input.data else {
        return syn::Error::new_spanned(name, "UiComponent can only be derived on structs")
            .to_compile_error()
            .into();
    };

    let Fields::Named(named) = &strct.fields else {
        return syn::Error::new_spanned(name, "UiComponent requires named fields")
            .to_compile_error()
            .into();
    };

    // Per-field pieces.
    let mut default_inits = Vec::new();
    let mut setters       = Vec::new();
    let mut render_attrs  = Vec::new();  // pushes into `attrs`
    let mut render_body   = Vec::new();  // pushes into `body`
    let mut container_impl = None;

    for field in &named.named {
        let fname = field.ident.as_ref().unwrap();
        let fty   = &field.ty;
        let cfg   = parse_field_cfg(&field.attrs);

        // --- Default init -----------------------------------------------
        let init_expr = if let Some(expr) = &cfg.default {
            quote! { #expr }
        } else if let Some(_inner) = option_inner(fty) {
            quote! { ::core::option::Option::None }
        } else if is_bool(fty) {
            quote! { false }
        } else {
            quote! { ::core::default::Default::default() }
        };
        default_inits.push(quote! { #fname: #init_expr });

        if cfg.skip { continue; }

        // --- Setter ------------------------------------------------------
        // For string / Option<String> we accept `impl Into<String>` for
        // ergonomics. For everything else we take the type by value.
        if !cfg.children {
            let setter = if is_string(fty) {
                quote! {
                    pub fn #fname(mut self, v: impl ::core::convert::Into<::std::string::String>) -> Self {
                        self.#fname = v.into(); self
                    }
                }
            } else if let Some(inner) = option_inner(fty) {
                if is_string(&inner) {
                    quote! {
                        pub fn #fname(mut self, v: impl ::core::convert::Into<::std::string::String>) -> Self {
                            self.#fname = ::core::option::Option::Some(v.into()); self
                        }
                    }
                } else {
                    quote! {
                        pub fn #fname(mut self, v: #inner) -> Self {
                            self.#fname = ::core::option::Option::Some(v); self
                        }
                    }
                }
            } else if is_bool(fty) {
                // Flag setters read most naturally as `.required()` (no arg,
                // sets to true). Provide `.set_<name>(bool)` if the caller
                // needs to force a value.
                let setter_name = fname;
                let set_ident   = format_ident!("set_{}", fname);
                quote! {
                    pub fn #setter_name(mut self) -> Self { self.#setter_name = true; self }
                    pub fn #set_ident(mut self, v: bool) -> Self { self.#fname = v; self }
                }
            } else {
                quote! {
                    pub fn #fname(mut self, v: #fty) -> Self { self.#fname = v; self }
                }
            };
            setters.push(setter);
        }

        // --- Render pieces -----------------------------------------------
        // Wrap generated pushes in a `skip_if` guard when configured.
        // Callers write `skip_if = "self.tone == Tone::Neutral"` to match
        // the pre-macro behavior of "don't emit the default value".
        let guard = |body: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
            if let Some(expr) = &cfg.skip_if {
                quote! { if !(#expr) { #body } }
            } else {
                body
            }
        };

        if let Some(wire) = &cfg.attr {
            let inner_push = if let Some(inner) = option_inner(fty) {
                if is_string(&inner) {
                    quote! {
                        if let ::core::option::Option::Some(ref v) = self.#fname {
                            attrs.push(::lit_ui::core::Attr::kv(#wire, v.as_str()));
                        }
                    }
                } else {
                    quote! {
                        if let ::core::option::Option::Some(ref v) = self.#fname {
                            attrs.push(::lit_ui::core::Attr::kv(#wire, v.to_string()));
                        }
                    }
                }
            } else if is_string(fty) {
                // Non-optional String — omit if empty (keeps output clean).
                quote! {
                    if !self.#fname.is_empty() {
                        attrs.push(::lit_ui::core::Attr::kv(#wire, self.#fname.as_str()));
                    }
                }
            } else {
                quote! {
                    attrs.push(::lit_ui::core::Attr::kv(#wire, self.#fname.to_string()));
                }
            };
            render_attrs.push(guard(inner_push));
        }

        if let Some(wire) = &cfg.enum_attr {
            let inner_push = if option_inner(fty).is_some() {
                quote! {
                    if let ::core::option::Option::Some(ref v) = self.#fname {
                        attrs.push(::lit_ui::core::Attr::kv(#wire, v.as_str()));
                    }
                }
            } else {
                quote! {
                    attrs.push(::lit_ui::core::Attr::kv(#wire, self.#fname.as_str()));
                }
            };
            render_attrs.push(guard(inner_push));
        }

        if let Some(wire) = &cfg.flag {
            render_attrs.push(guard(quote! {
                if self.#fname { attrs.push(::lit_ui::core::Attr::flag(#wire)); }
            }));
        }

        if cfg.slot {
            if let Some(inner) = option_inner(fty) {
                if is_string(&inner) {
                    render_body.push(quote! {
                        if let ::core::option::Option::Some(ref s) = self.#fname {
                            if !s.is_empty() { body.push_str(&::lit_ui::core::escape_html(s)); }
                        }
                    });
                }
            } else if is_string(fty) {
                render_body.push(quote! {
                    if !self.#fname.is_empty() {
                        body.push_str(&::lit_ui::core::escape_html(&self.#fname));
                    }
                });
            }
        }

        if cfg.children {
            // Children body — rendered after any slot text.
            render_body.push(quote! {
                for c in &self.#fname { body.push_str(&c.render()); }
            });
            // Also auto-implement .add() / .children() on the struct.
            container_impl = Some(quote! {
                impl #name {
                    /// Push a single child into this container.
                    pub fn add(mut self, child: impl ::lit_ui::core::Component + 'static) -> Self {
                        self.#fname.push(::std::boxed::Box::new(child)); self
                    }
                    /// Push many children in one call.
                    pub fn children<I, C>(mut self, iter: I) -> Self
                    where
                        I: ::core::iter::IntoIterator<Item = C>,
                        C: ::lit_ui::core::Component + 'static,
                    {
                        for c in iter { self.#fname.push(::std::boxed::Box::new(c)); }
                        self
                    }
                }
            });
        }
    }

    let container_tokens = container_impl.into_iter().collect::<proc_macro2::TokenStream>();

    // Free-function constructor, unless the struct opts out via
    // `#[ui(no_ctor)]` (e.g. `badge(label)` wants a custom signature).
    let ctor_tokens: proc_macro2::TokenStream = if struct_cfg.no_ctor {
        proc_macro2::TokenStream::new()
    } else {
        quote! {
            /// Free-function constructor. Equivalent to `Self::default()`
            /// but reads better in a chain:
            /// `button().label("Save").variant(Variant::Primary)`.
            pub fn #ctor() -> #name { <#name as ::core::default::Default>::default() }
        }
    };

    let expanded = quote! {
        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self { #(#default_inits),* }
            }
        }

        #ctor_tokens

        impl #name {
            #(#setters)*
        }

        #container_tokens

        impl ::lit_ui::core::Component for #name {
            fn render(&self) -> ::std::string::String {
                let mut attrs: ::std::vec::Vec<::lit_ui::core::Attr> = ::std::vec::Vec::new();
                #(#render_attrs)*
                let mut body = ::std::string::String::new();
                #(#render_body)*
                ::lit_ui::core::wrap(#tag, &attrs, &body)
            }
        }
    };

    expanded.into()
}
