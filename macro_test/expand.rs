[
    Field {
        attrs: [],
        vis: Visibility::Public(
            Pub,
        ),
        mutability: FieldMutability::None,
        ident: Some(
            Ident {
                ident: "other_field",
                span: #0 bytes(111..122),
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
                            span: #0 bytes(124..130),
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
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use ams_macro::GenerateNoIdStruct;
pub struct WithId {
    pub id: i32,
    pub other_field: String,
}
pub struct WithIdNoId {
    pub other_field: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for WithIdNoId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "WithIdNoId",
            "other_field",
            &&self.other_field,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for WithIdNoId {
    #[inline]
    fn clone(&self) -> WithIdNoId {
        WithIdNoId {
            other_field: ::core::clone::Clone::clone(&self.other_field),
        }
    }
}
