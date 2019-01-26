#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use js_sys::{Math, Array};

// macro_rules! vec_of_strings {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }
// let names = vec_of_strings!["a", "b", "c", "d"];

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;

    Ok(())
}

#[wasm_bindgen]
pub fn random_num(max: i32) -> i32 {
    (Math::random() * max as f64) as i32
}

pub fn collect_names(js_names: &JsValue) -> Result<Vec<String>, JsValue> {
    let mut names: Vec<String> = Vec::new();

    let iterator = js_sys::try_iter(js_names)?.ok_or_else(|| {
        "JS value not iterable!"
    })?;
    console_log!("hi {}", 23);

    for x in iterator {
        // If the iterator's `next` method throws an error, propagate it up to the caller.
        let x = x?;

        match x.as_string() {
            Some(name) => names.push(name),
            None => console_warn!("only string values will be collected")
        }
    }

    Ok(names)
}

#[wasm_bindgen]
pub struct NameBuilder {
    names: Vec<String>
}

// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl NameBuilder {
    pub fn new(js_names: Array) -> NameBuilder {
        if(js_names.length() < 1) {
            panic!("must pass an array with at least 1 value");
        }

        let names = match collect_names(&js_names) {
            Ok(names) => names,
            _ => panic!("must pass an iterable JS value")
        };

        NameBuilder {
            names
        }
    }

    pub fn get_random_name(&self) -> String {
        self.names[random_num(self.names.len() as i32) as usize].to_string()
    }
}
