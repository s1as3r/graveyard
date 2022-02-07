use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::{app_from_crate, App, AppSettings, Arg, ArgMatches};

pub fn get_matches() -> ArgMatches {
    app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .subcommand(
            App::new("encode")
                .about("encode message into a png file")
                .arg(
                    Arg::new("file_path")
                        .required(true)
                        .value_name("FILE")
                        .help("path to the PNG file"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .required(true)
                        .value_name("CHUNK_TYPE")
                        .help("PNG Chunk Type"),
                )
                .arg(
                    Arg::new("message")
                        .required(true)
                        .value_name("MESSAGE")
                        .help("message to encode"),
                )
                .arg(
                    Arg::new("outupt_file")
                        .required(false)
                        .value_name("OUTPUT_FILE")
                        .help("output to <OUTPUT_FILE> instead of overwriting"),
                ),
        )
        .subcommand(
            App::new("decode")
                .about("decode message from a png file")
                .arg(
                    Arg::new("file_path")
                        .required(true)
                        .value_name("FILE")
                        .help("path to the PNG file"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .required(true)
                        .value_name("CHUNK_TYPE")
                        .help("type of PNG chunk to search for"),
                ),
        )
        .subcommand(
            App::new("remove")
                .about("remove chunk from a png file")
                .arg(
                    Arg::new("file_path")
                        .required(true)
                        .value_name("FILE")
                        .help("path to the PNG file"),
                )
                .arg(
                    Arg::new("chunk_type")
                        .required(true)
                        .value_name("CHUNK_TYPE")
                        .help("PNG chunk type to remove"),
                ),
        )
        .subcommand(
            App::new("print").about("print the png file").arg(
                Arg::new("file_path")
                    .required(true)
                    .value_name("FILE")
                    .help("path to the PNG file"),
            ),
        )
        .get_matches()
}

pub fn handle_encode(args: &ArgMatches) {
    let file_path = args.value_of("file_path").unwrap();
    let chunk_type = ChunkType::from_str(args.value_of("chunk_type").unwrap()).unwrap();
    let message = args.value_of("message").unwrap();
    let output_path = args.value_of("outupt_file").unwrap_or(file_path);
    let mut png = Png::from_file(file_path).unwrap();
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);
    png.to_file(output_path).unwrap();
}

pub fn handle_decode(args: &ArgMatches) {
    let file_path = args.value_of("file_path").unwrap();
    let chunk_type = args.value_of("chunk_type").unwrap();
    let mut png = Png::from_file(file_path).unwrap();
    while let Some(chunk) = png.chunk_by_type(chunk_type) {
        println!("{}", chunk);
        png.remove_chunk(chunk_type).unwrap();
    }
}

pub fn handle_remove(args: &ArgMatches) {
    let file_path = args.value_of("file_path").unwrap();
    let chunk_type = args.value_of("chunk_type").unwrap();
    let mut png = Png::from_file(file_path).unwrap();
    let removed = png.remove_chunk(chunk_type).unwrap();
    println!("Removed: {}", removed);
    png.to_file(file_path).unwrap();
}

pub fn handle_print(args: &ArgMatches) {  
    let file_path = args.value_of("file_path").unwrap();
    let png = Png::from_file(file_path).unwrap();
    println!("{}", png);
}
