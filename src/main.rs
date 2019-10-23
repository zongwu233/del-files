use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn delete_fix(name: &String) {
    if name.ends_with("target") {
        let mut cmd = Command::new("rm")
            .arg("-rf")
            .arg(name)
            .output()
            .expect("failed to execute process");

        let res = cmd.stdout;
    }
}

fn delete_target(p: String) -> std::io::Result<()> {
    let paths = fs::read_dir(p).unwrap();

    for path in paths {
        let children = path.unwrap().path();

        if children.is_dir() {
            match children.file_name().unwrap().to_str().unwrap() {
                "target" => {
                    let name = children.display().to_string();
                    println!("delete {}", name);
                    let res = delete_fix(&name);
                }
                _ => {
                    delete_target(children.display().to_string());
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

    let p = args.get(1).unwrap();
    delete_target(p.to_string());
    Ok(())
}
