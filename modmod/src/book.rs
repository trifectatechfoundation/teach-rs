use std::path::PathBuf;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn builder(title: &str) -> BookBuilder {
        BookBuilder {
            title: title.to_string(),
            chapters: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub subsections: Vec<SubSection>,
}

#[derive(Debug)]
pub struct SubSection {
    pub content: PathBuf,
}

pub struct BookBuilder {
    title: String,
    chapters: Vec<Chapter>,
}

impl BookBuilder {
    pub fn chapter(&mut self, title: &str) -> ChapterBuilder {
        ChapterBuilder {
            book_builder: self,
            title: title.to_string(),
            sections: vec![],
        }
    }

    pub fn build(self) -> Book {
        let BookBuilder { title, chapters } = self;
        Book { title, chapters }
    }
}

pub struct ChapterBuilder<'bb> {
    book_builder: &'bb mut BookBuilder,
    title: String,
    sections: Vec<Section>,
}

impl<'bb> ChapterBuilder<'bb> {
    pub fn add(self) -> &'bb mut BookBuilder {
        let ChapterBuilder {
            book_builder,
            title,
            sections,
        } = self;
        book_builder.chapters.push(Chapter { title, sections });
        book_builder
    }

    pub fn section<'cb>(&'cb mut self, title: &str) -> SectionBuilder<'bb, 'cb> {
        SectionBuilder {
            chapter_builder: self,
            title: title.to_string(),
            subsections: vec![],
        }
    }
}

pub struct SectionBuilder<'bb, 'cb> {
    chapter_builder: &'cb mut ChapterBuilder<'bb>,
    title: String,
    subsections: Vec<SubSection>,
}

impl<'bb, 'cb> SectionBuilder<'bb, 'cb> {
    pub fn add(self) -> &'cb mut ChapterBuilder<'bb> {
        let SectionBuilder {
            chapter_builder,
            title,
            subsections,
        } = self;
        chapter_builder
            .sections
            .push(Section { title, subsections });
        chapter_builder
    }

    pub fn subsection(&mut self, content: PathBuf) {
        self.subsections.push(SubSection { content });
    }
}
