use libnx_rs::sys::*;
use std::{ptr, result::Result};

// mod app;

fn main() {
    // let app = app::TemplateApp::default();
    // let native_options = eframe::NativeOptions::default();
    // eframe::run_native(Box::new(app), native_options);

    // Initialize console interface
    // Using NULL as argument tells the console library to use the internal console structure as current one
    unsafe { consoleInit(ptr::null_mut()) };

    // Move the cursor to row 16 and column 20 and then print "Hello World!"
    // To move the cursor you have to print `\x1b[r;cH`, where `r` and `c` are respectively the row and column where you want your cursor to move
    println!("\x1b[16;20HHello World!");

    while unsafe { appletMainLoop() } {
        // Get the identifier of the last pressed key
        let key = unsafe {
            // Scan all the inputs. This should be done once for each frame
            hidScanInput();

            // `hidKeysDown` returns information about which buttons have been just pressed (and they weren't in the previous frame)
            hidKeysDown(HidControllerID_CONTROLLER_P1_AUTO) as u32
        };

        // Break the main loop if the last pressed key was (+)
        if let HidControllerKeys_KEY_PLUS = key {
            break;
        }

        // Print console output
        unsafe { consoleUpdate(ptr::null_mut()) };
    }

    // Exit console and return to `hbmenu` in an orderly fashion
    unsafe { consoleExit(ptr::null_mut()) };
}
