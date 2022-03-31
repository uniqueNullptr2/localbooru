use std::{ffi::OsString, rc::Rc, collections::{HashMap, hash_map::Entry}, cell::RefCell, fs::{OpenOptions, write}, path::Path, io::BufReader};
use crypto::{sha3::Sha3, digest::Digest};
use image::load_from_memory;
use img_hash::{HasherConfig, HashAlg};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    path: OsString,
    percept_hash: String,
    file_hash: String,
    data_hash: String,
    keywords: Vec<String>
}

impl Image {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let buf = std::fs::read(&path)?;
        let img = load_from_memory(&buf)?;
        let hasher = HasherConfig::new()
                                .hash_alg(HashAlg::DoubleGradient)
                                .hash_size(10, 8)
                                .preproc_dct()
                                .to_hasher();
        let h1 = hasher.hash_image(&img);
        let mut sha_hasher = Sha3::sha3_256();
        sha_hasher.input(&buf);
        Ok(Self{
            path: path.as_ref().as_os_str().to_owned(),
            percept_hash: h1.to_base64(),
            file_hash: sha_hasher.result_str(),
            data_hash: "".to_owned(),
            keywords: vec!()
        })
    }

    pub fn with_keywords(mut self, keywords: &[&str]) -> Self {
        self.keywords = keywords
                            .into_iter()
                            .map(|s| s.to_owned().to_owned())
                            .collect();
        self
    }
}

pub struct ImageSearchCache {
    inner: Vec<Rc<RefCell<Image>>>,
    path_map: HashMap<OsString, Rc<RefCell<Image>>>,
    phash_map: HashMap<String, Rc<RefCell<Image>>>,
    fhash_map: HashMap<String, Rc<RefCell<Image>>>,
    dhash_map: HashMap<String, Rc<RefCell<Image>>>,
    keyword_map: HashMap<String, Vec<Rc<RefCell<Image>>>>,
}

impl ImageSearchCache {

    pub fn new() -> Self {
        Self{
            inner: vec!(),
            path_map: HashMap::new(),
            phash_map: HashMap::new(),
            fhash_map: HashMap::new(),
            dhash_map: HashMap::new(),
            keyword_map: HashMap::new(),
        }
    }
    pub fn init<T: AsRef<Path>>(path: T) -> Result<Self> {
        let file = OpenOptions::new()
                            .read(true)
                            .open(path)?;
        let b: bson::Document = bson::from_reader(BufReader::new(file))?;
        let v: Vec<Image> = bson::from_bson(b.get("images").unwrap().to_owned())?;
        let mut r = Self::new();
        for img in v {
            r.insert(img);
        }
        Ok(r)
    }

    pub fn insert(&mut self, img: Image) {
        let rc_img = Rc::new(RefCell::new(img));
        self.inner.push(rc_img.clone());
        self.path_map.insert(rc_img.borrow().path.clone(), rc_img.clone());
        self.phash_map.insert(rc_img.borrow().percept_hash.clone(), rc_img.clone());
        self.fhash_map.insert(rc_img.borrow().file_hash.clone(), rc_img.clone());

        for kw in &rc_img.borrow().keywords {
            match self.keyword_map.entry(kw.clone()) {
                Entry::Occupied(o) => o.into_mut().push(rc_img.clone()),
                Entry::Vacant(v) => {v.insert(vec!(rc_img.clone()));}
            }
        }
        //not so nice but saves one clone
        let data_hash = rc_img.borrow().data_hash.clone();
        self.dhash_map.insert(data_hash, rc_img);
    }

    pub fn write_to_file<T: AsRef<Path>>(&self, path: T) -> Result<()>{
        //how to avoid so much allocation?
        let v: Vec<Image> = self.inner
                                .iter()
                                .map(|img| img.borrow().clone())
                                .collect();
        let mut doc = bson::document::Document::new();
        let arr = bson::to_bson(&v)?;
        doc.insert("images", arr);
        let buf = bson::to_vec(&doc)?;
        write(path, buf)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}