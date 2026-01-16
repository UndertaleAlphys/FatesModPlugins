use sha2::{Digest, Sha512};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn verify_mod_file() -> Result<(), String> {
    const ROOT_DIR: &str = "sd:/engage/mods/IF mod (Cobalt)";
    let ignore_list = vec!["/h", "/libIFmodPlugins.nro"];
    let hash_path = format!("{ROOT_DIR}/h");
    let hash_path = Path::new(hash_path.as_str());
    if !hash_path.exists() || !hash_path.is_file() {
        return Err("Failed to find hash file.".to_string());
    }
    let hash_map =
        bincode::deserialize::<BTreeMap<String, Vec<u8>>>(&fs::read(hash_path).unwrap()[..]);
    if hash_map.is_err() {
        return Err("Failed to deserialize hash file.".to_string());
    }
    let mut hash_map = hash_map.unwrap().clone();
    for entry in WalkDir::new(ROOT_DIR) {
        if let Ok(entry) = entry {
            if entry.path().exists() && entry.path().is_file() {
                let file_content = fs::read(entry.path()).unwrap();
                let mut hasher = Sha512::new();
                hasher.update(file_content);
                let actual_hash = hasher.finalize().to_vec();
                let path_str = entry.path().to_str();
                if path_str.is_none() {
                    return Err("Failed to convert path to string.".to_string());
                }
                let path_str = path_str.unwrap().replace("\\", "/");
                let relative_path = path_str.strip_prefix(ROOT_DIR);
                if relative_path.is_none() {
                    return Err("Failed to strip prefix.".to_string());
                }
                let relative_path = relative_path.unwrap();
                if ignore_list.contains(&relative_path) {
                    continue;
                }
                let hash = hash_map.get(relative_path);
                if hash.is_none() {
                    return Err(format!("{}: Failed to find hash.", relative_path));
                }
                let expected_hash = hash.unwrap();
                if *expected_hash != actual_hash {
                    return Err(format!("{relative_path}: Mismatched hash"));
                }
                hash_map.remove(relative_path);
            }
        }
    }
    if hash_map.is_empty() {
        Ok(())
    } else {
        let mut msg = "The following files were not found:\n".to_string();
        for file_name in hash_map.keys() {
            msg.push_str(file_name);
            msg.push('\n');
        }
        Err(msg)
    }
}
