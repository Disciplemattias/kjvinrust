# KJV in Rust

First milestone: an offline Linux desktop KJV reader. Select a book and chapter,
move between chapters with Previous/Next, select and copy text, and adjust text
size or theme under View. Help includes the original source and its license.
Word and phrase search are planned for the next milestone.

## Run

```bash
cd /home/mattias/projects/kjv-in-rust
source "$HOME/.cargo/env"
cargo run
```

For a distributable executable, run `cargo build --release`; the executable is
`target/release/kjv-in-rust`. All Bible data is embedded, so running it does not
require internet access or a specific working directory. Building initially
downloads Rust dependencies. Linux needs a graphical session and OpenGL support.
The UI uses eframe/egui 0.33 with X11 and Wayland support.

## Text and verification

`data/KJV.txt` is an unchanged copy of `/home/mattias/Downloads/KJV.txt`,
Project Gutenberg eBook #10 (October 29, 2024 edition). Its original notices
and full Project Gutenberg license remain included. Review that license before
redistributing the bundled text. This project is independent of SwordSearcher.

`scripts/import_kjv.py` converts this specific edition into `data/verses.tsv`.
It removes exact book headings and the Gutenberg front/back matter, joins wrapped
lines and continuation paragraphs, and separates inline verse references.
It preserves wording and punctuation; it does not modernize the text.
The import rejects missing verse numbers and unexpected book/chapter totals.

```bash
python3 scripts/import_kjv.py
python3 -m py_compile scripts/import_kjv.py
cargo test
cargo fmt --check
```

Tests check verse numbering, Bible totals, known passages, and continuation
paragraphs. This validates the import's structure, not an independent scholarly
comparison of every word against another KJV edition.

No services, cron jobs, firewall rules, or production paths are changed.
