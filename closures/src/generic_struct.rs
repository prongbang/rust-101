use std::borrow::Borrow;

pub trait MyTrait {
    fn bar();
}

struct MyStructImpl;

impl MyTrait for MyStructImpl {
    fn bar() {
        println!("bar");
    }
}

struct MyStruct<T>
    where
        T: MyTrait
{
    foo: T,
}

impl<T> MyStruct<T> {
    fn new(f: &impl MyTrait) -> Self <T> {
        MyStruct { foo: f }
    }
}