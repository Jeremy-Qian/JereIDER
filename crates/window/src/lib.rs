/// Sets the document-edited state on the native macOS window.
/// When `edited` is `true`, a small dark dot appears inside the red close button,
/// indicating unsaved changes — just like Notes, TextEdit, and every other native app.
#[cfg(target_os = "macos")]
pub fn set_document_edited(frame: &eframe::Frame, edited: bool) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(handle) = frame.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return;
    };

    let ns_view = appkit.ns_view.as_ptr() as *mut AnyObject;

    unsafe {
        let ns_window: *mut AnyObject = msg_send![ns_view, window];
        if ns_window.is_null() {
            return;
        }
        let _: () = msg_send![ns_window, setDocumentEdited: edited];
    }
}

#[cfg(target_os = "macos")]
pub fn position_traffic_lights(frame: &eframe::Frame, offset_x: f64, offset_y: f64) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use objc2_foundation::{NSPoint, NSRect};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(handle) = frame.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return;
    };

    let ns_view = appkit.ns_view.as_ptr() as *mut AnyObject;

    unsafe {
        // Get the NSWindow from the NSView
        let ns_window: *mut AnyObject = msg_send![ns_view, window];
        if ns_window.is_null() {
            return;
        }

        // NSWindowButton constants: Close=0, Miniaturize=1, Zoom=2
        for tag in 0i64..3 {
            let button: *mut AnyObject = msg_send![ns_window, standardWindowButton: tag];
            if button.is_null() {
                continue;
            }

            // Read current frame
            let frame: NSRect = msg_send![button, frame];

            // Shift the button by `offset_x` and `offset_y`
            let new_frame = NSRect {
                origin: NSPoint {
                    x: frame.origin.x + offset_x,
                    y: frame.origin.y + offset_y,
                },
                size: frame.size,
            };
            let _: () = msg_send![button, setFrame: new_frame];
        }
    }
}
