use colored::*;
use chrono::*;

fn main() {
    let user = std::env::var("USER")
        .expect("USER variable does not exist");

    let time: (String, String) = {
        let t = offset::Local::now();
        let hour = t.hour();
        let full_time = {
            if t.hour() < 10 && t.minute() < 10 { format!("0{}:0{}", t.hour(), t.minute()) }
            else if t.hour() < 10 && t.minute() >= 10 { format!("0{}:{}", t.hour(), t.minute()) }
            else if t.hour() >= 10 && t.minute() < 10 { format!("{}:0{}", t.hour(), t.minute()) }
            else { format!("{}:{}", t.hour(), t.minute()) }
        };

        let message = {
            if hour > 20 && hour < 06 { format!("{}", "Good Night".bold().blue()) }
            else if hour >= 06 && hour < 12 { format!("{}", "Good Morning".bold().bright_yellow()) }
            else if hour >= 12 && hour < 18 { format!("{}", "Good Afternoon".bold().cyan()) }
            else { format!("{}", "Good Evening".bold().bright_blue()) }
        };

        (message, full_time)
    };

    println!("{} {}, it is currently {}", time.0, user.bold().red(), time.1.bold().green());
}
