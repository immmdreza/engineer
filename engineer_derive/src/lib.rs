extern crate proc_macro;
extern crate proc_macro2;

use darling::{util::Flag, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, GenericArgument, Path, PathArguments, Type};

#[derive(Debug, Clone, FromMeta)]
struct Retype {
    to: String,
    #[darling(rename = "re")]
    restore: String,
}

impl Retype {
    fn new(to: &str, restore: &str) -> Self {
        Self {
            to: to.to_string(),
            restore: restore.to_string(),
        }
    }
}

#[derive(Debug, Clone, FromMeta)]
struct GlobRetype {
    from: String,
    to: String,
    #[darling(rename = "re")]
    restore: String,
}

#[derive(FromField, Clone, Debug)]
#[darling(attributes(engineer), forward_attrs(allow, doc, cfg))]
struct EngineerField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    default_value: Option<String>,
    retype: Option<Retype>,

    default: Flag,
    /// Shorthand for `retype(to = "impl Into<String>", re = ".into()")`,
    str_retype: Flag,
}

impl EngineerField {
    fn apply_shorthands(&mut self) {
        if self.str_retype.is_present() {
            self.retype = Some(Retype::new("impl Into<String>", ".into()"))
        }

        if self.default.is_present() {
            self.default_value = Some("Default::default()".to_string())
        }
    }

    fn is_option(&self) -> bool {
        type_is_option(&self.ty)
    }

    fn is_retyped(&self) -> bool {
        self.retype.is_some()
    }

    fn retyped(&self) -> proc_macro2::TokenStream {
        match &self.retype {
            Some(retype) => retype.to.parse().unwrap(),
            None => {
                let ty = if self.is_option() {
                    extract_type_from_option(&self.ty).unwrap()
                } else {
                    &self.ty
                };

                quote!(#ty)
            }
        }
    }

    fn restorer(&self) -> proc_macro2::TokenStream {
        match &self.retype {
            Some(retype) => retype.restore.parse().unwrap(),
            None => Default::default(),
        }
    }

    fn as_struct_field(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;
        let ty = &self.ty;

        quote!(#name: #ty,)
    }

    fn as_struct_setter(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;
        let mut name_ts = quote!(#name);

        if self.is_retyped() {
            let restore = self.restorer();
            quote!( : #name #restore).to_tokens(&mut name_ts)
        }

        name_ts
    }

    fn as_func_argument(&self) -> proc_macro2::TokenStream {
        let name = &self.ident;
        let ty = self.retyped();

        quote!(#name: #ty,)
    }
}

#[derive(FromDeriveInput, Clone, Debug)]
#[darling(
    attributes(engineer),
    supports(struct_named),
    forward_attrs(allow, doc, cfg)
)]
struct EngineerOptions {
    ident: syn::Ident,
    vis: syn::Visibility,
    data: darling::ast::Data<darling::util::Ignored, EngineerField>,

    engineer_name: Option<String>,
    builder_func: Option<String>,

    #[darling(multiple, rename = "retype")]
    retypes: Vec<GlobRetype>,
    str_retype: Flag,
    new: Flag,

    #[darling(skip)]
    fields_ref: Option<Vec<EngineerField>>,
}

impl EngineerOptions {
    fn from_derive_input_delegate(input: &DeriveInput) -> Result<EngineerOptions, darling::Error> {
        let mut s = Self::from_derive_input(input)?
            .apply_self_shorthands()
            .apply_global_retypes()
            .apply_fields_shorthands();

        s.set_fields_ref();

        Ok(s)
    }

    fn apply_self_shorthands(mut self) -> Self {
        if self.str_retype.is_present() {
            self.retypes.push(GlobRetype {
                from: "String".to_string(),
                to: "impl Into<String>".to_string(),
                restore: ".into()".to_string(),
            })
        }

        if self.new.is_present() {
            self.builder_func = Some("new".to_string());
        }

        self
    }

    fn apply_global_retypes(mut self) -> Self {
        self.data = self.data.map_struct_fields(|mut f| {
            let ty_str = if f.is_option() {
                extract_type_from_option(&f.ty).unwrap()
            } else {
                &f.ty
            }
            .to_token_stream()
            .to_string();

            for r in &self.retypes {
                if ty_str == r.from {
                    f.retype = Some(Retype {
                        to: r.to.clone(),
                        restore: r.restore.clone(),
                    });
                }
            }

            f
        });

        self
    }

    fn apply_fields_shorthands(mut self) -> EngineerOptions {
        self.data = self.data.map_struct_fields(|mut f| {
            f.apply_shorthands();
            f
        });

        self
    }

    fn set_fields_ref(&mut self) {
        self.fields_ref = Some(self.data.clone().take_struct().unwrap().fields);
    }

    fn engineer_name(&self) -> syn::Ident {
        format_ident!(
            "{}",
            self.engineer_name
                .clone()
                .unwrap_or(format!("{}Engineer", self.ident))
        )
    }

    fn builder_name(&self) -> syn::Ident {
        format_ident!(
            "{}",
            self.builder_func
                .clone()
                .unwrap_or_else(|| "engineer".to_string())
        )
    }

    fn fields(&self) -> Vec<EngineerField> {
        self.data.clone().take_struct().unwrap().fields
    }

    fn fields_ref(&self) -> &Vec<EngineerField> {
        self.fields_ref.as_ref().unwrap()
    }
}

trait FieldsHelpers<'e>: Iterator<Item = &'e EngineerField> + Sized {
    fn filter_normals(self) -> Vec<&'e EngineerField> {
        self.filter(|f| !f.is_option()).collect()
    }

    fn filter_options(self) -> Vec<&'e EngineerField> {
        self.filter(|f| f.is_option()).collect()
    }

    fn map_names(self) -> Vec<&'e Option<syn::Ident>> {
        self.map(|f| &f.ident).collect()
    }

