use clap::{Parser, Subcommand};
use ignore::WalkBuilder;
use std::fs::write;
use std::path::Path;

#[cfg(feature = "configuration")]
mod configuration;
mod error_handling;
mod naive_implementation;
mod path_manipulation;

// Config File Management
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    //prepares current directory to be used as a cram store
    Init,
}

type FileList = Vec<Box<Path>>;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init) => initialize_directory()?,
        None => build()?,
    };
    Ok(())
}

fn initialize_directory() -> std::io::Result<()> {
    #[cfg(feature = "configuration")]
    write("cram.toml", "")?;
    write(".ignore", ".git\n.gitignore\n.ignore\ncram.toml")?;
    Ok(())
}

fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    println!("linking <{}> to <{}>", link.display(), target.display());
    #[cfg(windows)]
    if target.is_dir() {
        return std::os::windows::fs::symlink_dir(target, link);
    } else {
        return std::os::windows::fs::symlink_file(target, link);
    }

    #[cfg(unix)]
    return std::os::unix::fs::symlink(target, link);

    #[cfg(not(any(unix, windows)))]
    return Err(io::Error::new(io::ErrorKind::Other, "Unsupported Platform"));
}

fn get_contents_of_current_directory() -> Result<FileList, ignore::Error> {
    let mut out = Vec::new();
    let cwd = std::env::current_dir().unwrap();

    let walker = WalkBuilder::new(&cwd).max_depth(Some(1)).build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if path == cwd {
            continue;
        }

        out.push(path.to_path_buf().into_boxed_path());
    }

    return Ok(out);
}

fn build() -> std::io::Result<()> {
    let paths = get_contents_of_current_directory().unwrap();
    #[cfg(not(feature = "configuration"))]
    return naive_implementation::make_symlinks(paths);
    #[cfg(feature = "configuration")]
    return configuration::make_symlinks(paths);
}
