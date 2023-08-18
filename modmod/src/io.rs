use std::{fs::{self, File}, path::Path, io, fmt};

use error_stack::{Context, IntoReport, ResultExt, Result};
use fs_extra::dir::DirContent;

pub fn create_dir_all<C: Context + Default>(path: impl AsRef<Path>) -> Result<(), C> {
    let path = path.as_ref();
    fs::create_dir_all(path)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Error creating directory at path {path}",
                path = path.to_string_lossy()
            )
        })
        .change_context(C::default())
}

pub fn read_to_string<C: Context + Default>(path: impl AsRef<Path>) -> Result<String, C> {
    let path = path.as_ref();

    fs::read_to_string(path)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Error reading file at path {path}",
                path = path.to_string_lossy()
            )
        })
        .change_context(C::default())
}

pub fn create_file<C: Context + Default>(path: impl AsRef<Path>) -> Result<File, C> {
    let path = path.as_ref();

    File::create(path)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Error creating file at path {path}",
                path = path.to_string_lossy()
            )
        })
        .change_context(C::default())
}

pub fn get_dir_content<C: Context + Default>(path: impl AsRef<Path>) -> Result<DirContent, C> {
    let path = path.as_ref();
    fs_extra::dir::get_dir_content(path)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Error getting contents of directory at path {path}",
                path = path.to_string_lossy()
            )
        })
        .change_context(C::default())
}

pub fn copy<C: Context + Default>(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), C> {
    let from = from.as_ref();
    let to = to.as_ref();
    fs::copy(from, to)
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Error copying file from {from} to {to}",
                from = from.to_string_lossy(),
                to = to.to_string_lossy()
            )
        })
        .change_context(C::default())?;
    Ok(())
}

pub fn write_fmt<C: Context + Default, W: io::Write>(
    mut dest: W,
    fmt: fmt::Arguments,
) -> Result<(), C> {
    dest.write_fmt(fmt)
        .into_report()
        .change_context(C::default())
}

pub fn write_all<C: Context + Default, W: io::Write>(
    mut dest: W,
    content: impl AsRef<[u8]>,
) -> Result<(), C> {
    dest.write_all(content.as_ref())
        .into_report()
        .change_context(C::default())
}
