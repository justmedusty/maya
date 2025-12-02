/*
    Copyright (C) 2025 Dustyn Gibb

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::fs::File;

pub struct ImageSupport {
    pub(crate) image_file: File,
    pub(crate) encoding: FileEncoding,
    pub(crate) encoding_method: FileEncodingMethod,
    pub(crate) file_encoding_function_derivation: FileEncodingFunctionDerivation,
    pub(crate) operation: Operation,
    pub(crate) data : Vec<u8>,
    pub(crate) encoding_support: Box<dyn FileEncodingSupport>,
}

#[derive(Debug, Clone, Copy)]
pub enum WaveFunction {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft,
    ZigZagHorizontal,
    ZigZagVertical,
    Sinusoidal,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Embed,
    Extract
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncoding {
    Lsb,
    PixelValueDifferencing,
    HammingMatrix,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncodingMethod {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    SinWave,
    CosWave,
    PolynomialFunction,
    FractalFunction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncodingFunctionDerivation {
    KeyBased,
}

pub trait FileEncodingSupport {
    fn new(filename : &str) -> Self where Self: Sized;

    fn parse_file(&mut self);

    fn embed_data(&mut self, data: &mut Vec<u8>, encoding: FileEncoding, encoding_method: FileEncodingMethod, file_encoding_function_derivation: FileEncodingFunctionDerivation);

    fn retrieve_data(&mut self,data: Vec<u8>,encoding: FileEncoding, encoding_method: FileEncodingMethod, file_encoding_function_derivation: FileEncodingFunctionDerivation);

    fn write_file(&mut self, new_file_location: &str);
    
}


/*
    The idea at the time of writing this is the other parameters like the FileEncoding as defined above, will be stored in the specific object and can be referenced internally
 */
pub trait FileEncodingAlgorithms{
    fn left_to_right(&self);
    fn right_to_left(&self);
    fn top_to_bottom(&self);
    fn wave(&self, wave_type: WaveFunction, amplitude: f32, phase: f32, frequency: f32);
}



impl WaveFunction {
    pub(crate) fn traverse(&self, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        match self {
            // Horizontal wave: Traverse each row from left to right
            WaveFunction::Horizontal => {
                for row in 0..rows {
                    for col in 0..cols {
                        positions.push((row, col));
                    }
                }
            }
            // Vertical wave: Traverse each column from top to bottom
            WaveFunction::Vertical => {
                for col in 0..cols {
                    for row in 0..rows {
                        positions.push((row, col));
                    }
                }
            }
            // DiagonalRight wave: Traverse diagonally from top-left to bottom-right
            WaveFunction::DiagonalRight => {
                let mut row = 0;
                let mut col = 0;
                while row < rows && col < cols {
                    positions.push((row, col));
                    row += 1;
                    col += 1;
                }
            }
            // DiagonalLeft wave: Traverse diagonally from top-right to bottom-left
            WaveFunction::DiagonalLeft => {
                let mut row = 0;
                let mut col = cols - 1;
                while row < rows && col >= 0 {
                    positions.push((row, col));
                    row += 1;
                    col = col.saturating_sub(1);
                }
            }
            // ZigZagHorizontal: Traverse rows in a zigzag pattern (left-right, then right-left)
            WaveFunction::ZigZagHorizontal => {
                for row in 0..rows {
                    if row % 2 == 0 {
                        for col in 0..cols {
                            positions.push((row, col));
                        }
                    } else {
                        for col in (0..cols).rev() {
                            positions.push((row, col));
                        }
                    }
                }
            }
            // ZigZagVertical: Traverse columns in a zigzag pattern (top-bottom, then bottom-top)
            WaveFunction::ZigZagVertical => {
                for col in 0..cols {
                    if col % 2 == 0 {
                        for row in 0..rows {
                            positions.push((row, col));
                        }
                    } else {
                        for row in (0..rows).rev() {
                            positions.push((row, col));
                        }
                    }
                }
            }
            // Sinusoidal wave: Simulate a sine wave pattern over rows and columns
            WaveFunction::Sinusoidal => {
                // A basic sine-wave-like pattern, with amplitude and frequency scaling
                for row in 0..rows {
                    let sine_wave_offset = ((row as f32 / rows as f32) * 2.0 * std::f32::consts::PI).sin();
                    // Translate sine value to column index (oscillation over columns)
                    let col_offset = ((sine_wave_offset + 1.0) * (cols as f32 / 2.0)) as usize;
                    positions.push((row, col_offset));
                }
            }
        }

        positions
    }
}