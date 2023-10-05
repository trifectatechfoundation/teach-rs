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

#[derive(Debug)]
pub struct SlidesPackage<'track> {
    /// Name of the package, corresponds to the name of the track
    name: &'track str,
    decks: Vec<SlideDeck<'track>>,
}

impl<'track> SlidesPackage<'track> {
    pub fn builder(name: &'track str) -> SlidesPackageBuilder<'track> {
        SlidesPackageBuilder {
            package: SlidesPackage {
                name,
                decks: vec![],
            },
        }
    }

    pub fn render(&self, out_dir: impl AsRef<Path>) -> Result<(), RenderSlidesError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SlideDeck<'track> {
    /// Name of the slide deck, corresponds to the name of the unit in the module
    name: &'track str,
    template: &'track Path,
    sections: Vec<Section<'track>>,
}

#[derive(Debug)]
pub struct Section<'track> {
    content: &'track Path,
    objectives: Vec<&'track str>,
    summary: Vec<&'track str>,
    further_reading: Vec<&'track str>,
}

pub struct SlidesPackageBuilder<'track> {
    package: SlidesPackage<'track>,
}

impl<'track> SlidesPackageBuilder<'track> {
    pub fn deck(
        &mut self,
        name: &'track str,
        template: &'track Path,
    ) -> SlideDeckBuilder<'track, '_> {
        SlideDeckBuilder {
            package_builder: self,
            slide_deck: SlideDeck {
                name,
                template,
                sections: vec![],
            },
        }
    }

    pub fn build(self) -> SlidesPackage<'track> {
        self.package
    }
}

pub struct SlideDeckBuilder<'track, 'p> {
    package_builder: &'p mut SlidesPackageBuilder<'track>,
    slide_deck: SlideDeck<'track>,
}

impl<'track, 'p> SlideDeckBuilder<'track, 'p> {
    pub fn section(&mut self, content: &'track Path) -> SlidesSectionBuilder<'track, 'p, '_> {
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

    pub fn add(self) -> &'p mut SlidesPackageBuilder<'track> {
        self.package_builder.package.decks.push(self.slide_deck);
        self.package_builder
    }
}

pub struct SlidesSectionBuilder<'track, 'p, 'd> {
    deck_builder: &'d mut SlideDeckBuilder<'track, 'p>,
    section: Section<'track>,
}

impl<'track, 'p, 'd> SlidesSectionBuilder<'track, 'p, 'd> {
    pub fn objective(&mut self, objective: &'track str) {
        self.section.objectives.push(objective);
    }

    pub fn summary(&mut self, summary: &'track str) {
        self.section.summary.push(summary);
    }

    pub fn further_reading(&mut self, further_reading: &'track str) {
        self.section.further_reading.push(further_reading);
    }

    pub fn add(self) -> &'d mut SlideDeckBuilder<'track, 'p> {
        self.deck_builder.slide_deck.sections.push(self.section);
        self.deck_builder
    }
}
