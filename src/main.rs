use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};

// Import resource manager modules
mod disk_manager;
use disk_manager::DiskManager;
mod printer_manager;
use printer_manager::PrinterManager;

mod user;
use user::User;

fn get_args() -> (usize, usize, usize) {
    let mut args = std::env::args();
    args.next(); // skip program name

    let Some(arg) = args.next() else {
        println!("You must provide a number of users, disks, and printers.");
        std::process::exit(1);
    };

    // Get number of users
    let num_users:usize = match arg.trim().parse::<usize>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of users");
            std::process::exit(1);
        }
    };

    let Some(arg) = args.next() else {
        println!("You must provide a number of users, disks, and printers.");
        std::process::exit(1);
    };

    // Get number of disks
    let num_disks:usize = match arg.trim().parse::<usize>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of disks");
            std::process::exit(1);
        }
    };

    let Some(arg) = args.next() else {
        println!("You must provide a number of users, disks, and printers.");
        std::process::exit(1);
    };

    // Get number of printers
    let num_printers:usize = match arg.trim().parse::<usize>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of printers");
            std::process::exit(1);
        }
    };

    return (num_users, num_disks, num_printers);
}

fn main() {
    // Get number of Users, Disks, and Printers
    let (num_users, num_disks, num_printers) = get_args();
    println!("Starting 141 OS Simulation with {num_users} users, {num_disks} disks, and {num_printers} printers");

    // Create resource managers
    let disk_manager = Arc::new(RwLock::new(DiskManager::new(num_disks)));
    let printer_manager = Arc::new(Mutex::new(PrinterManager::new(num_printers)));

    // Begin user threads
    let mut user_handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..num_users {
        let mut user = User::new(
            i,
            Arc::clone(&disk_manager),
            Arc::clone(&printer_manager)
        );
        user_handles.push(thread::spawn(move || user.run()));
    }

    // Join user threads
    for user_handle in user_handles {
        user_handle.join().unwrap();
    }
}
