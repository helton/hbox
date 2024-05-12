use crate::packages::Package;

pub fn pull(package: &Package) -> bool {
    let command = format!("docker pull {}", package.container_image_url());
    run_command(&command)
}

pub fn run(package: &Package, params: &Vec<String>) -> bool {
    let command = format!("docker run {} {}", package.name, params.join(" "));
    run_command(&command)
}

fn run_command(command: &str) -> bool {
    println!("Pretending I'm running {}", command);
    true
}
