use nix::unistd::{chown, Uid, Gid};

/// Permission engine for processing file permission modifications
pub fn modify_file_permissions(file_path: String) -> Result<String, String> {
    let uid = Some(Uid::from_raw(1000));
    let gid = Some(Gid::from_raw(1000));

    //CWE 732
    //SINK
    let _ = chown(file_path.as_str(), uid, gid);

    std::env::set_var("PERMISSION_TARGET", &file_path);

    Ok(format!("Chown completed on: {}", file_path))
}
