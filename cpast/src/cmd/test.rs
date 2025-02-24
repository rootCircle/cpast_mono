use std::process::exit;

use crate::cli::cli_parser::TestArgs;
use cpast::{DEFAULT_FAIL_EXIT_CODE, compile_and_test};

pub(crate) async fn test_call(args: TestArgs) {
    let correct_binding = args.correct_file.unwrap_or_default();
    let test_binding = args.test_file.unwrap_or_default();
    let language = args.generator.unwrap_or_default();
    let iterations = args.iterations;
    let no_stop = args.no_stop;
    let do_force_compile = args.force_recompile;
    let debug = args.debug;

    compile_and_test(
        correct_binding,
        test_binding,
        language,
        iterations,
        no_stop,
        do_force_compile,
        debug,
    )
    .await
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(DEFAULT_FAIL_EXIT_CODE);
    });
}
