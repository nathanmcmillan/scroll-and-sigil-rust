pub mod app;
pub mod run;
pub mod webgl;

use app::App;
use js_sys::Object;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;
use web_sys::WebGl2RenderingContext;
use web_sys::Window;

fn print(s: &'static str) {
    console::log_1(&s.into());
}

fn dimensions(window: &Window) -> (u32, u32) {
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    (width, height)
}

fn canvas(document: &Document, width: u32, height: u32) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(width);
    canvas.set_height(height);

    let style = canvas.style();
    style.set_property("display", "block")?;
    style.set_property("position", "absolute")?;
    style.set_property("left", "0")?;
    style.set_property("right", "0")?;
    style.set_property("top", "0")?;
    style.set_property("bottom", "0")?;
    style.set_property("margin", "0")?;

    document.body().unwrap().append_child(&canvas)?;

    Ok(canvas)
}

fn webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, Object> {
    canvas
        .get_context("webgl2")?
        .expect("Unable to get WebGL2 context")
        .dyn_into::<WebGl2RenderingContext>()
}

fn request_animation_frame(function: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(function.as_ref().unchecked_ref())
        .unwrap();
}

fn tick(app: &mut App) {
    app.update();
    app.render();
}

#[cfg(feature = "console_error_panic_hook")]
fn console_panic_hook() {
    print("using console panic hook feature");
    use std::panic;
    extern crate console_error_panic_hook;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(not(feature = "console_error_panic_hook"))]
pub fn console_panic_hook() {}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    print("scroll and sigil");
    console_panic_hook();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let (width, height) = dimensions(&window);
    let canvas = canvas(&document, width, height)?;
    let context = webgl_context(&canvas)?;
    let document = Rc::new(document);
    let context = Rc::new(context);
    {
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            print("mouse down!");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            print("key down!");
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    let mut app = App::new(context.clone());
    app.initialize()?;
    app.resize(width, height);
    // {
    //     let closure = Closure::wrap(Box::new(move || {
    //         let (width, height) = dimensions(&get_window());
    //         print("resize!");
    //         app.resize(width, height);
    //     }) as Box<dyn FnMut()>);
    //     window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            tick(&mut app);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}
