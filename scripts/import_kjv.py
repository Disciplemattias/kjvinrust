"""Import the bundled Gutenberg edition, retaining all verse continuations."""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
BOOKS = "Genesis|Exodus|Leviticus|Numbers|Deuteronomy|Joshua|Judges|Ruth|1 Samuel|2 Samuel|1 Kings|2 Kings|1 Chronicles|2 Chronicles|Ezra|Nehemiah|Esther|Job|Psalms|Proverbs|Ecclesiastes|Song of Solomon|Isaiah|Jeremiah|Lamentations|Ezekiel|Daniel|Hosea|Joel|Amos|Obadiah|Jonah|Micah|Nahum|Habakkuk|Zephaniah|Haggai|Zechariah|Malachi|Matthew|Mark|Luke|John|Acts|Romans|1 Corinthians|2 Corinthians|Galatians|Ephesians|Philippians|Colossians|1 Thessalonians|2 Thessalonians|1 Timothy|2 Timothy|Titus|Philemon|Hebrews|James|1 Peter|2 Peter|1 John|2 John|3 John|Jude|Revelation".split("|")


def import_text(source):
    body = source.split("*** START OF THE PROJECT GUTENBERG EBOOK THE KING JAMES VERSION OF THE BIBLE ***", 1)[1]
    body = body.split("*** END OF THE PROJECT GUTENBERG EBOOK", 1)[0]
    front, body = body.split("1:1 ", 1)
    headings = {line.strip() for line in front.splitlines() if line.strip()}
    headings.update({"Otherwise Called:", "Commonly Called:", "The Third Book of the Kings", "The Fourth Book of the Kings", "or", "The Preacher", "***"})
    # Remove only exact heading lines; preserve paragraphs continuing a verse.
    body = " ".join(line.strip() for line in ("1:1 " + body).splitlines()
                    if line.strip() and line.strip() not in headings)
    markers = list(re.finditer(r"\b(\d+):(\d+)\s+", body))
    rows = []
    book = -1
    previous = None
    for i, marker in enumerate(markers):
        chapter, verse = map(int, marker.groups())
        if (chapter, verse) == (1, 1):
            book += 1
        elif previous is None or not (
            (chapter == previous[0] and verse == previous[1] + 1)
            or (chapter == previous[0] + 1 and verse == 1)
        ):
            raise ValueError(f"Unexpected verse sequence: {previous} -> {chapter}:{verse}")
        end = markers[i + 1].start() if i + 1 < len(markers) else len(body)
        text = body[marker.end():end].strip()
        if not text:
            raise ValueError("Empty verse")
        rows.append((BOOKS[book], chapter, verse, text))
        previous = (chapter, verse)
    if len(rows) != 31102 or book != 65 or sum(v == 1 for _, _, v, _ in rows) != 1189:
        raise ValueError("Bible totals did not match; refusing to write an incomplete import")
    return rows


if __name__ == "__main__":
    rows = import_text((ROOT / "data/KJV.txt").read_text())
    (ROOT / "data/verses.tsv").write_text(
        "".join(f"{book}\t{chapter}\t{verse}\t{text}\n" for book, chapter, verse, text in rows)
    )
    print(f"Imported {len(rows):,} verses, 1,189 chapters, 66 books.")
