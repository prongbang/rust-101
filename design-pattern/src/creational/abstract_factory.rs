pub enum Os {
    Windows,
    Mac,
}

pub trait Button {
    fn paint(&self);
}

pub trait Checkbox {
    fn paint(&self);
}

struct WindowsButton {}

struct WindowsCheckbox {}

impl Button for WindowsButton {
    fn paint(&self) {
        println!("\twindows os button");
    }
}

impl Checkbox for WindowsCheckbox {
    fn paint(&self) {
        println!("\twindows os checkbox");
    }
}

struct MacButton;

struct MacCheckbox;

impl Button for MacButton {
    fn paint(&self) {
        println!("\tmac os button");
    }
}

impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("\tmac os checkbox");
    }
}

pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

struct WindowsFactory;

struct MacFactory;

impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox {})
    }
}

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox {})
    }
}

pub struct Application;

impl Application {
    pub fn new(os: &Os) -> Box<dyn GUIFactory> {
        match os {
            Os::Windows => Box::new(WindowsFactory {}),
            Os::Mac => Box::new(MacFactory {}),
        }
    }
}