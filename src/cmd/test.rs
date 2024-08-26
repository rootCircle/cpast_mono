use crate::cli::cli_parser::TestCliArgs;
use cpast::compile_and_test;

pub(crate) async fn test_call(args: TestCliArgs) {
    let correct_binding = args.correct_file.unwrap_or_default();
    let test_binding = args.test_file.unwrap_or_default();
    let language = args.generator.unwrap_or_default();
    let iterations = args.iterations;
    let no_stop = args.no_stop;
    let do_force_compile = args.force_recompile;

    compile_and_test(
        correct_binding,
        test_binding,
        language,
        iterations,
        no_stop,
        do_force_compile,
    )
    .await
    .unwrap_or_else(|err| err.print_and_exit());
}
