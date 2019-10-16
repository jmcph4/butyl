#![allow(dead_code)]

use std::mem;
use std::result::Result;
use crate::errors::*;

const DOS_HEADER_FIELD_LEN_RES1: usize = 4 * 2;
const DOS_HEADER_FIELD_LEN_RES2: usize = 10 * 2;

#[derive(Debug, Default)]
pub struct DosHeader {
    e_magic: u16,                               /* magic number */
    e_cblp: u16,                                /* num of bytes on last page */
    e_cp: u16,                                  /* num of pages in file */
    e_crlc: u16,                                /* number of relocations */
    e_cparhdr: u16,                             /* size of header in pages */
    e_minalloc: u16,                            /* min extra paragraphs */
    e_maxalloc: u16,                            /* max extra paragraphs */
    e_ss: u16,                                  /* init (relative) SS value */
    e_sp: u16,                                  /* init SP value */
    e_csum: u16,                                /* checksum */
    e_ip: u16,                                  /* init IP value */
    e_cs: u16,                                  /* init (relative) CS value */
    e_lfarlc: u16,                              /* addr relocation table */
    e_ovno: u16,                                /* overlay number */
    e_res: [u8; DOS_HEADER_FIELD_LEN_RES1],     /* (reserved) */
    e_oemid: u16,                               /* OEM identifier */
    e_oeminfo: u16,                             /* OEM information */
    e_res2: [u8; DOS_HEADER_FIELD_LEN_RES2],    /* (reserved) */
    e_lfanew: u32                               /* file address of PE header */
}

impl DosHeader {
    pub fn get_magic(&self) -> u16 {
        self.e_magic
    }

    pub fn set_magic(&mut self, magic: u16) {
        self.e_magic = magic;
    }

    pub fn get_cblp(&self) -> u16 {
        self.e_cblp
    }

    pub fn set_cblp(&mut self, cblp: u16) {
        self.e_cblp = cblp;
    }

    pub fn get_cp(&self) -> u16 {
        self.e_cp
    }

    pub fn set_cp(&mut self, cp: u16) {
        self.e_cp = cp;
    }

    pub fn get_crlc(&self) -> u16 {
        self.e_crlc
    }

    pub fn set_crlc(&mut self, crlc: u16) {
        self.e_crlc = crlc;
    }

    pub fn get_cparhdr(&self) -> u16 {
        self.e_cparhdr
    }

    pub fn set_cparhdr(&mut self, cparhdr: u16) {
        self.e_cparhdr = cparhdr;
    }

    pub fn get_minalloc(&self) -> u16 {
        self.e_minalloc
    }

    pub fn set_minalloc(&mut self, minalloc: u16) {
        self.e_minalloc = minalloc;
    }

    pub fn get_maxalloc(&self) -> u16 {
        self.e_maxalloc
    }

    pub fn set_maxalloc(&mut self, maxalloc: u16) {
        self.e_maxalloc = maxalloc;
    }
   
    pub fn get_ss(&self) -> u16 {
        self.e_ss
    }

    pub fn set_ss(&mut self, ss: u16) {
        self.e_ss = ss;
    }

    pub fn get_sp(&self) -> u16 {
        self.e_sp
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.e_sp = sp;
    }

    pub fn get_csum(&self) -> u16 {
        self.e_csum
    }

    pub fn set_csum(&mut self, csum: u16) {
        self.e_csum = csum;
    }
   
    pub fn get_ip(&self) -> u16 {
        self.e_ip
    }

    pub fn set_ip(&mut self, ip: u16) {
        self.e_ip = ip;
    }
 
    pub fn get_cs(&self) -> u16 {
        self.e_cs
    }

    pub fn set_cs(&mut self, cs: u16) {
        self.e_cs = cs;
    }

    pub fn get_lfarlc(&self) -> u16 {
        self.e_lfarlc
    }

    pub fn set_lfarlc(&mut self, lfarlc: u16) {
        self.e_lfarlc = lfarlc;
    }

    pub fn get_ovno(&self) -> u16 {
        self.e_ovno
    }

    pub fn set_ovno(&mut self, ovno: u16) {
        self.e_ovno = ovno;
    }

    pub fn get_res(&self) -> [u8; DOS_HEADER_FIELD_LEN_RES1] {
        self.e_res
    }

    pub fn set_res(&mut self, res: [u8; DOS_HEADER_FIELD_LEN_RES1]) {
        self.e_res = res;
    }

    pub fn get_oemid(&self) -> u16 {
        self.e_oemid
    }

    pub fn set_oemid(&mut self, oemid: u16) {
        self.e_oemid = oemid;
    }

    pub fn get_oeminfo(&self) -> u16 {
        self.e_oeminfo
    }

    pub fn set_oeminfo(&mut self, oeminfo: u16) {
        self.e_oeminfo = oeminfo;
    }
    
    pub fn get_res2(&self) -> [u8; DOS_HEADER_FIELD_LEN_RES2] {
        self.e_res2
    }

    pub fn set_res2(&mut self, res2: [u8; DOS_HEADER_FIELD_LEN_RES2]) {
        self.e_res2 = res2;
    }

    pub fn get_lfanew(&self) -> u32 {
        self.e_lfanew
    }

