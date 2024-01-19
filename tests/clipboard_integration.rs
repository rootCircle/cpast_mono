use cli_clipboard::{ClipboardContext, ClipboardProvider};
#[test]
fn send_to_clipboard_works() {
    let ctx = ClipboardContext::new();
    if ctx.is_ok() {
        let mut ctx = ctx.unwrap();
        let the_string = "Hello, world!";
        ctx.set_contents(the_string.to_owned()).unwrap();
        assert_eq!(ctx.get_contents().unwrap(), the_string);
    }
    // Ignoring if Clipboard can't be initialised cases for brevity! Useful for GitHub Actions Workflows
}
