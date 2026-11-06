# KJV in Rust

First milestone: an offline Linux desktop KJV reader. Select a book and chapter,
move between chapters with Previous/Next, select and copy text, and adjust text
size or theme under View. Help includes the original source and its license.

## Word and phrase search

Type a word such as `sword` or a phrase such as `word of God` in the search field at the top and press Enter or
click Search. Results appear in the same window, in Bible order, with matching
words highlighted. The summary distinguishes total occurrences from matching
verses: a verse containing the word twice is listed once but counts twice.

Search is case-insensitive and matches whole words: `sword` matches `Sword`
and `sword,`, but not `swords`. Punctuation, including apostrophes and hyphens,
separates words, so `sword’s` also matches. Phrases match consecutive whole words
in the same verse; they do not span verse boundaries. Punctuation between words
is ignored, so `word of God` also matches `word, of God`. Extra spaces are ignored.
Enter words separated by spaces, without quotation marks or punctuation.
Wildcard searches are not implemented. Overlapping occurrences are counted:
`a a` occurs twice in `a a a`, with the overlapping text highlighted once.

Results are shown 50 verses per page; use Previous results / Next results to
browse every match. Click a verse reference to open its chapter and scroll to
the verse. Back to results returns to the result list; Read chapter returns to
reading without selecting a result. Book/chapter navigation also returns to reading.
All searches run locally without an internet connection.

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
