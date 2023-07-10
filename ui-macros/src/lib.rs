use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DataStruct, DeriveInput, Data, FieldsNamed, Fields, Meta, Expr};

#[proc_macro_attribute]
pub fn element(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let attrs = ast.attrs.iter().map(|attr| {
        let ident = attr.path().get_ident();
        let args: Expr = attr.parse_args().unwrap();
        quote! {
            #[#ident(#args)]
        }
    }).collect::<Vec<_>>();

    let fields = match ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) => named,
        _ => unimplemented!("UiElement must be a struct")
    };

    let mut set_fields = Vec::new();

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        if let Some(_) = f.attrs.iter().find(
            |attr| if let Meta::Path(path) = &attr.meta {
                if let Some(ident) = path.get_ident() {
                    ident.to_string() == String::from("set")
                } else { false }
            } else { false }
        ) {
            let f_name = format_ident!("set_{}", name.as_ref().expect("Field must have a ident to be set"));
            set_fields.push(quote! {
                pub fn #f_name(mut self, v: #ty) -> Self {
                    self.#name = v;
                    self
                }
            })
        }
        quote! { #vis #name: #ty }
    });

    quote! {
        #(#attrs)*
        pub struct #name {
            pub x: i16,
            pub y: i16,
            pub width: i16,
            pub height: i16,
            pub children: Vec<std::rc::Rc<std::cell::RefCell<dyn ui::Element>>>,
            #(#builder_fields),*
        }
        impl #name {
            pub fn set_x(mut self, v: i16) -> Self {
                self.x = v;
                self
            }
            pub fn set_y(mut self, v: i16) -> Self {
                self.y = v;
                self
            }
            pub fn set_width(mut self, v: i16) -> Self {
                self.width = v;
                self
            }
            pub fn set_height(mut self, v: i16) -> Self {
                self.height = v;
                self
            }
            #(#set_fields)*
        }
    }.into()
}