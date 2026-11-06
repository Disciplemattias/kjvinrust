pub struct Book {
    pub name: &'static str,
    pub chapters: Vec<Vec<&'static str>>,
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
