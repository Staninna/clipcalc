use clipboard_win::{get_clipboard, set_clipboard, Unicode};
use evalexpr::eval;

fn main() {
    let mut last: String = get_clipboard(Unicode).unwrap();
    eval_clipboard(&last);

    loop {
        let current: String = match get_clipboard(Unicode) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if current != last {
            last = current.clone();
            eval_clipboard(&current);
        }
    }
}

fn eval_clipboard(text: &str) {
    let result = eval(text);
    if result.is_ok() && set_clipboard(Unicode, format!("{}", result.unwrap())).is_err() {
        println!("Failed to set clipboard");
    }
}
