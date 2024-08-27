# Sphynx
Sphynx is a markdown-based terminal parser for easily creating HTML files!

# Installation

As there are no official releases yet, please follow the build instructions below to install.

# Build Instructions

1. Ensure that you have Rust installed. If not, download according to your operating system (here)[https://www.rust-lang.org/tools/install]
2. Clone the repo: ```git clone https://github.com/AnAngryRaven/sphynx```
3. Change into the appropriate directory: ```cd your/local/directory/here```
4. Build: ```cargo run```

# Usage

To use, simply run the application, and begin typing in Markdown! When finished, simply `ctrl+z` to exit, and the program will generate an appropriate .html file in its directory.

Note that functionality is currently limited (to put it mildly) due to the freshness of this project.

# todo()

The following is a list (in no particular order) of what I consider to be "base functionality", or the bare minimum for the application to be considered "complete":

- [ ] Proper parsing of Markdown syntax
- [ ] File name customisation
- [ ] Custom output paths
- [ ] Non-malformed HTML / CSS outputs
- [ ] Editing of user-specified files

# Future (Additional) Features

Below is a list of features that are planned, besides getting the base functionality to work:

* Allow for insertion of a custom CSS file into the generated document
* Allow for a boilerplate <head> tag, to permit customisation of <meta> tags
* Allow for pre-existing markdown files to be automatically parsed, without opening in the editor first
* More to come I'm sure!

# Why?

This project mostly exists to help me learn Rust, though I do have a genuine use case for it!

I want to replace my currently existing Wordpress site with just static webpages, and I thought it'd be interesting to develop something like this to help me easily generate each of the pages, instead of needing to code them in HTML each time. A full-blown WYSIWYG seemed like a little too large of a scope, so I went with a simple terminal application.

# Contributions

Due to the above, currently contributions aren't generally accepted!

Note that when base functionality is achieved, this will 100% change ^_^