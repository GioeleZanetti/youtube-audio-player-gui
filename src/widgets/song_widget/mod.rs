use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::yap_cli::yap_cli::Yap;

use gtk::prelude::*;
use relm4::{
    factory::FactoryVecDeque,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub enum SongEvent {
    Play(String),
    Delete(String, DynamicIndex),
    AddToQueue(String),
}

pub struct SongModel {
    songs: FactoryVecDeque<SongEntry>,
}

#[relm4::component(pub)]
impl SimpleComponent for SongModel {
    type Input = SongEvent;
    type Output = ();
    type Init = String;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,

            gtk::ScrolledWindow{
                set_vexpand: true,
                set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Automatic),

                #[local_ref]
                songs_list -> gtk::ListBox {
                    set_vexpand: true,
                    add_css_class: "not-transparent"
                }
            }
        }
    }

    fn init(
        minuatures_directory: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let songs = Yap::get_songs();
        let mut song_entries = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                SongEntryOutput::AddToQueue(song) => SongEvent::AddToQueue(song),
                SongEntryOutput::Play(song) => SongEvent::Play(song),
                SongEntryOutput::Delete(song, index) => SongEvent::Delete(song, index),
            });
        for song in songs {
            let img_path = {
                let path = Path::new(&minuatures_directory).join(format!("{}.jpg", song.name));
                if File::open(path.clone()).is_ok() {
                    Some(path)
                } else {
                    None
                }
            };

            song_entries.guard().push_back(SongEntryInit {
                song_name: song.name,
                song_artist: song.artist,
                img_path,
            });
        }
        let model = SongModel {
            songs: song_entries,
        };
        let songs_list = model.songs.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SongEvent::Play(song) => Yap::play_song(&song),
            SongEvent::Delete(song, index) => {
                Yap::delete_song(&song);
                self.songs.guard().remove(index.current_index());
            }
            SongEvent::AddToQueue(song) => Yap::add_to_queue(&song),
        }
    }
}

#[derive(Debug)]
pub struct SongEntry {
    song_name: String,
    song_artist: String,
    img_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum SongEntryOutput {
    AddToQueue(String),
    Play(String),
    Delete(String, DynamicIndex),
}

pub struct SongEntryInit {
    song_name: String,
    song_artist: String,
    img_path: Option<PathBuf>,
}

#[relm4::factory(pub)]
impl FactoryComponent for SongEntry {
    type ParentWidget = gtk::ListBox;
    type Input = ();
    type Output = SongEntryOutput;
    type Init = SongEntryInit;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ListBoxRow::builder()
            .hexpand(true)
            .height_request(30)
            .css_classes(["song-list-row", "list-row"])
            .build()
        {
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_height_request: 30,

                gtk::Image {
                    set_width_request: 150,
                    set_height_request: 150,
                    set_margin_end: 20,
                    set_margin_start: 20,
                    set_from_file: self.img_path.clone()
                },

                gtk::Label {
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,
                    add_css_class: "song-label",
                    set_label: &format!("{} - {}", &self.song_name, &self.song_artist),
                },

                gtk::Button {
                    set_label: "󰐒",
                    add_css_class: "song-button",
                    connect_clicked[sender, song_name = self.song_name.clone()] => move |_| {
                        sender.output(SongEntryOutput::AddToQueue(song_name.to_string())).unwrap();
                    },
                },

                gtk::Button {
                    set_label: "󰆴",
                    add_css_class: "song-button",
                    connect_clicked[sender, song_name = self.song_name.clone(), index] => move |_| {
                        sender.output(
                            SongEntryOutput::Delete(song_name.to_string(), index.clone())
                        ).unwrap();
                    },
                },

                gtk::Button {
                    set_label: "",
                    add_css_class: "song-button",
                    connect_clicked[sender, song_name = self.song_name.clone()] => move |_| {
                        sender.output(SongEntryOutput::Play(song_name.to_string())).unwrap();
                    },
                },
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            song_name: init.song_name,
            song_artist: init.song_artist,
            img_path: init.img_path,
        }
    }
}
