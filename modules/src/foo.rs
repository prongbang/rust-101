
// module using module struct
use crate::bar::Bar;

pub struct Foo {
    bar: Bar,
}

// Implement Foo
impl Foo {
    // method level module Foo
    pub fn new(bar: String) -> Self {
        Self {
            bar: Bar::new(bar),
        }
    }

    // method for Foo
    pub fn get_bar(&self) -> Bar {
        Bar::new(self.bar.get_foo().to_string())
    }
}