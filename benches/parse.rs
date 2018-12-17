#![feature(test)]

extern crate test;

#[macro_use]
extern crate darling;
extern crate syn;

use darling::FromDeriveInput;

#[derive(Debug, Clone, FromMeta)]
struct Wrapper<T>(pub T);

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hello))]
struct Foo<T> {
    lorem: Wrapper<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_parse_struct(b: &mut Bencher) {
        let di = syn::parse_str(
            r#"
            #[hello(lorem = "Hello")]
            pub struct Foo;
        "#,
        ).unwrap();

        b.iter(|| {
            Foo::<String>::from_derive_input(&di).unwrap()
        })
    }
}
