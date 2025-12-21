use std::env::set_current_dir;
use std::io::{self, Write};
use std::fs;
use std::fs::write;
use std::process;

fn main() {
    println!("--- Theos Project Generator ---");

    // Get Project Name & Create Directory
    // In Rust, variables are immutable by default. use 'mut' if they change.
    print!("Enter the project name: ");
    let project_name = read_input();

    let project_path = std::path::Path::new(&project_name);
    if project_path.exists() {
        eprintln!("Error: That folder already exists!");
        process::exit(1);
    }

    // Create the "Namespace" version of the name (e.g. "My Project" -> "MyProject")
    // This is vital for the Makefile and Control file to work correctly.
    let name_ns = project_name.replace(" ", "");

    // Success/Error switch statement type shit
    match fs::create_dir(&project_name) {
        Ok(_) => println!("Created directory: {}", project_name),
        Err(e) => {
            eprintln!("Failed to create directory: {}", e);
            process::exit(1);
        }
    }

    // get metadata
    print!("Enter package bundle id: ");
    let bundle = read_input();
    print!("Enter package description: ");
    let description = read_input();
    print!("Enter name of app tweaked: ");
    let app = read_input();
    print!("Enter bundle id of app tweaked: ");
    let app_bundle = read_input();
    print!("Enter name of the tweak author: ");
    let author = read_input();

    // change directory
    set_current_dir(project_path).unwrap();

    // wait this somehow makes sense??
    // kinda feels like
    // init(bundle: String) {
    //     bundle = self.bundle
    // }
    let control_file_content = format!(
        "Package: {bundle}\n\
         Name: {name_ns}\n\
         Version: 0.0.1\n\
         Architecture: iphoneos-arm\n\
         Description: {description}\n\
         Author: {author}\n\
         Maintainer: {author}\n\
         Section: Tweaks\n\
         Depends: firmware (>= 11.0)\n",
        bundle = bundle,
        name_ns = name_ns,
        description = description,
        author = author
    );

    let tweak_content = format!(
        "#import <Foundation/Foundation.h>\n\
         %config(generator=internal);\n\
         // Happy coding, {author}!\n",
        author = author
    );

    let makefile_content = format!(
        "TARGET := iphone:clang:latest:11.0\n\
         INSTALL_TARGET_PROCESSES = {app}\n\
         ARCHS = arm64\n\n\
         include $(THEOS)/makefiles/common.mk\n\n\
         TWEAK_NAME = {name_ns}\n\n\
         $(TWEAK_NAME)_FILES = Tweak.xm\n\
         $(TWEAK_NAME)_CFLAGS = -fobjc-arc\n\
         $(TWEAK_NAME)_LOGOS_DEFAULT_GENERATOR = internal\n\n\
         include $(THEOS_MAKE_PATH)/tweak.mk\n",
        app = app,
        name_ns = name_ns
    );

    let plist_content = format!(
        "{{ Filter = {{ Bundles = ( \"{app_bundle}\" ); }}; }}",
        app_bundle = app_bundle
    );

    // Writing files
    write("control", control_file_content).unwrap();
    write("Tweak.xm", tweak_content).unwrap();
    write("Makefile", makefile_content).unwrap();

    write(format!("{}.plist", name_ns), plist_content).unwrap();

    // similar to \(projectName) in Swift
    // {} is used to interpolate strings and use variables in rust.
    println!("Successfully created project: {}", project_name);
}

fn read_input() -> String {
    // flush() ensures the prompt appears before the user types
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // trim() removes the newline character (\n) at the end
    input.trim().to_string()
}