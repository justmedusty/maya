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

pub mod arg_handling {
    const ERROR : i32 = 1;
    const SUCCESS : i32 = 0;

    use std::fs::File;
    use std::process::exit;
    use crate::file_encoding_support;
    use crate::file_encoding_support::file_encoding_support::{FileEncoding, FileEncodingFunctionDerivation, FileEncodingMethod, FileEncodingSupport, ImageSupport, Operation};
    use crate::filetype_support::bmp::BmpImageParser;
    use crate::filetype_support::filetype_support::FileType::Bmp;

    pub fn parse_arguments(args: Vec<String>) -> ImageSupport {
        if args.len() == 2 && args[1] == "--help" {
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("This is a stegonagraphy tool for embedding and extracting secret messages within images.");
            println!("Options: --help, --version");
            exit(SUCCESS);
        }

        if (args.len() == 2 && args[1] == "--version") {
            println!("Maya version {}", env!("CARGO_PKG_VERSION"));
            exit(SUCCESS);
        }
        if (args.len() < 5 ) {
            println!("Too few arguments!");
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }

        if (args.len() > 6 ) {
            println!("Too many arguments!");
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }


        if { args[3] == "embed" &&  args.len() != 6 } {
            println!("You must specific a message with the embed option!");
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }

        let encoding = match args[1].as_str() {
            "Lsb" => {FileEncoding::Lsb},
            "PixelValueDifferencing" => {FileEncoding::PixelValueDifferencing},
            "Hamming" => {FileEncoding::HammingMatrix},
            _ => {
                println!("Invalid encoding found! : {}", args[1].as_str());
                exit(1);
            }

        };

        let encoding_method = match args[2].as_str() {
            "LeftRight" => {FileEncodingMethod::LeftToRight},
            "TopBottom" => {FileEncodingMethod::TopToBottom},
            "RightLeft" => {FileEncodingMethod::RightToLeft},
            "CosWave" => {FileEncodingMethod::CosWave},
            "SinWave" => {FileEncodingMethod::SinWave},
            "FractalFunc" => {FileEncodingMethod::FractalFunction},
            "PolynomialFunc" => {FileEncodingMethod::PolynomialFunction},
            _ => {
                println!("Invalid encoding method found! : {}", args[2].as_str());
                exit(1);
            }

        };

        let operation = match args[3].as_str() {
            "embed" => Operation::Embed,
            "extract" => Operation::Extract,
            _ => {
                println!("Invalid operation found! : {}", args[3]);
                exit(1);
            }
        };

        let mut message = Vec::new();
        if(operation == Operation::Embed) {
            message = args[4].as_bytes().to_vec();
        }

        /*
            position is different depending on if the operation is embed (embed has the message extra)
         */
        let file_no = match operation {
            Operation::Embed   => 5,
            Operation::Extract => 4
        };


        let file_ext = args[file_no].as_str().split_at(".".parse().unwrap()).1;

        if(file_ext == "bmp"){
            let image_parser : ImageSupport = ImageSupport{
                encoding_support: Box::new(BmpImageParser::new( args[file_no].as_str())),
                image_file : File::open(args[file_no].as_str()).unwrap(),
                encoding :encoding,
                encoding_method : encoding_method,
                file_encoding_function_derivation: FileEncodingFunctionDerivation::KeyBased, //This isnt implemented yet
                operation : operation,
                data : message
            };



             return image_parser;
        }





        println!("Invalid file ext found! : {}", args[file_no]);
        exit(ERROR);




    }
}