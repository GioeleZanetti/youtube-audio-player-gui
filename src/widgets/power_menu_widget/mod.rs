use std::process::Command;

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PowerMenuModel {}

#[derive(Debug)]
pub enum PowerMenuEvents {
    Close,
    RestartMpd,
}

#[relm4::component(pub)]
impl SimpleComponent for PowerMenuModel {
    type Input = PowerMenuEvents;
    type Output = ();
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_hexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_margin_end: 20,
                set_margin_bottom: 20,
                add_css_class: "not-transparent",

                gtk::Label{
                    set_widget_name: "app-title",
                    set_label: "YAP-GUI",
                }
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: false,
                set_vexpand: false,
                set_height_request: 100,
                set_width_request: 100,

                gtk::Button::builder()
                    .label("󰅖")
                    .margin_end(20)
                    .margin_bottom(20)
                    .height_request(100)
                    .width_request(100)
                    .vexpand(false)
                    .hexpand(false)
                    .build()
                {
                    add_css_class: "power-button",
                    add_css_class: "not-transparent",
                    connect_clicked[sender] => move |_| {
                        sender.input(PowerMenuEvents::Close)
                    }
                },

                gtk::Button::builder()
                    .label("")
                    .height_request(100)
                    .width_request(100)
                    .margin_bottom(20)
                    .vexpand(false)
                    .hexpand(false)
                    .build()
                {
                    add_css_class: "power-button",
                    add_css_class: "not-transparent",
                    connect_clicked[sender] => move |_| {
                        sender.input(PowerMenuEvents::RestartMpd);
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PowerMenuModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PowerMenuEvents::Close => relm4::main_application().quit(),
            PowerMenuEvents::RestartMpd => restart_mpd(),
        }
    }
}

fn restart_mpd() {
    Command::new("mpd").output().unwrap();
}
