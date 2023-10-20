#![allow(dead_code)]
use std::fmt::{self, Write};
use std::path::Path;

use error_stack::Result;
use serde_json::Value as JsonValue;

type JsonObject = serde_json::Map<String, JsonValue>;

use crate::{
    io::{PathExt, WriteExt},
    to_prefixed_tag, to_tag,
};

const PACKAGE_JSON_CONTENT_STUB: &str = include_str!("../include/slides/package.json");

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
        let mut package_json: JsonObject = serde_json::from_str(PACKAGE_JSON_CONTENT_STUB).unwrap();
        package_json.insert("name".into(), to_tag(self.name).into());
        let mut package_scripts = JsonObject::new();

        let output_dir = out_dir.as_ref();
        let slides_output_dir = output_dir.join("slides");
        slides_output_dir.create_dir_all()?;

        let slide_images_dir = slides_output_dir.join("images");
        slide_images_dir.create_dir_all()?;

        for (deck, i) in self.decks.iter().zip(1..) {
            let deck_output = {
                let mut o = slides_output_dir.join(to_prefixed_tag(deck.name, i));
                o.set_extension("md");
                o
            };
            let mut deck_file = deck_output.create_file()?;

            {
                let deck_output_str = deck_output
                    .strip_prefix(&slides_output_dir)
                    .unwrap()
                    .to_str()
                    .unwrap();

                package_scripts.insert(
                    format!("dev-{i}"),
                    format!("slidev {deck_output_str}").into(),
                );

                package_scripts.insert(
                    format!("build-{i}"),
                    format!("slidev build --out dist-{i} --base /slides/{i}/ {deck_output_str}")
                        .into(),
                );
                package_scripts.insert(
                    format!("export-{i}"),
                    format!("slidev export {deck_output_str}").into(),
                );
            }

            let template_content = deck.template.read_to_string()?;
            let mut unit_content = String::new();
            let mut unit_objectives = String::new();
            let mut unit_summary = String::new();

            for section in deck.sections.iter() {
                let topic_content = section.content.read_to_string()?;
                let topic_content = topic_content.trim();

                if !topic_content.starts_with("---") {
                    unit_content.write_str("---\n\n").unwrap();
                }
                unit_content.write_str(topic_content).unwrap();
                unit_content.write_str("\n").unwrap();

                for objective in section.objectives.iter() {
                    unit_objectives += &format!("- {}\n", objective.trim());
                }

                for item in section.summary.iter() {
                    unit_summary += &format!("- {}\n", item.trim());
                }

                section
                    .images
                    .iter()
                    .filter_map(|path| path.file_name().map(|name| (path, name)))
                    .try_for_each(|(path, name)| path.copy(slide_images_dir.join(name)))?;
            }

            let slides_content = template_content
                .replace("#[modmod:content]", &unit_content)
                .replace("#[modmod:objectives]", &unit_objectives)
                .replace("#[modmod:summary]", &unit_summary);

            deck_file.write_all(slides_content)?;
        }

        package_json.insert("scripts".into(), package_scripts.into());
        let package_json = serde_json::to_string_pretty(&package_json).unwrap();
        let package_json_file = slides_output_dir.join("package.json");
        let mut package_json_file = package_json_file.create_file()?;
        package_json_file.write_all(package_json)?;

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
    images: Vec<&'track Path>,
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
                images: vec![],
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

    pub fn image(&mut self, image: &'track Path) {
        self.section.images.push(image);
    }

    pub fn add(self) -> &'d mut SlideDeckBuilder<'track, 'p> {
        self.deck_builder.slide_deck.sections.push(self.section);
        self.deck_builder
    }
}
