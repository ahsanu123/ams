[
    Field {
        attrs: [],
        vis: Visibility::Public(
            Pub,
        ),
        mutability: FieldMutability::None,
        ident: Some(
            Ident {
                ident: "title",
                span: #0 bytes(111..116),
            },
        ),
        colon_token: Some(
            Colon,
        ),
        ty: Type::Path {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            ident: "String",
                            span: #0 bytes(118..124),
                        },
                        arguments: PathArguments::None,
                    },
                ],
            },
        },
    },
    Field {
        attrs: [],
        vis: Visibility::Public(
            Pub,
        ),
        mutability: FieldMutability::None,
        ident: Some(
            Ident {
                ident: "description",
                span: #0 bytes(134..145),
            },
        ),
        colon_token: Some(
            Colon,
        ),
        ty: Type::Path {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            ident: "String",
                            span: #0 bytes(147..153),
                        },
                        arguments: PathArguments::None,
                    },
                ],
            },
        },
    },
]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate ams_macro;
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: String,
}
pub struct BookNoId {
    pub title: String,
    pub description: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for BookNoId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BookNoId",
            "title",
            &self.title,
            "description",
            &&self.description,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BookNoId {
    #[inline]
    fn clone(&self) -> BookNoId {
        BookNoId {
            title: ::core::clone::Clone::clone(&self.title),
            description: ::core::clone::Clone::clone(&self.description),
        }
    }
}
