use std::fs::OpenOptions;
use std::io::Write;
use std::{env, fs};
use std::time::{Duration, Instant};
use std::thread::sleep;
use sysinfo::{ProcessesToUpdate, Signal, System};
use toml::Value;
use std::ffi::OsStr;

fn main() -> std::io::Result<()> {
    let mut system = System::new_all();
    let mut high_cpu_usage_start: Option<Instant> = None;
    let mut pid = None;

    // Get the config file path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a config file path");
        std::process::exit(1);
    }

    let config_file_path = &args[1];

    println!("Loading config file from {}", config_file_path);

    // Read the config file
    let config_file_content = fs::read_to_string(config_file_path)
        .expect("Could not read config file");

    // Parse the TOML content
    let config: Value = config_file_content.parse()
        .expect("Could not parse config file");

    // Get the process name from the config
    let process_name = config.get("process_name")
        .and_then(Value::as_str)
        .expect("Could not find 'process_name' in config file");

    let os_process_name: &OsStr = OsStr::new(process_name);

    let stats_path = config.get("stats_path")
        .and_then(Value::as_str)
        .expect("Could not find 'stats_path' in config file");

    let mut csv_file = OpenOptions::new()
        .append(true) // Open in append mode
        .create(true) // Create the file if it doesn't exist
        .open(stats_path)?;

    println!("Length of file, {}", csv_file.metadata()?.len());

    // Check if stats is empty, if so add headers
    if csv_file.metadata()?.len() == 0
    {
        writeln!(csv_file, "date time, process_name")?;
    }
    
    // Log that we are monitoring the process
    println!("process-bonk is now monitoring the {} process", process_name);

    loop {
        // Set pid if it is not set yet
        if pid.is_none() || !system.process(pid.unwrap()).is_some() {
            if let Some(process) = system.processes_by_exact_name(os_process_name).next() {
                pid = Some(process.pid());
                println!("Found {} process with PID {}", process_name, pid.unwrap());
            }
        }

        // Update the specific process information in sysinfo
        system.refresh_processes(ProcessesToUpdate::Some(&[pid.unwrap()]));
        
        if let Some(process) = system.process(pid.unwrap()) {

            let cpu_usage = process.cpu_usage();

            if cpu_usage >= 90.0 {
                match high_cpu_usage_start {
                    Some(start_time) if start_time.elapsed() > Duration::from_secs(60*15) => {
                        // If the process has been using 100% CPU for more than 5 minutes, kill it
                        println!("{} has been using lots of CPU for more than 2 minutes, bonking it", process_name);
                        process.kill_with(Signal::Kill);

                        // Append to stats_path CSV file that we bonked the process in format: date time, process_name. Use standard library
                        let row = format!("{}, {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), process_name);
                        // This line does not append to the file, it overwrites it. Use append instead of write
                        writeln!(csv_file, "{}", row)?;
                        
                        // User has been notified, reset the tracking
                        high_cpu_usage_start = None;
                        pid = None;
                    }
                    None => {
                        // Start tracking when the process started using 100% CPU
                        println!("{} started using lots of CPU at {}%, will bonk if not behaving", process_name, cpu_usage);
                        high_cpu_usage_start = Some(Instant::now());
                    }
                    _ => {
                        println!("{} is still using lots of CPU at {}% for {} minutes", process_name, cpu_usage, high_cpu_usage_start.unwrap().elapsed().as_secs() / 60);
                    }
                }
            } else {
                // If the process is not using 100% CPU anymore, reset tracking
                high_cpu_usage_start = None;
            }
        } else {
            println!("{} process not found, sleeping for 60 seconds", process_name);
        }
        // Sleep for a while before the next iteration
        sleep(Duration::from_secs(60));
    }
}