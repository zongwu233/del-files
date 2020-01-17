use std::env;
use std::fs;
use std::process::Command;

fn delete_fix(name: &String, target_dir: &str) {
    if name.ends_with(target_dir) {
        let cmd = Command::new("rm")
            .arg("-rf")
            .arg(name)
            .output()
            .expect("failed to execute process");

        assert!(cmd.status.success());
    }
}

fn delete_target(p: String, target_dir: &str) -> std::io::Result<()> {
    let paths = fs::read_dir(p).unwrap();

    for path in paths {
        let children = path.unwrap().path();

        if children.is_dir() {
            match children.file_name().unwrap().to_str() {
                Some(x) if x == target_dir => {
                    let name = children.display().to_string();
                    println!("delete {}", name);
                    delete_fix(&name, target_dir);
                    //println!("{:?}",res );
                }
                _ => {
                    delete_target(children.display().to_string(), target_dir).expect("failed");
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
    let default_value = String::from("target");
    let target_dir: &String = args.get(2).unwrap_or(&default_value);

    println!("delete files pattern {}", target_dir);

    delete_target(p.to_string(), target_dir.as_str()).expect("failed");
    Ok(())
}
