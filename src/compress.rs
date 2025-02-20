use std::fs;
use std::io::Error;
use std::path::Path;
use crate::frequencies::{self, word_frequencies};


pub fn compress(in_file: &Path, out_file: &Path) -> Result<(), Error>
{
    let text = fs::read_to_string(in_file)?;
    let lines: Vec<_> = text.split('\n').map(|x| x.to_string()).collect();
    let lines_count = lines.len();

    let freqs = word_frequencies(&lines);
    println!("{:?}", freqs);
    // let tree = huffman::huffman_tree(&freqs);
    // Build the frequency table
    // Create a huffman tree from the frequency table
    // Serialize the huffman tree to the out_file
    // Open the in_file process it and write out the compressed version to the out_file
    // Write out an eof pattern
    todo!()
}