    pub fn set_lfanew(&mut self, lfanew: u32) {
        self.e_lfanew = lfanew;
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Result<DosHeader, ButylError> {
        if bytes.len() < mem::size_of::<DosHeader>() { /* bounds check */
            return Err(ButylError::InsufficientDataError);
        }

        let mut dos_header: DosHeader = DosHeader::default();

        dos_header.e_magic = ((bytes[0] as u16) << 8) | bytes[1] as u16;
        dos_header.e_cblp = ((bytes[2] as u16) << 8) | bytes[3] as u16;
        dos_header.e_cp = ((bytes[4] as u16) << 8) | bytes[5] as u16;
        dos_header.e_crlc = ((bytes[6] as u16) << 8) | bytes[7] as u16;
        dos_header.e_cparhdr = ((bytes[8] as u16) << 8) | bytes[9] as u16;
        dos_header.e_minalloc = ((bytes[10] as u16) << 8) | bytes[11] as u16;
        dos_header.e_maxalloc = ((bytes[12] as u16) << 8) | bytes[13] as u16;
        dos_header.e_ss = ((bytes[14] as u16) << 8) | bytes[15] as u16;
        dos_header.e_sp = ((bytes[16] as u16) << 8) | bytes[17] as u16; 
        dos_header.e_csum = ((bytes[18] as u16) << 8) | bytes[19] as u16;
        dos_header.e_ip = ((bytes[20] as u16) << 8) | bytes[21] as u16;
        dos_header.e_cs = ((bytes[22] as u16) << 8) | bytes[23] as u16;
        dos_header.e_lfarlc = ((bytes[24] as u16) << 8) | bytes[25] as u16;
        dos_header.e_ovno = ((bytes[26] as u16) << 8) | bytes[27] as u16;
        
        dos_header.e_res = [bytes[28], bytes[29], bytes[30], bytes[31],
        bytes[32], bytes[33], bytes[34], bytes[35]];

        dos_header.e_oemid = ((bytes[36] as u16) << 8) as u16 |
            bytes[37] as u16;
        dos_header.e_oeminfo = ((bytes[38] as u16) << 8) | bytes[39] as u16;
        
        dos_header.e_res2 = [
            bytes[40], bytes[41], bytes[42], bytes[43],
            bytes[44], bytes[45], bytes[46], bytes[47],
            bytes[48], bytes[49], bytes[50], bytes[51],
            bytes[52], bytes[53], bytes[54], bytes[55],
            bytes[56], bytes[57], bytes[58], bytes[59],
        ];

        dos_header.e_lfanew = ((bytes[60] as u32) << 24) as u32 | 
            ((bytes[61] as u32) << 16) as u32 | 
            ((bytes[62] as u32) << 8) as u32 |
            bytes[63] as u32;

        Ok(dos_header)
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Result<DosHeader, ButylError> {
        if bytes.len() < mem::size_of::<DosHeader>() { /* bounds check */
            return Err(ButylError::InsufficientDataError);
        }

        let mut dos_header: DosHeader = DosHeader::default();

        dos_header.e_magic = ((bytes[1] as u16) << 8) | bytes[0] as u16;
        dos_header.e_cblp = ((bytes[3] as u16) << 8) | bytes[2] as u16;
        dos_header.e_cp = ((bytes[5] as u16) << 8) | bytes[4] as u16;
        dos_header.e_crlc = ((bytes[7] as u16) << 8) | bytes[6] as u16;
        dos_header.e_cparhdr = ((bytes[9] as u16) << 8) | bytes[8] as u16;
        dos_header.e_minalloc = ((bytes[11] as u16) << 8) | bytes[10] as u16;
        dos_header.e_maxalloc = ((bytes[13] as u16) << 8) | bytes[12] as u16;
        dos_header.e_ss = ((bytes[15] as u16) << 8) | bytes[14] as u16;
        dos_header.e_sp = ((bytes[17] as u16) << 8) | bytes[16] as u16; 
        dos_header.e_csum = ((bytes[19] as u16) << 8) | bytes[18] as u16;
        dos_header.e_ip = ((bytes[21] as u16) << 8) | bytes[20] as u16;
        dos_header.e_cs = ((bytes[23] as u16) << 8) | bytes[22] as u16;
        dos_header.e_lfarlc = ((bytes[25] as u16) << 8) | bytes[24] as u16;
        dos_header.e_ovno = ((bytes[27] as u16) << 8) | bytes[26] as u16;
        
        dos_header.e_res = [
            bytes[35], bytes[34], bytes[33], bytes[32],
            bytes[31], bytes[30], bytes[29], bytes[28]
        ];

        dos_header.e_oemid = ((bytes[37] as u16) << 8) as u16 |
            bytes[36] as u16;
        dos_header.e_oeminfo = ((bytes[39] as u16) << 8) | bytes[38] as u16;
        
        dos_header.e_res2 = [
            bytes[59], bytes[58], bytes[57], bytes[56],
            bytes[55], bytes[54], bytes[53], bytes[52],
            bytes[51], bytes[50], bytes[49], bytes[48],
            bytes[47], bytes[46], bytes[45], bytes[44],
            bytes[43], bytes[42], bytes[41], bytes[40],
        ];

        dos_header.e_lfanew = ((bytes[63] as u32) << 24) as u32 | 
            ((bytes[62] as u32) << 16) as u32 | 
            ((bytes[61] as u32) << 8) as u32 |
            bytes[60] as u32;

        Ok(dos_header)
    }
}

