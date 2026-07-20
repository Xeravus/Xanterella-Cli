use crate::engine::lexer::vfs::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_fsdata_initialization() {
        let vfs = FsData::new("/dummy/path"); 
        
        assert_eq!(vfs.path, "/dummy/path");
        assert!(vfs.files.is_empty()); 
        if let FsNodes::Dir(map) = vfs.fsnodes {
            assert!(map.is_empty());
        } else {
            panic!("Der Root-Knoten muss ein Dir sein!");
        }
    }
    #[test]
    fn test_get_files_single_nix_file() {
        let mut vfs = FsData::new("einzelne_datei.nix");
        vfs.get_files(); 
        
        assert_eq!(vfs.files.len(), 1);
        assert_eq!(vfs.files[0], "einzelne_datei.nix");
    }
    #[test]
    fn test_gen_tree_with_real_files() {
        let temp_dir = env::temp_dir().join("prolyxena_test_vfs");
        let sub_dir = temp_dir.join("hosts").join("node1");
        fs::create_dir_all(&sub_dir).unwrap();
        let file_path = sub_dir.join("config.nix");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"true").unwrap();
        let mut vfs = FsData::new(temp_dir.to_str().unwrap());
        vfs.load(); 
        fs::remove_dir_all(&temp_dir).unwrap();
        assert_eq!(vfs.files.len(), 1);
        if let FsNodes::Dir(root_map) = &vfs.fsnodes {
            let hosts_node = root_map.get("hosts").expect("Ordner 'hosts' fehlt im Baum");
            
            if let FsNodes::Dir(hosts_map) = hosts_node {
                let node1_node = hosts_map.get("node1").expect("Ordner 'node1' fehlt im Baum");
                
                if let FsNodes::Dir(node1_map) = node1_node {
                    let config_file = node1_map.get("config.nix").expect("Datei 'config.nix' fehlt");
                    if let FsNodes::File { name, ast: _ } = config_file {
                        assert_eq!(name, "config.nix");
                    } else {
                        panic!("config.nix wurde nicht als FsNodes::File gespeichert!");
                    }
                } else {
                    panic!("node1 ist kein Ordner!");
                }
            } else {
                panic!("hosts ist kein Ordner!");
            }
        } else {
            panic!("Wurzel ist kein Ordner!");
        }
    }
}
