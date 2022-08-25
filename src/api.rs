use std::fs::read_to_string;
use error_chain::error_chain;
use reqwest;
use std::io::Read;
use serde::Deserialize;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// Calls RPS API for information on the given package.
// Internally, this makes a call to /api/lookup/{package}.
// Which should return the package info.
pub fn lookup(package: &String) -> Result<Package> {
    let mut res = reqwest::blocking::get(
        format!("https://rps.rect.ml/api/lookup/{}", package.trim()),
    )?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let package: Package = serde_json::from_str(&*body)
        .expect("JSON not formatted correctly!");

    Ok(package)
}

// Calls RPS API to download a compressed .zip of the package.
// Internally, this makes a call to /api/download/{package}.
// It's important to know this function does not handle.
// Placing the file or extracting it in anyway, only downloading the files.
pub fn download(package: String) {

}

// This directly downloads the package from the RPS file system.
pub fn direct_download(link: String) {

}

// Calls RPS API to fetch the online package index.
// Internally, this makes a call to /api/list.
// The package index is just a list of all the packages on RPS.
// Owl keeps a local version of this index to reduce API calls.
pub fn list() -> Result<Vec<Package>>{
    let mut res = reqwest::blocking::get("https://rps.rect.ml/api/list")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let list_packages: Vec<ListPackage> = serde_json::from_str(&*body)
        .expect("JSON not formatted correctly!");

    let mut packages: Vec<Package> = Vec::new();
    for package in list_packages {
        packages.push(Package::from(&package));
    }

    Ok(packages)
}

// Package struct stores all the necessary package information
// This does not store HTMLDescription or LastDownload because they aren't needed
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    #[serde(rename = "ID")]
    id: i32,
    package_name: String,
    author: String,
    description: String,
    downloads: i32,
    link: String,
    version: String,
}

// This exists because the RPS API is so bad it can return two different types of json objects.
// A /api/lookup/ will return Package shown above, but /api/list will return the struct shown below.
// The structs have identical content but differ in order of objects, and naming.
// When using list(), this format will be used and convert to Package internally.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPackage {
    #[serde(rename = "ID")]
    id: i32,
    package_name: String,
    version: String,
    author: String,
    description: String,
    downloads: i32,
    direct_source: String,
}

// This function implemented the conversion from ListPackage to Package
impl Package {
    pub fn from(pkg: &ListPackage) -> Package {
        Package {
            id: pkg.id,
            package_name: pkg.package_name.clone(),
            author: pkg.author.clone(),
            description: pkg.description.clone(),
            downloads: pkg.downloads,
            link: pkg.direct_source.clone(),
            version: pkg.version.clone(),
        }
    }
}