mod bible;

use eframe::egui;

struct Reader {
    books: Vec<bible::Book>,
    book: usize,
    chapter: usize,
    text_size: f32,
    about: bool,
    source: bool,
}

impl Default for Reader {
    fn default() -> Self {
        Self {
            books: bible::load(),
            book: 0,
            chapter: 0,
            text_size: 21.0,
            about: false,
            source: false,
        }
    }
}

impl Reader {
    fn previous(&mut self) {
        if self.chapter > 0 {
            self.chapter -= 1;
        } else if self.book > 0 {
            self.book -= 1;
            self.chapter = self.books[self.book].chapters.len() - 1;
        }
    }

    fn next(&mut self) {
        if self.chapter + 1 < self.books[self.book].chapters.len() {
            self.chapter += 1;
        } else if self.book + 1 < self.books.len() {
            self.book += 1;
            self.chapter = 0;
        }
    }
}

impl eframe::App for Reader {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let old_location = (self.book, self.chapter);
        egui::TopBottomPanel::top("navigation").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.add(egui::Slider::new(&mut self.text_size, 14.0..=34.0).text("Text size"));
                    egui::widgets::global_theme_preference_buttons(ui);
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About KJV in Rust").clicked() {
                        self.about = true;
                        ui.close();
                    }
                    if ui.button("Source text and license").clicked() {
                        self.source = true;
                        ui.close();
                    }
                });
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Book");
                egui::ComboBox::from_id_salt("book")
                    .selected_text(self.books[self.book].name)
                    .show_ui(ui, |ui| {
                        for (index, book) in self.books.iter().enumerate() {
                            if ui
                                .selectable_value(&mut self.book, index, book.name)
                                .changed()
                            {
                                self.chapter = 0;
                            }
                        }
                    });
                ui.label("Chapter");
                egui::ComboBox::from_id_salt("chapter")
                    .selected_text((self.chapter + 1).to_string())
                    .show_ui(ui, |ui| {
                        for index in 0..self.books[self.book].chapters.len() {
                            ui.selectable_value(&mut self.chapter, index, (index + 1).to_string());
                        }
                    });
                if ui
                    .add_enabled(
                        self.book > 0 || self.chapter > 0,
                        egui::Button::new("Previous"),
                    )
                    .clicked()
                {
                    self.previous();
                }
                let last = self.book == self.books.len() - 1
                    && self.chapter == self.books[self.book].chapters.len() - 1;
                if ui.add_enabled(!last, egui::Button::new("Next")).clicked() {
                    self.next();
                }
            });
            ui.add_space(6.0);
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(format!(
                "King James Version  ·  {} {}  ·  {} verses  ·  Offline reader",
                self.books[self.book].name,
                self.chapter + 1,
                self.books[self.book].chapters[self.chapter].len()
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut scroll = egui::ScrollArea::vertical().id_salt("reading");
            if old_location != (self.book, self.chapter) {
                scroll = scroll.vertical_scroll_offset(0.0);
            }
            scroll.show(ui, |ui| {
                ui.set_max_width(900.0);
                ui.add_space(16.0);
                ui.heading(format!(
                    "{} {}",
                    self.books[self.book].name,
                    self.chapter + 1
                ));
                ui.add_space(16.0);
                for (index, text) in self.books[self.book].chapters[self.chapter]
                    .iter()
                    .enumerate()
                {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("{}  {}", index + 1, text))
                                .size(self.text_size),
                        )
                        .wrap()
                        .selectable(true),
                    );
                    ui.add_space(10.0);
                }
            });
        });
        egui::Window::new("About KJV in Rust")
            .open(&mut self.about)
            .show(ctx, |ui| {
                ui.label("KJV in Rust — version 0.1.0");
                ui.label("An offline Bible reader for Linux.");
                ui.label("66 books · 1,189 chapters · 31,102 verses");
                ui.label("Text: Project Gutenberg eBook #10, October 29, 2024 edition.");
                ui.label("The original text and license are bundled in data/KJV.txt.");
            });
        egui::Window::new("Source text and license")
            .open(&mut self.source)
            .default_size([700.0, 500.0])
            .show(ctx, |ui| {
                let lines: Vec<_> = include_str!("../data/KJV.txt").lines().collect();
                egui::ScrollArea::both().show_rows(ui, 18.0, lines.len(), |ui, range| {
                    for index in range {
                        ui.add(egui::Label::new(lines[index]).selectable(true));
                    }
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 760.0])
            .with_min_inner_size([480.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "KJV in Rust",
        options,
        Box::new(|_| Ok(Box::new(Reader::default()))),
    )
}
