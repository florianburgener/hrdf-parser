use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = hrdf_parser::run().await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
