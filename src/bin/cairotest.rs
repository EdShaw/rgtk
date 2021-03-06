
#![feature(globs)]

extern crate rgtk;
extern crate log;
extern crate debug;
extern crate collections;

use std::f64::consts::PI_2;

use rgtk::*;
use rgtk::gtk::signals;
use rgtk::gtk::DrawingArea;

use rgtk::cairo::enums::{
    FontSlantNormal,
    FontWeightNormal
};

fn main() {
    gtk::init();

    drawable(500, 500, |cr| {
        cr.scale(500.0, 500.0);

        cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
        cr.paint();

        cr.set_line_width(0.05);

        // border
        cr.set_source_rgb(0.3, 0.3, 0.3);
        cr.rectangle(0.0, 0.0, 1.0, 1.0);
        cr.stroke();

        cr.set_line_width(0.03);

        // draw circle
        cr.arc(0.5, 0.5, 0.4, 0.0, PI_2);
        cr.stroke();


        // mouth
        let mouth_top = 0.68;
        let mouth_width = 0.38;

        let mouth_dx = 0.10;
        let mouth_dy = 0.10;

        cr.move_to( 0.50 - mouth_width/2.0, mouth_top);
        cr.curve_to(0.50 - mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_width/2.0, mouth_top);

        println!("Extents: {}", cr.fill_extents());

        cr.stroke();

        let eye_y = 0.38;
        let eye_dx = 0.15;
        cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI_2);
        cr.fill();

        cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI_2);
        cr.fill();
    });

    drawable(500, 500, |cr| {
        cr.scale(500.0, 500.0);

        cr.select_font_face("Sans",
                              cairo::enums::FontSlantNormal,
                              cairo::enums::FontWeightNormal);
        cr.set_font_size(0.35);

        cr.move_to(0.04, 0.53);
        cr.show_text("Hello");

        cr.move_to(0.27, 0.65);
        cr.text_path("void");
        cr.set_source_rgb(0.5, 0.5, 1.0);
        cr.fill_preserve();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(0.01);
        cr.stroke();

        cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        cr.arc(0.04, 0.53, 0.02, 0.0, PI_2);
        cr.arc(0.27, 0.65, 0.02, 0.0, PI_2);
        cr.fill();
    });

    gtk::main();
}

pub fn drawable(width: i32, height: i32, draw_fn: |cairo::Context|){
    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    let drawing_area = box DrawingArea::new().unwrap();

    drawing_area.connect(signals::Draw::new(draw_fn));

    window.set_default_size(width, height);

    window.connect(signals::DeleteEvent::new(|_|{
        gtk::main_quit();
        true
    }));
    window.add(&*drawing_area);
    window.show_all();
}