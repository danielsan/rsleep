use indicatif::{ProgressBar, ProgressStyle};
use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <seconds>", args[0]);
        eprintln!("Example: {} 5.5", args[0]);
        std::process::exit(1);
    }
    
    let seconds: f64 = match args[1].parse() {
        Ok(num) => {
            if num < 0.0 {
                eprintln!("Error: Duration must be positive");
                std::process::exit(1);
            }
            num
        },
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            std::process::exit(1);
        }
    };
    
    // Calculate total steps (4 steps per second for 0.25 second intervals)
    let total_steps = (seconds * 4.0).ceil() as u64;
    
    let pb = ProgressBar::new(total_steps);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Show the progress bar immediately at 0
    pb.set_position(0);

    for i in 0..total_steps {
        thread::sleep(Duration::from_millis(250)); // 0.25 seconds
        pb.inc(1);
        
        // Check if we've reached the exact duration
        let elapsed = (i + 1) as f64 * 0.25;
        if elapsed >= seconds {
            break;
        }
    }
    pb.finish_with_message("Done");
}