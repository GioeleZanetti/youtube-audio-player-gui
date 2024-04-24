use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::playlist_widget::PlaylistModel;
use super::song_widget::SongModel;

#[derive(Debug)]
pub enum MusicEvents {
    StartWidget,
}

pub struct MusicModel {
    song_widget: Controller<SongModel>,
    playlist_widget: Controller<PlaylistModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for MusicModel {
    type Input = MusicEvents;
    type Output = ();
    type Init = String;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_hexpand: true,
            set_width_request: 850,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_vexpand: true,
                set_hexpand: true,
                add_css_class: "not-transparent",

                #[name = "sidebar"]
                gtk::StackSidebar {},

                #[name = "stack"]
                gtk::Stack {
                    add_css_class: "not-transparent",

                    add_titled: (model.song_widget.widget(), Some("songs"), "Songs"),
                    add_titled: (model.playlist_widget.widget(), Some("playlists"), "Playlists"),
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let song_widget = SongModel::builder()
            .launch(init)
            .forward(sender.input_sender(), |()| MusicEvents::StartWidget);
        let playlist_widget = PlaylistModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| MusicEvents::StartWidget);
        let model = MusicModel {
            song_widget,
            playlist_widget,
        };
        let widgets = view_output!();
        widgets.sidebar.set_stack(&widgets.stack);
        ComponentParts { model, widgets }
    }
}
