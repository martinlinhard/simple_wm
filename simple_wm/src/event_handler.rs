pub struct EventHandler;
use crate::data::*;
use crate::window_manager::WindowManager;
use std::cmp::max;
use std::mem::MaybeUninit;
use x11::xlib;
const BORDER: i32 = 3;
const MOUSEMASK: u32 =
    (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::PointerMotionMask) as u32;
use crate::event_codes::*;

impl EventHandler {
    pub fn on_configure_request(wm: &mut WindowManager, event: xlib::XEvent) {
        let conf_event = xlib::XConfigureRequestEvent::from(event);
        let mut changes = xlib::XWindowChanges {
            x: conf_event.x,
            y: conf_event.y,
            width: wm.window_system.width - (2 * BORDER),
            height: wm.window_system.height - (2 * BORDER),
            border_width: BORDER,
            sibling: conf_event.above,
            stack_mode: conf_event.detail,
        };

        if let Some(frame) = wm.clients.get(&conf_event.window) {
            unsafe {
                xlib::XConfigureWindow(
                    wm.window_system.display,
                    *frame,
                    conf_event.value_mask as u32,
                    &mut changes,
                );
            }
        }

        unsafe {
            xlib::XConfigureWindow(
                wm.window_system.display,
                conf_event.window,
                conf_event.value_mask as u32,
                &mut changes,
            );
        }
    }
    pub fn on_map_request(wm: &mut WindowManager, event: xlib::XEvent) {
        let mut map_event = xlib::XMapRequestEvent::from(event);
        frame_window(&mut map_event.window, wm);
        unsafe {
            xlib::XMapWindow(wm.window_system.display, map_event.window);
        }
    }
    pub fn on_unmap_notify(wm: &mut WindowManager, window: *mut xlib::Window) {
        unsafe {
            let frame = wm.clients.get(&*window).unwrap();
            xlib::XUnmapWindow(wm.window_system.display, *frame);
            xlib::XReparentWindow(
                wm.window_system.display,
                *window,
                wm.window_system.root,
                0,
                0,
            );
            xlib::XRemoveFromSaveSet(wm.window_system.display, *window);
            xlib::XDestroyWindow(wm.window_system.display, *frame);
            wm.clients.remove(&*window);
        }
    }

    ///Mouse was clicked
    pub fn on_button_press(wm: &mut WindowManager, event: xlib::XEvent) {
        let button_press_event = xlib::XButtonEvent::from(event);
        if let Some(frame) = wm.clients.get(&button_press_event.window) {
            //Initial cursor position
            wm.drag_start_position =
                Position::new(button_press_event.x_root, button_press_event.y_root);

            //Initial window information
            let mut returned_window = unsafe {
                let returned_window: xlib::Window = MaybeUninit::uninit().assume_init();
                returned_window
            };

            let mut x: i32 = 0;
            let mut y: i32 = 0;

            let mut width: u32 = 0;
            let mut height: u32 = 0;
            let mut border_width: u32 = 0;
            let mut depth: u32 = 0;

            unsafe {
                xlib::XGetGeometry(
                    wm.window_system.display,
                    *frame,
                    &mut returned_window,
                    &mut x,
                    &mut y,
                    &mut width,
                    &mut height,
                    &mut border_width,
                    &mut depth,
                );
            }

            wm.drag_start_frame_position = Position::new(x, y);
            wm.drag_start_frame_size = Size::new(width, height);

            //Raise clicked window to top.
            unsafe {
                xlib::XRaiseWindow(wm.window_system.display, *frame);
            }
        }
    }

