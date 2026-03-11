fn main() {
    let mut args = std::env::args();
    args.next(); // skip program name

    let Some(arg) = args.next() else {
        println!("You must provide a number of users, disks, and printers.");
        return;
    };

    // Get number of users
    let num_users:i32 = match arg.trim().parse::<i32>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of users");
            return;
        }
    };

    // Get number of disks
    let num_disks:i32 = match arg.trim().parse::<i32>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of disks");
            return;
        }
    };

    // Get number of printers
    let num_printers:i32 = match arg.trim().parse::<i32>().ok() {
        Some(num) => num,
        None => {
            println!("Failed to parse number of printers");
            return;
        }
    };

    println!("Starting 141 OS Simulation with {num_users} users, {num_disks} disks, and {num_printers} printers");
}
