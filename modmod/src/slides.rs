#![allow(dead_code)]
use std::fmt;
use std::path::{Path, PathBuf};

use error_stack::Result;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RenderSlidesError;

impl fmt::Display for RenderSlidesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render slides")
    }
}

impl error_stack::Context for RenderSlidesError {}

pub struct SlidesPackage {
    /// Name of the package, corresponds to the name of the track
    name: String,
    decks: Vec<SlideDeck>,
}

impl SlidesPackage {
    pub fn builder(name: &str) -> SlidesPackageBuilder {
        SlidesPackageBuilder {
            package: SlidesPackage {
                name: name.to_string(),
                decks: vec![],
            },
        }
    }

    pub fn render(&self, out_dir: impl AsRef<Path>) -> Result<(), RenderSlidesError> {
        todo!()
    }
}

pub struct SlideDeck {
    /// Name of the slide deck, corresponds to the name of the unit in the module
    name: String,
    template: PathBuf,
    sections: Vec<Section>,
}

pub struct Section {
    content: PathBuf,
    objectives: Vec<String>,
    summary: Vec<String>,
    further_reading: Vec<String>,
}

pub struct SlidesPackageBuilder {
    package: SlidesPackage,
}

impl SlidesPackageBuilder {
    pub fn deck(&mut self, name: &str, template: PathBuf) -> SlideDeckBuilder<'_> {
        SlideDeckBuilder {
            package_builder: self,
            slide_deck: SlideDeck {
                name: name.to_string(),
                template,
                sections: vec![],
            },
        }
    }

    pub fn build(self) -> SlidesPackage {
        self.package
    }
}

pub struct SlideDeckBuilder<'p> {
    package_builder: &'p mut SlidesPackageBuilder,
    slide_deck: SlideDeck,
}

impl<'p> SlideDeckBuilder<'p> {
    pub fn section(&mut self, content: PathBuf) -> SlidesSectionBuilder<'p, '_> {
        SlidesSectionBuilder {
            deck_builder: self,
            section: Section {
                content,
                objectives: vec![],
                summary: vec![],
                further_reading: vec![],
            },
        }
    }

    pub fn add(self) -> &'p mut SlidesPackageBuilder {
        self.package_builder.package.decks.push(self.slide_deck);
        self.package_builder
    }
}

pub struct SlidesSectionBuilder<'p, 'd> {
    deck_builder: &'d mut SlideDeckBuilder<'p>,
    section: Section,
}

impl<'p, 'd> SlidesSectionBuilder<'p, 'd> {
    pub fn objective(&mut self, objective: &str) {
        self.section.objectives.push(objective.to_string());
    }

    pub fn summary(&mut self, summary: String) {
        self.section.summary.push(summary);
    }

    pub fn further_reading(&mut self, further_reading: String) {
        self.section.further_reading.push(further_reading);
    }

    pub fn add(self) -> &'d mut SlideDeckBuilder<'p> {
        self.deck_builder.slide_deck.sections.push(self.section);
        self.deck_builder
    }
}
