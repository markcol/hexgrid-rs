//! A library to work with hexgrids written in Rust.

#![forbid(overflowing_literals)]
#![warn(
    missing_docs,
    clippy::all,
    rust_2018_idioms,
    intra_doc_link_resolution_failure,
    missing_copy_implementations,
    missing_debug_implementations,
    path_statements,
    trivial_bounds,
    type_alias_bounds,
    unconditional_recursion,
    unions_with_drop_fields,
    while_true,
    bad_style,
    future_incompatible,
    rust_2018_compatibility,
    rust_2018_idioms,
    unused
)]
#![allow(
    dead_code,
    unknown_lints,
    clippy::cyclomatic_complexity,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments
)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert!(true);
    }
}
