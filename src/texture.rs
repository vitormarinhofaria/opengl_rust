use std::future::Future;
use gl::*;
#[derive(Debug, Clone)]
pub struct RusteezeTexture2D {
    pub image_path: &'static str,
    pub loaded: bool,
    pub raw: Vec<u8>,
    pub id: u32,
    pub width: i32,
    pub height: i32,
}

pub fn load(path: String, buffer: &mut Vec<u16>, w: &mut i32, h: &mut i32) {
    let dinamic_image = image::open(path).expect("Error loading texture image");
    let rgb8 = dinamic_image.as_rgb16().unwrap().clone();
    *w = rgb8.width() as i32;
    *h = rgb8.height() as i32;
    *buffer = rgb8.into_raw();
}

// -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>>
impl RusteezeTexture2D {
    // pub fn load_async<'a>(path: &'static str) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    //     let dinamic_image = image::open(path).expect("Error loading texture image");
    //     dinamic_image.as_rgb8().unwrap().clone()
    // }
    // pub fn create(&'a mut self) {
    //     std::thread::spawn(move || {
    //         load(
    //             self.image_path.to_string(),
    //             &mut self.raw,
    //             &mut self.width,
    //             &mut self.height,
    //         );
    //         self.loaded = &true;
    //     });
    // }

    // pub fn new(image_path: &'static str) -> RusteezeTexture2D {
    //     let loaded = false;
    //     let raw = &Vec::new();
    //     let id = 0;
    //     let width = 0;
    //     let height = 0;
    //     return RusteezeTexture2D {
    //         image_path,
    //         loaded,
    //         raw,
    //         id,
    //         width,
    //         height,
    //     };
    // }

    // pub fn load_async<'a>(&'a mut self) -> impl Future<Output = ()> + 'a {
    //     async move {
    //         let dinamic_image = image::open(self.image_path).expect("Error loading texture image");
    //         let rgb8 = dinamic_image.as_rgb16().unwrap().clone();
    //         self.width = rgb8.width() as i32;
    //         self.height = rgb8.height() as i32;
    //         self.raw = rgb8.into_raw();
    //     }
    // }
    pub fn load_sync(&mut self) {
        let dinamic_image = image::open(self.image_path).expect("Error loading texture image");
        let rgb8 = dinamic_image.as_rgba8().expect("Erro ao pegar image buffer").clone();
        self.width = rgb8.width() as i32;
        self.height = rgb8.height() as i32;
        self.raw = rgb8.into_raw();
    }

    pub fn gl_load(&mut self) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width,
                self.height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &self.raw[0] as *const u8 as *const std::ffi::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            self.loaded = true
        }
    }

    pub fn use_texture(&mut self) {
        unsafe {
            if self.loaded == false {
                self.gl_load();
                self.raw.clear();
                self.raw.shrink_to_fit();
            }

            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
