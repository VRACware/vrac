use vrac::{ControlFlow, Vrac, WindowDescriptor};
use vulkano_util::window::WindowMode;
use winit::event::{Event, WindowEvent};

fn main() {
    let mut vrac = Vrac::init();
    println!("Creating window context");
    vrac.control_flow(ControlFlow::Poll);

    println!("Adding 4 different windows");
    // let _w_simple = vrac.add_window(WindowDescriptor::default());
    let w_primary = vrac.add_window(
        WindowDescriptor {
            title: "Primary window - borderless fullscreen".to_owned(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        }
    );
    let _w_1 = vrac.add_window(
        WindowDescriptor {
            title: "Window 1 _ unresizable and with invisible cursor".to_owned(),
            resizable: false,
            cursor_visible: false,
            ..Default::default()
        }
    );
    let _w_2 = vrac.add_window(
        WindowDescriptor {
            title: "Window 2 - undecorated".to_owned(),
            decorations: false,
            ..Default::default()
        }
    );
    let _w_3 = vrac.add_window(
        WindowDescriptor {
            title: "Window 3 - Transparent".to_owned(),
            transparent: true,
            ..Default::default()
        }
    );


    println!("Running an event loop");
    let mut events_count: u32 = 0;
    vrac.event_loop.run(move |event, elwt| {
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