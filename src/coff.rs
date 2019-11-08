#![allow(dead_code)]
extern crate chrono;

use std::mem;
use std::result::Result;
use std::convert::TryInto;

use chrono::{DateTime, Utc, NaiveDateTime};

use crate::errors::*;

#[derive(Debug, Default)]
pub struct CoffHeader {
    f_machine: u16,     /* machine type */
    f_nscns: u16,       /* number of sections */
    f_timdat: u32,      /* timestamp */
    f_symptr: u32,      /* pointer to symbol table */
    f_nsyms: u32,       /* number of symbols */
    f_opthdr: u16,      /* pointer to optional header */
    f_flags: u16        /* flags */
}

#[derive(Debug)]
pub enum CoffHeaderMachineType {
    Unknown,
    AM33,
    AMD64,
    ARM,
    ARM64,
    ARMNT,
    EBC,
    I386,
    IA64,
    M32R,
    MIPS16,
    MIPSFPU,
    MIPSFPU16,
    PowerPC,
    PowerPCFP,
    R4000,
    RISCV32,
    RISCV64,
    RISCV128,
    SH3,
    SH3DSP,
    SH4,
    SH5,
    Thumb,
    WCEMIPSV2
}

impl CoffHeaderMachineType {
    pub fn to_value(machine: CoffHeaderMachineType) -> u16 {
        match machine {
            CoffHeaderMachineType::Unknown => 0x0000,
            CoffHeaderMachineType::AM33 => 0x01D3,
            CoffHeaderMachineType::AMD64 => 0x8664,
            CoffHeaderMachineType::ARM => 0x01C0,
            CoffHeaderMachineType::ARM64 => 0xAA64,
            CoffHeaderMachineType::ARMNT => 0x01C4,
            CoffHeaderMachineType::EBC => 0x0EBC,
            CoffHeaderMachineType::I386 => 0x014C,
            CoffHeaderMachineType::IA64 => 0x0200,
            CoffHeaderMachineType::M32R => 0x9041,
            CoffHeaderMachineType::MIPS16 => 0x0266,
            CoffHeaderMachineType::MIPSFPU => 0x0366,
            CoffHeaderMachineType::MIPSFPU16 => 0x0466,
            CoffHeaderMachineType::PowerPC => 0x01F0,
            CoffHeaderMachineType::PowerPCFP => 0x01F1,
            CoffHeaderMachineType::R4000 => 0x0166,
            CoffHeaderMachineType::RISCV32 => 0x5032,
            CoffHeaderMachineType::RISCV64 => 0x5064,
            CoffHeaderMachineType::RISCV128 => 0x5128,
            CoffHeaderMachineType::SH3 => 0x01A2,
            CoffHeaderMachineType::SH3DSP => 0x01A3,
            CoffHeaderMachineType::SH4 => 0x01A6,
            CoffHeaderMachineType::SH5 => 0x01A8,
            CoffHeaderMachineType::Thumb => 0x01C2,
            CoffHeaderMachineType::WCEMIPSV2 => 0x0169
        }
    }

    pub fn from_value(value: u16) -> CoffHeaderMachineType {
        match value {
            0x0000 => CoffHeaderMachineType::Unknown,
            0x01D3 => CoffHeaderMachineType::AM33,
            0x8664 => CoffHeaderMachineType::AMD64,
            0x01C0 => CoffHeaderMachineType::ARM,
            0xAA64 => CoffHeaderMachineType::ARM64,
            0x01C4 => CoffHeaderMachineType::ARMNT,
            0x0EBC => CoffHeaderMachineType::EBC,
            0x014C => CoffHeaderMachineType::I386,
            0x0200 => CoffHeaderMachineType::IA64,
            0x9041 => CoffHeaderMachineType::M32R,
            0x0266 => CoffHeaderMachineType::MIPS16,
            0x0366 => CoffHeaderMachineType::MIPSFPU,
            0x0466 => CoffHeaderMachineType::MIPSFPU16,
            0x01F0 => CoffHeaderMachineType::PowerPC,
            0x01F1 => CoffHeaderMachineType::PowerPCFP,
            0x0166 => CoffHeaderMachineType::R4000,
            0x5032 => CoffHeaderMachineType::RISCV32,
            0x5064 => CoffHeaderMachineType::RISCV64,
            0x5128 => CoffHeaderMachineType::RISCV128,
            0x01A2 => CoffHeaderMachineType::SH3,
            0x01A3 => CoffHeaderMachineType::SH3DSP,
            0x01A6 => CoffHeaderMachineType::SH4,
            0x01A8 => CoffHeaderMachineType::SH5,
            0x01C2 => CoffHeaderMachineType::Thumb,
            0x0169 => CoffHeaderMachineType::WCEMIPSV2,
            _ => CoffHeaderMachineType::Unknown
        }
    }
}

