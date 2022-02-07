use std::error::Error;
use std::path;

use crate::rapid::{read_rapid_from_file, Repo, SDP};
use crate::sdp::SDPPackage;

pub struct RapidStore<'a> {
    pub root_folder: &'a path::Path,
}

impl<'a> RapidStore<'a> {
    // pub fn find_repo(&self, name: &str) -> Result<Option<Repo>, Box<dyn Error>> {
    //     let repos = parse_repos_from_file(&self.get_repo_path(name))?;
    //     Ok(repos.into_iter().find(|repo| repo.name.contains(name)))
    // }

    pub fn find_sdp(&self, repo: &Repo, name: &str) -> Result<Option<SDP>, Box<dyn Error>> {
        let repo_path = self.root_folder.join(&format!(
            "rapid/repos.springrts.com/{}/version.gz",
            repo.name
        ));
        let sdps = read_rapid_from_file(&repo_path)?;
        Ok(sdps
            .into_iter()
            .find(|sdp| sdp.fullname.contains(name) || sdp.alias.contains(name)))
    }

    pub fn find_nonexisting_files(&self, sdp_files: Vec<SDPPackage>) -> Vec<SDPPackage> {
        sdp_files
            .into_iter()
            .filter(|sdp_file| {
                let mut file_path = path::PathBuf::new();
                file_path.push(self.root_folder);
                file_path.push("pool");
                file_path.push(format!("{}{}", sdp_file.md5[0], sdp_file.md5[1]));
                file_path.push(format!(
                    "{}.gz",
                    &sdp_file.md5[2..32].into_iter().collect::<String>()
                ));

                !file_path.exists()
            })
            .collect()
    }

    pub fn get_registry_path(&self) -> path::PathBuf {
        self.root_folder.join("rapid/repos.springrts.com/repos.gz")
    }

    pub fn get_repo_path(&self, repo: &Repo) -> path::PathBuf {
        let mut http_split: Vec<&str> = repo.url.split("http://").collect();
        if http_split.len() != 2 {
            http_split = repo.url.split("https://").collect();
        }
        let name = http_split[1];
        self.root_folder.join(format!("rapid/{}/version.gz", name))
    }

    // pub fn get_sdp_path(&self, sdp: &SDP) -> path::PathBuf {
    //     self.get_sdp_path_from_md5(&sdp.md5)
    // }

    pub fn get_sdp_path_from_md5(&self, sdp_md5: &str) -> path::PathBuf {
        self.root_folder
            .join(path::PathBuf::from(format!("packages/{}.sdp", sdp_md5)))
    }

    // pub fn get_nonexisting_files_download_map(&self, sdp_files: Vec<SDPPackage>) -> Vec<u8> {
    //     let map_length = sdp_files.len() / 8 + 1;
    //     let mut download_map: Vec<u8> = vec![0; map_length];

    //     let mut index = 0;
    //     for sdp_file in sdp_files {
    //         let file_name = self.root_folder.join(format!(
    //             "pool/{}{}/{}.gz",
    //             sdp_file.md5[0],
    //             sdp_file.md5[1],
    //             &sdp_file.md5[2..32].into_iter().collect::<String>()
    //         ));
    //         let file_path = path::Path::new(&file_name);

    //         if !file_path.exists() {
    //             download_map[index / 8] |= 1 << (index % 8);
    //         }
    //         index += 1;
    //     }

    //     download_map
    // }
}
