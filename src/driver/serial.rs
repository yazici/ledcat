use std::fs;
use std::os::unix::io::AsRawFd;
use std::path;
use nix::sys::termios;
use driver;

pub fn open<P: AsRef<path::Path>>(path: P, baudrate: u32) -> Result<fs::File, driver::Error> {
    let tty = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)?;
    let fd = tty.as_raw_fd();
    let mut tio = termios::tcgetattr(fd)?;
    tio.input_flags &= !(termios::ICRNL|termios::BRKINT);
    tio.output_flags &= !(termios::OPOST|termios::ONLCR);
    tio.local_flags &= !(termios::ICANON|termios::ISIG|termios::ECHO);
    termios::cfsetspeed(&mut tio, map_baudrate(baudrate))?;
    termios::tcsetattr(fd, termios::SetArg::TCSANOW, &tio)?;
    Ok(tty)
}

pub fn is_serial(path: &path::Path) -> bool {
    path.to_str()
        .map(|p| p.starts_with("/dev/tty"))
        .unwrap_or(false)
}

fn map_baudrate(b: u32) -> termios::BaudRate {
    let map = [
        (4_000_000, termios::BaudRate::B4000000),
        (3_500_000, termios::BaudRate::B3500000),
        (3_000_000, termios::BaudRate::B3000000),
        (2_500_000, termios::BaudRate::B2500000),
        (2_000_000, termios::BaudRate::B2000000),
        (1_500_000, termios::BaudRate::B1500000),
        (1_152_000, termios::BaudRate::B1152000),
        (1_000_000, termios::BaudRate::B1000000),
        (921_600, termios::BaudRate::B921600),
        (576_000, termios::BaudRate::B576000),
        (500_000, termios::BaudRate::B500000),
        (460_800, termios::BaudRate::B460800),
        (230_400, termios::BaudRate::B230400),
        (115_200, termios::BaudRate::B115200),
        (57_600, termios::BaudRate::B57600),
        (38_400, termios::BaudRate::B38400),
        (19_200, termios::BaudRate::B19200),
        (9600, termios::BaudRate::B9600),
        (4800, termios::BaudRate::B4800),
        (2400, termios::BaudRate::B2400),
        (1800, termios::BaudRate::B1800),
        (1200, termios::BaudRate::B1200),
        (600, termios::BaudRate::B600),
        (300, termios::BaudRate::B300),
        (200, termios::BaudRate::B200),
        (150, termios::BaudRate::B150),
        (134, termios::BaudRate::B134),
        (110, termios::BaudRate::B110),
        (75, termios::BaudRate::B75),
        (50, termios::BaudRate::B50),
    ];
    for &(num, br) in &map {
        if b >= num {
            return br;
        }
    }
    termios::BaudRate::B0
}
