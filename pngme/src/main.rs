mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let matches = args::get_matches();
    match matches.subcommand() {
        Some(("encode", encode_args)) => args::handle_encode(encode_args),
        Some(("decode", decode_args)) => args::handle_decode(decode_args),
        Some(("remove", remove_args)) => args::handle_remove(remove_args),
        Some(("print", print_args)) => args::handle_print(print_args),
        _ => (),
    }
    Ok(())
}
