use std::{fs, ops::Not, process::Command};

fn main() {
    // probably not needed in the newer rust versions
    cargo_build::rerun_if_changed("./build.rs");
    // rerun if we're on a different commit
    cargo_build::rerun_if_changed("./.git/HEAD");
    // rerun if we're on a different submodule commit
    cargo_build::rerun_if_changed("./.git/modules/vinting-web/HEAD");
    // this will run the build script if the directory is not present
    cargo_build::rerun_if_changed("./web/");

    // npm gives a rather cryptic error if this happens
    if !fs::exists("./vinting-web/package.json").unwrap_or(false) {
        cargo_build::error(
            "Git repo cloned without submodules, please clone the repo with the --recursive flag",
        );
    }

    let npm = option_env!("NPM").unwrap_or("npm");
    let rebuild = option_env!("REBUILD").is_some();

    let build_out = !fs::exists("./web/").unwrap_or(false) || rebuild;
    let install_deps = !fs::exists("./vinting-web/node_modules/").unwrap_or(false) || rebuild;

    // check if npm is present in $PATH and is executable
    // spawning the process won't error if it's executable
    // npm will exit with an error with these args, but we don't check that
    Command::new(npm).spawn().is_err().then(|| {
        cargo_build::error(&format!("'{npm}' is not in your $PATH"));
    });

    if install_deps {
        cargo_build::warning("Installing the web dependencies, this may take a while");
        // download deps
        let mut i = Command::new(npm)
            .current_dir("./vinting-web/")
            .args(["install"])
            .spawn()
            .unwrap();
        i.wait()
            .unwrap()
            .success()
            .not()
            .then(|| panic!("There was an error running 'npm install'"));
    }

    // ~10s without this check as opposed with ~1.5s (on my machine) btw
    if build_out {
        // build the website
        let mut c = Command::new(npm)
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
}
