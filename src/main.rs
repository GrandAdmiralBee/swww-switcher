use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Wallpapers {
    current_wallpaper: String,
    wallpapers: Vec<String>,
}

impl Default for Wallpapers {
    fn default() -> Self {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ls /home/karim/Pictures/Wallpapers/")
            .output()
            .unwrap();
        let wallpaper_list = std::str::from_utf8(&output.stdout).unwrap();

        let mut current_wallpaper = String::new();
        let mut wallpapers = Vec::new();
        let mut first_wallpaper = true;
        for line in wallpaper_list.lines() {
            if first_wallpaper {
                current_wallpaper = line.to_string();
                first_wallpaper = false;
            }
            wallpapers.push(line.to_string());
        }
        Self {
            current_wallpaper,
            wallpapers,
        }
    }
}

impl Wallpapers {
    pub fn from_file() -> Self {
        let path = "/home/karim/.config/swwws/wallpapers.toml";
        let contents = std::fs::read_to_string(path);
        let wallpapers = match contents {
            Ok(str) => toml::from_str(&str).unwrap(),
            Err(err) => {
                println!("{}", err);
                Wallpapers::default()
            }
        };
        wallpapers.write_to_file();
        wallpapers
    }

    pub fn write_to_file(&self) {
        let wallpapers = toml::to_string(self).unwrap();
        std::fs::write("/home/karim/.config/swwws/wallpapers.toml", wallpapers).unwrap();
    }

    pub fn switch(&mut self) {
        let mut index = self
            .wallpapers
            .binary_search(&self.current_wallpaper)
            .unwrap();
        if index == self.wallpapers.len() - 1 {
            index = 0;
        } else {
            index += 1;
        }
        self.current_wallpaper = self.wallpapers.get(index).unwrap().to_string();
        self.write_to_file();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--update" => {
                Wallpapers::default().write_to_file();
            }
            "--init" => {
                let wallpapers = Wallpapers::from_file();
                let full_path = format!(
                    "/home/karim/Pictures/Wallpapers/{}",
                    wallpapers.current_wallpaper
                );
                let _ = Command::new("sh").arg("-c").arg("swww init").spawn();
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(format!("swww img {}", full_path))
                    .spawn();
            }
            _ => {
                println!("Wrong flag");
            }
        }
    } else {
        let mut wallpapers = Wallpapers::from_file();
        wallpapers.switch();
        let full_path = format!(
            "/home/karim/Pictures/Wallpapers/{}",
            wallpapers.current_wallpaper
        );
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("swww img {}", full_path))
            .spawn();
    }
}
