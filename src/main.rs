use gl::types::*;
use sdl2::{event::Event, log};
use sdl2::keyboard::Keycode;
use sdl2::{event::WindowEvent, video::GLProfile};

use std::{
    os::raw::c_void,
    sync::{Arc, Mutex},
    thread,
};

mod shaders;
mod texture;
mod debugger;

use debugger::*;
use texture::*;

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct WindowData {
    w: u32,
    h: u32,
    title: String,
}

fn main() {
    let window_data = WindowData {
        w: 800,
        h: 800,
        title: "FPS: ".to_string(),
    };

    let clear_color: Color = Color {
        r: 0.0,
        g: 0.5,
        b: 0.1,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let mut window = video_subsystem
        .window(&window_data.title, window_data.w, window_data.h)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _context = window.gl_create_context().unwrap();
    video_subsystem.gl_set_swap_interval(1).unwrap();

    gl::load_with(|loader| video_subsystem.gl_get_proc_address(loader) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, window_data.w as i32, window_data.h as i32);
    }

    let vertices: [f32; 20] = [
        0.5, 0.5, 0.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, -0.5, -0.5, 0.0, 1.0, 1.0, -0.5, 0.5,
        0.0, 1.0, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    //let textures_cordinates: [f64; 6] = [
    //    0.0, 0.0, // lower-left corner
    //    1.0, 0.0, // lower-right corner
    //    0.5, 1.0, // top-center corner
    //];

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    let gl_shader = shaders::GLShaderProgram::new_from_file(
        "content/shaders/vertexShader.glsl",
        "content/shaders/fragmentShader.glsl",
    );

    let druid = Arc::new(Mutex::new(RusteezeTexture2D {
        image_path: "content/textures/Druid.png",
        height: 0,
        width: 0,
        loaded: false,
        raw: vec![],
        id: 0,
    }));
    let sword_man = Arc::new(Mutex::new(RusteezeTexture2D {
        image_path: "content/textures/sword_man.png",
        height: 0,
        width: 0,
        loaded: false,
        raw: vec![],
        id: 0,
    }));
    let fallen_reaper = Arc::new(Mutex::new(RusteezeTexture2D {
        image_path: "content/textures/fallen_reaper.png",
        height: 0,
        width: 0,
        loaded: false,
        raw: vec![],
        id: 0,
    }));

    let c_fallen_reaper = fallen_reaper.clone();
    thread::spawn(|| {
        //let img_path = c_fallen_reaper.try_lock().unwrap().image_path;
        load_mt(c_fallen_reaper);
        dbg_log("Loaded reaper");
    });

    let c_druid = druid.clone();
    thread::spawn(|| {
        load_mt(c_druid);
        dbg_log("Loaded druid");
    });

    let c_sword_man = sword_man.clone();
    // let hd3 =
    thread::spawn(move || {
        load_mt(c_sword_man);
        dbg_log("Loaded sword");
    });

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut delta_time;
    let mut cor = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    let timer = sdl_context.timer().unwrap();
    let mut now = timer.performance_counter();
    let mut last;

    let mut frame_count = 0;

    let mut should_run = true;
    while should_run {
        last = now;
        now = timer.performance_counter();
        delta_time = (((now - last) * 1000u64) as f64 / (timer.performance_frequency() as f64))
            * 0.001 as f64;

        cor.r = cor.r + (0.5 * delta_time) as f32;
        cor.g = cor.g + (0.5 * delta_time) as f32;
        cor.b = cor.b + (0.5 * delta_time) as f32;

        if cor.r > 1.0 {
            cor.r = 0.0
        }
        if cor.g > 1.0 {
            cor.g = 0.0
        }
        if cor.g > 1.0 {
            cor.g = 0.0
        }
        if cor.r <= 1.0 && cor.r > 0.6 {
            let mut lock = druid.try_lock();
            if let Ok(ref mut mutex) = lock {
                mutex.use_texture();
            } else {
                //dbg!("try_lock failed druid");
            }
        } else if cor.r < 0.6 && cor.r > 0.3 {
            let mut lock = sword_man.try_lock();
            if let Ok(ref mut mutex) = lock {
                mutex.use_texture();
            } else {
                //dbg!("try_lock failed sword_man");
            }
        } else {
            let mut lock = fallen_reaper.try_lock();
            if let Ok(ref mut mutex) = lock {
                mutex.use_texture();
            } else {
                //dbg!("try_lock failed fallen reaper");
            }
        }

        unsafe {
            gl::ClearColor(clear_color.r, clear_color.g, clear_color.b, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl_shader.use_program();
            // gl_shader.set_u3f(cor.r, cor.g, cor.b, "newColor");
            gl_shader.set_u3f(1.0, 1.0, 1.0, "newColor");

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            handle_window_event(&mut window, event, &mut should_run);
        }

        if frame_count == 60 {
            window
                .set_title(
                    (format!("{} | Frametime: {:.2}", window_data.title, delta_time * 1000.0)).as_str(),
                )
                .unwrap();
            frame_count = 0;
        } else {
            frame_count += 1;
        }
    }
}

fn handle_window_event(
    window: &mut sdl2::video::Window,
    event: sdl2::event::Event,
    running: &mut bool,
) {
    match event {
        Event::Quit { .. } => {
            *running = false;
        }
        Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
        } => {
            window.set_title("Respects").unwrap();
        }
        Event::Window {
            win_event: WindowEvent::Resized(width, height),
            ..
        } => {
            println!("Resized window, X: {} Y:{}", width, height);
            unsafe { gl::Viewport(0, 0, width, height) };
        }
        _ => {}
    }
}

fn load_mt(arc: Arc<Mutex<RusteezeTexture2D>>) {
    let mut lock = arc.try_lock();
    if let Ok(ref mut mutex) = lock {
        mutex.load_sync();
    } else {
        println!("try_lock failed");
    }
}
