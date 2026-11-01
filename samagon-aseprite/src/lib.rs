use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

pub type AsepriteByte = u8;
pub type AsepriteWord = u16;
pub type AsepriteShort = i16;
pub type AsepriteDword = u32;
pub type AsepriteLong = i32;
pub type AsepriteFloat = f32;
pub type AsepriteDouble = f64;
pub type AsepriteQword = u64;
pub type AsepriteLong64 = i64;

#[derive(Debug)]
pub struct AsepriteFixed {
    pub integer: u16,
    pub fractionalional: u16,
}

#[derive(Debug)]
pub struct AsepriteByteArray {
    pub bytes: Vec<AsepriteByte>,
}

#[derive(Debug)]
pub struct AsepriteString {
    pub length: AsepriteWord,
    pub characters: Vec<AsepriteByte>,
}

#[derive(Debug)]
pub struct AsepritePoint {
    pub x: AsepriteLong,
    pub y: AsepriteLong,
}

#[derive(Debug)]
pub struct AsepriteSize {
    pub width: AsepriteLong,
    pub height: AsepriteLong,
}

#[derive(Debug)]
pub struct AsepriteRect {
    pub origin: AsepritePoint,
    pub size: AsepriteSize,
}

#[derive(Debug)]
pub enum AsepritePixel {
    RGBA {
        red: AsepriteByte,
        green: AsepriteByte,
        blue: AsepriteByte,
        alpha: AsepriteByte,
    },
    Grayscale {
        value: AsepriteByte,
        alpha: AsepriteByte,
    },
    Indexed {
        index: AsepriteByte,
    },
}

#[derive(Debug)]
pub enum AsepriteTile {
    Bit8(AsepriteByte),
    Bit16(AsepriteWord),
    Bit32(AsepriteDword),
}

pub type AsepriteUUID = [AsepriteByte; 16];

#[derive(Debug)]
pub enum AsepriteFileReadError {
    FileIOError(io::Error),
    HeaderParseIOError(io::Error),
}

impl fmt::Display for AsepriteFileReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsepriteFileReadError::FileIOError(_) => {
                write!(f, "IO error during Aseprite file reading")
            }
            AsepriteFileReadError::HeaderParseIOError(_) => {
                write!(f, "IO error during Aseprite file header parsing")
            }
        }
    }
}

impl Error for AsepriteFileReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AsepriteFileReadError::FileIOError(error) => Some(error),
            AsepriteFileReadError::HeaderParseIOError(error) => Some(error),
        }
    }
}

#[derive(Debug)]
pub struct AsepriteHeaderRaw {
    pub file_size: AsepriteDword,
    pub magic: AsepriteWord,
    pub frames: AsepriteWord,
    pub width: AsepriteWord,
    pub height: AsepriteWord,
    pub color_depth: AsepriteWord,
    pub flags: AsepriteDword,
    pub speed: AsepriteWord,
    pub zero_dword1: AsepriteDword,
    pub zero_dword2: AsepriteDword,
    pub pallete_entry: AsepriteByte,
    pub ignored_bytes: [AsepriteByte; 3],
    pub color_count: AsepriteWord,
    pub pixel_width: AsepriteByte,
    pub pixel_height: AsepriteByte,
    pub grid_x: AsepriteShort,
    pub grid_y: AsepriteShort,
    pub grid_width: AsepriteWord,
    pub grid_height: AsepriteWord,
    pub zero_reserved: [AsepriteByte; 84],
}

type AsepriteHeaderByteArray = [u8; 128];

#[derive(Debug)]
pub struct AsepriteFileRaw {
    pub header_raw: AsepriteHeaderRaw,
}

pub fn read_file_raw<P: AsRef<Path>>(path: P) -> Result<AsepriteFileRaw, AsepriteFileReadError> {
    let file = File::open(path).map_err(AsepriteFileReadError::FileIOError)?;
    let mut file_reader = BufReader::new(file);

    let mut header_bytes = [0; 128];

    file_reader
        .read_exact(&mut header_bytes)
        .map_err(AsepriteFileReadError::FileIOError)?;

    let header_raw =
        read_header_raw(header_bytes).map_err(AsepriteFileReadError::HeaderParseIOError)?;

    Ok(AsepriteFileRaw { header_raw })
}

