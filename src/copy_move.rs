use std::io;
use std::fs;
use std::path::Path;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    // Early return if source is a file (copy it directly to dst, no nesting)
    if src.is_file()
    {
        return fs::copy(src, dst).map(|_| ());
    }

    // Compute the target destination: dst / src's basename (e.g., dst/TestFolder1)
    let src_name = src.file_name().ok_or_else(||
        {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Source path has no file name (e.g., root directory)",
            )
        })?;

    let target_dst = dst.join(src_name);

    // Create the target destination directory (and parents) recursively
    fs::create_dir_all(&target_dst)?;

    // Read directory entries
    let entries: Vec<_> = fs::read_dir(src)?.collect::<io::Result<Vec<_>>>()?;

    // Process each entry
    entries.iter().try_for_each(|entry| -> io::Result<()>
        {
            let entry_path = entry.path();
            let file_name = entry.file_name();  // Safe: read_dir guarantees valid names
            let dest_path = target_dst.join(file_name);  // Use target_dst for nesting under src_name

            match entry.file_type()
            {
                Ok(ty) if ty.is_dir() =>
                    {
                        // Recursive copy for subdirectories (preserves full structure)
                        copy_dir_all(&entry_path, &dest_path)?;
                        Ok(())
                    }
                Ok(ty) if ty.is_file() =>
                    {
                        // Copy files
                        fs::copy(&entry_path, &dest_path).map(|_| ())?;
                        Ok(())
                    }
                Ok(_) =>
                    {
                        // Skip symlinks, sockets, etc. (add handling if needed)
                        Ok(())
                    }
                Err(e) => Err(e),
            }
        })?;
    Ok(())
}

fn move_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    // If the source is a file, move (rename) it or fallback to copy+delete
    if src.is_file()
    {
        return match fs::rename(src, dst)
        {
            Ok(_) => Ok(()),
            Err(_) =>
                {
                    fs::copy(src, dst)?;
                    fs::remove_file(src)?;
                    Ok(())
                }
        };
    }

    // Compute target directory: dst / src's basename
    let src_name = src.file_name().ok_or_else(||
        {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Source path has no file name (e.g., root directory)",
            )
        })?;
    let target_dst = dst.join(src_name);

    // Try to move the whole directory
    match fs::rename(src, &target_dst)
    {
        Ok(_) => Ok(()),
        Err(_) =>
            {
            // Fallback: create the destination directory
                fs::create_dir_all(&target_dst)?;

                // Functional style: process entries with iterator adapters
                fs::read_dir(src)?
                    .filter_map(Result::ok)
                    .map(|entry|
                        {
                            let entry_path = entry.path();
                            let file_name = entry.file_name();
                            let dest_path = target_dst.join(&file_name);

                            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                            {
                                move_dir_all(&entry_path, &target_dst)
                            }
                            else
                            {
                                match fs::rename(&entry_path, &dest_path)
                                {
                                    Ok(_) => Ok(()),
                                    Err(_) =>
                                        {
                                            fs::copy(&entry_path, &dest_path)?;
                                            fs::remove_file(&entry_path)?;
                                            Ok(())
                                        }
                                }
                            }
                        })
                    // Combine errors with try_fold
                    .try_fold((), |_, res| res)?;

                // Remove the emptied source directory recursively
                fs::remove_dir_all(src)?;
                Ok(())
            }
    }
}

pub fn rust_copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>
{
    copy_dir_all(src, dst)?;
    Ok(())
}

pub fn rust_move(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>
{
    move_dir_all(src, dst)?;
    Ok(())
}