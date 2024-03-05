use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident;
    let struct_fields = if let syn::Data::Struct(s) = &ast.data {
        &s.fields
    } else {
        return TokenStream::from(quote! {
            compile_error!("Expected struct");
        });
    };
    let fields_names = struct_fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string());
    let gen = quote! {
        use sqlx_pro::core::model::Model;
        impl Model for #struct_name {
            fn get_fields(&self) -> Vec<String> {
                vec![
                    #(#fields_names.to_string(),)*
                ]
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn table_name(attr_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    // 解析属性参数
    let attr_args = syn::parse_macro_input!(attr_input as syn::LitStr);
    let table_name = attr_args.value(); // 直接使用 `value` 获取字符串字面值

    // 解析结构体定义
    let annotated_item_ast = syn::parse_macro_input!(annotated_item as DeriveInput);
    let struct_ident = &annotated_item_ast.ident;
    let struct_fields = if let syn::Data::Struct(s) = &annotated_item_ast.data {
        &s.fields
    } else {
        return TokenStream::from(quote! {
            compile_error!("Expected struct");
        });
    };
    let fields_names = struct_fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string());
    // 创建一个实现TableName trait的方法，返回字符串化后的表名
    let table_name_impl = quote! {
        use sqlx_pro::core::model::TableName;
        use sqlx_pro::core::model::Model;
        impl TableName for #struct_ident {
            fn get_table_name(&self) -> String {
                #table_name.to_string()
            }
        }
        impl Model for #struct_ident {
            fn get_fields(&self) -> Vec<String> {
                vec![
                    #(#fields_names.to_string(),)*
                ]
            }
        }
    };
    // 将原结构体、TableName trait 和新生成的impl代码合并为新的TokenStream
    let output = quote! {
        #annotated_item_ast
        #table_name_impl
    };

    // 返回处理后的TokenStream
    output.into()
}
