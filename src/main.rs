mod widgets;
mod yap_cli;

use relm4::RelmApp;
use widgets::yap_widget::YapModel;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub general: General,
    pub database: Database,
}
#[derive(Deserialize, Serialize)]
pub struct General {
    pub music_directory: String,
    pub miniature_directory: String,
    pub download_miniature: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General {
                music_directory: "~/Music/songs/".to_string(),
                miniature_directory: "~/Music/miniatures".to_string(),
                download_miniature: false,
            },
            database: Database {
                database_path: "~/.config/yap/yap.db".to_string(),
            },
        }
    }
}

fn main() {
    let config: Config = match confy::load("yap", "yap.config") {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let app = RelmApp::new("org.relm4.song_widget");
    app.run::<YapModel>(config.general.miniature_directory.clone());
}
// let stdin = stdin();
// let mut stdout = stdout().into_raw_mode().unwrap();
//
// print!("Press \"p\" to select playlist to play\n\rPress \"s\" to select song to play\n\r");
// stdout.flush().unwrap();
// for c in stdin.keys() {
//     match c.unwrap() {
//         Key::Char('p') => {
//             let app = RelmApp::new("org.relm4.song_widget");
//             app.run::<PowerMenuModel>(());
//             break;
//         }
//         Key::Char('s') => {
//             let app = RelmApp::new("org.relm4.song_widget");
//             app.run::<MusicModel>(());
//             break;
//         }
//         Key::Char('c') => {
//             let app = RelmApp::new("org.relm4.control_widget");
//             app.run::<ControlModel>(());
//             break;
//         }
//         Key::Char('v') => {
//             let app = RelmApp::new("org.relm4.volume_widget");
//             app.run::<VolumeModel>(());
//             break;
//         }
//         Key::Char('q') => break,
//         _ => {}
//     }
//     stdout.flush().unwrap();
// // }
// pub fn activate(application: &gtk::Application) {
//     let window = gtk::ApplicationWindow::new(application);
//
//     // Before the window is first realized, set it up to be a layer surface
//     window.init_layer_shell();
//
//     // Display it above normal windows
//     window.set_layer(Layer::Overlay);
//
//     // Push other windows out of the way
//     window.auto_exclusive_zone_enable();
//
//     // The margins are the gaps around the window's edges
//     // Margins and anchors can be set like this...
//     window.set_layer_shell_margin(Edge::Left, 40);
//     window.set_layer_shell_margin(Edge::Right, 40);
//     window.set_layer_shell_margin(Edge::Top, 20);
//
//     // ... or like this
//     // Anchors are if the window is pinned to each edge of the output
//     let anchors = [
//         (Edge::Left, true),
//         (Edge::Right, true),
//         (Edge::Top, false),
//         (Edge::Bottom, true),
//     ];
//
//     for (anchor, state) in anchors {
//         window.set_anchor(anchor, state);
//     }
//
//     // Set up a widget
//     let label = gtk::Label::new(Some(""));
//     window.add(&label);
//     window.set_border_width(12);
//     window.show_all()
// }
