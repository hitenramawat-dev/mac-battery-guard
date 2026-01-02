use std::io::{BufReader};
use std::thread;
use std::time::Duration;
use std::u8;
use std::{process::Command};
use std::fs::{File};

use anyhow::{Result};
use rodio::{OutputStream};



struct Battery{
    threshold:u8,
}

impl Battery {
    fn new(threshold: u8) -> Self {
        Self { threshold }
    }

    fn get_battery_percentage(&self) -> Result<u8> {
        let output = Command::new("pmset")
            .arg("-g")
            .arg("batt")
            .output()?;

        let text = String::from_utf8(output.stdout)?;
        let chars: Vec<char> = text.chars().collect();


        let percent_index = chars
            .iter()
            .position(|&c| c == '%')
            .ok_or_else(|| anyhow::anyhow!("Could not find % symbol"))?;

    
        let start = chars[..percent_index]
            .iter()
            .rposition(|c| !c.is_ascii_digit())
            .map(|i| i + 1)
            .unwrap_or(0);

        let digits: String = chars[start..percent_index].iter().collect();
        let percentage = digits.parse::<u8>()?;

        Ok(percentage)
    }

    fn exceeded(&self) -> bool {
        match self.get_battery_percentage().ok() {
            Some(value) => {
                println!("Battery: {}%", value);
                value >= self.threshold
            }
            None => false,
        }
    }
}


struct Alarm {
    stream: OutputStream,
}


impl Alarm {
    fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to open audio stream");
        Self { stream }
    }

    fn play(&self, file_path: &str) {
        let file = BufReader::new(File::open(file_path).expect("Audio file not found"));
        let sink = rodio::play(&self.stream.mixer(), file).unwrap();
        sink.sleep_until_end();
    }
}



fn main() {
    let monitor = Battery::new(80);
    let alarm = Alarm::new();


    loop {
        if monitor.exceeded() {
            println!("Unplug charger");
            alarm.play("./DAMN.mp3");
            break;
        }

        thread::sleep(Duration::from_secs(60));
    }
   



}
