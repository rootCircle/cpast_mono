use std::env;

use cpast::{CodeOrPath, compile_and_test};

#[tokio::main(flavor = "multi_thread", worker_threads = 64)]
async fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    compile_and_test(
        CodeOrPath::Path(format!("{manifest_dir}/examples/res/hello.py")),
        format!("{manifest_dir}/examples/res/hello.java"),
        "N".to_owned(),
        100,
        true,
        false,
        false,
    )
    .await
    .unwrap_or_else(|err| {
        eprintln!("{err}");
    });
}
