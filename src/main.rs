use std::{thread::sleep, time::Duration};
use std::thread;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::collections::HashMap;

use notify_rust::Notification;
use tray_item::TrayItem;
use rodio::{Decoder, OutputStream, source::Source};
use serde::{Deserialize, Serialize};
use home;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MainConfig {
    config: HashMap<String, String>,
}

fn main() {
    let mut tray = TrayItem::new("remember-to-stand-tray", "").unwrap();
    let tray_icon_set = tray.set_icon("icon.png");

    if tray_icon_set.is_err() {
        println!("Error setting tray icon: {}", tray_icon_set.err().unwrap());
    }

    tray.add_label("Remember to stand").unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");

    thread::spawn(|| {
        let mut loop_count = 0;
        let mut stand_time_secs = 3600;
        let mut sit_time_secs = 3600;
        let mut custom_sit_msg = "Sit down!".to_string();
        let mut custom_stand_msg = "Stand up!".to_string();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let home_dir = home::home_dir().unwrap().as_path().to_owned();
        let config_path_buf = std::path::Path::join(home_dir.to_owned().as_path(), ".remembertostand");
        let config_file_result = File::open(config_path_buf.as_path());
        let mut main_config = MainConfig {
            config: Default::default(),
        };

        if config_file_result.is_err() {
            let new_config_file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(config_path_buf.as_path())
                .expect("Couldn't create config file");

            let mut new_config_writer = BufWriter::new(new_config_file);
            serde_json::to_writer(&mut new_config_writer, &main_config).unwrap();
            new_config_writer.flush().unwrap();
            println!("Created new config file: {:?}", main_config);
        } else {
            let config_file = config_file_result.unwrap();
            let config_file_reader = BufReader::new(config_file);
            main_config = serde_json::from_reader(config_file_reader).unwrap();
            println!("Loaded configuration from from .remembertostand config file: {:?}", main_config);
        }

        if main_config.config.contains_key("standtimesecs") {
            stand_time_secs = main_config.config.get("standtimesecs").unwrap().parse().ok().unwrap_or(3600);
            println!("stand time secs set to: {}", stand_time_secs);
        } else {
            main_config.config.insert("standtimesecs".to_string(), "3600".to_string());
        }

        if main_config.config.contains_key("sittimesecs") {
            sit_time_secs = main_config.config.get("sittimesecs").unwrap().parse().ok().unwrap_or(3600);
            println!("sit time secs set to: {}", sit_time_secs);
        } else {
            main_config.config.insert("sittimesecs".to_string(), "3600".to_string());
        }

        if main_config.config.contains_key("customstandmsg") {
            custom_stand_msg = main_config.config.get("customstandmsg").unwrap().to_string();
            println!("custom stand msg set to: {}", custom_stand_msg);
        } else {
            main_config.config.insert("customstandmsg".to_string(), "Stand up!".to_string());
        }

        if main_config.config.contains_key("customsitmsg") {
            custom_sit_msg = main_config.config.get("customsitmsg").unwrap().to_string();
            println!("custom sit msg set to: {}", custom_sit_msg);
        } else {
            main_config.config.insert("customsitmsg".to_string(), "Sit down!".to_string());
        }

        let releases_state_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config_path_buf.as_path())
            .expect("Couldn't open config file for writing");

        let mut releases_state_writer = BufWriter::new(releases_state_file);
        serde_json::to_writer(&mut releases_state_writer, &main_config).unwrap();
        releases_state_writer.flush().unwrap();
        
        loop {
            loop_count += 1;
            println!("Loop count: {}", loop_count);

            let file = BufReader::new(File::open("./resources/notify.wav").unwrap());
            let source = Decoder::new(file).unwrap();
            let play_result = stream_handle.play_raw(source.convert_samples());
            if play_result.is_err() {
                println!("Error playing notify sound: {}", play_result.err().unwrap());
            }

            if loop_count % 2 == 0 {
                let mut notification = Notification::new();
                notification
                    .summary(custom_stand_msg.as_str())
                    .body(&*format!("You've been sitting for {} minutes. It's time to stand up.", sit_time_secs / 60))
                    .icon("icon.png")
                    .appname("remember-to-stand")
                    .timeout(10000)
                    .show()
                    .unwrap();
                sleep(Duration::from_secs(stand_time_secs));
            } else {
                let mut notification = Notification::new();
                notification
                    .summary(custom_sit_msg.as_str())
                    .body(&*format!("You've been standing for {} minutes. Time to relax and sit for a while.", stand_time_secs / 60))
                    .icon("icon.png")
                    .appname("remember-to-stand")
                    .timeout(10000)
                    .show()
                    .unwrap();
                
                sleep(Duration::from_secs(sit_time_secs));
            }
        }
     });

    inner.display();
}