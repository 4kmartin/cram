use crate::error_handling::build_io_error;
use std::path::Path;

pub(crate) fn get_path_relative_to_cwd(path: &Path) -> std::io::Result<&Path> {
    let cwd = match std::env::current_dir() {
        Ok(v) => Ok(v),
        Err(_) => Err(build_io_error("Current Directory Not Found")),
    }?;
    match path.strip_prefix(cwd) {
        Ok(v) => Ok(v),
        Err(_) => Err(build_io_error("failed to costruct target")),
    }
}

pub(crate) fn build_link(config_file: &Path) -> std::io::Result<Box<Path>> {
    let dest = get_path_relative_to_cwd(config_file)?;
    let home = std::env::home_dir().ok_or(build_io_error("couldn't get home directory"))?;
    Ok(home.join(dest).into_boxed_path())
}

pub(crate) fn resolve_path(path: &str) -> std::io::Result<Box<Path>> {
    println!("--- resolve path ---");
    if path.starts_with("~") {
        println!("starts with ~");
        let b = path
            .replace("~", "")
            .replacen("/", "", 1)
            .replacen("\\", "", 1);
        let a = Path::new(&b);
        println!("less ~ : {}", a.display());
        std::env::home_dir()
            .and_then(|x| {
                println!("home: {}", x.display());
                let y = Some(x.join(a).into_boxed_path());
                println!("joined path {:#?}", &y);
                y
            })
            .ok_or(build_io_error(
                "Failed to get home directory. Is this a valid session?",
            ))
    } else if path.starts_with(".") {
        println!("starts with . ");
        let b = path.replacen(".", "", 1);
        let a = Path::new(&b);
        std::env::current_dir().and_then(|x| Ok(x.join(a).into_boxed_path()))
    } else {
        Ok(Path::new(path).to_path_buf().into_boxed_path())
    }
}
