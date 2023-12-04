use std::fmt::Display;

pub use super::types::{
    GLbitField, GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint,
};
use crate::{Error, Loader};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Shader(GLuint);

impl Shader {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Program(GLuint);

impl Program {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VertexArray(GLuint);
impl VertexArray {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Buffer(GLuint);
impl Buffer {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Capability(GLenum);
impl Capability {
    pub const DEBUG_OUTPUT: Self = Self(0x92E0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferTarget(GLenum);
impl BufferTarget {
    //GL 3.3
    pub const ARRAY_BUFFER: Self = Self(0x8892);
    pub const COPY_READ_BUFFER: Self = Self(0x8F36);
    pub const COPY_WRITE_BUFFER: Self = Self(0x8F37);
    pub const ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
    pub const UNIFORM_BUFFER: Self = Self(0x8A11);
    pub const TEXTURE_BUFFER: Self = Self(0x8C2A);

    //since 4.3
    pub const SHADER_STORAGE_BUFFER: Self = Self(0x90D2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferUsage(GLenum);
impl BufferUsage {
    pub const STREAM_DRAW: Self = Self(0x88E0);
    pub const STREAM_READ: Self = Self(0x88E1);
    pub const STREAM_COPY: Self = Self(0x88E2);
    pub const STATIC_DRAW: Self = Self(0x88E4);
    pub const STATIC_READ: Self = Self(0x88E5);
    pub const STATIC_COPY: Self = Self(0x88E6);
    pub const DYNAMIC_DRAW: Self = Self(0x88E8);
    pub const DYNAMIC_READ: Self = Self(0x88E9);
    pub const DYNAMIC_COPY: Self = Self(0x88EA);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VertexAttributeKind(GLenum);

impl VertexAttributeKind {
    pub const BYTE: Self = Self(0x1400);
    pub const UNSIGNED_BYTE: Self = Self(0x1401);
    pub const SHORT: Self = Self(0x1402);
    pub const UNSIGNED_SHORT: Self = Self(0x1403);
    pub const INT: Self = Self(0x1404);
    pub const UNSIGNED_INT: Self = Self(0x1405);
    pub const FLOAT: Self = Self(0x1406);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ClearMask(GLbitField);
impl ClearMask {
    pub const NONE: Self = Self(0);
    pub const COLOR: Self = Self(0x4000);
    pub const DEPTH: Self = Self(0x100);
    pub const STENCIL: Self = Self(0x400);
    //TODO: replace with const impl of bitor once it stabilizes
    pub const ALL: Self = Self(Self::COLOR.0 | Self::DEPTH.0 | Self::STENCIL.0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Primitive(GLenum);
impl Primitive {
    pub const TRIANGLES: Self = Self(0x4);
    pub const TRIANGLE_STRIP: Self = Self(0x0005);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ShaderKind(GLenum);
impl ShaderKind {
    pub const FRAGMENT: Self = Self(0x8B30);
    pub const VERTEX: Self = Self(0x8B31);
}

/// Bindings to a curated subset of OpenGL 4.3
pub struct Api {
    //4.3 API
    debug_message_callback_ptr: unsafe extern "system" fn(
        callback: Option<DebugMessageCallback>,
        user_param: *mut std::ffi::c_void,
    ),
    // previous versions
    //
    //state
    enable_ptr: unsafe extern "system" fn(cap: Capability),
    clear_ptr: unsafe extern "system" fn(mask: ClearMask),
    clear_color_ptr: unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat),
    //draw
    draw_arrays_ptr: unsafe extern "system" fn(mode: Primitive, first: GLint, count: GLsizei),
    //vertex arrays
    gen_vertex_arrays_ptr: unsafe extern "system" fn(n: GLsizei, arrays: *mut VertexArray),
    bind_vertex_array_ptr: unsafe extern "system" fn(array: VertexArray),
    enable_vertex_attrib_array_ptr: unsafe extern "system" fn(index: GLuint),
    vertex_attrib_pointer_ptr: unsafe extern "system" fn(
        index: GLuint,
        size: GLint,
        kind: VertexAttributeKind,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const std::ffi::c_void,
    ),

    delete_vertex_arrays_ptr: unsafe extern "system" fn(n: GLsizei, arrays: *mut VertexArray),
    //buffers
    gen_buffers_ptr: unsafe extern "system" fn(n: GLsizei, buffers: *mut Buffer),
    bind_buffer_ptr: unsafe extern "system" fn(target: BufferTarget, buffer: Buffer),
    buffer_data_ptr: unsafe extern "system" fn(
        target: BufferTarget,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
        usage: BufferUsage,
    ),
    delete_buffer_ptr: unsafe extern "system" fn(buffer: Buffer),
    //shaders
    create_shader_ptr: unsafe extern "system" fn(kind: ShaderKind) -> Shader,
    shader_source_ptr: unsafe extern "system" fn(
        shader: Shader,
        count: GLsizei,
        source: *const *const GLchar,
        lenght: *const GLint,
    ),
    compile_shader_ptr: unsafe extern "system" fn(shader: Shader),
    delete_shader_ptr: unsafe extern "system" fn(shader: Shader),
    create_program_ptr: unsafe extern "system" fn() -> Program,
    attach_shader_ptr: unsafe extern "system" fn(program: Program, shader: Shader),
    link_program_ptr: unsafe extern "system" fn(program: Program),
    detach_shader_ptr: unsafe extern "system" fn(program: Program, Shader: Shader),
    use_program_ptr: unsafe extern "system" fn(program: Program),
    delete_program_ptr: unsafe extern "system" fn(program: Program),
}

impl Api {
    /// Loads all function pointers using a context function loader,
    /// also called ProcAddress. Works with GLFW, SDL2 any pretty much any OpenGL context.
    ///
    /// # Errors
    /// This function will return an error if any function pointer returns a null pointer.
    ///
    /// # Safety
    /// Unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers.
    ///
    pub unsafe fn with_loader(
        loader: &impl Fn(&str) -> *const std::ffi::c_void,
    ) -> Result<Self, Error> {
        Ok(Self {
            // OpenGL 4.3
            debug_message_callback_ptr: loader.load("glDebugMessageCallback")?,
            // OpenGL 3.3
            //
            //state
            enable_ptr: loader.load("glEnable")?,
            clear_ptr: loader.load("glClear")?,
            clear_color_ptr: loader.load("glClearColor")?,
            //draw
            draw_arrays_ptr: loader.load("glDrawArrays")?,
            //vertex arrays
            gen_vertex_arrays_ptr: loader.load("glGenVertexArrays")?,
            bind_vertex_array_ptr: loader.load("glBindVertexArray")?,
            enable_vertex_attrib_array_ptr: loader.load("glEnableVertexAttribArray")?,
            vertex_attrib_pointer_ptr: loader.load("glVertexAttribPointer")?,
            delete_vertex_arrays_ptr: loader.load("glDeleteVertexArrays")?,
            //buffers
            gen_buffers_ptr: loader.load("glGenBuffers")?,
            bind_buffer_ptr: loader.load("glBindBuffer")?,
            buffer_data_ptr: loader.load("glBufferData")?,
            delete_buffer_ptr: loader.load("glDeleteBuffer")?,
            //shaders
            create_shader_ptr: loader.load("glCreateShader")?,
            shader_source_ptr: loader.load("glShaderSource")?,
            compile_shader_ptr: loader.load("glCompileShader")?,
            delete_shader_ptr: loader.load("glDeleteShader")?,
            //program
            create_program_ptr: loader.load("glCreateProgram")?,
            attach_shader_ptr: loader.load("glAttachShader")?,
            link_program_ptr: loader.load("glLinkProgram")?,
            use_program_ptr: loader.load("glUseProgram")?,
            detach_shader_ptr: loader.load("glDetachShader")?,
            delete_program_ptr: loader.load("glDeleteProgram")?,
        })
    }

    /// Set the debug message callback.
    /// Make sure to `enable` `Capability::DEBUG` and to use a debug context.
    ///
    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn debug_message_callback(
        &self,
        callback: Option<DebugMessageCallback>,
        user_data: *mut std::ffi::c_void,
    ) {
        unsafe { (self.debug_message_callback_ptr)(callback, user_data) }
    }

    // OpenGL 3.3

    /// Enables certain state or context capabilities.
    ///
    /// # Safety
    /// See the safety note in `with_loader`
    // STATE
    #[inline]
    pub unsafe fn enable(&self, cap: Capability) {
        unsafe { (self.enable_ptr)(cap) }
    }

    /// Sets the clear color
    ///
    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe { (self.clear_color_ptr)(r, g, b, a) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn clear(&self, mask: ClearMask) {
        unsafe { (self.clear_ptr)(mask) }
    }

    // DRAW
    //
    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn draw_arrays(&self, mode: Primitive, start: GLint, count: GLsizei) {
        unsafe { (self.draw_arrays_ptr)(mode, start, count) }
    }

    // VERTEX ARRAYS
    //
    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, n: GLsizei, arrays: *mut VertexArray) {
        unsafe { (self.gen_vertex_arrays_ptr)(n, arrays) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn bind_vertex_array(&self, array: VertexArray) {
        unsafe { (self.bind_vertex_array_ptr)(array) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        unsafe { (self.enable_vertex_attrib_array_ptr)(index) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        kind: VertexAttributeKind,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const std::ffi::c_void,
    ) {
        unsafe { (self.vertex_attrib_pointer_ptr)(index, size, kind, normalized, stride, pointer) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, n: GLsizei, arrays: *mut VertexArray) {
        unsafe { (self.delete_vertex_arrays_ptr)(n, arrays) }
    }

    // BUFFERS

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn gen_buffers(&self, n: GLsizei, buffers: *mut Buffer) {
        unsafe { (self.gen_buffers_ptr)(n, buffers) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn bind_buffer(&self, target: BufferTarget, buffer: Buffer) {
        unsafe { (self.bind_buffer_ptr)(target, buffer) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn buffer_data(
        &self,
        target: BufferTarget,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
        usage: BufferUsage,
    ) {
        unsafe { (self.buffer_data_ptr)(target, size, data, usage) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn delete_buffer(&self, buffer: Buffer) {
        unsafe { (self.delete_buffer_ptr)(buffer) }
    }

    // SHADERS

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    #[must_use]
    pub unsafe fn create_shader(&self, kind: ShaderKind) -> Shader {
        unsafe { (self.create_shader_ptr)(kind) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn shader_source(
        &self,
        shader: Shader,
        count: GLsizei,
        source: *const *const GLchar,
        length: *const GLint,
    ) {
        unsafe { (self.shader_source_ptr)(shader, count, source, length) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn compile_shader(&self, shader: Shader) {
        unsafe { (self.compile_shader_ptr)(shader) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn delete_shader(&self, shader: Shader) {
        unsafe { (self.delete_shader_ptr)(shader) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    #[must_use]
    pub unsafe fn create_program(&self) -> Program {
        unsafe { (self.create_program_ptr)() }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn attach_shader(&self, program: Program, shader: Shader) {
        unsafe { (self.attach_shader_ptr)(program, shader) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn link_program(&self, program: Program) {
        unsafe { (self.link_program_ptr)(program) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn detach_shader(&self, program: Program, shader: Shader) {
        unsafe { (self.detach_shader_ptr)(program, shader) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn use_program(&self, program: Program) {
        unsafe { (self.use_program_ptr)(program) }
    }

    /// # Safety
    /// See the safety note in `with_loader`
    #[inline]
    pub unsafe fn delete_program(&self, program: Program) {
        unsafe { (self.delete_program_ptr)(program) }
    }
}

type DebugMessageCallback = extern "system" fn(
    source: DebugSource,
    kind: DebugType,
    id: GLuint,
    severity: DebugSeverity,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut std::ffi::c_void,
);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugSource(GLenum);

impl DebugSource {
    pub const API: Self = Self(0x8246);
    pub const WINDOW_SYSTEM: Self = Self(0x8247);
    pub const SHADER_COMPILER: Self = Self(0x8248);
    pub const THIRD_PARTY: Self = Self(0x8249);
    pub const APPLICATION: Self = Self(0x824A);
    pub const OTHER: Self = Self(0x824B);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugType(GLenum);

impl DebugType {
    pub const ERROR: Self = Self(0x824C);
    pub const DEPRECATED_BEHAVIOUR: Self = Self(0x824D);
    pub const UNDEFINED_BEHAVIOUR: Self = Self(0x824E);
    pub const PORTABILITY: Self = Self(0x824F);
    pub const PERFORMANCE: Self = Self(0x8250);
    pub const OTHER: Self = Self(0x8251);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugSeverity(GLenum);

impl DebugSeverity {
    pub const HIGH: Self = Self(0x9146);
    pub const MEDIUM: Self = Self(0x9147);
    pub const LOW: Self = Self(0x9148);
    pub const NOTIFICATION: Self = Self(0x826B);
}

impl Display for DebugSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SHADER_COMPILER => write!(f, "SHADER_COMPILER"),
            Self::API => write!(f, "API"),
            Self::OTHER => write!(f, "OTHER"),
            Self::THIRD_PARTY => write!(f, "THIRD_PARTY"),
            Self::APPLICATION => write!(f, "APPLICATION"),
            Self::WINDOW_SYSTEM => write!(f, "WINDOW_SYSTEM"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl Display for DebugType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::OTHER => write!(f, "OTHER"),
            Self::ERROR => write!(f, "ERROR"),
            Self::PORTABILITY => write!(f, "PORTABILITY"),
            Self::PERFORMANCE => write!(f, "PERFORMANCE"),
            Self::UNDEFINED_BEHAVIOUR => write!(f, "UNDEFINED_BEHAVIOUR"),
            Self::DEPRECATED_BEHAVIOUR => write!(f, "DEPRECATED"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}