    fn map_types(self) -> Vec<&'e Type> {
        self.map(|f| &f.ty).collect()
    }
}

impl<'e, T> FieldsHelpers<'e> for T where T: Iterator<Item = &'e EngineerField> {}

struct EngineerStructDefinition<'e>(&'e EngineerOptions);

impl<'e> EngineerStructDefinition<'e> {
    fn name(&self) -> proc_macro2::Ident {
        self.0.engineer_name()
    }

    fn struct_definition(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.0.ident;
        let vis = &self.0.vis;
        let engineer_name = &self.0.engineer_name();

        let fields = self.0.fields_ref();
        let struct_fields = fields.iter().map(|f| f.as_struct_field());

        quote! {
            #vis struct #engineer_name {
                #(
                    #struct_fields
                )*
            }

            impl Builder<#struct_name> for #engineer_name {
                fn done(self) -> #struct_name {
                    self.__done()
                }
            }

            impl From<#engineer_name> for #struct_name
            {
                fn from(value: #engineer_name) -> Self {
                    value.done()
                }
            }
        }
    }

    fn new_func(&self) -> proc_macro2::TokenStream {
        let engineer_name = self.name();
        let vis = &self.0.vis;

        let fields = self.0.fields_ref();
        let nrm_fields = fields.iter().filter_normals();

        let opt_names = fields.iter().filter(|f| f.is_option()).map(|f| &f.ident);
        let opt_values = fields
            .iter()
            .filter(|f| f.is_option())
            .map(|f| match &f.default_value {
                Some(sec) => {
                    let t = sec.parse::<proc_macro2::TokenStream>().unwrap();
                    quote!(Some(#t))
                }
                _ => quote!(None),
            });

        let func_args = nrm_fields.iter().map(|f| f.as_func_argument());
        let struct_setters = nrm_fields.iter().map(|f| f.as_struct_setter());

        quote! {
            #vis fn new(#(#func_args)*) -> Self {
                #engineer_name {
                    #(
                        #struct_setters,
                    )*

                    #(
                        #opt_names: #opt_values,
                    )*
                }
            }
        }
    }

    fn opt_setters(&self) -> proc_macro2::TokenStream {
        let vis = &self.0.vis;

        let fields = self.0.fields();
        let opt_fields = fields.iter().filter(|f| f.is_option());
        let opt_names = opt_fields.clone().map(|f| &f.ident);

        let opt_types = opt_fields.clone().map(|f| f.retyped());
        let opt_restores = opt_fields.map(|f| f.restorer());

        quote! {
            #(
                #vis fn #opt_names(mut self, #opt_names: #opt_types) -> Self {
                    self.#opt_names = Some(#opt_names #opt_restores);
                    self
                }
            )*
        }
    }

    fn done_func(&self) -> proc_macro2::TokenStream {
        let name = &self.0.ident;

        let fields = self.0.fields();
        let names = fields.iter().map(|f| &f.ident);

        quote! {
            fn __done(self) -> #name {
                #name {
                    #(
                        #names: self.#names,
                    )*
                }
            }
        }
    }

    fn struct_impl(&self) -> proc_macro2::TokenStream {
        let engineer_name = &self.name();
        let new_func = self.new_func();
        let opt_setters = self.opt_setters();
        let done_func = self.done_func();

        quote! {
            impl #engineer_name {
                #new_func
                #opt_setters
                #done_func
            }
        }
    }
}

