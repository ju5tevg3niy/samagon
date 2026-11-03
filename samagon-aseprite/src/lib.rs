use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
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
    pub fractional: u16,
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
    FileIOError {
        inner: io::Error,
    },
    HeaderParseIOError {
        inner: io::Error,
    },
    FrameHeaderParseIOError {
        inner: io::Error,
        frame_num: usize,
    },
    FrameChunkHeaderParseIOError {
        inner: io::Error,
        frame_num: usize,
        frame_chunk_num: usize,
    },
    FrameChunkParseIOError {
        inner: io::Error,
        frame_num: usize,
        frame_chunk_num: usize,
    },
    FrameChunkUnknown {
        chunk_type: AsepriteWord,
        frame_num: usize,
        frame_chunk_num: usize,
    },
}

impl fmt::Display for AsepriteFileReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileIOError { .. } => {
                write!(f, "IO error during Aseprite file reading")
            }
            Self::HeaderParseIOError { .. } => {
                write!(f, "IO error during Aseprite file header parsing")
            }
            Self::FrameHeaderParseIOError { frame_num, .. } => {
                write!(
                    f,
                    "IO error during Aseprite frame #{frame_num} header parsing"
                )
            }
            Self::FrameChunkHeaderParseIOError {
                frame_num,
                frame_chunk_num,
                ..
            } => {
                write!(
                    f,
                    "IO error during Aseprite frame #{frame_num} chunk #{frame_chunk_num} header parsing"
                )
            }
            Self::FrameChunkParseIOError {
                frame_num,
                frame_chunk_num,
                ..
            } => {
                write!(
                    f,
                    "IO error during Aseprite frame #{frame_num} chunk #{frame_chunk_num} parsing"
                )
            }
            Self::FrameChunkUnknown {
                chunk_type,
                frame_num,
                frame_chunk_num,
            } => {
                write!(
                    f,
                    "encountered unknown chunk type {chunk_type:#x} in frame #{frame_num} chunk #{frame_chunk_num}"
                )
            }
        }
    }
}

impl Error for AsepriteFileReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileIOError { inner } => Some(inner),
            Self::HeaderParseIOError { inner } => Some(inner),
            Self::FrameHeaderParseIOError { inner, .. } => Some(inner),
            Self::FrameChunkHeaderParseIOError { inner, .. } => Some(inner),
            Self::FrameChunkParseIOError { inner, .. } => Some(inner),
            Self::FrameChunkUnknown { .. } => None,
        }
    }
}

#[derive(Debug)]
pub struct AsepriteFileRaw {
    pub header_raw: AsepriteHeaderRaw,
    pub frames_raw: Vec<AsepriteFrameRaw>,
}

