use std::fs;
use std::thread;
use slint::SharedString;

slint::slint! {
    import { TextEdit, Button, HorizontalBox, VerticalBox } from "std-widgets.slint";

    export component NoteinWindow inherits Window {
        title: "notein";
        preferred-width: 700px;
        preferred-height: 500px;

        // Variabel untuk menyimpan isi teks
        in-out property <string> text_content;
        
        callback open_file();
        callback save_file();

        VerticalBox {
            padding: 5px;
            HorizontalBox {
                alignment: start;
                Button { 
                    text: "Buka File"; 
                    clicked => { root.open_file() }
                }
                Button { 
                    text: "Simpan File"; 
                    clicked => { root.save_file() }
                }
            }
            TextEdit {
                // OPTIMASI: Gunakan two-way binding agar tidak lag saat mengetik
                text <=> root.text_content;
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = NoteinWindow::new()?;

    // --- Logika "Buka File" ---
    let app_weak = app.as_weak();
    app.on_open_file(move || {
        let app = app_weak.unwrap();
        
        // Membuka dialog (OS native)
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt"])
            .pick_file() 
        {
            let app_weak_thread = app.as_weak();
            
            // OPTIMASI: Pindahkan proses baca file besar ke background thread
            thread::spawn(move || {
                if let Ok(content) = fs::read_to_string(path) {
                    // Update UI kembali ke Main Thread dengan aman
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_thread.upgrade() {
                            app.set_text_content(SharedString::from(content));
                        }
                    });
                }
            });
        }
    });

    // --- Logika "Simpan File" ---
    let app_weak2 = app.as_weak();
    app.on_save_file(move || {
        let app = app_weak2.unwrap();
        // Ambil teks sebelum masuk ke thread
        // Ambil teks (SharedString di-clone secara dangkal, sangat cepat)
        let text = app.get_text_content(); 
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt"])
            .set_file_name("catatan_baru.txt")
            .save_file() 
        {
            // OPTIMASI: Simpan file di background thread menggunakan SharedString langsung
            thread::spawn(move || {
                let _ = fs::write(path, text.as_str());
            });
        }
    });

    app.run()
}