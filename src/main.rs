use std::env;
use std::fs;
use std::process::Command;
use walkdir::WalkDir;

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
                    println!("delete {},{}", name,file_size_pretty(cal_file_total_size(&name)));
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

fn cal_file_total_size(dir: &String) -> u64{
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len())

}

fn file_size_pretty(toal_size:u64)->String{
    let k_size:f64 = toal_size as f64 / 1024 as f64;
    let m_size:f64 = k_size as f64  / 1024 as f64 ;
    let g_size:f64 = m_size as f64 / 1024 as f64 ;

    if g_size > 1.0 {
        format!("{:.2?}G", g_size)
    } else if m_size > 1.0 {
        format!("{:.2?}M", m_size)
    } else {
        format!("{:.2?}K", toal_size)
    }
}

