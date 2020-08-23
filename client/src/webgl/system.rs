use crate::webgl;
use crate::webgl::buffer::WebGlRenderBuffer;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlProgram;
use web_sys::WebGlTexture;

pub struct WebGlRenderSystem {
    program: usize,
    programs: Vec<WebGlProgram>,
    context: Rc<WebGl2RenderingContext>,
}

impl WebGlRenderSystem {
    pub fn new(context: Rc<WebGl2RenderingContext>) -> Self {
        WebGlRenderSystem {
            program: 0,
            programs: Vec::new(),
            context,
        }
    }

    pub fn add_program(&mut self, vertex: &str, fragment: &str) -> Result<(), String> {
        let program = webgl::shader::program(&self.context, &vertex, &fragment)?;
        self.programs.push(program);
        Ok(())
    }

    pub fn use_program(&mut self, index: usize) {
        self.program = index;
        self.context.use_program(Some(&self.programs[index]));
    }

    pub fn bind_vao_attributes(&self, b: &WebGlRenderBuffer) {
        let context = &self.context;
        let mut index = 0;
        let mut offset = 0;
        let buffer = &b.buffer;
        let stride = 4 * (buffer.position + buffer.color + buffer.texture + buffer.normal + buffer.bone) as i32;
        if buffer.position > 0 {
            context.vertex_attrib_pointer_with_i32(index, buffer.position as i32, GL::FLOAT, false, stride, 0);
            context.enable_vertex_attrib_array(index);
            index += 1;
            offset += buffer.position as i32;
        }
        if buffer.color > 0 {
            context.vertex_attrib_pointer_with_i32(index, buffer.color as i32, GL::FLOAT, false, stride, offset * 4);
            context.enable_vertex_attrib_array(index);
            index += 1;
            offset += buffer.color as i32;
        }
        if buffer.texture > 0 {
            context.vertex_attrib_pointer_with_i32(index, buffer.texture as i32, GL::FLOAT, false, stride, offset * 4);
            context.enable_vertex_attrib_array(index);
            index += 1;
            offset += buffer.texture as i32;
        }
        if buffer.normal > 0 {
            context.vertex_attrib_pointer_with_i32(index, buffer.normal as i32, GL::FLOAT, false, stride, offset * 4);
            context.enable_vertex_attrib_array(index);
            index += 1;
            offset += buffer.normal as i32;
        }
        if buffer.bone > 0 {
            context.vertex_attrib_pointer_with_i32(index, buffer.bone as i32, GL::FLOAT, false, stride, offset * 4);
            context.enable_vertex_attrib_array(index);
        }
        context.bind_vertex_array(Option::None);
        context.bind_buffer(GL::ARRAY_BUFFER, Option::None);
        context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Option::None);
    }

    pub fn make_vao(&self, b: &mut WebGlRenderBuffer) {
        let context = &self.context;
        b.vbo = context.create_buffer();
        b.ebo = context.create_buffer();
        b.vao = context.create_vertex_array();
        context.bind_vertex_array(b.vao.as_ref());
        context.bind_buffer(GL::ARRAY_BUFFER, b.vbo.as_ref());
        context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, b.ebo.as_ref());
        self.bind_vao_attributes(b);
    }

    pub fn bind_texture(&self, active: u32, texture: &WebGlTexture) {
        self.context.active_texture(active);
        self.context.bind_texture(GL::TEXTURE_2D, Some(texture));
    }

    pub fn update_uniform_matrix(&self, name: &str, matrix: &[f32]) {
        let location = self.context.get_uniform_location(&self.programs[self.program], name);
        self.context.uniform_matrix4fv_with_f32_array(location.as_ref(), false, matrix);
    }

    pub fn update_view(&self, x: i32, y: i32, width: i32, height: i32) {
        self.context.viewport(x, y, width, height);
        self.context.scissor(x, y, width, height);
    }

    pub fn update_vao(&self, b: &WebGlRenderBuffer, hint: u32) {
        let context = &self.context;
        context.bind_vertex_array(b.vao.as_ref());
        context.bind_buffer(GL::ARRAY_BUFFER, b.vbo.as_ref());
        unsafe {
            let vertices = js_sys::Float32Array::view(&b.buffer.vertices);
            context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, hint);
        }
        context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, b.ebo.as_ref());
        unsafe {
            let indices = js_sys::Uint32Array::view(&b.buffer.indices);
            context.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices, hint);
        }
    }

    pub fn bind_and_draw(&self, b: &WebGlRenderBuffer) {
        let count = b.buffer.index_position as i32;
        if count == 0 {
            return;
        }
        let context = &self.context;
        context.bind_buffer(GL::ARRAY_BUFFER, b.vbo.as_ref());
        context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, b.ebo.as_ref());
        context.draw_elements_with_i32(GL::TRIANGLES, count, GL::UNSIGNED_INT, 0);
    }
    pub fn update_and_draw(&self, b: &WebGlRenderBuffer) {
        let count = b.buffer.index_position as i32;
        if count == 0 {
            return;
        }
        self.update_vao(b, GL::DYNAMIC_DRAW);
        self.context.draw_elements_with_i32(GL::TRIANGLES, count, GL::UNSIGNED_INT, 0);
    }
}
