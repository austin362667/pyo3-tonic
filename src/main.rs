use std::{
    env,
    io::{self, Read, Write},
    process::exit,
};

use prost::Message;
use protoc_gen_prost::GeneratorResultExt;
use pyo3_tonic;

fn main() -> io::Result<()> {
    if env::args().any(|x| x == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = pyo3_tonic::execute(buf.as_slice()).unwrap_codegen_response();

    buf.clear();
    response.encode(&mut buf).expect("error encoding response");
    io::stdout().write_all(&buf)?;

    Ok(())
}
