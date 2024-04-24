use std::process::Command;

pub struct Yap {}

#[derive(Clone, Debug)]
pub struct Song {
    pub name: String,
    pub artist: String,
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.artist == other.artist
    }
}

#[derive(Debug)]
pub struct Time {
    pub min: u32,
    pub sec: u32,
    pub tot_min: u32,
    pub tot_sec: u32,
    pub perc: u8,
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.sec == other.sec
            && self.tot_min == other.tot_min
            && self.tot_sec == other.tot_sec
            && self.perc == other.perc
    }
}

impl Time {
    fn from_str(s: &str) -> Option<Time> {
        let parts: Vec<&str> = s
            .split(|c| c == ':' || c == '/' || c == '(' || c == '%' || c == ')')
            .collect();

        if parts.len() >= 6 {
            let min = parts[0].parse().ok()?;
            let sec = parts[1].parse().ok()?;
            let tot_min = parts[2].parse().ok()?;
            let tot_sec = parts[3].trim().parse().ok()?;
            let perc = parts[4].parse().ok()?;

            let time = Time {
                min,
                sec,
                tot_min,
                tot_sec,
                perc,
            };

            return Some(time);
        }

        None
    }
}

pub struct Status {
    pub repeat: bool,
    pub random: bool,
    pub is_paused: bool,
}

impl Yap {
    pub fn get_playlists() -> Vec<String> {
        let command = Command::new("yap")
            .arg("playlist")
            .arg("list")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&command.stdout);
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn get_songs() -> Vec<Song> {
        let command = Command::new("yap")
            .arg("song")
            .arg("list")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&command.stdout);
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| {
                let parts: Vec<&str> = s.split(" - ").collect();
                Song {
                    name: parts[0].to_string(),
                    artist: parts[1].to_string(),
                }
            })
            .collect()
    }

    pub fn play_playlist(playlist: &str) {
        let _command = Command::new("yap")
            .arg("play")
            .arg("playlist")
            .arg("--name")
            .arg(playlist)
            .output()
            .unwrap();
    }

    pub fn play_song(song: &str) {
        let _command = Command::new("yap")
            .arg("play")
            .arg("song")
            .arg("--name")
            .arg(song)
            .output()
            .unwrap();
    }

    pub fn current() -> Option<(Song, Time)> {
        let command = Command::new("yap")
            .arg("mpd")
            .arg("current")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&command.stdout);

        let lines: Vec<&str> = output.lines().collect();

        // Check if the input has the expected format
        if lines.len() == 3
            && lines[0].starts_with("Current song: ")
            && lines[1].starts_with("Artist: ")
        {
            // Extract the song and artist
            let name = lines[0]["Current song: ".len()..].to_string();
            let artist = lines[1]["Artist: ".len()..].to_string();
            let time = Time::from_str(lines[2])?;

            Some((Song { name, artist }, time))
        } else {
            None
        }
    }

    pub fn seek(percentage: u64) {
        let _command = Command::new("yap")
            .arg("mpd")
            .arg("seek")
            .arg("--percentage")
            .arg(percentage.to_string())
            .output()
            .unwrap();
    }

    pub fn status() -> Result<Status, String> {
        let command = Command::new("yap")
            .arg("mpd")
            .arg("status")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&command.stdout);
        let mut pause = false;
        let mut random = false;
        let mut repeat = false;

        for part in output.split('\t') {
            let mut iter = part.split(':');

            if let Some(key) = iter.next() {
                if let Some(value) = iter.next() {
                    match key.trim() {
                        "Pause" => pause = value.trim().parse().map_err(|_| "Invalid bool")?,
                        "Random" => random = value.trim().parse().map_err(|_| "Invalid bool")?,
                        "Repeat" => repeat = value.trim().parse().map_err(|_| "Invalid bool")?,
                        _ => return Err("Invalid key".to_string()),
                    }
                } else {
                    return Err("Invalid key-value pair".to_string());
                }
            } else {
                return Err("Invalid key-value pair".to_string());
            }
        }

        Ok(Status {
            is_paused: pause,
            random,
            repeat,
        })
    }

    pub fn random() {
        let _command = Command::new("yap")
            .arg("mpd")
            .arg("shuffle")
            .output()
            .unwrap();
    }

    pub fn next() {
        let _command = Command::new("yap").arg("mpd").arg("next").output().unwrap();
    }

    pub fn play() {
        let _command = Command::new("yap").args(["mpd", "play"]).output().unwrap();
    }

    pub fn toggle_pause() {
        let _command = Command::new("yap")
            .arg("mpd")
            .arg("pause")
            .output()
            .unwrap();
    }

    pub fn prev() {
        let _command = Command::new("yap")
            .arg("mpd")
            .arg("previous")
            .output()
            .unwrap();
    }

    pub fn repeat() {
        let _command = Command::new("yap")
            .arg("mpd")
            .arg("repeat")
            .output()
            .unwrap();
    }

    pub fn delete_song(song_name: &str) {
        let _command = Command::new("yap")
            .args(["song", "delete", "--name", song_name])
            .output()
            .unwrap();
    }

    pub fn add_to_queue(song_name: &str) {
        let _command = Command::new("yap")
            .args(["mpd", "queue-add", "--song-name", song_name])
            .output()
            .unwrap();
    }

    pub fn get_queue() -> Vec<Song> {
        let command = Command::new("yap").args(["mpd", "queue"]).output().unwrap();
        let output = String::from_utf8_lossy(&command.stdout);
        output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| {
                let parts: Vec<&str> = s.split(" - ").collect();
                Song {
                    name: parts[0].to_string(),
                    artist: parts[1].to_string(),
                }
            })
            .collect()
    }

    pub fn remove_from_queue(song_name: &str) {
        let _command = Command::new("yap")
            .args(["mpd", "queue-remove", "--song-name", song_name])
            .output()
            .unwrap();
    }

    pub fn clear_queue() {
        let _command = Command::new("yap").args(["mpd", "clear"]).output().unwrap();
    }

    pub fn shuffle_queue() {
        let _command = Command::new("yap")
            .args(["mpd", "queue-shuffle"])
            .output()
            .unwrap();
    }
}
