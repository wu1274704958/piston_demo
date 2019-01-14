extern crate glutin;
extern crate gl;
extern crate input;
extern crate window;
extern crate shader_version;
extern crate piston_window;

use glutin::GlContext;

use std::collections::VecDeque;
use shader_version::OpenGL;
use glutin::GlRequest;
use piston_window::PistonWindow;


#[derive(Clone)]
pub struct WindowSettings {
    title: String,
    size: (u32,u32),
    samples: u8,
    fullscreen: bool,
    exit_on_esc: bool,
    vsync: bool,
    opengl: Option<OpenGL>,
    srgb: bool,
    resizable: bool,
    decorated: bool,
    controllers: bool,
    transparent: bool,
    always_on_top : bool,
}

impl WindowSettings {
    /// Creates window settings with defaults.
    ///
    /// - samples: 0
    /// - fullscreen: false
    /// - exit_on_esc: false
    /// - vsync: false
    /// - srgb: true
    /// - resizable: true
    /// - decorated: true
    /// - controllers: true
    pub fn new<T: Into<String>>(title: T, size: (u32,u32)) -> WindowSettings {
        WindowSettings {
            title: title.into(),
            size: size.into(),
            samples: 0,
            fullscreen: false,
            exit_on_esc: false,
            vsync: false,
            opengl: None,
            srgb: true,
            resizable: true,
            decorated: true,
            controllers: true,
            transparent: false,
            always_on_top : false
        }
    }

    pub fn build(&self) ->  Result< PistonWindow<glutin_window::GlutinWindow>,String> {
        create_window(self)
    }

    /// Gets the title of built windows.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Sets the title of built windows.
    pub fn set_title(&mut self, value: String) {
        self.title = value;
    }

    /// Sets the title of built windows.
    ///
    /// This method moves the current window data,
    /// unlike [`set_title()`](#method.set_title),
    /// so that it can be used in method chaining.
    pub fn title(mut self, value: String) -> Self {
        self.set_title(value);
        self
    }

    /// Gets the size of built windows.
    pub fn get_size(&self) -> (u32,u32) {
        self.size
    }

    /// Sets the size of built windows.
    pub fn set_size(&mut self, value: (u32,u32)) {
        self.size = value;
    }

    /// Sets the size of built windows.
    ///
    /// This method moves the current window data,
    /// unlike [`set_size()`](#method.set_size),
    /// so that it can be used in method chaining.
    pub fn size(mut self, value: (u32,u32)) -> Self {
        self.set_size(value);
        self
    }

