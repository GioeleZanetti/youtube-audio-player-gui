use crate::yap_cli::yap_cli::Yap;

use gtk::prelude::*;
use relm4::{
    factory::FactoryVecDeque,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PlaylistModel {
    playlists: FactoryVecDeque<PlaylistEntry>,
}

#[derive(Debug)]
pub enum PlaylistEvents {
    Play(String),
    Delete(String, DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for PlaylistModel {
    type Input = PlaylistEvents;
    type Output = ();
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,

            gtk::ScrolledWindow{
                set_vexpand: true,
                set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Automatic),

                #[local_ref]
                playlist_list -> gtk::ListBox {
                    set_vexpand: true,
                    add_css_class: "not-transparent"
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let playlists = Yap::get_playlists();
        let mut playlist_entries = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                PlaylistEntryOutput::Play(song) => PlaylistEvents::Play(song),
                PlaylistEntryOutput::Delete(song, index) => PlaylistEvents::Delete(song, index),
            });
        for playlist in playlists {
            playlist_entries.guard().push_back(PlaylistEntryInit {
                playlist_name: playlist,
            });
        }
        let model = PlaylistModel {
            playlists: playlist_entries,
        };
        let playlist_list = model.playlists.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PlaylistEvents::Play(playlist) => Yap::play_playlist(&playlist),
            PlaylistEvents::Delete(playlist, index) => {
                //Yap::delete_playlist(playlist)
                self.playlists.guard().remove(index.current_index());
            }
        }
    }
}

#[derive(Debug)]
pub struct PlaylistEntry {
    playlist_name: String,
}

#[derive(Debug)]
pub enum PlaylistEntryOutput {
    Play(String),
    Delete(String, DynamicIndex),
}

pub struct PlaylistEntryInit {
    pub playlist_name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for PlaylistEntry {
    type ParentWidget = gtk::ListBox;
    type Input = ();
    type Output = PlaylistEntryOutput;
    type Init = PlaylistEntryInit;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ListBoxRow::builder()
            .hexpand(true)
            .height_request(30)
            .css_classes(["playlist-list-row", "list-row"])
            .build()
        {
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_height_request: 30,

                gtk::Label{
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    add_css_class: "playlist-label",
                    set_label: &self.playlist_name,
                },

                gtk::Button {
                    set_label: "󰆴",
                    add_css_class: "playlist-button",
                    connect_clicked[sender, playlist_name = self.playlist_name.clone(), index] => move |_| {
                        sender.output(
                            PlaylistEntryOutput::Delete(playlist_name.to_string(), index.clone())
                        ).unwrap();
                    },
                },

                gtk::Button {
                    set_label: "",
                    add_css_class: "playlist-button",
                    connect_clicked[sender, playlist_name = self.playlist_name.clone()] => move |_| {
                        sender.output(PlaylistEntryOutput::Play(playlist_name.to_string())).unwrap();
                    },
                },
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            playlist_name: init.playlist_name,
        }
    }
}
