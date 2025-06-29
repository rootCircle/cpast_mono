#[test]
#[cfg(any(
    all(unix, not(any(target_os = "android", target_os = "emscripten"))),
    windows,
))]
fn send_to_clipboard_works() {
    use cli_clipboard::{ClipboardContext, ClipboardProvider};
    let ctx = ClipboardContext::new();
    if let Err(e) = ctx {
        eprintln!("Error: {e:?}");
        return; // Skip the test as in no display env as in CI runners
    }

    let mut ctx = ctx.unwrap();
    let the_string = "Hello, world!";
    ctx.set_contents(the_string.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), the_string);
}
