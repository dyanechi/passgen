use super::*;
use std::cell::Cell;
use std::rc::Rc;

use super::*;
use gtk::glib::{self, clone};
use gtk::{prelude::*, Button, Orientation};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.dyanechi.passgen";

pub fn build() -> Application {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app
}

fn build_ui(app: &Application) {   
    let label_header = gtk::Label::builder()
        .label("Welcome to PSG - Password Generator!")
        .build();


    let (prefix, postfix) = (
        Rc::new(Cell::new(String::new())),
        Rc::new(Cell::new(String::new())),
    );

    let box_control = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .build();

    let input_prefix = gtk::Entry::builder()
        .placeholder_text("prefix")
        .halign(gtk::Align::Center)
        .margin(5)
        .build();

    let input_postfix = gtk::Entry::builder()
        .placeholder_text("postfix")
        .margin(5)
        .build();

    input_prefix.connect_text_notify(clone!(@strong prefix => move |entry| {
        prefix.set(entry.text().to_string());
        eprintln!("Editing finished! '{}'", entry.text());
    }));
    input_postfix.connect_text_notify(clone!(@strong postfix => move |entry| {
        postfix.set(entry.text().to_string());
        eprintln!("Editing finished! '{}'", entry.text());
    }));

    box_control.add(&input_prefix);
    box_control.add(&input_postfix);     
    


    let adjustment = gtk::Adjustment::new(16.0, 3.0, 256.0, 1.0, 10.0, 0.0);
    let label_set_passlen = gtk::Label::new(Some("Password Length"));
    let input_set_passlen = gtk::SpinButton::builder()
        .editable(true)
        .max_length(100)
        .numeric(true)
        .digits(0)
        .adjustment(&adjustment)
        .build();

    // let box_passlen = gtk::Box::new(Orientation::Vertical, 10);
    let box_passlen = gtk::FlowBox::builder()
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .orientation(Orientation::Horizontal)
        .column_spacing(50)
        .build();
    box_passlen.add(&label_set_passlen);
    box_passlen.add(&input_set_passlen);

    let button_genpass = Button::builder()
        .label("Generate Password")
        .build();

    let label_rand_pass = gtk::Label::builder()
        .label("<Password>")
        .margin(10)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::Char)
        .selectable(true)
        .build();

    let r_str = Rc::new(Cell::new(String::new()));
    button_genpass.connect_clicked(clone!(
        @strong r_str,
        @strong label_rand_pass as lrp,
        @strong input_set_passlen as lsp,
        @strong prefix,
        @strong postfix,
    => move |_| {
        let (pre, post) = (prefix.take(), postfix.take());
        let rpass = String::from(format!(
            "{}{}{}",
            pre,
            util::alphanum(lsp.value() as usize),
            post,
        ));
        prefix.set(pre);
        postfix.set(post);
        r_str.set(rpass.clone());
        lrp.set_label(rpass.as_str());
    }));

    // Build gtk container box 
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .can_focus(true)
        .focus_on_click(true)
        .margin(10)
        .build();
    gtk_box.add(&label_header);
    gtk_box.add(&box_control);
    gtk_box.add(&box_passlen);
    gtk_box.add(&button_genpass);
    gtk_box.add(&label_rand_pass);
    // gtk_box.add(&button_delete);
    // gtk_box.add(&btn_increment);
    // gtk_box.add(&btn_decrement);
    // gtk_box.add(&text);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PSG - Password Generator")
        .child(&gtk_box)
        .build();

    window.show_all();
}
