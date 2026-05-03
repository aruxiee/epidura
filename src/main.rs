use std::{env, ffi::OsStr, os::windows::ffi::OsStrExt};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows::Win32::System::Pipes::{CreateNamedPipeW, ConnectNamedPipe, PIPE_TYPE_BYTE, PIPE_WAIT};
use windows::Win32::Storage::FileSystem::{ReadFile, PIPE_ACCESS_DUPLEX};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SetWindowTextW};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: epidura.exe <PID>");
        return Ok(());
    }
    
    let _pid: u32 = args[1].parse()?; 
    let pipe_name: Vec<u16> = OsStr::new("\\\\.\\pipe\\SpoolerService\0").encode_wide().collect();

    unsafe {
        let h_pipe = CreateNamedPipeW(
            PCWSTR(pipe_name.as_ptr()),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_BYTE | PIPE_WAIT,
            1, 1024, 1024, 0, None,
        );

        if h_pipe == INVALID_HANDLE_VALUE {
            println!("[-] failed to create pipe. try running powershell as admin.");
            return Ok(());
        }

        println!("[*] pipe created. waiting for stager...");
        
        let _ = ConnectNamedPipe(h_pipe, None);

        let mut buffer = [0u8; 512];
        let mut bytes_read = 0u32;
        let _ = ReadFile(h_pipe, Some(&mut buffer), Some(&mut bytes_read), None);
        println!("[+] trigger received. attempting to manipulate notepad pid...");

        let hwnd = FindWindowW(w!("Notepad"), None);
        
        if hwnd.0 != 0 {
            let _ = SetWindowTextW(hwnd, w!("procinjection successful."));
            println!("[!] success: title has been changed.");
        } else {
            println!("[-] error: could not find a window with class 'notepad'.");
            println!("[-] make sure notepad is open on your desktop.");
        }

        let _ = CloseHandle(h_pipe);
    }

    println!("[+] injection complete.");
    
    Ok(())
}