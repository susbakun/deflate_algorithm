mod serializer;
mod commandline;

use std::{fs, path::PathBuf};

use serializer::{serialize, deserialize};
use commandline::CommandLine;


fn main(){
    let commandline = CommandLine::new();

    let input_path = commandline.str_arg(
        "i",
        "resources/L.txt"
        );

    let output_path = commandline.str_arg(
        "o",
        "output.dfl"
    );

    let should_compress = commandline.is_present("c");
    let should_decompress = commandline.is_present("d");

    if should_compress {
        let input = fs::read_to_string(input_path)
            .expect("Failed to read the input file");

        let lz_output = lz77::run(&input);
        let serialized = serialize(&lz_output);
        huffman::run(&serialized, &output_path);
    } else if should_decompress {
        let input = huffman::read_huffman_file(
            PathBuf::from(&input_path));
        todo!()
    }

}