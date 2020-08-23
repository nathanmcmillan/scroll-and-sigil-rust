use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;

pub fn compile(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    let ok = context.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false);
    if ok {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn program(context: &WebGl2RenderingContext, vertex: &str, fragment: &str) -> Result<WebGlProgram, String> {
    let vertex = compile(&context, GL::VERTEX_SHADER, vertex)?;
    let fragment = compile(&context, GL::FRAGMENT_SHADER, fragment)?;
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.attach_shader(&program, &vertex);
    context.attach_shader(&program, &fragment);
    context.link_program(&program);
    let ok = context.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false);
    if !ok {
        return Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")));
    }
    context.delete_shader(Some(&vertex));
    context.delete_shader(Some(&fragment));
    context.use_program(Some(&program));
    let texture = context.get_uniform_location(&program, "u_texture");
    context.uniform1i(texture.as_ref(), 0);
    context.use_program(Option::None);
    Ok(program)
}
