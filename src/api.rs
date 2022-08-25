
// Calls RPS API for information on the given package.
// Internally, this makes a call to /api/lookup/{package}.
// Which should return the package info.
pub fn lookup(package: String) {

}

// Calls RPS API to download a compressed .zip of the package.
// Internally, this makes a call to /api/download/{pacakge}.
// It's important to know this function does not handle.
// Placing the file or extracting it in anyway, only downloading the files.
pub fn download(package: String) {

}

// Calls RPS API to fetch the online package index.
// Internally, this makes a call to /api/list.
// The package index is just a list of all the packages on RPS.
// Owl keeps a local version of this index to reduce API calls.
pub fn list() {

}