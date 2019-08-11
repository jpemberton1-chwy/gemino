#[macro_use]
extern crate clap;
use clap::App;
use std::io;
use std::io::{Read, Write, Seek, SeekFrom};
use std::process::{Command, Output};
use std::fs::{File, OpenOptions, create_dir};
use std::path::Path;

fn main() {
    let yaml = load_yaml!("gemino.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        match matches.value_of("project") {
            Some(folder) => {
                handle_new_project(&folder);
            },
            None => {
                println!("Please provide a folder");
                return;
            },
        }
    }
}

enum GeminoError {
    FileWriteError,
}

impl From<io::Error> for GeminoError {
    fn from(_e: io::Error) -> GeminoError {
        GeminoError::FileWriteError
    }
}

fn handle_new_project(folder: &str) {
    println!("Building new project in {}", folder);

    if let Ok(_) = handle_create_project_scaffolding(folder) {
        println!("Project scaffolded successfully");
    } else {
        println!("Failed to create project");
    }
}

fn handle_create_project_scaffolding(folder: &str) -> Result<(), GeminoError> {
    create_project_folder(folder)?;
    create_babel_config(folder)?;
    create_jest_config(folder)?;
    create_index_file(folder)?;
    create_example_component(folder)?;
    create_example_test(folder)?;
    create_package_json(folder)?;
    package_dev_install("jest")?;
    package_install("lodash")?;
    insert_test_script()?;

    Ok(())
}

fn create_project_folder(folder: &str) -> std::io::Result<()> {
    create_dir(Path::new(folder))?;
    create_dir(Path::new(format!("{}/src", folder).as_str()))?;

    Ok(())
}

fn create_babel_config(folder: &str) -> std::io::Result<File> {
    let mut file = File::create(Path::new(format!("{}/babel.config.js", folder).as_str()))?;

    file.write_all(b"const baseBabelConfig = require('../../babel.config');

module.exports = { ...baseBabelConfig };")?;
    file.sync_all()?;

    Ok(file)
}

fn create_jest_config(folder: &str) -> std::io::Result<File> {
    let mut file = File::create(Path::new(format!("{}/jest.config.js", folder).as_str()))?;

    file.write_all(b"const baseJestConfig = require('../../jest.config');

module.exports = { ...baseJestConfig };")?;
    file.sync_all()?;

    Ok(file)
}

fn create_package_json(folder: &str) -> std::io::Result<Output> {
    use std::env;
    env::set_current_dir(Path::new(folder))?;

    let command = Command::new("yarn")
        .arg("init")
        .arg("-yp")
        .output()?;

    Ok(command)
}

fn insert_test_script() -> std::io::Result<File> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(Path::new("package.json"))
        .expect("failed to open");

    let mut buf = String::new();

    file.read_to_string(&mut buf).expect("Failed to open package.json");
    file.set_len(0).expect("failed to truncate");

    file.seek(SeekFrom::Start(0))?;

    let modified_buf = buf.replace("\"private\": true,", "\"private\": true,
  \"scripts\": {
    \"test\": \"./node_modules/.bin/jest\"
  },");

    file.write_all(modified_buf.as_bytes()).expect("failed to write");
    file.sync_all().expect("failed to sync");

    Ok(file)
}

fn create_index_file(folder: &str) -> std::io::Result<File> {
    let mut file = File::create(Path::new(format!("{}/index.js", folder).as_str()))?;

    file.write_all(b"import App from './src/App';

export default App;")?;
    file.sync_all()?;

    Ok(file)
}

fn create_example_component(folder: &str) -> std::io::Result<File> {
    let mut file = File::create(Path::new(format!("{}/src/App.js", folder).as_str()))?;

    file.write_all(b"import React from 'react';
import Template from '@chewy/react-template';

const App = props => (
    <Template>
        <main className=\"app\" aria-role=\"content\">
            {'Hello World'}
        </main>
    </Template>
);

export default App;")?;
    file.sync_all()?;

    Ok(file)
}

fn create_example_test(folder: &str) -> std::io::Result<File> {
    let mut file = File::create(Path::new(format!("{}/src/App.test.js", folder).as_str()))?;

    file.write_all(b"import App from './App';

describe('<App />', () => {
    const element = testComponent(App);

    beforeEach(() => element.reset());

    test(\"it should render hello\", () => {
        expect(element()).toHaveText('Hello World');
    });
});")?;
    file.sync_all()?;

    Ok(file)
}

fn yarn_install_package(package: &str, dev_only: bool) -> std::io::Result<Output> {
    let mut command = Command::new("yarn");

    command.arg("add");
    command.arg(package);
    command.arg("--exact");

    if dev_only { command.arg("--dev"); }

    Ok(command.output()?)
}

fn package_dev_install(package: &str) -> std::io::Result<Output> {
    yarn_install_package(package, true)
}

fn package_install(package: &str) -> std::io::Result<Output> {
    yarn_install_package(package, false)
}

#[cfg(test)]
mod test {
    use super::handle_new_project;
    use std::path::Path;
    use std::fs::remove_dir_all;

    const TEST_PROJECT: &str = "project";

    fn clean_up() {
        let result = remove_dir_all(TEST_PROJECT);

        if result.is_err() {}
    }

    #[test]
    fn it_creates_a_project_directory() {
        handle_new_project(TEST_PROJECT);
        assert!(Path::new(TEST_PROJECT).exists(), "Directory does not exist");
        assert!(Path::new(format!("{}/babel.config.js", TEST_PROJECT).as_str()).exists(), "Babel config does not exist");
        assert!(Path::new(format!("{}/jest.config.js", TEST_PROJECT).as_str()).exists(), "Jest config does not exist");
        assert!(Path::new(format!("{}/package.json", TEST_PROJECT).as_str()).exists(), "Jest config does not exist");
        assert!(Path::new(format!("{}/index.js", TEST_PROJECT).as_str()).exists(), "index.js does not exist");
        clean_up();
    }
}
