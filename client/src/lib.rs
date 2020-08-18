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
use web_sys::WebGlRenderingContext;
use web_sys::Window;

fn print(s: &'static str) {
    console::log_1(&s.into());
}

fn window() -> Window {
    web_sys::window().unwrap()
}

fn canvas(window: &Window, document: &Document) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;

    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

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

fn webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGlRenderingContext, Object> {
    canvas.get_context("webgl")?.unwrap().dyn_into::<WebGlRenderingContext>()
}

fn request_animation_frame(function: &Closure<dyn FnMut()>) {
    window().request_animation_frame(function.as_ref().unchecked_ref()).unwrap();
}

fn webgl_setup(context: &WebGlRenderingContext) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.depth_func(WebGlRenderingContext::EQUAL);
    context.cull_face(WebGlRenderingContext::BACK);
    context.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
    context.disable(WebGlRenderingContext::CULL_FACE);
    context.disable(WebGlRenderingContext::BLEND);
    context.disable(WebGlRenderingContext::DEPTH_TEST);
}

fn tick(app: &mut App) {
    app.update();
    app.render();
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    print("scroll and sigil");

    let window = window();
    let document = window.document().unwrap();
    let canvas = canvas(&window, &document)?;
    let context = webgl_context(&canvas)?;
    let window = Rc::new(window);
    let document = Rc::new(document);
    let context = Rc::new(context);
    webgl_setup(&context);
    {
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            print("mouse down!");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let document = document.clone();
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            print("key down!");
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let window = window.clone();
        let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
            print("resize!");
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let vert_shader = webgl::shader::compile(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
       attribute vec4 position;
       void main() {
           gl_Position = position;
       }
   "#,
    )?;
    let frag_shader = webgl::shader::compile(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
       void main() {
           gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
       }
   "#,
    )?;

    let program = webgl::shader::program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW);
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    let mut app = App::new(context.clone());

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