    /// Gets whether built windows will be fullscreen.
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Sets whether built windows will be fullscreen.
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }

    /// Sets whether built windows will be fullscreen.
    ///
    /// This method moves the current window data,
    /// unlike [`set_fullscreen()`](#method.set_fullscreen),
    /// so that it can be used in method chaining.
    pub fn fullscreen(mut self, value: bool) -> Self {
        self.set_fullscreen(value);
        self
    }

    /// Gets whether built windows should exit when the Esc key is pressed.
    pub fn get_exit_on_esc(&self) -> bool {
        self.exit_on_esc
    }

    /// Sets whether built windows should exit when the Esc key is pressed.
    pub fn set_exit_on_esc(&mut self, value: bool) {
        self.exit_on_esc = value;
    }

    /// Sets whether built windows should exit when the Esc key is pressed.
    ///
    /// This method moves the current window data,
    /// unlike [`set_exit_on_esc()`](#method.set_exit_on_esc),
    /// so that it can be used in method chaining.
    pub fn exit_on_esc(mut self, value: bool) -> Self {
        self.set_exit_on_esc(value);
        self
    }

    /// Gets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn get_samples(&self) -> u8 {
        self.samples
    }

    /// Sets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    pub fn set_samples(&mut self, value: u8) {
        self.samples = value;
    }

    /// Sets the number of samples to use for anti-aliasing.
    ///
    /// See https://en.wikipedia.org/wiki/Multisample_anti-aliasing
    /// for more information.
    ///
    /// This method moves the current window data,
    /// unlike [`set_samples()`](#method.set_samples)
    /// so that it can be used in method chaining.
    pub fn samples(mut self, value: u8) -> Self {
        self.set_samples(value);
        self
    }

    /// Gets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn get_vsync(&self) -> bool {
        self.vsync
    }

    /// Sets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    /// Sets whether built windows should use vsync.
    ///
    /// See https://en.wikipedia.org/wiki/Screen_tearing for more information
    /// about vsync.
    ///
    /// This method moves the current window data,
    /// unlike [`set_vsync()`](#method.set_vsync),
    /// so that it can be used in method chaining.
    pub fn vsync(mut self, value: bool) -> Self {
        self.set_vsync(value);
        self
    }

    /// Gets the OpenGL version of built windows.
    ///
    /// If None is returned, the default OpenGL version is being used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
    /// For more information about the OpenGL setting, see the
    /// [`OpenGLWindow`](trait.OpenGLWindow.html) trait.
    pub fn get_maybe_opengl(&self) -> Option<OpenGL> {
        self.opengl
    }

    /// Sets OpenGL version of built windows.
    ///
    /// If None is passed, the default OpenGL version is used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
    /// For more information about the OpenGL setting, see the
    /// [`OpenGLWindow`](trait.OpenGLWindow.html) trait.
    pub fn set_maybe_opengl(&mut self, value: Option<OpenGL>) {
        self.opengl = value;
    }

    /// Sets OpenGL version of built windows.
    ///
    /// If None is passed, the default OpenGL version is used. This
    /// is often a forward compatible version of OpenGL::V3_2 or
    /// higher that works with newer versions of graphics libraries.
    ///
    /// For more information about the OpenGL setting, see the
    /// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
    ///
    /// This method moves the current window data,
    /// unlike [`set_maybe_opengl()`](#method.set_maybe_opengl),
    /// so that it can be used in method chaining.
    pub fn maybe_opengl(mut self, value: Option<OpenGL>) -> Self {
        self.set_maybe_opengl(value);
        self
    }

    /// Sets OpenGL version of built windows.
    ///
    /// For setting the OpenGL version back to default, see
    /// [`set_maybe_opengl()`](#method.set_maybe_opengl).
    ///
    /// For more information about the opengl setting, see the
    /// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
    pub fn set_opengl(&mut self, value: OpenGL) {
        self.opengl = Some(value);
    }

    /// Sets the OpenGL version of built windows.
    ///
    /// For setting the OpenGL version back to default, see
    /// [`maybe_opengl()`](#method.maybe_opengl).
    ///
    /// For more information about the opengl setting, see the
    /// [`OpenGLWindow`](./trait.OpenGLWindow.html) trait.
    ///
    /// This method moves the current window data,
    /// unlike [`set_opengl()`](#method.set_opengl),
    /// so that it can be used in method chaining.
    pub fn opengl(mut self, value: OpenGL) -> Self {
        self.set_opengl(value);
        self
    }

    /// Gets whether built windows should use hardware accelerated color conversion.
    ///
    /// If true, the graphics hardware uses customized circuitry
    /// to convert colors from sRGB to linear color space in graphics
    /// shaders, and then converts pixel fragments back to sRGB
    /// depending on the color format of the frame buffer. This feature
    /// is supported by most graphics hardware and set to true by
    /// default.
    ///
    /// See https://en.wikipedia.org/wiki/SRGB for more information.
    pub fn get_srgb(&self) -> bool {
        self.srgb
    }

    /// Sets whether built windows should use hardware accelerated color conversion.
    ///
    /// See [`get_srgb()`](#method.get_srgb) for more information about
    /// the srgb setting.
    pub fn set_srgb(&mut self, value: bool) {
        self.srgb = value;
    }

    /// Sets whether built windows should use hardware accelerated color conversion.
    ///
    /// See [`get_srgb()`](#method.get_srgb) for more information about
    /// the srgb setting.
    ///
    /// This method moves the current window data,
    /// unlike [`set_srgb()`](#method.set_srgb),
    /// so that it can be used in method chaining.
    pub fn srgb(mut self, value: bool) -> Self {
        self.set_srgb(value);
        self
    }

    /// Gets whether built windows should be resizable.
    pub fn get_resizable(&self) -> bool {
        self.resizable
    }

    /// Sets whether built windows should be resizable.
    pub fn set_resizable(&mut self, value: bool) {
        self.resizable = value;
    }

    /// Sets whether built windows should be resizable.
    ///
    /// This method moves the current window data,
    /// unlike [`set_resizable()`](#method.set_resizable),
    /// so that it can be used in method chaining.
    pub fn resizable(mut self, value: bool) -> Self {
        self.set_resizable(value);
        self
    }

    /// Gets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    pub fn get_decorated(&self) -> bool {
        self.decorated
    }

    /// Sets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    pub fn set_decorated(&mut self, value: bool) {
        self.decorated = value;
    }

    /// Sets whether built windows should be decorated.
    ///
    /// Decoration on a window refers to the Operating System's
    /// header above the window, and the window border.
    ///
    /// For more information, see
    /// https://en.wikipedia.org/wiki/Window_decoration
    ///
    /// This method moves the current window data,
    /// unlike [`set_decorated()`](#method.set_decorated),
    /// so that it can be used in method chaining.
    pub fn decorated(mut self, value: bool) -> Self {
        self.set_decorated(value);
        self
    }

    /// Gets whether built windows should listen to controller input.
    pub fn get_controllers(&self) -> bool {
        self.controllers
    }

    /// Sets whether built windows should listen to controller input.
    pub fn set_controllers(&mut self, value: bool) {
        self.controllers = value;
    }

    /// Sets whether build windows should listen to controller input.
    ///
    /// This method moves the current window data,
    /// unlike [`set_controllers()`](#method.set_controllers),
    /// so that it can be used in method chaining.
    pub fn controllers(mut self, value: bool) -> Self {
        self.set_controllers(value);
        self
    }
    /// Gets whether the background of the window should be transparent.
    pub fn set_transparent(&mut self, value: bool) {
        self.transparent = value;
    }
    /// Sets whether the background of the window should be transparent.
    pub fn get_transparent(&self) -> bool {
        self.transparent
    }
    /// Sets whether the background of the window should be transparent.
    pub fn transparent(mut self, value: bool) -> Self {
        self.set_transparent(value);
        self
    }

    /// Gets whether the window should be always on top.
    pub fn set_always_on_top(&mut self, value: bool) {
        self.always_on_top = value;
    }
    /// Sets whether the window should be always on top.
    pub fn get_always_on_top(&self) -> bool {
        self.always_on_top
    }

    /// Sets whether the window should be always on top.
    pub fn always_on_top(mut self, value: bool) -> Self {
        self.set_always_on_top(value);
        self
    }
}