pub enum CoffHeaderCharacteristic {
    RelocsStripped,
    ExecutableImage,
    LineNumsStripped,
    LocalSymsStripped,
    AggressiveWsTrim,
    LargeAddressAware,
    Reserved,
    BytesReversedLo,
    Machine32Bit,
    DebugStripped,
    RemovableRunFromSwap,
    NetRunFromSwap,
    FileSystem,
    Dll,
    UpSystemOnly,
    BytesReversedHi
}

impl CoffHeaderCharacteristic {
    pub fn to_value(characteristic: CoffHeaderCharacteristic) -> u16 {
        match characteristic {
            CoffHeaderCharacteristic::RelocsStripped => 0x0001,
            CoffHeaderCharacteristic::ExecutableImage => 0x0002,
            CoffHeaderCharacteristic::LineNumsStripped => 0x0004,
            CoffHeaderCharacteristic::LocalSymsStripped => 0x0008,
            CoffHeaderCharacteristic::AggressiveWsTrim => 0x0010,
            CoffHeaderCharacteristic::LargeAddressAware => 0x0020,
            CoffHeaderCharacteristic::Reserved => 0x0040,
            CoffHeaderCharacteristic::BytesReversedLo => 0x0080,
            CoffHeaderCharacteristic::Machine32Bit => 0x0100,
            CoffHeaderCharacteristic::DebugStripped => 0x0200,
            CoffHeaderCharacteristic::RemovableRunFromSwap => 0x0400,
            CoffHeaderCharacteristic::NetRunFromSwap => 0x0800,
            CoffHeaderCharacteristic::FileSystem => 0x1000,
            CoffHeaderCharacteristic::Dll => 0x2000,
            CoffHeaderCharacteristic::UpSystemOnly => 0x4000,
            CoffHeaderCharacteristic::BytesReversedHi => 0x8000
        }
    }
}

impl CoffHeader {
    pub fn get_machine(&self) -> u16 {
        self.f_machine
    }

    pub fn get_machine_as_enum(&self) -> CoffHeaderMachineType {
        CoffHeaderMachineType::from_value(self.f_machine)
    }

    pub fn set_machine(&mut self, machine: u16) {
        self.f_machine = machine;
    }

    pub fn set_machine_as_enum(&mut self, machine: CoffHeaderMachineType) {
        self.f_machine = CoffHeaderMachineType::to_value(machine);
    }

    pub fn get_nscns(&self) -> u16 {
        self.f_nscns
    }

    pub fn set_nscns(&mut self, nscns: u16) {
        self.f_nscns = nscns;
    }

    pub fn get_timdat(&self) -> u32 {
        self.f_timdat
    }