    pub fn on_motion_notify(wm: &mut WindowManager, event: xlib::XEvent) {
        let motion_event: xlib::XMotionEvent = xlib::XMotionEvent::from(event);
        if let Some(frame) = wm.clients.get(&motion_event.window) {
            let current_pos = Position::new(motion_event.x_root, motion_event.y_root);
            let delta: Vector2D = current_pos - wm.drag_start_position.clone();
            // alt + left button: Move window.
            if motion_event.state & xlib::Button1Mask == 256 {
                let dest_frame_pos = wm.drag_start_frame_position.add_vec(&delta);

                unsafe {
                    xlib::XMoveWindow(
                        wm.window_system.display,
                        *frame,
                        dest_frame_pos.x,
                        dest_frame_pos.y,
                    );
                }
            } else if motion_event.state & xlib::Button3Mask == 1024 {
                let coords_x = motion_event.x + (wm.drag_start_frame_size.width as i32);
                let coords_y = motion_event.y + (wm.drag_start_frame_size.height as i32);
                //let coords_x = 100;
                //let coords_y = 100;
                /*let size_delta = Vector2D::new(
                    max(delta.x, -(wm.drag_start_frame_size.width as i32)),
                    max(delta.y, -(wm.drag_start_frame_size.height as i32)),
                );
                let dest_frame_size = wm.drag_start_frame_size.add_vec(&size_delta);
                unsafe {
                    xlib::XResizeWindow(
                        wm.window_system.display,
                        *frame,
                        dest_frame_size.width,
                        dest_frame_size.height,
                    );
                    xlib::XResizeWindow(
                        wm.window_system.display,
                        motion_event.window,
                        dest_frame_size.width,
                        dest_frame_size.height,
                    );
                }*/
                //1. move cursor to the bottom right corner
                unsafe {
                    let mut event: xlib::XEvent = MaybeUninit::uninit().assume_init();
                    let frame = wm.clients.get(&motion_event.window).unwrap();
                    if xlib::XGrabPointer(
                        wm.window_system.display,
                        wm.window_system.root,
                        xlib::False,
                        MOUSEMASK,
                        xlib::GrabModeAsync,
                        xlib::GrabModeAsync,
                        xlib::NoValue as u64,
                        xlib::NoValue as u64,
                        xlib::CurrentTime,
                    ) == xlib::GrabSuccess
                    {
                        xlib::XWarpPointer(
                            wm.window_system.display,
                            xlib::NoValue as u64,
                            motion_event.window,
                            0,
                            0,
                            0,
                            0,
                            coords_x,
                            coords_y,
                        );
                        while {
                            xlib::XMaskEvent(
                                wm.window_system.display,
                                (MOUSEMASK
                                    | xlib::ExposureMask as u32
                                    | xlib::SubstructureRedirectMask as u32)
                                    as i64,
                                &mut event,
                            );
                            let event_type = event.get_type();
                            match event_type as usize {
                                MotionNotify => {
                                    xlib::XSync(wm.window_system.display, xlib::False);
                                }
                                _ => (),
                            }
                            event_type != xlib::ButtonRelease
                        } {}
                        xlib::XWarpPointer(
                            wm.window_system.display,
                            xlib::NoValue as u64,
                            motion_event.window,
                            0,
                            0,
                            0,
                            0,
                            coords_x,
                            coords_y,
                        );
                        xlib::XUngrabPointer(wm.window_system.display, xlib::CurrentTime);
                        while xlib::XCheckMaskEvent(
                            wm.window_system.display,
                            xlib::EnterWindowMask,
                            &mut event,
                        ) > 0
                        {}
                    }
                }
            }
        }
    }

    pub fn on_key_press(wm: &mut WindowManager, event: xlib::XEvent) {
        let key_event: xlib::XKeyEvent = xlib::XKeyEvent::from(event);
        unsafe {
            if (key_event.state == xlib::Mod1Mask)
                && (key_event.keycode
                    == xlib::XKeysymToKeycode(wm.window_system.display, x11::keysym::XK_F4.into())
                        .into())
            {
                //close window
                xlib::XKillClient(wm.window_system.display, key_event.window);
            } else if (key_event.state == xlib::Mod4Mask)
                && (key_event.keycode
                    == xlib::XKeysymToKeycode(wm.window_system.display, x11::keysym::XK_T.into())
                        .into())
            {
                println!("here!");
            }
        }
    }
}

fn frame_window(window: *mut xlib::Window, wm: &mut WindowManager) {
    const BORDER_WIDTH: u32 = BORDER as u32;
    const BORDER_COLOR: u64 = 0xff0000;
    const BG_COLOR: u64 = 0x0000ff;

    let window_attributes: xlib::XWindowAttributes = unsafe {
        let mut window_attributes: xlib::XWindowAttributes = MaybeUninit::uninit().assume_init();
        xlib::XGetWindowAttributes(wm.window_system.display, *window, &mut window_attributes);
        window_attributes
    };

    let frame: xlib::Window = unsafe {
        let frame: xlib::Window = xlib::XCreateSimpleWindow(
            wm.window_system.display,
            wm.window_system.root,
            window_attributes.x,
            window_attributes.y,
            (wm.window_system.width - (2 * BORDER)) as u32,
            (wm.window_system.height - (2 * BORDER)) as u32,
            BORDER_WIDTH,
            BORDER_COLOR,
            BG_COLOR,
        );
        frame
    };

    unsafe {
        xlib::XSelectInput(
            wm.window_system.display,
            frame,
            xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask,
        );
        xlib::XAddToSaveSet(wm.window_system.display, *window);
        xlib::XReparentWindow(wm.window_system.display, *window, frame, 0, 0);
        xlib::XMapWindow(wm.window_system.display, frame);
        wm.add_client(*window, frame);
        grab_keys(wm, window);
    };
}

unsafe fn grab_keys(wm: &mut WindowManager, w: *mut xlib::Window) {
    xlib::XGrabButton(
        wm.window_system.display,
        xlib::Button1,
        xlib::Mod4Mask,
        *w,
        0,
        (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::ButtonMotionMask) as u32,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
        0,
        0,
    );
    //   b. Resize windows with alt + right button.
    xlib::XGrabButton(
        wm.window_system.display,
        xlib::Button3,
        xlib::Mod4Mask,
        *w,
        0,
        (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::ButtonMotionMask) as u32,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
        0,
        0,
    );
    //   c. Kill windows with alt + f4.
    xlib::XGrabKey(
        wm.window_system.display,
        xlib::XKeysymToKeycode(wm.window_system.display, x11::keysym::XK_F4.into()).into(),
        xlib::Mod1Mask,
        *w,
        0,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
    );

    xlib::XGrabKey(
        wm.window_system.display,
        xlib::XKeysymToKeycode(wm.window_system.display, x11::keysym::XK_T.into()).into(),
        xlib::Mod4Mask,
        *w,
        0,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
    );
    //   d. Switch windows with alt + tab.
}
