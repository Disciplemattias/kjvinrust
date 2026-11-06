pub struct Book {
    pub name: &'static str,
    pub chapters: Vec<Vec<&'static str>>,
}

#[derive(Debug, PartialEq)]
pub struct SearchHit {
    pub book: usize,
    pub chapter: usize,
    pub verse: usize,
    pub occurrences: usize,
}

/// Byte ranges in the original text, suitable for highlighting without changing it.
/// Punctuation (including apostrophes and hyphens) separates words.
pub fn word_ranges(text: &str, word: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    if word.is_empty() {
        return ranges;
    }
    let mut start = None;
    for (index, ch) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        if ch.is_alphanumeric() {
            start.get_or_insert(index);
        } else if let Some(begin) = start.take()
            && text[begin..index].to_lowercase() == word
        {
            ranges.push(begin..index);
        }
    }
    ranges
}

pub fn search(books: &[Book], query: &str) -> Result<Vec<SearchHit>, &'static str> {
    let word = query.trim().to_lowercase();
    if word.is_empty() || !word.chars().all(char::is_alphanumeric) {
        return Err("Enter one word, such as sword. Phrase search is not available yet.");
    }
    let mut hits = Vec::new();
    for (book, content) in books.iter().enumerate() {
        for (chapter, verses) in content.chapters.iter().enumerate() {
            for (verse, text) in verses.iter().enumerate() {
                let occurrences = word_ranges(text, &word).len();
                if occurrences > 0 {
                    hits.push(SearchHit {
                        book,
                        chapter,
                        verse,
                        occurrences,
                    });
                }
            }
        }
    }
    Ok(hits)
}

pub fn load() -> Vec<Book> {
    let mut books: Vec<Book> = Vec::new();
    for line in include_str!("../data/verses.tsv").lines() {
        let mut fields = line.splitn(4, '\t');
        let name = fields.next().expect("book name");
        let chapter: usize = fields.next().unwrap().parse().unwrap();
        let verse: usize = fields.next().unwrap().parse().unwrap();
        let text = fields.next().expect("verse text");
        if books.last().is_none_or(|book| book.name != name) {
            books.push(Book {
                name,
                chapters: Vec::new(),
            });
        }
        let book = books.last_mut().unwrap();
        if chapter == book.chapters.len() + 1 {
            book.chapters.push(Vec::new());
        }
        assert_eq!(chapter, book.chapters.len(), "chapter sequence");
        let verses = book.chapters.last_mut().unwrap();
        assert_eq!(verse, verses.len() + 1, "verse sequence");
        verses.push(text);
    }
    books
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_words_counts_and_validation() {
        let books = vec![Book {
            name: "Sample",
            chapters: vec![vec![
                "Sword, SWORD! swords sword’s sword-bearer.",
                "No match.",
                "éSword sword",
            ]],
        }];
        let hits = search(&books, "  SwOrD  ").unwrap();
        assert_eq!(
            hits,
            vec![
                SearchHit {
                    book: 0,
                    chapter: 0,
                    verse: 0,
                    occurrences: 4
                },
                SearchHit {
                    book: 0,
                    chapter: 0,
                    verse: 2,
                    occurrences: 1
                },
            ]
        );
        assert!(search(&books, "absent").unwrap().is_empty());
        for invalid in ["", "   ", "two words", "sword*", "."] {
            assert!(search(&books, invalid).is_err());
        }
        let text = "é Sword’s sword";
        let matched: Vec<_> = word_ranges(text, "sword")
            .into_iter()
            .map(|r| &text[r])
            .collect();
        assert_eq!(matched, ["Sword", "sword"]);
    }

    #[test]
    fn sword_search_finds_known_verses_in_bible_order() {
        let books = load();
        let hits = search(&books, "sword").unwrap();
        // Cross-checked against an independent regex scan of the bundled TSV.
        assert_eq!(hits.len(), 384);
        assert_eq!(hits.iter().map(|hit| hit.occurrences).sum::<usize>(), 424);
        assert_eq!((hits[0].book, hits[0].chapter, hits[0].verse), (0, 2, 23));
        assert!(
            hits.iter()
                .any(|h| (h.book, h.chapter, h.verse) == (48, 5, 16))
        );
        assert!(hits.windows(2).all(|pair| {
            (pair[0].book, pair[0].chapter, pair[0].verse)
                < (pair[1].book, pair[1].chapter, pair[1].verse)
        }));
        assert_eq!(hits, search(&books, "SWORD").unwrap());
    }

    #[test]
    fn complete_bible_and_known_passages() {
        let books = load();
        assert_eq!(books.len(), 66);
        assert_eq!(books.iter().map(|b| b.chapters.len()).sum::<usize>(), 1189);
        assert_eq!(
            books
                .iter()
                .flat_map(|b| &b.chapters)
                .map(Vec::len)
                .sum::<usize>(),
            31102
        );
        assert_eq!(
            books[0].chapters[0][0],
            "In the beginning God created the heaven and the earth."
        );
        assert_eq!(
            books[65].chapters[21][20],
            "The grace of our Lord Jesus Christ be with you all. Amen."
        );
        assert!(books[42].chapters[2][15].starts_with("For God so loved the world"));
        // Gutenberg includes continuation paragraphs without verse numbers.
        assert!(books[0].chapters[30][47].ends_with("Therefore was the name of it called Galeed;"));
        assert!(books[0].chapters[30][48].starts_with("And Mizpah;"));
        assert!(books[0].chapters[49][25].ends_with("in Egypt."));
        assert_eq!(books[18].chapters.len(), 150);
        assert_eq!(books[18].chapters[118].len(), 176);
    }
}
