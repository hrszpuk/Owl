use std::fs;

pub fn get_modules() -> Vec<String> {
    let paths = fs::read_dir("./packages").unwrap();
    let mut modules: Vec<String> = Vec::new();

    for path in paths {
        let absolute_path = path.unwrap().path();
        if absolute_path.is_file()
            && absolute_path.ends_with(".ll") {
            modules.push(absolute_path
                .into_os_string()
                .into_string()
                .unwrap()
            );
        }
    }

    return modules;
}