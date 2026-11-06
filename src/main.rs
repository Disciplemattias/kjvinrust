mod bible;

use eframe::egui;

struct Reader {
    books: Vec<bible::Book>,
    book: usize,
    chapter: usize,
    text_size: f32,
    about: bool,
    source: bool,
    query: String,
    searched_query: String,
    hits: Vec<bible::SearchHit>,
    search_error: Option<&'static str>,
    show_results: bool,
    results_page: usize,
    jump_to_verse: Option<usize>,
}

impl Default for Reader {
    fn default() -> Self {
        Self {
            books: bible::load(),
            book: 0,
            chapter: 0,
            text_size: 22.0,
            about: false,
            source: false,
            query: String::new(),
            searched_query: String::new(),
            hits: Vec::new(),
            search_error: None,
            show_results: false,
            results_page: 0,
            jump_to_verse: None,
        }
    }
}

impl Reader {
    fn run_search(&mut self) {
        match bible::search(&self.books, &self.query) {
            Ok(hits) => {
                self.hits = hits;
                self.searched_query = self
                    .query
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_lowercase();
                self.search_error = None;
                self.show_results = true;
                self.results_page = 0;
            }
            Err(error) => self.search_error = Some(error),
        }
    }

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
                ui.label("Search the KJV");
                let input = ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Word or phrase, e.g. word of God")
                        .desired_width(240.0),
                );
                let submitted = input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Search").clicked() || submitted {
                    self.run_search();
                }
                if !self.searched_query.is_empty()
                    && !self.show_results
                    && ui.button("Back to results").clicked()
                {
                    self.show_results = true;
                }
                if self.show_results && ui.button("Read chapter").clicked() {
                    self.show_results = false;
                }
            });
            if let Some(error) = self.search_error {
                ui.colored_label(ui.visuals().error_fg_color, error);
            }
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
            if old_location != (self.book, self.chapter) {
                self.show_results = false;
            }
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
            if self.show_results {
                let occurrences: usize = self.hits.iter().map(|hit| hit.occurrences).sum();
                ui.heading(format!("Results for ‘{}’", self.searched_query));
                ui.label(format!(
                    "{occurrences} occurrences in {} verses · Consecutive whole words · Case-insensitive",
                    self.hits.len()
                ));
                if self.hits.is_empty() {
                    ui.add_space(16.0);
                    ui.label("No matching verses. Try another word or phrase.");
                    return;
                }
                const PAGE_SIZE: usize = 50;
                let pages = self.hits.len().div_ceil(PAGE_SIZE);
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(self.results_page > 0, egui::Button::new("Previous results"))
                        .clicked()
                    {
                        self.results_page -= 1;
                    }
                    ui.label(format!("Page {} of {pages}", self.results_page + 1));
                    if ui
                        .add_enabled(
                            self.results_page + 1 < pages,
                            egui::Button::new("Next results"),
                        )
                        .clicked()
                    {
                        self.results_page += 1;
                    }
                });
                egui::ScrollArea::vertical()
                    .id_salt(("results", &self.searched_query, self.results_page))
                    .show(ui, |ui| {
                        for hit in self
                            .hits
                            .iter()
                            .skip(self.results_page * PAGE_SIZE)
                            .take(PAGE_SIZE)
                        {
                            let book = &self.books[hit.book];
                            if ui
                                .link(format!(
                                    "{} {}:{} — Open chapter",
                                    book.name,
                                    hit.chapter + 1,
                                    hit.verse + 1
                                ))
                                .clicked()
                            {
                                self.book = hit.book;
                                self.chapter = hit.chapter;
                                self.jump_to_verse = Some(hit.verse);
                                self.show_results = false;
                            }
                            ui.add(
                                egui::Label::new(highlighted_text(
                                    book.chapters[hit.chapter][hit.verse],
                                    &self.searched_query,
                                    self.text_size,
                                    ui.visuals().text_color(),
                                ))
                                .wrap()
                                .selectable(true),
                            );
                            ui.add_space(10.0);
                            ui.separator();
                        }
                    });
                return;
            }
            let mut scroll = egui::ScrollArea::vertical().id_salt("reading");
            if old_location != (self.book, self.chapter) {
                scroll = scroll.vertical_scroll_offset(0.0);
            }
            scroll.show(ui, |ui| {
                // Keep the reading column within the viewport when text or window size changes.
                ui.set_max_width(ui.available_width().min(900.0));
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
                    let response = ui.add(
                        egui::Label::new(highlighted_text(
                            &format!("{}  {}", index + 1, text),
                            &self.searched_query,
                            self.text_size,
                            ui.visuals().text_color(),
                        ))
                        .wrap()
                        .selectable(true),
                    );
                    if self.jump_to_verse == Some(index) {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                    ui.add_space(10.0);
                }
            });
            self.jump_to_verse = None;
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

fn highlighted_text(
    text: &str,
    word: &str,
    size: f32,
    color: egui::Color32,
) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let normal = egui::TextFormat {
        font_id: egui::FontId::proportional(size),
        color,
        ..Default::default()
    };
    let highlight = egui::TextFormat {
        background: egui::Color32::from_rgb(255, 220, 110),
        color: egui::Color32::BLACK,
        ..normal.clone()
    };
    let mut end = 0;
    for range in bible::match_ranges(text, word) {
        if range.start > end {
            job.append(&text[end..range.start], 0.0, normal.clone());
        }
        job.append(
            &text[range.start.max(end)..range.end],
            0.0,
            highlight.clone(),
        );
        end = range.end;
    }
    job.append(&text[end..], 0.0, normal);
    job
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
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::ThemePreference::Light);
            cc.egui_ctx.set_zoom_factor(2.3);
            Ok(Box::new(Reader::default()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_phrase_highlights_preserve_text() {
        let text = "é a a a end";
        let job = highlighted_text(text, "a a", 21.0, egui::Color32::WHITE);
        assert_eq!(job.text, text);
        let highlighted: String = job
            .sections
            .iter()
            .filter(|section| section.format.background != egui::Color32::TRANSPARENT)
            .map(|section| &job.text[section.byte_range.clone()])
            .collect();
        assert_eq!(highlighted, "a a a");
    }
}
