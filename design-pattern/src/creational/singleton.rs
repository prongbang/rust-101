use std::sync::{Mutex, Once};
use std::mem::MaybeUninit;

#[derive(Clone)]
pub struct Config {
    pub db_connection_str: String,
}

// 'static is label
pub fn get_config() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    // || unsafe {} is a function
    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: String::from("mongodb://host@pass/db")
        }));
    });

    unsafe { &*CONF.as_ptr() }
}