    pub fn get_timdat_as_dt(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.f_timdat as i64, 0), Utc)
    }

    pub fn set_timdat(&mut self, timdat: u32) {
        self.f_timdat = timdat;
    }

    pub fn set_timdat_as_dt(&mut self, timdat: DateTime<Utc>) ->
        Result<(), ButylError> {
        match timdat.timestamp().try_into() {
            Ok(t) => {
                self.f_timdat = t;
                return Ok(());
            },
            Err(_e) => return Err(ButylError::ExcessiveDataError)
        };
    }

    pub fn get_symptr(&self) -> u32 {
        self.f_symptr
    }

    pub fn set_symptr(&mut self, symptr: u32) {
        self.f_symptr = symptr;
    }

    pub fn get_nsyms(&self) -> u32 {
        self.f_nsyms
    }

    pub fn set_nsyms(&mut self, nsyms: u32) {
        self.f_nsyms = nsyms;
    }

    pub fn get_opthdr(&self) -> u16 {
        self.f_opthdr
    }

    pub fn set_opthdr(&mut self, opthdr: u16) {
        self.f_opthdr = opthdr;
    }

    pub fn get_flags(&self) -> u16 {
        self.f_flags
    }

    pub fn set_flags(&mut self, flags: u16) {
        self.f_flags = flags;
    }

    pub fn is_flag_set(&self, flag: CoffHeaderCharacteristic) -> bool {
        self.f_flags & CoffHeaderCharacteristic::to_value(flag) != 0
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Result<CoffHeader, ButylError> {
        if bytes.len() < mem::size_of::<CoffHeader>() { /* bounds check */
            return Err(ButylError::InsufficientDataError);
        }

        let mut coff_header: CoffHeader = CoffHeader::default();

        coff_header.f_machine = ((bytes[0] as u16) << 8) | bytes[1] as u16;
        coff_header.f_nscns = ((bytes[2] as u16) << 8) | bytes[3] as u16;
        coff_header.f_timdat =
            ((bytes[4] as u32) << 24) |
            ((bytes[5] as u32) << 16) | 
            ((bytes[6] as u32) << 8) | 
            bytes[7] as u32;
        coff_header.f_symptr = 
            ((bytes[8] as u32) << 24) |
            ((bytes[9] as u32) << 16) |
            ((bytes[10] as u32) << 8) |
            bytes[11] as u32;
        coff_header.f_nsyms = 
            ((bytes[12] as u32) << 24) |
            ((bytes[13] as u32) << 16) |
            ((bytes[14] as u32) << 8) |
            bytes[15] as u32;
        coff_header.f_opthdr = ((bytes[16] as u16) << 8) | bytes[17] as u16;
        coff_header.f_flags = ((bytes[17] as u16) << 8) | bytes[18] as u16;

        Ok(coff_header)
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Result<CoffHeader, ButylError> {
        if bytes.len() < mem::size_of::<CoffHeader>() { /* bounds check */
            return Err(ButylError::InsufficientDataError);
        }

        let mut coff_header: CoffHeader = CoffHeader::default();

        coff_header.f_machine = ((bytes[1] as u16) << 8) | bytes[0] as u16;
        coff_header.f_nscns = ((bytes[3] as u16) << 8) | bytes[2] as u16;
        coff_header.f_timdat =
            ((bytes[7] as u32) << 24) |
            ((bytes[6] as u32) << 16) | 
            ((bytes[5] as u32) << 8) | 
            bytes[4] as u32;
        coff_header.f_symptr = 
            ((bytes[11] as u32) << 24) |
            ((bytes[10] as u32) << 16) |
            ((bytes[9] as u32) << 8) |
            bytes[8] as u32;
        coff_header.f_nsyms = 
            ((bytes[15] as u32) << 24) |
            ((bytes[14] as u32) << 16) |
            ((bytes[13] as u32) << 8) |
            bytes[12] as u32;
        coff_header.f_opthdr = ((bytes[17] as u16) << 8) | bytes[16] as u16;
        coff_header.f_flags = ((bytes[19] as u16) << 8) | bytes[18] as u16;

        Ok(coff_header)
    }
}

#[derive(Debug)]
pub struct CoffFile<'a> {
    header: CoffHeader,
    data: &'a[u8]
}

impl<'a> CoffFile<'a> {
    pub fn from_be_bytes(data: &[u8]) -> Result<CoffFile, ButylError> {
        Ok(CoffFile {
            header: CoffHeader::from_be_bytes(data)?,
            data: data
        })
    }

    pub fn from_le_bytes(data: &[u8]) -> Result<CoffFile, ButylError> {
        Ok(CoffFile {
            header: CoffHeader::from_le_bytes(data)?,
            data: data
        })
    }
}

