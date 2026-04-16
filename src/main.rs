use std::fs;
use slint::SharedString;

slint::slint! {
    import { TextEdit, Button, HorizontalBox, VerticalBox } from "std-widgets.slint";

    export component NoteinWindow inherits Window {
        title: "notein";
        preferred-width: 700px;
        preferred-height: 500px;

        // Variabel untuk menyimpan isi teks
        in-out property <string> text_content;
        
        // Sinyal ke Rust saat tombol diklik
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
                text: root.text_content;
                // Selalu update variabel saat user mengetik
                edited => { root.text_content = self.text; } 
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    // Inisialisasi jendela Notein
    let app = NoteinWindow::new()?;

    // --- Logika untuk tombol "Buka File" ---
    let app_weak = app.as_weak();
    app.on_open_file(move || {
        let app = app_weak.unwrap();
        // Munculkan jendela pilih file .txt
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt"])
            .pick_file() 
        {
            // Baca isi file dan tampilkan ke layar
            if let Ok(content) = fs::read_to_string(path) {
                app.set_text_content(SharedString::from(content));
            }
        }
    });

    // --- Logika untuk tombol "Simpan File" ---
    let app_weak2 = app.as_weak();
    app.on_save_file(move || {
        let app = app_weak2.unwrap();
        let text = app.get_text_content();
        
        // Munculkan jendela simpan file
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text", &["txt"])
            .set_file_name("catatan_baru.txt")
            .save_file() 
        {
            // Tulis isi teks dari layar ke file
            let _ = fs::write(path, text.as_str());
        }
    });

    // Jalankan aplikasi
    app.run()
}