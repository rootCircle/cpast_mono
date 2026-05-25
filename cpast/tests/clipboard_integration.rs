use flaky_test::flaky_test;

#[test]
#[flaky_test]
#[cfg(any(
    all(unix, not(any(target_os = "android", target_os = "emscripten"))),
    windows,
))]
fn send_to_clipboard_works() {
    use arboard::Clipboard;

    // arboard segfaults (SIGSEGV) in headless environments instead of returning Err.
    // Skip the test early when no display server is available (e.g. GitHub Actions).
    #[cfg(unix)]
    {
        let has_display =
            std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();
        if !has_display {
            eprintln!(
                "Skipping clipboard test: no display server (DISPLAY/WAYLAND_DISPLAY not set)"
            );
            return;
        }
    }

    let ctx: Result<_, _> = Clipboard::new();
    if let Err(e) = ctx {
        eprintln!("Error: {e:?}");
        return; // Skip the test as in no display env as in CI runners
    }

    let mut ctx = ctx.unwrap();
    let the_string = "Hello, world!";
    ctx.set_text(the_string.to_owned()).unwrap();
    assert_eq!(ctx.get_text().unwrap(), the_string);
}
