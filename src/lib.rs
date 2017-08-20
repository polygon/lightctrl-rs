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