use gtk::glib::signal::Propagation;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum VolumeEvents {
    Change(f64),
}

pub struct VolumeModel {}

#[relm4::component(pub)]
impl SimpleComponent for VolumeModel {
    type Input = VolumeEvents;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {
            set_hexpand: false,
            set_width_request: 50,
            set_height_request: 400,
            set_margin_end: 20,
            set_margin_bottom: 20,
            add_css_class: "not-transparent",

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,
                set_hexpand: true,
                set_margin_top: 10,
                set_margin_bottom: 10,

                gtk::Scale {
                    set_orientation: gtk::Orientation::Vertical,
                    set_adjustment: &gtk::Adjustment::new(0., 0., 100., 1., 1., 1.),
                    set_vexpand: true,
                    set_hexpand: true,
                    set_inverted: true,
                    set_value: get_current_volume(),

                    connect_change_value[sender] => move |_, _, value| {
                        sender.input(VolumeEvents::Change(value));
                        Propagation::Proceed
                    }
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = VolumeModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            VolumeEvents::Change(volume) => set_volume(volume),
        }
    }
}

fn get_current_volume() -> f64 {
    let output = Command::new("pamixer")
        .arg("--get-volume-human")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute pamixer command");
    let output_piped = Command::new("tr")
        .arg("-d")
        .arg("%")
        .stdin(Stdio::from(
            output.stdout.expect("Failed to get ouput of pamixer"),
        ))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute pr command");
    let output = output_piped
        .wait_with_output()
        .expect("Failed to get pr output");
    let result = std::str::from_utf8(&output.stdout)
        .expect("Failed to convert output of pr to string")
        .trim();
    result.parse::<f64>().expect("Couldn't parse output")
}

fn set_volume(volume: f64) {
    let _output = Command::new("amixer")
        .arg("-D")
        .arg("default")
        .arg("sset")
        .arg("Master")
        .arg(&format!("{}%", volume.to_string()))
        .output()
        .unwrap();
}
