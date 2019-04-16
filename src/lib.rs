#![deny(trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications
       )]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod rego;

