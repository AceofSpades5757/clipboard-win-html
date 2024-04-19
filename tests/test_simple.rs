#[cfg(test)]
mod tests {

    #[test]
    fn test_for_no_errors() {
        use clipboard_win_html::set_clipboard_html;

        // A user said they were getting errors when running this more than once. The user
        // acutally fixed the come themself, but this should cover that case moving forward.
        set_clipboard_html("<h1>Pure, valid, HTML.</h1>".to_string()).unwrap();
        set_clipboard_html("<h1>2nd Pure, valid, HTML.</h1>".to_string()).unwrap();
        set_clipboard_html("<h1>3nd Pure, valid, HTML.</h1>".to_string()).unwrap();
        set_clipboard_html("<h1>4th Pure, valid, HTML.</h1>".to_string()).unwrap();
    }
}
