use glib::clone;
use glib::ControlFlow;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use std::time::Duration;

use crate::yap_cli::yap_cli::Song;
use crate::yap_cli::yap_cli::Yap;

pub struct QueueModel {}

#[derive(Debug)]
pub enum QueueEvents {
    RemoveFromQueue(String),
    ClearQueue,
    ShuffleQueue,
}

impl SimpleComponent for QueueModel {
    type Root = gtk::Box;
    type Input = QueueEvents;
    type Output = ();
    type Init = ();
    type Widgets = ();

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .width_request(300)
            .margin_bottom(20)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut songs_in_queue = Yap::get_queue();
        let mut current_song = Yap::current();
        let queue_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .build();
        let scrolled_window = gtk::ScrolledWindow::builder().vexpand(true).build();
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        let title_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let title = gtk::Label::builder()
            .label("Queue")
            .name("title")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .build();
        let clear_queue = gtk::Button::with_label("");
        let shuffle_queue = gtk::Button::with_label("󰒟");
        let separator = gtk::Separator::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let queue_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let list_box = build_list(&songs_in_queue, &sender);
        queue_box.append(&list_box);
        queue_container.add_css_class("not-transparent");
        clear_queue.add_css_class("playlist-button");
        shuffle_queue.add_css_class("playlist-button");
        title_box.append(&title);
        title_box.append(&shuffle_queue);
        title_box.append(&clear_queue);
        queue_container.append(&title_box);
        queue_container.append(&separator);
        queue_container.append(&queue_box);
        scrolled_window.set_child(Some(&queue_container));
        root.append(&scrolled_window);

        clear_queue.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(QueueEvents::ClearQueue);
        }));

        shuffle_queue.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(QueueEvents::ShuffleQueue);
        }));

        glib::timeout_add_local(
            Duration::from_millis(500),
            clone!(
                @weak queue_box,
                @strong sender,
                => @default-return ControlFlow::Break, move || {
                    let current_song_loop = Yap::current();
                    let queue = Yap::get_queue();
                    if songs_in_queue != queue || current_song != current_song_loop {
                        let list = build_list(&queue, &sender);
                        queue_box.remove(&queue_box.first_child().unwrap());
                        queue_box.append(&list);
                        songs_in_queue = queue;
                        current_song = current_song_loop;
                    }
                }
                ControlFlow::Continue
            ),
        );

        let model = QueueModel {};
        let widgets = ();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            QueueEvents::RemoveFromQueue(song) => Yap::remove_from_queue(&song),
            QueueEvents::ClearQueue => Yap::clear_queue(),
            QueueEvents::ShuffleQueue => Yap::shuffle_queue(),
        }
    }
}

fn build_list(songs_in_queue: &Vec<Song>, sender: &ComponentSender<QueueModel>) -> gtk::ListBox {
    let current_song = Yap::current();
    let list_box = gtk::ListBox::builder()
        .name("queue_list")
        .vexpand(true)
        .hexpand(true)
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("not-transparent");
    list_box.add_css_class("queue");
    for song in songs_in_queue {
        let song_name = &song.name;
        let song_artist = &song.artist;
        let list_box_row = gtk::ListBoxRow::builder()
            .hexpand(true)
            .height_request(30)
            .name(song_name)
            .build();
        list_box_row.add_css_class("list-row");
        let hbox: gtk::Box = gtk::Box::builder()
            .hexpand(true)
            .orientation(gtk::Orientation::Horizontal)
            .height_request(30)
            .build();
        let label_song = gtk::Label::builder()
            .label(&format!("{} - {}", song_name, song_artist))
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build();
        let delete_button = gtk::Button::with_label("󰆴");
        delete_button.add_css_class("playlist-button");
        delete_button.add_css_class("delete");
        hbox.append(&label_song);
        hbox.append(&delete_button);
        delete_button.connect_clicked(clone!(@strong sender, @strong song_name => move |_| {
            sender.input(QueueEvents::RemoveFromQueue(song_name.to_string()));
        }));

        if current_song.is_some() {
            list_box_row.set_class_active(
                "current-song",
                current_song.as_ref().unwrap().0.name == song.name,
            );
        }
        list_box_row.set_child(Some(&hbox));
        list_box.append(&list_box_row);
    }

    list_box
}
