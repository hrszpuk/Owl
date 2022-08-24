<h1 align="center">Owl</h1>
<p align="center">[WIP] :owl: Owl is a pacman-like ReCT Package System CLI application :owl:</p>

<p align="center">
<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-GPL-green.svg"></a>
<a href="https://github.com/hrszpuk"><img src="https://img.shields.io/github/followers/hrszpuk?style=social"></a>
<a href="https://twitter.com/hrszpuk"><img src="https://img.shields.io/twitter/follow/hrszpuk?style=social"></a>
<a href="https://github.com/hrszpuk/Owl/issues"><img src="https://img.shields.io/github/issues/hrszpuk/Owl"></a>
</p>

<p align="center">
Owl is a CLI application to easily install ReCT packages from https://rps.rect.ml.
Owl is designed after the Arch Linux package manager, pacman.
Packages can be installed, removed, updated, and more!
</p>

An example of basic owl usage is shown below:
```sh
$ owl -Syu      // Update the RPS packages
$ owl -S sys    // Install "sys" package from RPS
$ owl -R sys    // Remove "sys" package from RPS
```

## Installation

### Build from source

### AUR installation

## Usage
The Owl help menu can be shown by typing `owl` or `owl --help`. 
Commands take a flag, and if that flag requires and argument it follows the flag.
```shell
$ owl <flag> [argument]
```

### Flags
All the flags that Owl accepts are based on the pacmam package manager flags. 
In the table below, some flags require an argument and use `example` instead of a package name.

| name                                  |  Flag   |         Example          | Explanation                                                                                                                                                                                                 |
|---------------------------------------|:-------:|:------------------------:|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Update Packages                       | `-Syu`  |        `owl -Syu`        | Update all packages and package dependencies on the system. This will also update the internal package index cache.                                                                                         |
| Install Package(s)                    |  `-S`   |     `owl -S example`     | Sync package with package database. This installs the package and allows it to be used in ReCT code. Multiple packages can be installed by separating each package with a space after the -S flag.          |
| Remove Package(s)                     |  `-R`   |     `owl -R example`     | Removes package from package database and deletes package from package storage. Multiple packages can be removed by separating each package with a space after the -R flag.                                 |
| Remove Package(s) and dependencies    |  `-Rs`  |    `owl -Rs example`     | Removes the package and it's dependencies from the database. Mutliple packages can be listed. Dependencies are only removed if they are not required by any other package.                                  |
| Remove redundant dependencies         | `-Qdtq` |       `owl -Qdtq`        | Removes any packages that are installed as a dependency, but are not used as a dependency by any package on the system. This may be useful for freeing up space.                                            |
| Search for package                    |  `-Ss`  |    `owl -Ss example`     | Search through package index for the given package. If found, package information will be shown in the console (including how to install the package), otherwise owl will inform you of it's non-existence. |
| Search for already installed packages |  `-Qs`  |    `owl -Qs example`     | Search through local package index for the given package. The package must already be installed for the search to be successful. Upon finding the package, the package information will be displayed.       |
| View dependencies of a package        | `-Qds`  |    `owl -Ds example`     | Shows a list of dependencies required by a given package. This works for both installed packages and packages on the RPS.                                                                                   |
| View all installed dependencies       |  `-Qd`  |        `owl -Qd`         | Shows a list of all dependency packages installed on the system. This also tells you what packages are using each dependency.                                                                               |
| View all installed packages           |  `-Q`   |        `owl -Qsq`        | Show a list of every package currently installed on the system as well as other useful information.                                                                                                         |
| Install a local package               |  `-U`   | `owl -U path/to/package` | Opens a dialog to create a new package and install it onto the local package index. This will not upload the package to the RPS, but will allow the package to be used by any ReCT program system-wide.     |


## Contributing
Contributing helps keep this tool safe to use and up to date. 
If you want to help, why not create an issue?

<a href="https://github.com/hrszpuk/owl/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=hrszpuk/owl" />
</a>
