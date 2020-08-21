use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlTexture;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub texture: WebGlTexture,
}

impl Texture {
    pub fn new(width: u32, height: u32, texture: WebGlTexture) -> Self {
        Texture { width, height, texture }
    }
}

pub async fn load(context: Rc<WebGl2RenderingContext>, source: &str, wrap: u32) -> Texture {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);

    let promise = js_sys::Promise::new(&mut move |resolve, _| {
        &image_clone.borrow().set_onload(Some(&resolve));
    });

    let image = image.borrow_mut();
    image.set_src(source);

    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();

    let texture = context.create_texture();
    context.bind_texture(GL::TEXTURE_2D, texture.as_ref());
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, wrap as i32);
    context.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, wrap as i32);
    context
        .tex_image_2d_with_u32_and_u32_and_html_image_element(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA, GL::UNSIGNED_BYTE, &image)
        .unwrap();
    context.bind_texture(GL::TEXTURE_2D, Option::None);

    let width = image.width();
    let height = image.height();

    Texture::new(width, height, texture.unwrap())
}
