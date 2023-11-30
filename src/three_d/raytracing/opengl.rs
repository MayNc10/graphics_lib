use gl::types::{GLint, GLuint};

/// A struct representing a framebuffer. which can be used to draw to the screen
pub struct Framebuffer {
    fb_id: GLuint,
    tex_id: GLuint,
}

impl Framebuffer {
    /// Create a new framebuffer, given the image dimensions
    pub fn new(image_width: i32, image_height: i32) -> Framebuffer {
        let mut fb = 0;
        let mut tex = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

            // - position color buffer
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA32F, image_width, image_height);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Framebuffer { fb_id: fb, tex_id: tex }
    }
    /// Copy pixel data to the framebuffer, and then blit it to the screen
    pub unsafe fn draw(&mut self, image_width: i32, image_height: i32, dims: (i32, i32), data: *const f32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            //gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0,
                              image_width, image_height, gl::RGBA, gl::FLOAT,
                              data as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb_id);

            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fb_id);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);

            gl::BlitFramebuffer(0, 0, image_width, image_height, 0, 0, dims.0, dims.1,
                                gl::COLOR_BUFFER_BIT, gl::LINEAR);

        }
    }
}