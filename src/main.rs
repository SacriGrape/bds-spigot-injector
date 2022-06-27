extern crate core;

use injrs::inject_windows::InjectorExt;
use injrs::process_windows::Process;
use std::process::Command;

fn main() {
    println!("[Spigot Injector INFO] Spawning BDS...");
    let id = Command::new("./bedrock_server.exe")
        .spawn()
        .expect("Failed to spawn BDS")
        .id();

    let bds_process = Process::from_pid(id).unwrap();
    println!("[Spigot Injector INFO] BDS Spawned!!! Injecting...");
    bds_process
        .inject("./spigot_loader.dll")
        .expect("Failed to inject DLL");

    println!("[Spigot Injector] INJECTION COMPLETE!!! Injector is done!!");
}
