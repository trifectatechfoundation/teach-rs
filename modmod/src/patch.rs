use error_stack::{IntoReport, Result, ResultExt};
use similar::TextDiff;

use std::{
    fmt,
    fs::{self},
    io::{self, Read, Seek},
    path::Path,
};

use crate::io::PathExt;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct GenPatchError;

impl fmt::Display for GenPatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render exercises")
    }
}

impl error_stack::Context for GenPatchError {}
pub struct GenPatchOptions<N: AsRef<Path>, O: AsRef<Path>, P: AsRef<Path>> {
    pub new_dir: N,
    pub old_dir: O,
    pub patch_file: P,
}

#[non_exhaustive]
pub struct Patch {}

impl Patch {
    pub fn render<N: AsRef<Path>, O: AsRef<Path>, P: AsRef<Path>>(
        GenPatchOptions {
            new_dir,
            old_dir,
            patch_file,
        }: GenPatchOptions<N, O, P>,
    ) -> Result<(), GenPatchError> {
        use std::io::Write;
        let mut patch_file = std::fs::File::create(&patch_file).unwrap();

        'files: for new_file_path in new_dir.as_ref().get_dir_content()?.files {
            let relative_file_path = Path::new(&new_file_path)
                .strip_prefix(new_dir.as_ref())
                .unwrap();

            let mut new_file = new_file_path.open_file()?;
            let old_file_path = old_dir.as_ref().join(relative_file_path);

            let mut old_file = match fs::File::open(&old_file_path) {
                Ok(f) => f,
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    println!("File not found at {}", old_file_path.to_str().unwrap());
                    #[cfg(unix)]
                    {
                        fs::File::open("/dev/null").unwrap()
                    }
                    #[cfg(windows)]
                    {
                        fs::File::open("nul").unwrap()
                    }
                }
                Err(e) => {
                    return Err(e)
                        .into_report()
                        .change_context(GenPatchError::default())
                }
            };

            'text: {
                let mut new = String::new();
                let mut old = String::new();
                let Ok(_) = new_file
                    .read_to_string(&mut new)
                    .and_then(|_| old_file.read_to_string(&mut old))
                else {
                    new_file.rewind().expect("New file should be rewindable");
                    old_file.rewind().expect("Old file should be rewindable");
                    break 'text;
                };

                let diff = TextDiff::from_lines(&old, &new);
                if diff.ops().len() == 1 {
                    if let Some(similar::DiffOp::Equal { .. }) = diff.ops().first() {
                        println!(
                            "{} and {} are the same",
                            new_file_path.as_str(),
                            old_file_path.to_str().unwrap()
                        );
                        continue 'files;
                    }
                }

                write!(
                    patch_file,
                    "{}",
                    diff.unified_diff().header(
                        &format!("a/{}", relative_file_path.to_str().unwrap()),
                        &format!("b/{}", relative_file_path.to_str().unwrap()),
                    )
                )
                .unwrap();

                continue 'files;
            }

            'binary: {
                let mut new = Vec::new();
                let mut old = Vec::new();
                let Ok(_) = new_file
                    .read_to_end(&mut new)
                    .and_then(|_| old_file.read_to_end(&mut old))
                else {
                    new_file.rewind().expect("New file should be rewindable");
                    old_file.rewind().expect("Old file should be rewindable");
                    break 'binary;
                };

                let diff = TextDiff::from_lines(&old, &new);

                diff.unified_diff()
                    .header(
                        &format!("a/{}", relative_file_path.to_str().unwrap()),
                        &format!("b/{}", relative_file_path.to_str().unwrap()),
                    )
                    .to_writer(&mut patch_file)
                    .into_report()
                    .change_context(GenPatchError::default())?;

                continue 'files;
            }

            unreachable!("All types of files should have been handled");
        }
        Ok(())
    }
}
