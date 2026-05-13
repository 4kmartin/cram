use crate::configuration::{Config, load_config};
use crate::error_handling::{build_io_error, handle_io_error};
use crate::path_manipulation::{build_link, get_path_relative_to_cwd, resolve_path};
use crate::{FileList, create_symlink, naive_implementation};
use std::collections::HashMap;
use std::path::Path;

type TargetMap = HashMap<Box<Path>, Box<Path>>;

pub(crate) fn make_symlinks(paths: FileList) -> std::io::Result<()> {
    let config = load_config();
    if config.is_empty() {
        return naive_implementation::make_symlinks(paths);
    } else {
        let target_map = build_target_map(&config, &paths)?;
        for (target, link) in target_map {
            match create_symlink(&target, &link) {
                Ok(_) => (),
                Err(err) => {
                    handle_io_error(&err);
                    if err.kind() != std::io::ErrorKind::AlreadyExists {
                        return Err(err);
                    }
                }
            }
        }
        return Ok(());
    }
}

fn build_target_map(config: &Config, paths: &FileList) -> std::io::Result<TargetMap> {
    let mut target_map = TargetMap::new();
    for path in paths {
        let config_file = get_path_relative_to_cwd(path)?
            .to_str()
            .ok_or(build_io_error(
                "tried to process path containing invalid unicode",
            ))?;
        match config.get_target_for_config_file(config_file) {
            Some(target) => custom_mapping(&mut target_map, path, target)?,
            None => default_mapping(&mut target_map, path)?,
        };
    }
    return Ok(target_map);
}

fn custom_mapping(tm: &mut TargetMap, config_file: &Path, target: &str) -> std::io::Result<()> {
    let tp: Box<Path> = resolve_path(target)?;
    tm.insert(config_file.to_path_buf().into_boxed_path(), tp);
    Ok(())
}

fn default_mapping(tm: &mut TargetMap, config_file: &Path) -> std::io::Result<()> {
    let tp: Box<Path> = build_link(config_file)?;
    tm.insert(config_file.to_path_buf().into_boxed_path(), tp);

    Ok(())
}
