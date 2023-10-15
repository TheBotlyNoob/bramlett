use rhai::plugin::*;

#[export_module]
pub mod functions {
    use std::path::PathBuf;

    #[rhai_fn(global)]
    pub fn parent(path: PathBuf) -> PathBuf {
        path.parent().unwrap().to_path_buf()
    }

    #[rhai_fn(global)]
    pub fn file_name(path: PathBuf) -> String {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }

    #[rhai_fn(global, return_raw)]
    pub fn copy_file(from: PathBuf, to: PathBuf) -> Result<(), Box<rhai::EvalAltResult>> {
        std::fs::copy(from, to).map_err(|e| rhai_err(format!("Failed to copy file: {e}")))?;
        Ok(())
    }

    #[rhai_fn(global, return_raw)]
    pub fn copy_dir(from: PathBuf, to: PathBuf) -> Result<(), Box<rhai::EvalAltResult>> {
        match std::fs::create_dir_all(&to) {
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(rhai_err(format!("Failed to create directory: {e}"))),
        };
        for entry in std::fs::read_dir(from)
            .map_err(|e| rhai_err(format!("Failed to read directory: {e}")))?
        {
            let entry =
                entry.map_err(|e| rhai_err(format!("Failed to read directory entry: {e}")))?;
            let path = entry.path();
            let new_path = to.join(path.file_name().unwrap());
            if path.is_dir() {
                copy_dir(path, new_path)?;
            } else {
                copy_file(path, new_path)?;
            }
        }
        Ok(())
    }

    fn rhai_err(s: String) -> Box<rhai::EvalAltResult> {
        Box::new(rhai::EvalAltResult::ErrorRuntime(s.into(), Position::NONE))
    }
}
