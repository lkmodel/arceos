pub mod elf;
pub mod auxv;

use core::fmt;
use axlog::debug;
use xmas_elf::{ElfFile, header};

pub const PLASH_START: usize = 0xffff_ffc0_2200_0000;
pub const EXEC_ZONE_START: usize = 0xffff_ffc0_8010_0000;
// pub const MAX_APP_SIZE: usize = 0x100000;

pub fn verify_elf_header(elf: &ElfFile) -> Result<(), LoadError> {
    let header = elf.header;
    let magic = header.pt1.magic;
    debug!("ELF header: {:?}", header);
    assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

    // 1. 验证目标架构
    if header.pt2.machine().as_machine() != header::Machine::RISC_V {
        debug!("Wrong architecture: expected RISC-V, got {:?}", header.pt2.machine());
        return Err(LoadError::WrongArchitecture);
    }

    // 2. 验证程序头表是否存在
    if header.pt2.ph_count() == 0 {
        debug!("No program headers found");
        return Err(LoadError::NoSegments);
    }

    // 3. 验证 ELF 版本
    if header.pt2.version() != 1 {
        debug!("Invalid ELF version");
        return Err(LoadError::InvalidVersion);
    }

    // 4. 验证入口点是否有效
    if header.pt2.entry_point() == 0 {
        debug!("Invalid entry point");
        return Err(LoadError::InvalidEntryPoint);
    }

    // 5. 验证程序头表偏移
    if header.pt2.ph_offset() == 0 {
        debug!("Invalid program header offset");
        return Err(LoadError::InvalidProgramHeaderOffset);
    }

    // 6. 验证 ELF 魔数
    if header.pt1.magic != [ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3] {
        debug!("Invalid ELF magic number");
        return Err(LoadError::InvalidMagic);
    }

    Ok(())
}

// 扩展错误类型
#[derive(Debug)]
pub enum LoadError {
    InvalidMagic,
    NotExecutable,
    WrongArchitecture,
    NoSegments,
    InvalidVersion,
    InvalidEntryPoint,
    InvalidProgramHeaderOffset,
    SegmentOutOfBounds,
    BadAlignment,
    RelocationError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "Invalid ELF magic number"),
            Self::NotExecutable => write!(f, "File is not executable"),
            Self::WrongArchitecture => write!(f, "Wrong target architecture"),
            Self::NoSegments => write!(f, "No program segments found"),
            Self::InvalidVersion => write!(f, "Invalid ELF version"),
            Self::InvalidEntryPoint => write!(f, "Invalid entry point"),
            Self::InvalidProgramHeaderOffset => write!(f, "Invalid program header offset"),
            Self::SegmentOutOfBounds => write!(f, "Segment out of bounds"),
            Self::BadAlignment => write!(f, "Bad segment alignment"),
            Self::RelocationError => write!(f, "Relocation error"),
        }
    }
}

// ELF 常量定义
const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = b'E';
const ELFMAG2: u8 = b'L';
const ELFMAG3: u8 = b'F';