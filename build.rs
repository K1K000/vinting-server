use std::{ops::Not, process::Command};

fn main() {
    cargo_build::rerun_if_changed("./build.rs");
    cargo_build::rerun_if_changed("./.git/HEAD");
    cargo_build::rerun_if_changed("./.git/modules/vinting-web/HEAD");
    // this will run the build script if the directory is not present
    cargo_build::rerun_if_changed("./web/");

    // check if npm is present in $PATH and is executable
    Command::new("npm").spawn().is_err().then(|| {
        panic!("'npm is not in your $PATH'");
    });

    // download deps
    // WARN: may take a while, even if the deps are present (that's npm for you)
    let mut i = Command::new("npm")
        .current_dir("./vinting-web/")
        .args(["install"])
        .spawn()
        .unwrap();
    i.wait()
        .unwrap()
        .success()
        .not()
        .then(|| panic!("There was an error running 'npm install'"));

    // build the website
    let mut c = Command::new("npm")
        .current_dir("./vinting-web/")
        .args(["run", "build"])
        .spawn()
        .unwrap();
    c.wait()
        .unwrap()
        .success()
        .not()
        .then(|| panic!("There was an error running 'npm run build'"));
}
