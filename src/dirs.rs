use directories::ProjectDirs;
use miette::{Result, Report};
use once_cell::sync::OnceCell;

static CELL: OnceCell<ProjectDirs> = OnceCell::new();

pub fn get_project_dirs() -> Result<&'static ProjectDirs> {
    CELL.get_or_try_init(|| {
        let dirs = ProjectDirs::from("io.github", "SkyfallWasTaken", "Snowflake");
        match dirs {
            Some(dirs) => Ok(dirs),
            _ => Err(Report::msg("couldn't read project dirs"))
        }
    })
}
