use directories::ProjectDirs;
use miette::{Report, Result};
use once_cell::sync::OnceCell;

static CELL: OnceCell<ProjectDirs> = OnceCell::new();

pub fn get_project_dirs() -> Result<&'static ProjectDirs> {
    CELL.get_or_try_init(|| {
        let dirs = ProjectDirs::from("io.github", "SkyfallWasTaken", "Snowdrop");
        match dirs {
            Some(dirs) => Ok(dirs),
            _ => Err(Report::msg("couldn't read project dirs")),
        }
    })
}
