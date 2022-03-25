use crate::behavioral::state::Post;
use crate::behavioral::strategy::{Duck, FlyNoWay, FlyWithWings, MallardDuck, ModelDuck};
use crate::creational::abstract_factory::{Application, Os};
use crate::creational::builder::{ConcreteBuilder1, ConcreteBuilder2, Director};
use crate::creational::factory::{ShapeFactory, ShapeType};
use crate::creational::singleton::get_config;

mod creational;
mod behavioral;
mod structural;

fn main() {
    println!("# Creational Patterns");
    println!(" 1. Factory Method");
    let shape = ShapeFactory::new(&ShapeType::Circle);
    shape.draw(); // draw a circle

    let shape = ShapeFactory::new(&ShapeType::Rectangle);
    shape.draw(); // draw a rectangle

    println!(" 2. Abstract Factory");
    let app = Application::new(&Os::Windows);
    let btn = app.create_button();
    let chk = app.create_checkbox();
    btn.paint(); // windows os button
    chk.paint(); // windows os checkbox

    let app = Application::new(&Os::Mac);
    let btn = app.create_button();
    let chk = app.create_checkbox();
    btn.paint(); // mac os button
    chk.paint(); // mac os checkbox

    println!(" 3. Builder");
    let builder = Box::new(ConcreteBuilder1::new());
    let mut direct = Director::new(builder);
    direct.construct();
    let product = direct.builder.get_product();
    product.list_parts();
    // 	part a1
    // 	part b1
    // 	part c1
    let builder = Box::new(ConcreteBuilder2::new());
    let mut direct = Director::new(builder);
    direct.construct();
    let product = direct.builder.get_product();
    product.list_parts();
    //  part a2
    //  part b2
    // 	part c2

    println!(" 4. Singleton");
    let f1 = get_config();
    // modify
    {
        let mut conf = f1.lock().unwrap();
        println!("\t{}", conf.db_connection_str); // mongodb://host@pass/db
        conf.db_connection_str = String::from("mongodb://host@pass/helloDb");
    }

    let f2 = get_config();
    let conf2 = f2.lock().unwrap();
    println!("\t{}", conf2.db_connection_str); // mongodb://host@pass/helloDb

    println!("# Behavioral Patterns");
    println!(" 1. Strategy");
    let mut mallard_duck = MallardDuck::new(Box::new(FlyWithWings));
    mallard_duck.fly();
    mallard_duck.set_fly_behaviour(Box::new(FlyNoWay));
    mallard_duck.fly();

    let model_duck = ModelDuck::new(Box::new(FlyNoWay));
    model_duck.fly();

    println!(" 2. State");
    let mut post = Post::new();

    let text = "State is a behavioral design pattern.";
    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
    println!("\tpost content: {}", post.content());
}
