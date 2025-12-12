/*
 * Copyright (C) 2025 Dustyn Gibb
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use crate::file_encoding_support::file_encoding_support::{
    FileEncoding, FileEncodingFunctionDerivation, FileEncodingMethod, FileEncodingSupport,
};
use crate::file_encoding_support::pixel::{embed_lsb_data_left_right, extract_lsb_data_left_right, Pixel};
use std::fs::File;
use std::io::{Read, Write};
use std::mem;
use std::process::exit;

const PNG_MAGIC : [u8;8] = [0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A];


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkType(pub [u8; 4]);

// -- Critical chunks --

/// Image header
pub const IHDR: ChunkType = ChunkType(*b"IHDR");
/// Palette
pub const PLTE: ChunkType = ChunkType(*b"PLTE");
/// Image data
pub const IDAT: ChunkType = ChunkType(*b"IDAT");
/// Image trailer
pub const IEND: ChunkType = ChunkType(*b"IEND");

// -- Ancillary chunks --

/// Transparency
pub const tRNS: ChunkType = ChunkType(*b"tRNS");
/// Background colour
pub const bKGD: ChunkType = ChunkType(*b"bKGD");
/// Image last-modification time
pub const tIME: ChunkType = ChunkType(*b"tIME");
/// Physical pixel dimensions
pub const pHYs: ChunkType = ChunkType(*b"pHYs");
/// Source system's pixel chromaticities
pub const cHRM: ChunkType = ChunkType(*b"cHRM");
/// Source system's gamma value
pub const gAMA: ChunkType = ChunkType(*b"gAMA");
/// sRGB color space chunk
pub const sRGB: ChunkType = ChunkType(*b"sRGB");
/// ICC profile chunk
pub const iCCP: ChunkType = ChunkType(*b"iCCP");
/// Coding-independent code points for video signal type identification chunk
pub const cICP: ChunkType = ChunkType(*b"cICP");
/// Mastering Display Color Volume chunk
pub const mDCV: ChunkType = ChunkType(*b"mDCV");
/// Content Light Level Information chunk
pub const cLLI: ChunkType = ChunkType(*b"cLLI");
/// EXIF metadata chunk
pub const eXIf: ChunkType = ChunkType(*b"eXIf");
/// Latin-1 uncompressed textual data
pub const tEXt: ChunkType = ChunkType(*b"tEXt");
/// Latin-1 compressed textual data
pub const zTXt: ChunkType = ChunkType(*b"zTXt");
/// UTF-8 textual data
pub const iTXt: ChunkType = ChunkType(*b"iTXt");
// Significant bits
pub const sBIT: ChunkType = ChunkType(*b"sBIT");

// -- Extension chunks --

/// Animation control
pub const acTL: ChunkType = ChunkType(*b"acTL");
/// Frame control
pub const fcTL: ChunkType = ChunkType(*b"fcTL");
/// Frame data
pub const fdAT: ChunkType = ChunkType(*b"fdAT");

// -- Chunk type determination --

/// Returns true if the chunk is critical.
pub fn is_critical(ChunkType(type_): ChunkType) -> bool {
    type_[0] & 32 == 0
}

/// Returns true if the chunk is private.
pub fn is_private(ChunkType(type_): ChunkType) -> bool {
    type_[1] & 32 != 0
}

/// Checks whether the reserved bit of the chunk name is set.
/// If it is set the chunk name is invalid.
pub fn reserved_set(ChunkType(type_): ChunkType) -> bool {
    type_[2] & 32 != 0
}

/// Returns true if the chunk is safe to copy if unknown.
pub fn safe_to_copy(ChunkType(type_): ChunkType) -> bool {
    type_[3] & 32 != 0
}