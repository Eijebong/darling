#[macro_use]
extern crate darling;
extern crate syn;

use std::ops::Add;

#[derive(Debug, Clone, FromMetaItem)]
#[darling(bound = "T: FromMetaItem + Add")]
struct Wrapper<T>(pub T);

impl<T: Add> Add for Wrapper<T> {
    type Output = Wrapper<<T as Add>::Output>;
    fn add(self, rhs: Self) -> Wrapper<<T as Add>::Output> {
        Wrapper(self.0 + rhs.0)
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(hello), bound = "Wrapper<T>: Add, T: FromMetaItem")]
struct Foo<T> {
    lorem: Wrapper<T>,
}

#[test]
fn expansion() {}