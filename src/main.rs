use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn delete_fix(name: &String, targetDir: &str) {
    if name.ends_with(targetDir) {
        let mut cmd = Command::new("rm")
            .arg("-rf")
            .arg(name)
            .output()
            .expect("failed to execute process");

        let res = cmd.stdout;
    }
}

fn delete_target(p: String, targetDir: &str) -> std::io::Result<()> {
    let paths = fs::read_dir(p).unwrap();

    for path in paths {
        let children = path.unwrap().path();

        if children.is_dir() {
            match children.file_name().unwrap().to_str().unwrap() {
                targetDir => {
                    let name = children.display().to_string();
                    println!("delete {}", name);
                    // let res = delete_fix(&name, targetDir);
                }
                _ => {
                    delete_target(children.display().to_string(), targetDir);
                }
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("need a dir param");
    }
    if args.len() >= 3 {
        println!("args {},{}", args.get(1).unwrap(), args.get(2).unwrap());
    }

    let p = args.get(1).unwrap();
    let defaultValue = String::from("target");
    let targetDir: &String = args.get(2).unwrap_or(&defaultValue);

    println!("delete files name {}", targetDir);

    delete_target(p.to_string(), targetDir.as_str());
    Ok(())
}