#[derive(Debug)]
pub struct AsepriteHeaderRaw {
    pub file_size: AsepriteDword,
    pub magic: AsepriteWord,
    pub frame_count: AsepriteWord,
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

#[derive(Debug)]
pub struct AsepriteFrameRaw {
    pub header_raw: AsepriteFrameHeaderRaw,
    pub chunks_raw: Vec<AsepriteFrameChunkRaw>,
}

#[derive(Debug)]
pub struct AsepriteFrameHeaderRaw {
    pub size_bytes: AsepriteDword,
    pub magic: AsepriteWord,
    pub chunk_count_old: AsepriteWord,
    pub duration_milliseconds: AsepriteWord,
    pub zero_reserved: [AsepriteByte; 2],
    pub chunk_count_new: AsepriteDword,
}

#[derive(Debug)]
pub struct AsepriteFrameChunkRaw {
    pub header_raw: AsepriteFrameChunkHeaderRaw,
    pub data: AsepriteFrameChunkDataRaw,
}

#[derive(Debug)]
pub struct AsepriteFrameChunkHeaderRaw {
    pub size: AsepriteDword,
    pub chunk_type: AsepriteWord,
}

#[derive(Debug)]
pub enum AsepriteFrameChunkDataRaw {
    ColorProfile {
        profile_type: AsepriteWord,
        flags: AsepriteWord,
        gamma: AsepriteFixed,
        zero_reserved: [AsepriteByte; 8],
        icc_profile: Option<AsepriteFrameChunkColorProfileICC>,
    },
    Ignored,
}

#[derive(Debug)]
pub struct AsepriteFrameChunkColorProfileICC {
    pub size: AsepriteDword,
    pub data: Vec<AsepriteByte>,
}

pub const ASEPRITE_CHUNK_COLOR_PROFILE: AsepriteWord = 0x2007;

pub fn read_file_raw<P: AsRef<Path>>(path: P) -> Result<AsepriteFileRaw, AsepriteFileReadError> {
    let file = File::open(path).map_err(|inner| AsepriteFileReadError::FileIOError { inner })?;
    let mut file_reader = BufReader::new(file);

    let header_raw = read_aseprite_header_raw(&mut file_reader)
        .map_err(|inner| AsepriteFileReadError::HeaderParseIOError { inner })?;

    let frame_count = header_raw.frame_count as usize;

    let mut frames_raw = Vec::with_capacity(frame_count);

    for frame_num in 0..frame_count {
        let frame_header_raw = read_aseprite_frame_header_raw(&mut file_reader)
            .map_err(|inner| AsepriteFileReadError::FrameHeaderParseIOError { inner, frame_num })?;

        let frame_chunk_count = if frame_header_raw.chunk_count_new != 0 {
            frame_header_raw.chunk_count_new as usize
        } else {
            frame_header_raw.chunk_count_old as usize
        };

        let mut frame_chunks_raw = Vec::with_capacity(frame_chunk_count);

        for frame_chunk_num in 0..frame_chunk_count {
            let chunk_header_raw =
                read_aseprite_frame_chunk_header_raw(&mut file_reader).map_err(|inner| {
                    AsepriteFileReadError::FrameChunkHeaderParseIOError {
                        inner,
                        frame_num,
                        frame_chunk_num,
                    }
                })?;

            let map_chunk_io_error_to_aseprite_error =
                |inner| AsepriteFileReadError::FrameChunkParseIOError {
                    inner,
                    frame_num,
                    frame_chunk_num,
                };

            let data = match chunk_header_raw.chunk_type {
                ASEPRITE_CHUNK_COLOR_PROFILE => {
                    read_aseprite_frame_chunk_color_raw(&mut file_reader)
                        .map_err(map_chunk_io_error_to_aseprite_error)?
                }
                _ => AsepriteFrameChunkDataRaw::Ignored,
                // unknown => {
                //     return Err(AsepriteFileReadError::FrameChunkUnknown {
                //         chunk_type: unknown,
                //         frame_num,
                //         frame_chunk_num,
                //     });
                // }
            };

            frame_chunks_raw.push(AsepriteFrameChunkRaw {
                header_raw: chunk_header_raw,
                data,
            })
        }

        frames_raw.push(AsepriteFrameRaw {
            header_raw: frame_header_raw,
            chunks_raw: frame_chunks_raw,
        });
    }

    Ok(AsepriteFileRaw {
        header_raw,
        frames_raw,
    })
}

pub const ASEPRITE_COLOR_PROFILE_TYPE_NONE: AsepriteWord = 0;
pub const ASEPRITE_COLOR_PROFILE_TYPE_SRGB: AsepriteWord = 1;
pub const ASEPRITE_COLOR_PROFILE_TYPE_ICC: AsepriteWord = 2;

fn read_aseprite_frame_chunk_color_raw<R: Read>(
    reader: &mut R,
) -> io::Result<AsepriteFrameChunkDataRaw> {
    let profile_type = read_aseprite_word(reader)?;

    let flags = read_aseprite_word(reader)?;

    let gamma = read_aseprite_fixed(reader)?;

    let mut zero_reserved = [0; 8];

    reader.read_exact(&mut zero_reserved)?;

    let icc_profile = if profile_type == ASEPRITE_COLOR_PROFILE_TYPE_ICC {
        let size = read_aseprite_dword(reader)?;

        let mut data = vec![0; size as usize];

        reader.read_exact(&mut data)?;

        Some(AsepriteFrameChunkColorProfileICC { size, data })
    } else {
        None
    };

    Ok(AsepriteFrameChunkDataRaw::ColorProfile {
        profile_type,
        flags,
        gamma,
        zero_reserved,
        icc_profile,
    })
}

fn read_aseprite_header_raw<R: Read>(reader: &mut R) -> io::Result<AsepriteHeaderRaw> {
    let file_size = read_aseprite_dword(reader)?;

    let magic = read_aseprite_word(reader)?;

    let frame_count = read_aseprite_word(reader)?;

    let width = read_aseprite_word(reader)?;

    let height = read_aseprite_word(reader)?;

    let color_depth = read_aseprite_word(reader)?;

    let flags = read_aseprite_dword(reader)?;

    let speed = read_aseprite_word(reader)?;

    let zero_dword1 = read_aseprite_dword(reader)?;

    let zero_dword2 = read_aseprite_dword(reader)?;

    let pallete_entry = read_aseprite_byte(reader)?;

    let mut ignored_bytes = [0; 3];

    reader.read_exact(&mut ignored_bytes)?;

    let color_count = read_aseprite_word(reader)?;

    let pixel_width = read_aseprite_byte(reader)?;

    let pixel_height = read_aseprite_byte(reader)?;

    let grid_x = read_aseprite_short(reader)?;

    let grid_y = read_aseprite_short(reader)?;

    let grid_width = read_aseprite_word(reader)?;

    let grid_height = read_aseprite_word(reader)?;

    let mut zero_reserved = [0; 84];

    reader.read_exact(&mut zero_reserved)?;

    Ok(AsepriteHeaderRaw {
        file_size,
        magic,
        frame_count,
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

fn read_aseprite_frame_header_raw<R: Read>(reader: &mut R) -> io::Result<AsepriteFrameHeaderRaw> {
    let size_bytes = read_aseprite_dword(reader)?;

    let magic = read_aseprite_word(reader)?;

    let chunk_count_old = read_aseprite_word(reader)?;

    let duration_milliseconds = read_aseprite_word(reader)?;

    let mut zero_reserved = [0; 2];

    reader.read_exact(&mut zero_reserved)?;

    let chunk_count_new = read_aseprite_dword(reader)?;

    Ok(AsepriteFrameHeaderRaw {
        size_bytes,
        magic,
        chunk_count_old,
        duration_milliseconds,
        zero_reserved,
        chunk_count_new,
    })
}

fn read_aseprite_frame_chunk_header_raw<R: Read>(
    reader: &mut R,
) -> io::Result<AsepriteFrameChunkHeaderRaw> {
    let size = read_aseprite_dword(reader)?;

    let chunk_type = read_aseprite_word(reader)?;

    Ok(AsepriteFrameChunkHeaderRaw { size, chunk_type })
}

fn read_aseprite_byte<R: Read>(reader: &mut R) -> io::Result<AsepriteByte> {
    let mut byte_buf = [0; 1];

    reader.read_exact(&mut byte_buf)?;

    Ok(u8::from_le_bytes(byte_buf))
}

fn read_aseprite_word<R: Read>(reader: &mut R) -> io::Result<AsepriteWord> {
    let mut byte_buf = [0; 2];

    reader.read_exact(&mut byte_buf)?;

    Ok(u16::from_le_bytes(byte_buf))
}

fn read_aseprite_short<R: Read>(reader: &mut R) -> io::Result<AsepriteShort> {
    let mut byte_buf = [0; 2];

    reader.read_exact(&mut byte_buf)?;

    Ok(i16::from_le_bytes(byte_buf))
}

fn read_aseprite_dword<R: Read>(reader: &mut R) -> io::Result<AsepriteDword> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(u32::from_le_bytes(byte_buf))
}

fn read_aseprite_long<R: Read>(reader: &mut R) -> io::Result<AsepriteLong> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(i32::from_le_bytes(byte_buf))
}

fn read_aseprite_fixed<R: Read>(reader: &mut R) -> io::Result<AsepriteFixed> {
    let mut byte_buf_integer = [0; 2];
    let mut byte_buf_fractional = [0; 2];

    reader.read_exact(&mut byte_buf_integer)?;
    reader.read_exact(&mut byte_buf_fractional)?;

    Ok(AsepriteFixed {
        integer: u16::from_le_bytes(byte_buf_integer),
        fractional: u16::from_le_bytes(byte_buf_fractional),
    })
}

fn read_aseprite_float<R: Read>(reader: &mut R) -> io::Result<AsepriteFloat> {
    let mut byte_buf = [0; 4];

    reader.read_exact(&mut byte_buf)?;

    Ok(f32::from_le_bytes(byte_buf))
}

fn read_aseprite_double<R: Read>(reader: &mut R) -> io::Result<AsepriteDouble> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(f64::from_le_bytes(byte_buf))
}

fn read_aseprite_qword<R: Read>(reader: &mut R) -> io::Result<AsepriteQword> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(u64::from_le_bytes(byte_buf))
}

fn read_aseprite_long64<R: Read>(reader: &mut R) -> io::Result<AsepriteLong64> {
    let mut byte_buf = [0; 8];

    reader.read_exact(&mut byte_buf)?;

    Ok(i64::from_le_bytes(byte_buf))
}
