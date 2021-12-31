pub struct Bar {
    foo: String,
}

impl Bar {
    pub fn new(foo: String) -> Self {
        Self {
            foo: foo,
        }
    }

    pub fn get_foo(&self) -> &str {
        &self.foo
    }
    
}