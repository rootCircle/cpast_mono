use std::{env, path::PathBuf};

use cpast::compile_and_test;

#[tokio::main(flavor = "multi_thread", worker_threads = 64)]
async fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // Just to ensure that compiled binaries are in examples dir
    env::set_current_dir(PathBuf::from(format!("{manifest_dir}/examples/artifacts"))).unwrap();

    compile_and_test(
        format!("{manifest_dir}/examples/res/correct_approach.cpp"),
        format!("{manifest_dir}/examples/res/my_approach.cpp"),
        "(N[1,5]) (?:(N[1,5]) (?:N[1,100]){\\2}){\\1}".to_owned(),
        100,
        true,
        false,
        false,
    )
    .await
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
