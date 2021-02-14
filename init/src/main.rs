use serde_json::Value;
use std::fs;
use std::fs::File;
use std::process::Command;

enum Runlevel {
    Start,
    Stop,
}

struct Service {
    name: String,
    command: String,
    runlevel: Runlevel,
}
fn execute(service: Service) {
    match service.runlevel {
        Runlevel::Start => {
            Command::new(service.command)
                .output()
                .expect("failed to run service");
        }
        Runlevel::Stop => {
            println!("not implemented")
        }
    }
}

fn service_parser(filename: String) -> Service {
    let content = File::open(filename).expect("cannot read services configuration");
    let v: Value = serde_json::from_reader(content).expect("error parsing service");
    let mut run_level = Runlevel::Start;
    if v["runlevel"] == "start" {
        run_level = Runlevel::Start;
    } else if v["runlevel"] == "stop" {
        run_level = Runlevel::Stop;
    };
    let service = Service {
        name: v["name"].to_string(),
        command: v["command"].to_string(),
        runlevel: run_level,
    };
    return service;
}

fn main() {
    let paths = fs::read_dir("/var/services").unwrap();
    for path in paths {
        let service = service_parser(path.unwrap().path().display().to_string());
        execute(service)
    }
}