fn window_builder_from_settings(settings: &WindowSettings) -> glutin::WindowBuilder {
    let size = settings.get_size();
    let transparent = settings.get_transparent();
    let mut builder = glutin::WindowBuilder::new()
        .with_dimensions((size.0, size.1).into())
        .with_decorations(settings.get_decorated())
        .with_multitouch()
        .with_title(settings.get_title())
        .with_resizable(settings.get_resizable())
        .with_transparency(transparent)
        .with_always_on_top(settings.get_always_on_top());

    if settings.get_fullscreen() {
        let events_loop = glutin::EventsLoop::new();
        builder = builder.with_fullscreen(Some(events_loop.get_primary_monitor()));
    }
    builder
}


fn context_builder_from_settings(settings: &WindowSettings) -> glutin::ContextBuilder {
    let opengl = settings.get_maybe_opengl().unwrap_or(OpenGL::V3_2);
    let (major, minor) = opengl.get_major_minor();
    let mut builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::GlThenGles {
            opengl_version: (major as u8, minor as u8),
            opengles_version: (major as u8, minor as u8),
        })
        .with_srgb(settings.get_srgb());
    let samples = settings.get_samples();
    if settings.get_vsync() {
        builder = builder.with_vsync(true);
    }
    if samples != 0 {
        builder = builder.with_multisampling(samples as u16);
    }
    builder}


use std::mem::{forget,transmute};
use std::ptr::swap;


pub struct GlutinWindow {
    /// The window.
    pub window: glutin::GlWindow,
    // The back-end does not remember the title.
    title: String,
    exit_on_esc: bool,
    should_close: bool,
    // Used to fake capturing of cursor,
    // to get relative mouse events.
    is_capturing_cursor: bool,
    // Stores the last known cursor position.
    last_cursor_pos: Option<[f64; 2]>,
    // Stores relative coordinates to emit on next poll.
    mouse_relative: Option<(f64, f64)>,
    // Used to emit cursor event after enter/leave.
    cursor_pos: Option<[f64; 2]>,
    // Polls events from window.
    events_loop: glutin::EventsLoop,
    // Stores list of events ready for processing.
    events: VecDeque<glutin::Event>,
}

fn create_glutin_window(settings: &WindowSettings) -> Result<glutin_window::GlutinWindow,String>
{
    use std::error::Error;
    use glutin::ContextError;
    let events_loop = glutin::EventsLoop::new();
    let title = settings.get_title();
    let exit_on_esc = settings.get_exit_on_esc();
    let window = glutin::GlWindow::new(
        window_builder_from_settings(&settings),
        context_builder_from_settings(&settings),
        &events_loop
    );
    let window = match window {
        Ok(window) => window,
        Err(_) => {
            glutin::GlWindow::new(
                window_builder_from_settings(&settings),
                context_builder_from_settings(&settings.clone().samples(0)),
                &events_loop
            ).map_err(|e| format!("{}", e))?
        }
    };
    unsafe {
        window.make_current().map_err(|e|
            // This can be simplified in next version of Glutin.
            match e {
                ContextError::OsError(err) => {
                    err
                }
                ContextError::IoError(ref err) => {
                    String::from(err.description())
                }
                ContextError::ContextLost => {
                    String::from("Context lost")
                }
            }
        )?;
    }
    // Load the OpenGL function pointers.
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let window:glutin_window::GlutinWindow = unsafe {
        let w :GlutinWindow = GlutinWindow {
            window,
            title,
            exit_on_esc,
            should_close: false,
            cursor_pos: None,
            is_capturing_cursor: false,
            last_cursor_pos: None,
            mouse_relative: None,
            events_loop,
            events: VecDeque::new(),
        };

        transmute(w)
    };

    Ok(window)
}

pub fn create_window(settings: &WindowSettings) -> Result< PistonWindow<glutin_window::GlutinWindow>,String>
{
    let glutin_window = create_glutin_window(settings);

    let settings = settings.clone().srgb(true);

    // Use OpenGL 3.2 by default, because this is what window backends
    // usually do.
    let opengl = settings.get_maybe_opengl().unwrap_or(OpenGL::V3_2);
    let samples = settings.get_samples();

    Ok(PistonWindow::new(opengl, samples, glutin_window?))
}