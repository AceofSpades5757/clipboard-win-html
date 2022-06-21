#[cfg(test)]
mod tests {

    #[test]
    fn test_for_no_errors() {
        use clipboard_win_html::set_clipboard_html;

        set_clipboard_html("<h1>Pure, valid, HTML.</h1>");
    }
}
