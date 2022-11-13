use crate::converter::{convert_type_to_wgpu, convert_mat_type_to_wgou};
use crate::parser::TokenVertexFormat;
use crate::parser::parse_attrs;

#[derive(Debug)]
struct Entity {
    fields: Vec<EntityFields>
}

#[derive(Debug)]
struct EntityFieldsAttrs {
    name: String,
    data: u32,
    ty: Option<String>
}

#[derive(Debug)]
struct EntityFields {
    attrs: Vec<EntityFieldsAttrs>,
}

fn get_entity_field(field: &syn::Field) -> Option<EntityFields> {
    let mut attrs: Vec<EntityFieldsAttrs> = Vec::new();

    parse_attrs(&field.attrs,Box::new(|attr| {
        let name = attr.segment.ident.to_string();
        if name.starts_with("mat") {
            let mat : crate::parser::AttrMat = attr.attribute.parse_args().unwrap();
            
            attrs.push(EntityFieldsAttrs {
                name,
                data: mat.data,
                ty: Some(mat.ident.to_string()),
            });

            return
        }

        let lint : syn::LitInt = attr.attribute.parse_args().expect("Only integer is authorize for shader location data");
    
        attrs.push(EntityFieldsAttrs {
            name: attr.segment.ident.to_string(),
            data: lint.base10_parse().unwrap(),
            ty: None
        });
    }));

    let entity_fields = EntityFields {
        attrs,
    };

    Some(entity_fields)
}

fn process_wgpu_type(
    format: &crate::converter::WGPUData, 
    shader_locations: &mut Vec<u32>,
    attrs: &mut Vec<proc_macro2::TokenStream>,
    offset: &u64
) {
    let tty = TokenVertexFormat { attribute: format.wgpu_type.ty};
    let shader_location = format.shader_location;

    if shader_locations.contains(&shader_location) {
        panic!("Cannot have two time the same location in the same struct");
    }

    shader_locations.push(shader_location);

    attrs.push(quote::quote! {
        wgpu::VertexAttribute {
            offset: #offset,
            format: #tty,
            shader_location: #shader_location
        }
    });
}

fn require_repr_c(attrs : &std::vec::Vec<syn::Attribute>) {
    let mut valid = false;

    parse_attrs(&attrs, Box::new(|attr| {
        let repr_attr = attr.attribute.parse_args::<syn::Ident>().unwrap().to_string();
        if attr.segment.ident.to_string() == "repr" && (repr_attr == "C" || repr_attr == "transparent") {
            valid = true;
        }
    }));

    if !valid {
        panic!("VertexBufferLayout derive macro require #[repr(C)] or #[repr(transparent)] attribute for safety measure");
    }
}

pub fn vertex_buffer_layout_derive(item: proc_macro::TokenStream, step_mode: wgpu::VertexStepMode) -> proc_macro::TokenStream {
    let syn::DeriveInput {ident, data, attrs, ..} = syn::parse_macro_input!(item as syn::DeriveInput);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, ..}),
        ..
    }) = data 
    {
        named
    } else {
        panic!("Only struct are supported by VertexBufferLayout supported");
    };

    require_repr_c(&attrs);

    let entity = Entity {
        fields: fields.iter().filter_map(|field| {get_entity_field(field)}).collect()
    };

    let mut attrs : Vec<proc_macro2::TokenStream> = Vec::new();

    let mut offset:u64 = 0;
    let mut shader_locations: Vec<u32> = Vec::new();

    for i in entity.fields {
        for attr in i.attrs {
            if attr.ty == None {
                let format = convert_type_to_wgpu(&attr.name, attr.data).unwrap();
                process_wgpu_type(&format, &mut shader_locations, &mut attrs, &offset);
                offset += format.wgpu_type.offset;
            } else {
                let mat_format = convert_mat_type_to_wgou(
                    &attr.name, 
                    attr.data,
                    &mut attr.ty.unwrap()
                );

                for format in mat_format {
                    process_wgpu_type(&format, &mut shader_locations, &mut attrs, &offset);
                    offset += format.wgpu_type.offset;
                }
            }
        }
    }

    let step_mode = crate::parser::TokenVertexStepMode {step_mode};

    quote::quote! {
        impl VertexBufferLayout for #ident {
            fn desc() -> wgpu::VertexBufferLayout<'static> {
                wgpu::VertexBufferLayout {
                    array_stride: #offset as wgpu::BufferAddress,
                    step_mode: #step_mode,
                    attributes: &[#(#attrs),*]
                }
            }
        }
    }.into()
}

pub fn bytemuck_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(item as syn::DeriveInput);

    quote::quote!(
        unsafe impl bytemuck::Zeroable for #ident {}
        unsafe impl bytemuck::Pod for #ident {}
    ).into()
}
