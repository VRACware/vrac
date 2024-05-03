use winit::{event_loop::EventLoop, window::Window};
use vulkano_util::{context::VulkanoContext, window::VulkanoWindows};

pub use winit::event_loop::ControlFlow;
pub use winit::window::WindowId;
pub use vulkano_util::window::WindowDescriptor;
pub use vulkano_util::context::VulkanoConfig;

pub struct Vrac<State> {
    event_loop: EventLoop<()>,
    context: VulkanoContext,
    windows: VulkanoWindows,

    state: Box<State>,
}

impl<State> Vrac<State> {
    /// Initializes Vrac with a pointer to your State struct, with default Vulkan configuration
    pub fn init(state: Box<State>) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let context = VulkanoContext::new(VulkanoConfig::default());
        let windows = VulkanoWindows::default();
        Self {
            event_loop,
            context,
            windows,

            state,
        }
    }
    
    /// Initializes Vrac with a pointer to your State struct, with the given Vulkan configuration.
    pub fn init_config(state: Box<State>, config: VulkanoConfig) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let context = VulkanoContext::new(config);
        let windows = VulkanoWindows::default();
        Self {
            event_loop,
            context,
            windows,

            state,
        }
    }

    /// Modifies the event loop control flow (default is `Wait`).
    pub fn control_flow(&mut self, cf: ControlFlow) {
        self.event_loop.set_control_flow(cf);
    }
    
    /// Creates a new window, returns a pointer to it.
    /// The first window created is the primary window. The primary window is usually responsible for terminating the full render target (all windows) when it is closed.
    pub fn add_window(&mut self, wd: WindowDescriptor) -> WindowId {
        self.windows.create_window(
            &self.event_loop,
            &self.context,
            &wd,
            |_| {},
        )
    }

    /// Gets a window from its id.
    pub fn get_window(&self, id: WindowId) -> &Window {
        self.windows.get_window(id).unwrap()
    }

    /// Closes an existing window.
    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.remove_renderer(id);
    }

    pub fn run(&mut self) {
        self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    #[cfg(any(target_os = "windows", target_os = "macos"))]
                    println!("The close button of a window (id: {}, title: \"{}\") was pressed. Closing this window.", u64::from(window_id), w.get_window(window_id).title());
    
                    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                    println!("The close button of a window (id: {}) was pressed. Closing this window.", u64::from(window_id));
    
    
                    // Several windows
                    if window_id==w_primary { elwt.exit(); }
                    w.remove_window(window_id);
    
                    // // Single window
                    // elwt.exit();
                },
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    .. // window_id,
                } => {
                    // w.get_window(window_id).request_redraw();
                    events_count+=1;
                    if events_count>3 {
                        println!("Exiting the event loop");
                        elwt.exit();
                    }
                },
                _ => ()
            }
        }).unwrap();
    }
}

// Widgets will also have generics