mod steganography;

use gtk::{Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, Entry, Inhibit, Label, Orientation, Switch};
use gtk::prelude::*;

use steganography::{encrypt, decrypt};

fn main() {
    let application = Application::new(
        Some("xyz.rabuu.steganos"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("steganos")
        .build();


    let global_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(40)
        .build();


    let mode_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(30)
        .spacing(10)
        .halign(Align::Center)
        .build();
    let mode_label_encrypt = Label::builder()
        .label("encrypt")
        .build();
    let mode_label_decrypt = Label::builder()
        .label("decrypt")
        .build();
    let mode_switch = Switch::builder()
        .build();
    mode_box.append(&mode_label_encrypt);
    mode_box.append(&mode_switch);
    mode_box.append(&mode_label_decrypt);


    let eom_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(15)
        .halign(Align::Center)
        .visible(false)
        .build();
    let eom_entry = Entry::builder()
        .placeholder_text("EOM (default is: *[END]*)")
        .build();
    let include_eom = CheckButton::builder()
        .label("Include EOM")
        .build();
    eom_box.append(&eom_entry);
    eom_box.append(&include_eom);


    let entry_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(Align::Center)
        .build();
    let key_entry = Entry::builder()
        .placeholder_text("Key")
        .width_chars(50)
        .build();
    let msg_entry = Entry::builder()
        .placeholder_text("Message")
        .width_chars(50)
        .build();
    let msg_entry_clone = msg_entry.clone();
    entry_box.append(&key_entry);
    entry_box.append(&msg_entry);


    let file_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(Align::Center)
        .homogeneous(true)
        .build();
    let input_entry = Entry::builder()
        .placeholder_text("Input file path")
        .width_chars(40)
        .build();
    let output_entry = Entry::builder()
        .placeholder_text("Output file path")
        .width_chars(40)
        .build();
    file_box.append(&input_entry);
    file_box.append(&output_entry);


    let run_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .build();
    let run_btn = Button::builder()
        .label("Run")
        .build();
    run_box.append(&run_btn);


    global_box.append(&mode_box);
    global_box.append(&eom_box);
    global_box.append(&entry_box);
    global_box.append(&file_box);
    global_box.append(&run_box);

    window.set_child(Some(&global_box));

    mode_switch.connect_state_set(move |_, state| {
        if state {
            eom_box.show();
            msg_entry.set_text("");
            msg_entry.set_editable(false);
        } else {
            eom_box.hide();
            msg_entry.set_editable(true);
        }
        Inhibit(false)
    });

    run_btn.connect_clicked(move |_| {
        if mode_switch.state() { // decrypt
            let image_path = &input_entry.text()[..];
            let key = &key_entry.text()[..];
            let mut eom = &eom_entry.text()[..];
            let include_eom = include_eom.is_active();

            if eom.is_empty() {
                eom = "*[END]*";
            }

            let decrypted_msg = decrypt(image_path, key, eom, include_eom).expect("Decryption failed");
            msg_entry_clone.set_text(&decrypted_msg[..]);

        } else { // encrypt
            let message = &msg_entry_clone.text()[..];
            let key = &key_entry.text()[..];
            let input_path = &input_entry.text()[..];
            let output_path = &output_entry.text()[..];

            let encrypted_img = encrypt(message, key, input_path).expect("Encryption failed");
            encrypted_img.save(output_path).expect("Saving image failed");
        }
    });

    window.show();

}
