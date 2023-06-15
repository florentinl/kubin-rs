extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

const CORNER_NAMES: [&str; 8] = ["ufr", "ufl", "ubr", "ulb", "dfr", "dlf", "drb", "dbl"];
const EDGE_NAMES: [&str; 12] = [
    "uf", "ub", "ur", "ul", "df", "db", "dr", "dl", "fr", "fl", "br", "bl",
];

#[proc_macro_derive(CubeSubset)]
pub fn cube_subset_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Ensure that the type is a struct
    if let Data::Struct(data) = input.data {
        // Ensure that all fields are named after the pieces and are of type (u8, u8)
        let mut edges = Vec::new();
        let mut corners = Vec::new();
        for field in data.fields {
            let name = match field.ident {
                Some(name) => name,
                None => panic!("CubeSubset can only be derived for named structs"),
            };

            assert!(check_is_u8_tuple(&field.ty));
            if EDGE_NAMES.contains(&name.to_string().as_str()) {
                edges.push(name);
            } else if CORNER_NAMES.contains(&name.to_string().as_str()) {
                corners.push(name);
            } else {
                panic!("Invalid piece name");
            }
        }

        let name = input.ident;
        let pieces_declaration = make_declarations(&edges, &corners);
        let edges_extraction = edge_extraction(&edges);
        let corners_extraction = corner_extraction(&corners);

        TokenStream::from(quote! {
            impl CubeSubset for #name {
                fn from_cube(cube: &cube::Cube) -> Self {
                    use cube::edge::Edge;
                    use cube::corner::Corner;
                    #(#pieces_declaration)*
                    #edges_extraction
                    #corners_extraction
                    #name {
                        #(#edges,)*
                        #(#corners,)*
                    }
                }
            }
        })
    } else {
        panic!("CubeSubset can only be derived for structs");
    }
}

fn make_declarations(edges: &[Ident], corners: &[Ident]) -> Vec<proc_macro2::TokenStream> {
    let pieces = edges.iter().chain(corners.iter()).collect::<Vec<_>>();
    pieces
        .iter()
        .map(|name| {
            quote! { let mut #name = (0, 0); }
        })
        .collect::<Vec<_>>()
}

fn edge_extraction(edges: &[Ident]) -> proc_macro2::TokenStream {
    let edge_match_statement = edges
        .iter()
        .map(|name| {
            let piece_name = Ident::new(&(name.to_string().to_uppercase()), name.span());
            quote! { cube::edge::EdgePiece::#piece_name => #name = (i, *orientation), }
        })
        .collect::<Vec<_>>();
    quote! {
        for (Edge { piece, orientation }, i) in cube.edges.iter().zip(0..) {
            match piece {
                #(#edge_match_statement)*
                _ => {}
            }
        }
    }
}

fn corner_extraction(corners: &[Ident]) -> proc_macro2::TokenStream {
    let corner_match_statement = corners
        .iter()
        .map(|name| {
            let piece_name =
                Ident::new(&(to_first_letter_uppercase(name.to_string())), name.span());
            quote! { cube::corner::CornerPiece::#piece_name => #name = (i, *orientation), }
        })
        .collect::<Vec<_>>();
    quote! {
        for (Corner { piece, orientation }, i) in cube.corners.iter().zip(0..) {
            match piece {
                #(#corner_match_statement)*
                _ => {}
            }
        }
    }
}

fn to_first_letter_uppercase(s: String) -> String {
    s[0..1].to_uppercase() + &s[1..]
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
