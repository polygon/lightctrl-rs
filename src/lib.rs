use std::net::{UdpSocket, ToSocketAddrs};
use std::io;

#[derive(Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug)]
pub struct LEDDevice {
    sock: UdpSocket,
    num_leds: usize,
}

#[derive(Debug)]
pub enum LEDError {
    IOError(io::Error),
    SizeError { received: usize, expected: usize }
}

type LEDResult<T> = Result<T, LEDError>;

impl From<io::Error> for LEDError {
    fn from(error: io::Error) -> Self {
        LEDError::IOError(error)
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color{red, green, blue}
    }
}

impl LEDDevice {
    pub fn connect<A: ToSocketAddrs>(addr: A, num_leds: usize) -> LEDResult<LEDDevice> {
        let sock = UdpSocket::bind("0.0.0.0:0")?;
        sock.connect(addr)?;
        Ok(LEDDevice{sock, num_leds})
    }

    pub fn update(&self, color_values: &[Color]) -> LEDResult<()> {
        if color_values.len() != self.num_leds {
            return Err(LEDError::SizeError { expected: self.num_leds, received: color_values.len() });
        }

        let mut data: Vec<u8> = Vec::new();

        for color in color_values.iter() {
            data.extend_from_slice(&LEDDevice::to_u8(color));
        }

        self.sock.send(&data)?;

        Ok(())
    }

    fn to_u8(color: &Color) -> [u8; 3] {
        let red = (color.red.max(0.0).min(1.0) * 255.0) as u8;
        let green = (color.green.max(0.0).min(1.0) * 255.0) as u8;
        let blue = (color.blue.max(0.0).min(1.0) * 255.0) as u8;
        [red, green, blue]
    }
}

#[cfg(test)]
mod tests {
    use super::{LEDDevice, LEDError, Color};
    use std::net::UdpSocket;
    use std::thread;
    #[test]
    fn test_wrong_size() {
        let dev = LEDDevice::connect("192.0.2.0:1234", 1).unwrap();
        match dev.update(&[Color::new(0.0, 0.0, 0.0), Color::new(1.0, 0.0, 1.0)]) {
            Ok(_) => panic!(),
            Err(LEDError::SizeError{expected, received}) => {assert_eq!(expected, 1); assert_eq!(received, 2);},
            Err(_) => panic!()
        }
    }

    #[test]
    fn test_data() {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let addr = socket.local_addr().unwrap();
        let _th = thread::spawn(move || {
            let dev = LEDDevice::connect(addr, 1).unwrap();
            dev.update(&[Color::new(1.0, 0.5, 0.0)]).unwrap();
        });
        let mut recv = [0; 3];
        socket.recv_from(&mut recv).unwrap();
        assert_eq!(recv, [255, 127, 0]);
    }
}