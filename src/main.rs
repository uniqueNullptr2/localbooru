use std::{path::Path};

use anyhow::Result;
use image_utils::{Image, ImageSearchCache};
use walkdir::{DirEntry, WalkDir};
use rayon::prelude::*;
mod image_utils;

fn search<T: AsRef<Path>>(path: T) -> Result<ImageSearchCache> {
    let mut cache = ImageSearchCache::new();


    let paths: Vec<DirEntry> = WalkDir::new(path.as_ref())
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file() && is_image(e))
                    .collect();
    let v:Result<Vec<Image>> = paths.par_iter().map(|e| Image::from_file(e.path())).collect();
    for img in v? {
        cache.insert(img);
    }
    Ok(cache)
}

fn main() -> Result<()> {
    let cache = search("/home/phil/Bilder/Ballou2")?;
    // let cache = ImageSearchCache::init("/home/phil/Bilder/search.db")?;
    println!("{}", cache.len());
    cache.write_to_file("/home/phil/Bilder/search.db")?;
    Ok(())
}

fn is_image(e: &DirEntry) -> bool {
    let f_name = e.file_name();
    let l_name = f_name.to_ascii_lowercase();
    let name = l_name.to_string_lossy();
    println!("{}", name);
    name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".jpeg")
}
