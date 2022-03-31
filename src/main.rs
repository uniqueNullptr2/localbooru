use std::path::Path;
use std::thread::spawn;

use anyhow::Result;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use image_utils::Image;
use image_utils::ImageSearchCache;
use walkdir::{DirEntry, WalkDir};

mod image_utils;


fn search<T: AsRef<Path>>(path: T) -> Result<ImageSearchCache>{
    let (s, r): (Sender<DirEntry>,Receiver<DirEntry>) = crossbeam_channel::bounded(200);
    let cpus = num_cpus::get();
    let mut v = vec!();
    let mut cache = ImageSearchCache::new();

    for _ in 0..cpus {
        let recv = r.clone();
        v.push(spawn(move ||{
            let mut v: Vec<Image> = vec!();
            loop {
                match recv.recv() {
                    Ok(entry) if is_image(&entry)=> v.push(Image::from_file(entry.path())?),
                    Err(_) => break,
                    _ => ()
                }
            }
            Ok(v)
        }));
    }
    for entry in WalkDir::new(path.as_ref()){
        let e = entry?;
        if e.file_type().is_file() {
            s.send(e)?;
        }
    }
    drop(s);
    let v: Result<Vec<Vec<Image>>> = v.into_iter().map(|h| h.join()).flatten().collect();
    for img in v?.into_iter().flatten() {
        cache.insert(img);
    }
    Ok(cache)
}

fn main() -> Result<()> {
    // let cache = search("/home/phil/Bilder/Ballou2")?;
    let cache = ImageSearchCache::init("/home/phil/Bilder/search.db")?;
    println!("{}", cache.len());
    // cache.write_to_file("/home/phil/Bilder/search.db")?;
    Ok(())
}

fn is_image(e: &DirEntry) -> bool{
    let f_name = e.file_name();
    let l_name = f_name.to_ascii_lowercase();
    let name = l_name.to_string_lossy();
    println!("{}", name);
    name.ends_with(".png") || name.ends_with(".jpg") ||name.ends_with(".jpeg")
}
