use std::mem::MaybeUninit;

use cac_gl_bindings::gl43 as gl;
use glfw::{Action, Context, Key};

fn create_vertex_buffer(gl: &gl::Api) -> gl::Buffer {
    #[rustfmt::skip]
    let triangle_data = [
        -0.5_f32,  0.5,    //TL
        -0.5, -0.5,        //BL
         0.5,  0.5,        //TR
    ];

    #[rustfmt::skip]
    let triangle_colors = [
        1.0_f32, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ];

    let vertex_data: Vec<f32> = triangle_data
        .chunks(2)
        .zip(triangle_colors.chunks(3))
        .flat_map(|(pos, color)| [pos[0], pos[1], color[0], color[1], color[2]])
        .collect();

    unsafe {
        let mut buffer = MaybeUninit::zeroed();
        gl.gen_buffers(2, buffer.as_mut_ptr());

        let vertex_buffer = buffer.assume_init();

        gl.bind_buffer(gl::BufferTarget::ARRAY_BUFFER, vertex_buffer);

        gl.buffer_data(
            gl::BufferTarget::ARRAY_BUFFER,
            (vertex_data.len() * std::mem::size_of::<f32>())
                .try_into()
                .expect("we won't have more than 15 floats"),
            vertex_data.as_ptr().cast(),
            gl::BufferUsage::STATIC_DRAW,
        );

        vertex_buffer
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use glfw::fail_on_errors;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    let gl = unsafe { gl::Api::with_loader(&|s| glfw.get_proc_address_raw(s))? };
    unsafe {
        gl.enable(gl::Capability::DEBUG_OUTPUT);
        gl.debug_message_callback(Some(debug_message_callback), std::ptr::null_mut());
    }
    let vao = unsafe {
        let mut vao = std::mem::MaybeUninit::zeroed();
        gl.gen_vertex_arrays(1, vao.as_mut_ptr());
        vao.assume_init()
    };

    unsafe {
        gl.clear_color(0.2, 0.2, 0.2, 1.0);
    }

    let vertex_buffer = create_vertex_buffer(&gl);

    unsafe {
        //vertex attributes
        gl.bind_vertex_array(vao);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer(
            0,
            2,
            gl::VertexAttributeKind::FLOAT,
            0,
            (std::mem::size_of::<f32>() * 5)
                .try_into()
                .expect("we won't have more than 20 floats"),
            std::ptr::null(),
        );

        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer(
            1,
            3,
            gl::VertexAttributeKind::FLOAT,
            0,
            (std::mem::size_of::<f32>() * 5)
                .try_into()
                .expect("we only have 5 floats"),
            (std::mem::size_of::<f32>() * 2) as _,
        );
    };

    let program = unsafe {
        let program = gl.create_program();

        let vs = gl.create_shader(gl::ShaderKind::VERTEX);
        let fs = gl.create_shader(gl::ShaderKind::FRAGMENT);

        gl.shader_source(vs, 1, [VS_SOURCE].as_ptr().cast(), std::ptr::null());
        gl.shader_source(fs, 1, [FS_SOURCE].as_ptr().cast(), std::ptr::null());

        gl.compile_shader(vs);
        gl.compile_shader(fs);

        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);

        gl.link_program(program);

        gl.detach_shader(program, vs);
        gl.detach_shader(program, fs);

        gl.use_program(program);

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        program
    };

    // Loop until the user closes the window
    while !window.should_close() {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        unsafe {
            gl.clear(gl::ClearMask::COLOR);
            gl.draw_arrays(gl::Primitive::TRIANGLE_STRIP, 0, 3);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
    unsafe {
        gl.delete_program(program);
        gl.delete_buffer(vertex_buffer);
    }

    Ok(())
}

extern "system" fn debug_message_callback(
    source: gl::DebugSource,
    kind: gl::DebugType,
    id: gl::GLuint,
    severity: gl::DebugSeverity,
    _length: gl::GLsizei,
    message: *const gl::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    let error_message = unsafe {
        std::ffi::CStr::from_ptr(message.cast())
            .to_str()
            .unwrap_or("[FAILED TO READ GL ERROR MESSAGE]")
    };

    match severity {
        gl::DebugSeverity::HIGH => log::error!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::MEDIUM => log::warn!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::LOW => log::info!("{id}: {kind} from {source}: {error_message}"),
        _ => log::trace!("{id}: {kind} from {source}: {error_message}"),
    }
}

const VS_SOURCE: &str = "#version 330

layout(location = 0) in vec2 pos;
layout(location = 1) in vec3 color;

out vec3 vec_color;

void main() {
    vec_color = color;
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}\0";

const FS_SOURCE: &str = "#version 330 
precision mediump float;

in vec3 vec_color;

out vec4 color; 
void main() {
    color = vec4(vec_color, 1.0);
}\0";
