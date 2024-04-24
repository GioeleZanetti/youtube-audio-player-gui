use std::path::Path;

use gtk::prelude::*;
use gtk::traits::OrientableExt;
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::gtk::traits::GtkWindowExt;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use super::control_widget::ControlModel;
use super::music_widget::MusicModel;
use super::power_menu_widget::PowerMenuModel;
use super::queue_widget::QueueModel;
use super::volume_widget::VolumeModel;

pub struct YapModel {
    volume: Controller<VolumeModel>,
    music: Controller<MusicModel>,
    controls: Controller<ControlModel>,
    queue: Controller<QueueModel>,
    power: Controller<PowerMenuModel>,
}

#[derive(Debug)]
pub enum YapEvents {
    StartWidget,
}

#[relm4::component(pub)]
impl SimpleComponent for YapModel {
    type Input = YapEvents;
    type Output = ();
    type Init = String;

    view! {
        gtk::Window {
            set_default_width: 1920,
            set_default_height: 1080,
            set_resizable: false,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
        set_vexpand: true,
        set_hexpand: true,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: false,

                    model.power.widget(),
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_end: 20,

                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,

                            model.volume.widget(),

                            model.queue.widget(),
                        },

                        model.controls.widget(),
                    },

                    model.music.widget(),
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(
            Path::new("src")
                .join("widgets")
                .join("yap_widget")
                .join("main.css"),
        );
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Error initializing css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        // let header = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Horizontal)
        //     .hexpand(true)
        //     .build();
        // let volume_queue = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Horizontal)
        //     .build();
        // let controls = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .margin_end(20)
        //     .build();
        // let widgets = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Horizontal)
        //     .build();
        // let window = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .build();
        root.init_layer_shell();
        root.set_default_size(1600, 600);
        root.set_layer(Layer::Overlay);
        root.set_anchor(gtk4_layer_shell::Edge::Top, true);
        root.set_anchor(gtk4_layer_shell::Edge::Bottom, false);
        root.set_exclusive_zone(1080);
        let volume_widget = VolumeModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| YapEvents::StartWidget);
        let control_widget = ControlModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| YapEvents::StartWidget);
        let music_widget = MusicModel::builder()
            .launch(init)
            .forward(sender.input_sender(), |()| YapEvents::StartWidget);
        let queue_widget = QueueModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| YapEvents::StartWidget);
        let power_menu_widget = PowerMenuModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| YapEvents::StartWidget);
        // header.append(power_menu_widget.widget());
        // volume_queue.append(volume_widget.widget());
        // volume_queue.append(queue_widget.widget());
        // controls.append(&volume_queue);
        // controls.append(control_widget.widget());
        // widgets.append(&controls);
        // widgets.append(music_widget.widget());
        // window.append(&header);
        // window.append(&widgets);
        // root.set_child(Some(&window));
        let model = YapModel {
            volume: volume_widget,
            music: music_widget,
            controls: control_widget,
            queue: queue_widget,
            power: power_menu_widget,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {}
}
