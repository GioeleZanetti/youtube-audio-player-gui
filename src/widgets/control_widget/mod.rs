use gtk::glib::signal::Propagation;
use std::time::Duration;

use crate::yap_cli::yap_cli::Yap;

use glib::ControlFlow;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use relm4::RelmWidgetExt;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct ControlModel {
    pub is_paused: bool,
    pub is_repeating: bool,
    pub is_random: bool,
    pub current_song: String,
}

#[derive(Debug)]
pub enum ControlEvents {
    Seek(f64),
    Toggle,
    Next,
    Prev,
    Rand,
    Repeat,
}

pub struct ControlWidgets {
    play_button: gtk::Button,
    repeat_button: gtk::Button,
    random_button: gtk::Button,
}

impl SimpleComponent for ControlModel {
    type Input = ControlEvents;
    type Output = ();
    type Init = ();
    type Root = gtk::Box;
    type Widgets = ControlWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(false)
            .build()
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current_status = Yap::status().expect("Error while retrieving YAP status");
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .vexpand(true)
            .hexpand(true)
            .build();
        hbox.add_css_class("not-transparent");
        let song_info = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .margin_start(10)
            .build();
        let song_name_label = gtk::Label::builder()
            .label("\t\t\t")
            .hexpand(false)
            .halign(gtk::Align::Center)
            .vexpand(true)
            .width_chars(20)
            .wrap_mode(gtk::pango::WrapMode::Word)
            .build();
        song_name_label.set_widget_name("song-name");
        let song_artist_label = gtk::Label::builder()
            .hexpand(true)
            .halign(gtk::Align::Center)
            .vexpand(true)
            .margin_bottom(20)
            .build();
        song_artist_label.set_widget_name("song-artist");
        let control_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .height_request(130)
            .margin_start(20)
            .margin_end(20)
            .margin_bottom(10)
            .build();
        let progress_bar = gtk::Scale::builder()
            .orientation(gtk::Orientation::Horizontal)
            .adjustment(&gtk::Adjustment::new(0., 0., 100., 1., 1., 1.))
            .hexpand(true)
            .vexpand(true)
            .build();
        let controls = gtk::Box::builder().hexpand(true).build();
        let rand = gtk::Button::with_label("󰒟");
        let prev = gtk::Button::with_label("󰒮");
        let toggle_play = gtk::Button::with_label(if current_status.is_paused {
            ""
        } else {
            "󰏤"
        });
        let next = gtk::Button::with_label("󰒭");
        let repeat = gtk::Button::with_label("󰑖");
        rand.set_class_active("active", current_status.random);
        repeat.set_class_active("active", current_status.repeat);
        rand.add_css_class("control-button");
        next.add_css_class("control-button");
        toggle_play.add_css_class("control-button");
        prev.add_css_class("control-button");
        repeat.add_css_class("control-button");
        controls.append(&rand);
        controls.append(&prev);
        controls.append(&toggle_play);
        controls.append(&next);
        controls.append(&repeat);
        control_box.append(&progress_bar);
        control_box.append(&controls);
        song_info.append(&song_name_label);
        song_info.append(&song_artist_label);
        hbox.append(&song_info);
        hbox.append(&control_box);
        root.append(&hbox);

        rand.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(ControlEvents::Rand);
        }));

        prev.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(ControlEvents::Prev);
        }));

        toggle_play.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(ControlEvents::Toggle);
        }));

        next.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(ControlEvents::Next);
        }));

        repeat.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(ControlEvents::Repeat);
        }));

        progress_bar.connect_change_value(clone!(@strong sender => move |_, _, value| {
            sender.input(ControlEvents::Seek(value));
            Propagation::Proceed
        }));

        glib::timeout_add_local(
            Duration::from_millis(1000),
            clone!(
                @weak progress_bar,
                @weak song_name_label,
                @weak song_artist_label,
                @weak toggle_play,
                @weak rand,
                @weak repeat
                => @default-return ControlFlow::Break, move || {
                    let current_status_option = Yap::status();
                    let current_song_info_option = Yap::current();
                    if let Some(current_song_info) = current_song_info_option {
                        song_name_label.set_label(&to_twenty_char(current_song_info.0.name));
                        song_artist_label.set_label(&current_song_info.0.artist);
                        progress_bar.set_value(current_song_info.1.perc as f64);
                    } else {
                        song_name_label.set_label("\t\t\t");
                        song_artist_label.set_label("");
                        progress_bar.set_value(0.);
                    }
                    if let Ok(current_status) = current_status_option {
                        toggle_play.set_label(if current_status.is_paused {
                            ""
                        } else {
                            "󰏤"
                        });
                        repeat.set_class_active("active", current_status.repeat);
                        rand.set_class_active("active", current_status.random);
                    }
                }
                ControlFlow::Continue
            ),
        );

        ComponentParts {
            model: ControlModel {
                is_paused: current_status.is_paused,
                is_repeating: current_status.repeat,
                is_random: current_status.random,
                current_song: "".to_string(),
            },
            widgets: ControlWidgets {
                play_button: toggle_play,
                random_button: rand,
                repeat_button: repeat,
            },
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ControlEvents::Toggle => {
                if self.is_paused {
                    Yap::play();
                } else {
                    Yap::toggle_pause();
                }
                self.is_paused = !self.is_paused
            }
            ControlEvents::Next => Yap::next(),
            ControlEvents::Prev => Yap::prev(),
            ControlEvents::Seek(percentage) => Yap::seek(percentage as u64),
            ControlEvents::Rand => {
                Yap::random();
                self.is_random = !self.is_random
            }
            ControlEvents::Repeat => {
                Yap::repeat();
                self.is_repeating = !self.is_repeating
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        if self.is_paused {
            widgets.play_button.set_label("")
        } else {
            widgets.play_button.set_label("󰏤")
        }
        widgets
            .repeat_button
            .set_class_active("active", self.is_repeating);
        widgets
            .random_button
            .set_class_active("active", self.is_random);
    }
}

fn to_twenty_char(string: String) -> String {
    if string.len() > 17 {
        let mut sub = string[..17].to_string();
        sub.push_str("...");
        sub
    } else {
        string
    }
}
