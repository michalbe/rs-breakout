
use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

static VERTEX_DATA: [GLfloat; 108] = [
    1.0, 9.0, 1.0, -1.0, 9.0, 1.0, -1.0, -9.0, 1.0,
    1.0, -9.0, 1.0, 1.0, 9.0, 1.0, -1.0, -9.0, 1.0,
    1.0, 9.0, 1.0, 1.0, -9.0, 1.0, 1.0, -9.0, -1.0,
    1.0, 9.0, -1.0, 1.0, 9.0, 1.0, 1.0, -9.0, -1.0,
    1.0, 9.0, 1.0, 1.0, 9.0, -1.0, -1.0, 9.0, -1.0,
    -1.0, 9.0, 1.0, 1.0, 9.0, 1.0, -1.0, 9.0, -1.0,
    -1.0, -9.0, -1.0, -1.0, 9.0, -1.0, 1.0, 9.0, -1.0,
    1.0, -9.0, -1.0, -1.0, -9.0, -1.0, 1.0, 9.0, -1.0,
    -1.0, -9.0, -1.0, -1.0, -9.0, 1.0, -1.0, 9.0, 1.0,
    -1.0, 9.0, -1.0, -1.0, -9.0, -1.0, -1.0, 9.0, 1.0,
    -1.0, -9.0, -1.0, 1.0, -9.0, -1.0, 1.0, -9.0, 1.0,
    -1.0, -9.0, 1.0, -1.0, -9.0, -1.0, 1.0, -9.0, 1.0,
];

static VS_SRC: &'static str = "#version 330
uniform float n;
uniform float rot;
// Vertex position in the mesh
in vec3 p;
// Vertex position in the instance
out vec4 f;
// Projection matrix
const mat4 P=mat4(
    1.299,0.,0.,0.,
    0.,1.732,0.,0.,
    0.,0.,-1.002,-1.,
    0.,0.,-2.002,0.);
// Compute the translation of the instance
vec3 t(float id,float o){
    float x=-707.+mod(id,707.)*2.;
    float z=-707.+(id/707.)*2.;
    // Make offset discrete in increments of the cube's width.
    float Z=z+floor(o/2.)*2.;
    return vec3(
        x,
        floor(
            // y
            9.*sin(x/30.)*sin(Z/20.)
            // Hills and valleys
            +99.*sin(x/99.)*sin(Z/299.)
            // Random noise, constant for a given (x, Z)
            +4.*fract(sin(Z)*99.)),
        z-mod(o,2.));
}
void main(){
    // The offset of the world
    float o=float(n)/99.;
    // The position of the camera
    float y=9.*sin(-o/30.)+30.;
    // Yaw
    float a=float(rot)*0.0063;
    // Pitch
    float b=float(381.0)*0.0016-.8;
    f = P * mat4(
        cos(a),sin(a)*sin(b),-sin(a)*cos(b),0.,
        0.,cos(b),sin(b),0.,
        sin(a),-cos(a)*sin(b),cos(a)*cos(b),0.,
        0.,-y,0.,1.)*vec4(p+t(float(gl_InstanceID),o),1.);
    gl_Position=f;
}";

static FS_SRC: &'static str = "#version 330
precision lowp float;
// Fragment position
in vec4 f;
// Fragment color
out vec4 c;
void main(){
    c=mix(
        // Normal of the fragment
        vec4(normalize(cross(dFdx(f).xyz,dFdy(f).xyz)),1.),
        // Fog color
        vec4(1.,.7,0.,1.),
        // Divide length by max fog distance
        clamp(length(f-vec4(0.))/999.,0.,1.));
}";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);


        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}


fn main() {
    use glutin::GlContext;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    gl_window.set_title("supeRSdisco by piesku.com");

    unsafe { gl_window.make_current() }.unwrap();

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        gl::UseProgram(program);

        let uniform_now = gl::GetUniformLocation(program, CString::new("n").unwrap().as_ptr());
        let uniform_rot = gl::GetUniformLocation(program, CString::new("rot").unwrap().as_ptr());

        // gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            ptr::null::<GLvoid>().add(0) as *const _,
        );

        let mut now_uni_content: f32 = 10.0;
        let mut rot: f32 = 0.0;
        let mut closed = false;

        while !closed {
            gl::ClearColor(1.0, 0.7, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                now_uni_content -= 100.0;
                // rot += 1.0;

                gl::Uniform1f(uniform_now, now_uni_content);
                gl::Uniform1f(uniform_rot, rot);

                gl::DrawArraysInstanced(gl::TRIANGLES, 0, 36, 707 * 707);

                gl_window.swap_buffers().unwrap();

            events_loop.poll_events(|event| {
                use glutin::{Event, WindowEvent};

                if let Event::WindowEvent { event, .. } = event {
                    if let WindowEvent::CloseRequested = event {
                        closed = true;
                    }
                }
            });
        }

    }
}
