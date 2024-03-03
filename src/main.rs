use std::ffi::c_void;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETMOUSETRAILS,
    SPI_SETMOUSETRAILS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Target mouse trails configuration value
    #[arg(short, long, default_value_t = 2)]
    mouse_trails_target: u32,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args: Args = Args::parse();

    if args.debug {
        println!(
            "Monitoring for mouse trail changes (target = {}).",
            args.mouse_trails_target
        );
    } else {
        unsafe {
            match FreeConsole() {
                Err(error) => {
                    eprintln!("Failed to detach from console: {:?}", error);
                }
                _ => (),
            }
        }
    }

    loop {
        unsafe {
            let mut mouse_trails_current: u32 = 0;
            match SystemParametersInfoW(
                SPI_GETMOUSETRAILS,
                0,
                Some(&mut mouse_trails_current as *mut _ as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            ) {
                Err(error) => {
                    eprintln!("Failed to get current mouse trails setting: {:?}", error);
                    continue;
                }
                _ => (),
            }

            if mouse_trails_current != args.mouse_trails_target {
                if args.debug {
                    println!(
                        "Detected mouse trail change (current = {}, target = {}).",
                        mouse_trails_current, args.mouse_trails_target
                    )
                }
                match SystemParametersInfoW(
                    SPI_SETMOUSETRAILS,
                    args.mouse_trails_target,
                    None,
                    SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
                ) {
                    Err(error) => {
                        eprintln!("Failed to set mouse trails setting: {:?}", error);
                        continue;
                    }
                    _ => (),
                }
            }
        }

        sleep(Duration::from_secs(1))
    }
}
