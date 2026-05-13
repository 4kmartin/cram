use crate::{
    FileList, create_symlink, error_handling::handle_io_error, path_manipulation::build_link,
};

pub(crate) fn make_symlinks(paths: FileList) -> std::io::Result<()> {
    for target in paths {
        let target = target
            .canonicalize()
            .expect("failed to determine full path for target");
        let link = build_link(target.as_path())?;
        match create_symlink(target.as_path(), &link) {
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
