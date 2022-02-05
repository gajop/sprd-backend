use std::error::Error;
use std::path;
use std::str;
use std::u32;

use std::fmt::Write;

#[derive(Debug, Default)]
pub struct SDPPackage {
    pub name: String,
    pub md5: [char; 32],
    pub crc32: [u8; 4],
    pub size: u32,
}

pub fn load_sdp_packages_from_file(dest: &path::Path) -> Result<Vec<SDPPackage>, Box<dyn Error>> {
    let data = crate::gz::read_binary_gz_from_file(dest)?;

    load_sdp_packages(&data)
}

pub fn load_sdp_packages(data: &Vec<u8>) -> Result<Vec<SDPPackage>, Box<dyn Error>> {
    let mut sdp_files = Vec::new();

    let mut index = 0;
    while index < data.len() {
        let length = data[index] as usize;
        index += 1;

        let name = str::from_utf8(&data[index..index + length]).unwrap();
        index += length;

        let md5_bin = &data[index..index + 16];
        index += 16;
        let crc32 = &data[index..index + 4];
        index += 4;
        let mut size: [u8; 4] = [0; 4];
        size.copy_from_slice(&data[index..index + 4]);
        index += 4;

        let mut md5 = String::new();
        for byte in md5_bin {
            write!(md5, "{:02x}", byte)?;
        }

        let mut sdp_file: SDPPackage = Default::default();
        sdp_file.name = name.to_owned();
        sdp_file.crc32.copy_from_slice(crc32);

        let md5_chars: Vec<char> = md5.chars().collect();
        for i in 0..md5_chars.len() {
            sdp_file.md5[i] = md5_chars[i];
        }
        sdp_file.size = u32::from_le_bytes(size);

        sdp_files.push(sdp_file);
    }

    Ok(sdp_files)
}