impl<'e> quote::ToTokens for EngineerStructDefinition<'e> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_definition = self.struct_definition();
        let struct_impl = self.struct_impl();

        quote! {
            #struct_definition
            #struct_impl
        }
        .to_tokens(tokens)
    }
}

struct StructImpl<'e>(&'e EngineerOptions);

impl<'e> StructImpl<'e> {
    fn builder_func(&self) -> proc_macro2::TokenStream {
        let engineer_name = self.0.engineer_name();
        let builder_name = self.0.builder_name();
        let vis = &self.0.vis;

        let fields = self.0.fields();
        let nrm_fields = fields.iter().filter(|f| !f.is_option());
        let nrm_names = nrm_fields.clone().map(|f| &f.ident);

        let func_args = nrm_fields.clone().map(|f| f.as_func_argument());

        quote! {
            #vis fn #builder_name(#(#func_args)*) -> #engineer_name {
                <#engineer_name>::new(#(#nrm_names,)*)
            }
        }
    }
}

impl<'e> quote::ToTokens for StructImpl<'e> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.0.ident;
        let builder_func = self.builder_func();

        quote! { impl #name { #builder_func } }.to_tokens(tokens)
    }
}

struct TraitImpl<'e>(&'e EngineerOptions);

impl<'e> quote::ToTokens for TraitImpl<'e> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.0.ident;
        let engineer_name = &self.0.engineer_name();

        let fields = self.0.fields();
        let nrm_fields = fields.iter().filter(|f| !f.is_option());

        let nrm_count = nrm_fields.clone().count();
        let opt_count = fields.len() - nrm_count;

        let nrm_fields_types = nrm_fields.map(|f| &f.ty);
        let nrm_fields_types_1 = nrm_fields_types.clone();

        let members = (0..nrm_count).map(|f| {
            format!("required.{}", f)
                .parse::<proc_macro2::TokenStream>()
                .unwrap()
        });

        quote! {
            impl Engineer<#engineer_name, (#(#nrm_fields_types,)*)> for #name {
                const NORMAL_FIELDS: usize = #nrm_count;
                const OPTIONAL_FIELDS: usize = #opt_count;

                fn get_builder(required: (#(#nrm_fields_types_1,)*)) -> #engineer_name {
                    #engineer_name::new(#(#members,)*)
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[proc_macro_derive(Engineer, attributes(engineer))]
pub fn engineer(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let engineer_opts = EngineerOptions::from_derive_input_delegate(&input).unwrap();

    let engineer_struct_definition = EngineerStructDefinition(&engineer_opts);
    let struct_impl = StructImpl(&engineer_opts);
    let trait_impl = TraitImpl(&engineer_opts);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #engineer_struct_definition
        #struct_impl
        #trait_impl
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn type_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(path_type) => {
            path_type.path.leading_colon.is_none()
                && path_type.path.segments.len() == 1
                && path_type.path.segments.iter().next().unwrap().ident == "Option"
        }
        _ => false,
    }
}

fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}

fn extract_type_from_option(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            // Get the first segment of the path (there is only one, in fact: "Option"):
            let type_params = &typepath.path.segments.iter().next().unwrap().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => Some(params.args.iter().next().unwrap()),
                _ => None,
            }?;
            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            }
        }
        _ => None,
    }
}
