use crate::api::{error::Result, games::Progress};
use std::{
    fs::File,
    io::{Cursor, Write},
    path::Path,
};

/// Extracts a 7zip file to a directory.
///
/// # Errors
/// Returns an error if the 7zip file is invalid or the directory can't be written to.
///
/// # Panics
/// Panics if a the 7zip file doesn't have a single root directory.
#[allow(clippy::needless_pass_by_value, clippy::cognitive_complexity)]
pub fn extract_zip_with_password(
    bytes: &[u8],
    dest: &Path,
    password: &str,
    progress: Progress,
) -> Result<()> {
    let mut sz =
        sevenz_rust::SevenZReader::new(Cursor::new(bytes), bytes.len() as u64, password.into())?;
    let total_files = sz.archive().files.len();
    progress.set_denominator(total_files as u64);
    sz.for_each_entries(|entry, reader| {
        if entry.is_directory() {
            progress.increment_numerator();
            return Ok(true); // we create the directory before creating files; removing this will cause an error with `File::create`
        }

        let path = Path::new(entry.name()); // TODO: handle invalid paths; we don't really need to worry about this but it's a good habit
        let mut components = path.components();
        components.next();
        let path = components.as_path();
        let path = dest.join(path);

        let mut buf = [0u8; 1024];
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        let res = loop {
            let read_size = reader.read(&mut buf)?;
            if read_size == 0 {
                break Ok(true);
            }
            file.write_all(&buf[..read_size])?;
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            file.set_permissions(perms)?;
        }

        progress.increment_numerator();

        res
    })?;

    Ok(())
}
