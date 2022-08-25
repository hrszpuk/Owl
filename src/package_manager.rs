
// Downloads a package, installs it, and adds it to the database.
// Flag: -S | get
pub fn sync(args: Vec<String>) {

}

// Updates the local package index, checks and updates outdated packages.
// Flag: -Syu | update
pub fn update() {

}

// Removes a package, deletes the files, and removes it from the database.
// Flag: -R | remove | rm
pub fn remove(args: Vec<String>) {

}

// Removes a package and it's dependencies (if the dependencies aren't used elsewhere).
// Flag: -Rs | eliminate
pub fn eliminate(args: Vec<String>) {

}

// Removes any unused dependencies, deletes the files, and removes it from the database.
// Flag: -Rrd | clean
pub fn clean() {

}

// Searches for packages, this only searches local package index (which could be out of date).
// Flag: -Ss | search | find
pub fn search(args: Vec<String>) {

}

// Lists all the installed packages, including author, version, description, etc.
// Flag: -Q | packages | list
pub fn list_packages() {

}

// Lists all the installed dependencies, including what packages require them.
// Flag: -Qd | deps
pub fn list_dependencies() {

}

// Lists all the dependencies of a package
pub fn list_package_dependencies(args: Vec<String>) {

}

// Install a package locally by providing a path to the package
// Flag: -U | install
pub fn local_install(args: Vec<String>) {

}

// Show information about a package
// Flag: -I | info
pub fn info(args: Vec<String>) {

}