fn read_header_raw(header_bytes: AsepriteHeaderByteArray) -> Result<AsepriteHeaderRaw, io::Error> {
    let mut cursor = Cursor::new(header_bytes);

    let file_size = read_aseprite_dword(&mut cursor)?;

    let magic = read_aseprite_word(&mut cursor)?;

    let frames = read_aseprite_word(&mut cursor)?;

    let width = read_aseprite_word(&mut cursor)?;

    let height = read_aseprite_word(&mut cursor)?;

    let color_depth = read_aseprite_word(&mut cursor)?;

    let flags = read_aseprite_dword(&mut cursor)?;

    let speed = read_aseprite_word(&mut cursor)?;

    let zero_dword1 = read_aseprite_dword(&mut cursor)?;

    let zero_dword2 = read_aseprite_dword(&mut cursor)?;

    let pallete_entry = read_aseprite_byte(&mut cursor)?;

    let mut ignored_bytes = [0; 3];

    cursor.read_exact(&mut ignored_bytes)?;

    let color_count = read_aseprite_word(&mut cursor)?;

    let pixel_width = read_aseprite_byte(&mut cursor)?;

    let pixel_height = read_aseprite_byte(&mut cursor)?;

    let grid_x = read_aseprite_short(&mut cursor)?;

    let grid_y = read_aseprite_short(&mut cursor)?;

    let grid_width = read_aseprite_word(&mut cursor)?;

    let grid_height = read_aseprite_word(&mut cursor)?;

    let mut zero_reserved = [0; 84];

    cursor.read_exact(&mut zero_reserved)?;

    Ok(AsepriteHeaderRaw {
        file_size,
        magic,
        frames,
        width,
        height,
        color_depth,
        flags,
        speed,
        zero_dword1,
        zero_dword2,
        pallete_entry,
        ignored_bytes,
        color_count,
        pixel_width,
        pixel_height,
        grid_x,
        grid_y,
        grid_width,
        grid_height,
        zero_reserved,
    })
}

fn read_aseprite_byte<R: Read>(reader: &mut R) -> Result<AsepriteByte, io::Error> {
    let mut byte_buf = [0; 1];

    reader.read_exact(&mut byte_buf)?;

    Ok(u8::from_le_bytes(byte_buf))
}

fn read_aseprite_word<R: Read>(reader: &mut R) -> Result<AsepriteWord, io::Error> {
    let mut byte_buf = [0; 2];

    reader.read_exact(&mut byte_buf)?;

    Ok(u16::from_le_bytes(byte_buf))
}

fn read_aseprite_short<R: Read>(reader: &mut R) -> Result<AsepriteShort, io::Error> {
    let mut byte_buf = [0; 2];

    reader.read_exact(&mut byte_buf)?;

    Ok(i16::from_le_bytes(byte_buf))
}

fn read_aseprite_dword<R: Read>(reader: &mut R) -> Result<AsepriteDword, io::Error> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(u32::from_le_bytes(byte_buf))
}

fn read_aseprite_long<R: Read>(reader: &mut R) -> Result<AsepriteLong, io::Error> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(i32::from_le_bytes(byte_buf))
}

fn read_aseprite_float<R: Read>(reader: &mut R) -> Result<AsepriteFloat, io::Error> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(f32::from_le_bytes(byte_buf))
}

fn read_aseprite_double<R: Read>(reader: &mut R) -> Result<AsepriteDouble, io::Error> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(f64::from_le_bytes(byte_buf))
}

fn read_aseprite_qword<R: Read>(reader: &mut R) -> Result<AsepriteQword, io::Error> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(u64::from_le_bytes(byte_buf))
}

fn read_aseprite_long64<R: Read>(reader: &mut R) -> Result<AsepriteLong64, io::Error> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(i64::from_le_bytes(byte_buf))
}
