use std::os::unix::fs::PermissionsExt;
use std::{fs, os::unix::net::UnixDatagram, process::Command};

fn parse_command(command: u8) -> (bool, usize) {
    let is_brightness = command >> 7 == 1;
    let value = command & (1 << 7) - 1;
    (is_brightness, value as usize)
}

fn execute_command(key: &str, value: &str, device: &str) {
    let mut command = Command::new("/bin/ddccontrol");
    let command = command.args(&["-r", key, "-w", value, device]);

    match command.status() {
        Err(e) => eprintln!("Failed to execute command: {}", e),
        Ok(status) => {
            if !status.success() {
                eprintln!("Command exited with non-zero exit code: {}", status)
            }
        }
    }
}

// TODO: Make this configurable (the 0x hardcodes, the /dev paths, etc.)
pub fn listen(path: &str) {
    fs::remove_file(path);

    let socket = UnixDatagram::bind(path).expect("Failed to bind socket");

    let metadata = fs::metadata(path).expect("Failed to retrieve domain socket metadata");
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o777);
    fs::set_permissions(path, permissions).expect("Failed to set the domain socket to 0777");

    loop {
        let mut buf = vec![0u8];
        socket.recv(buf.as_mut_slice()).expect("recv failed");

        let command = buf[0];

        let (is_brightness, value) = parse_command(command);

        if is_brightness {
            execute_command("0x10", &value.to_string(), "dev:/dev/i2c-5");
        } else {
            execute_command("0x12", &value.to_string(), "dev:/dev/i2c-5");
        }
    }
}

pub fn set_brightness(path: &str, value: usize) {
    let value = if value > 100 { 100 } else { value };

    // Low 7 bits for the value, high bit set to 1 == brightness
    let encoded = value as u8 | 1 << 7;

    let socket = UnixDatagram::unbound().expect("Failed to create client socket");
    socket.send_to(vec![encoded].as_slice(), path).unwrap();
}

pub fn set_contrast(path: &str, value: usize) {
    let value = if value > 100 { 100 } else { value };

    // Low 7 bits for the value, high bit set to 0 == contrast
    let encoded = value as u8 & (1 << 7) - 1;

    let socket = UnixDatagram::unbound().expect("Failed to create client socket");
    socket.send_to(vec![encoded].as_slice(), path).unwrap();
}
