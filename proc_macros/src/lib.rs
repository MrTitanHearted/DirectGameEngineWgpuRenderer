extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

const FORMATS: &[&str] = &[
    "Uint8x2",
    "Uint8x4",
    "Sint8x2",
    "Sint8x4",
    "Unorm8x2",
    "Unorm8x4",
    "Snorm8x2",
    "Snorm8x4",
    "Uint16x2",
    "Uint16x4",
    "Sint16x2",
    "Sint16x4",
    "Unorm16x2",
    "Unorm16x4",
    "Snorm16x2",
    "Snorm16x4",
    "Float16x2",
    "Float16x4",
    "Float32",
    "Float32x2",
    "Float32x3",
    "Float32x4",
    "Uint32",
    "Uint32x2",
    "Uint32x3",
    "Uint32x4",
    "Sint32",
    "Sint32x2",
    "Sint32x3",
    "Sint32x4",
    "Float64",
    "Float64x2",
    "Float64x3",
    "Float64x4",
];

#[proc_macro_derive(VertexBufferLayout, attributes())]
pub fn vertex_buffer_layout(input: TokenStream) -> TokenStream {
    generate_impl(&syn::parse_str(&input.to_string()).unwrap())
}

fn generate_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let default = syn::WhereClause {
        where_token: syn::token::Where::default(),
        predicates: syn::punctuated::Punctuated::default(),
    };
    let where_clause = ast
        .generics
        .where_clause
        .as_ref()
        .unwrap_or(&default);
    let fields = match ast.data {
        syn::Data::Struct(ref data) => data.fields.iter().map(|a| a).collect::<Vec<&syn::Field>>(),
        _ => panic!("#[derive(VertexBufferLayout)] is only defined for structs"),
    };

    let vertex_attributes = fields.into_iter().map(|field| {
        let field_name = match field.ident {
            Some(ref ident) => format!("{ident}"),
            None => String::new(),
        };

        let shader_location = field
            .attrs
            .iter()
            .filter(|attr| attr.tokens.to_string().contains("location"))
            .next()
            .unwrap_or_else(|| panic!("Field {field_name} is missing #[location = ?] attribute"))
            .tokens.to_string().parse::<u32>().unwrap();
        

        let format: String = field
        .attrs
        .iter()
        .filter(|attr| FORMATS.contains(&attr.tokens.to_string().as_str()))
        .next()
        .unwrap_or_else(|| panic!("Field {field_name} is missing its type attribute"))
        .tokens.to_string().parse().unwrap();                
        
        let field_type = &field.ty;

        quote!(
            {

                let attrib = wgpu::VertexAttribute {
                    format: #format,
                    offset,
                    shader_location: #shader_location,
                }
                
                offset = offset + ::std::mem::size_of::<#field_type>();
            
                attrib
            },
        )
    }).collect::<Vec<_>>();

    quote!(
        impl VertexBufferLayout for #ident #generics #where_clause {
            fn desc(&self) -> wgpu::VertexBufferLayout<'static> {
                let mut offset = 0;
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<#ident>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        #(#vertex_attributes)*
                    ]
                }
            }
        }
    ).into()
}
