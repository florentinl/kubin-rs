extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

const CORNER_NAMES: [&str; 8] = ["urf", "ufl", "ubr", "ulb", "dfr", "dlf", "drb", "dbl"];
const EDGE_NAMES: [&str; 12] = [
    "uf", "ub", "ur", "ul", "df", "db", "dr", "dl", "fr", "fl", "br", "bl",
];

const POSITION_ONLY_SUFFIX: &str = "p";
const ORIENTATION_ONLY_SUFFIX: &str = "o";

enum PieceInfo {
    PositionAndOrientation,
    PositionOnly,
    OrientationOnly,
}

enum PieceType {
    Edge,
    Corner,
}

fn parse_piece_name(name: &Ident) -> (String, PieceType, PieceInfo) {
    let name = name.to_string();
    let mut piece_name = name.clone();
    let piece_type;
    let mut piece_info = None;
    // If the name contains and underscore, we know it's a position-only or orientation-only piece
    if name.contains('_') {
        let mut split = name.split('_');
        piece_name = split.next().unwrap().to_string();
        piece_info = split.next();
    }

    if EDGE_NAMES.contains(&piece_name.as_str()) {
        piece_type = PieceType::Edge;
    } else if CORNER_NAMES.contains(&piece_name.as_str()) {
        piece_type = PieceType::Corner;
    } else {
        panic!("Invalid piece name");
    }

    match piece_info {
        Some(POSITION_ONLY_SUFFIX) => (piece_name, piece_type, PieceInfo::PositionOnly),
        Some(ORIENTATION_ONLY_SUFFIX) => (piece_name, piece_type, PieceInfo::OrientationOnly),
        None => (piece_name, piece_type, PieceInfo::PositionAndOrientation),
        _ => panic!("Invalid piece name"),
    }
}

#[proc_macro_derive(CubeSubset)]
/// # Panics
/// Panics if the struct does not contain valid field names.
pub fn cube_subset_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Ensure that the type is a struct
    if let Data::Struct(data) = input.data {
        // Ensure that all fields are named after the pieces and are of type (u8, u8)
        let mut edges = Vec::new();
        let mut corners = Vec::new();
        for field in data.fields {
            let Some(name) = field.ident else {
                panic!("CubeSubset can only be derived for named structs");
            };

            let (piece_name, piece_type, piece_info) = parse_piece_name(&name);
            match piece_type {
                PieceType::Edge => edges.push((name, piece_name, piece_info)),
                PieceType::Corner => corners.push((name, piece_name, piece_info)),
            }
        }

        let name = input.ident;
        let pieces_declaration = make_declarations(&edges, &corners);
        let edges_extraction = edge_extraction(&edges);
        let corners_extraction = corner_extraction(&corners);
        let edges = edges
            .iter()
            .map(|(name, _, _)| quote! { #name: #name.clone() });
        let corners = corners
            .iter()
            .map(|(name, _, _)| quote! { #name: #name.clone() });

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

fn make_declarations(
    edges: &[(Ident, String, PieceInfo)],
    corners: &[(Ident, String, PieceInfo)],
) -> Vec<proc_macro2::TokenStream> {
    let pieces = edges.iter().chain(corners.iter()).collect::<Vec<_>>();
    pieces
        .iter()
        .map(|(name, _, piece_info)| match piece_info {
            PieceInfo::PositionAndOrientation => {
                quote! { let mut #name = (0, 0); }
            }
            PieceInfo::PositionOnly | PieceInfo::OrientationOnly => {
                quote! { let mut #name = 0; }
            }
        })
        .collect::<Vec<_>>()
}

fn edge_extraction(edges: &[(Ident, String, PieceInfo)]) -> proc_macro2::TokenStream {
    let edge_match_statement = edges
        .iter()
        .map(|(name, piece_name, piece_info)| {
            let piece_name = Ident::new(&(piece_name.to_uppercase()), name.span());
            match piece_info {
                PieceInfo::PositionAndOrientation => {
                    quote! { cube::edge::Piece::#piece_name => #name = (i, *orientation), }
                }
                PieceInfo::PositionOnly => {
                    quote! { cube::edge::Piece::#piece_name => #name = i, }
                }
                PieceInfo::OrientationOnly => {
                    quote! { cube::edge::Piece::#piece_name => #name = *orientation, }
                }
            }
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

fn corner_extraction(corners: &[(Ident, String, PieceInfo)]) -> proc_macro2::TokenStream {
    let corner_match_statement = corners
        .iter()
        .map(|(name, piece_name, piece_info)| {
            let piece_name = Ident::new(
                &(to_first_letter_uppercase(&piece_name.to_string())),
                name.span(),
            );
            match piece_info {
                PieceInfo::PositionAndOrientation => quote! {
                    cube::corner::Piece::#piece_name => #name = (i, *orientation),
                },
                PieceInfo::PositionOnly => {
                    quote! { cube::corner::Piece::#piece_name => #name = i, }
                }
                PieceInfo::OrientationOnly => {
                    quote! { cube::corner::Piece::#piece_name => #name = *orientation, }
                }
            }
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

fn to_first_letter_uppercase(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}
