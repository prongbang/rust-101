
pub struct Foo {
    bar: String,
}

// Implement Foo
impl Foo {
    // method level module Foo
    pub fn new(bar: String) -> Self {
        Self {
            bar: bar,
        }
    }

    // method for Foo
    pub fn get_bar(&self) -> &str {
        &self.bar
    }
}