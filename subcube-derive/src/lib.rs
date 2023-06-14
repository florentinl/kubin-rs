extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

const PIECES_NAMES: [&str; 20] = [
    "ufr", "ufl", "ubr", "ulb", "dfr", "dlf", "drb", "dbl", "uf", "ub", "ur", "ul", "df", "db",
    "dr", "dl", "fr", "fl", "br", "bl",
];

#[proc_macro_derive(CubeSubset)]
pub fn cube_subset_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Ensure that the type is a struct
    if let Data::Struct(data) = input.data {
        // Ensure that all fields are named after the pieces and are of type (u8, u8)
        let name = input.ident;
        let fields = data.fields;
        let mut edges = Vec::new();
        let mut corners = Vec::new();
        for field in fields {
            let name = field.ident;
            let name = match name {
                Some(name) => name,
                None => panic!("CubeSubset can only be derived for named structs"),
            };
            let ty = field.ty;
            assert!(PIECES_NAMES.contains(&name.to_string().as_str()));
            assert!(check_is_u8_tuple(&ty));
            if name.to_string().len() == 2 {
                edges.push(name);
            } else {
                corners.push(name);
            }
        }

        let edge_declarations = edges
            .iter()
            .map(|name| {
                quote! { let mut #name: (u8, u8); }
            })
            .collect::<Vec<_>>();
        let edge_match_statement = edges
            .iter()
            .map(|name| {
                let piece_name = name.to_string().to_uppercase();
                quote! { cube::edge::EdgePiece::#piece_name => #name = (i, *orientation), }
            })
            .collect::<Vec<_>>();
        let edge_part = quote! {
            #(#edge_declarations)*
            for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
                match piece {
                    #(#edge_match_statement)*
                    _ => {}
                }
            }
        };

        let corner_declarations = corners
            .iter()
            .map(|name| {
                quote! { let mut #name: (u8, u8); }
            })
            .collect::<Vec<_>>();
        let corner_match_statement = corners
            .iter()
            .map(|name| {
                // Upper case the first letter only
                let piece_name = name
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .to_string()
                    + &name.to_string()[1..];
                quote! { cube::corner::CornerPiece::#piece_name => #name = (i, *orientation), }
            })
            .collect::<Vec<_>>();
        let corner_part = quote! {
            #(#corner_declarations)*
            for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
                match piece {
                    #(#corner_match_statement)*
                    _ => {}
                }
            }
        };

        let expanded = quote! {
            impl CubeSubset for #name {
                fn from_cube(cube: &cube::Cube) -> Self {
                    #edge_part
                    #corner_part
                    #name {
                        #(#edges,)*
                        #(#corners,)*
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("CubeSubset can only be derived for structs");
    }
}

fn check_is_u8_tuple(ty: &Type) -> bool {
    match ty {
        Type::Tuple(tuple) => {
            let mut iter = tuple.elems.iter();
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            let first = match first {
                Type::Path(path) => path,
                _ => return false,
            };
            let second = match second {
                Type::Path(path) => path,
                _ => return false,
            };
            let first = first.path.segments.iter().next().unwrap();
            let second = second.path.segments.iter().next().unwrap();
            let first = &first.ident;
            let second = &second.ident;
            if first != "u8" || second != "u8" {
                return false;
            }
            true
        }
        _ => false,
    }
}
