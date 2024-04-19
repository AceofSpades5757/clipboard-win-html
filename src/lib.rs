//! A library to set HTML to the clipboard.
//!
//! ## Example
//! ```rust
//! use clipboard_win_html::set_clipboard_html;
//!
//! set_clipboard_html("<h1>Hello, World!</h1>".to_string());
//! ```
//! ## Platform
//! - Windows
//!
//! ## Windows References
//! - [HTML Clipboard Format](https://docs.microsoft.com/en-us/windows/win32/dataxchg/html-clipboard-format)
//! - [Clipboard Functions](https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard-functions)
//! - [RegisterClipboardFormatW function](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclipboardformatw)
//! - [GlobalAlloc function](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc)
//! - [GlobalLock function](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
//! - [GlobalUnlock function](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
//! - [SetClipboardData function](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
//! - [EmptyClipboard function](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
//! - [CloseClipboard function](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
//! - [OpenClipboard function](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard)
//! - [Clipboard Formats](https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats)
//! - [Clipboard Data Formats](https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard-data-formats)
//! - [Clipboard](https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard)
use std::fmt;
use std::ffi::CString;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
};

/// Any errors that might occur during the process of setting HTML to the clipboard.
#[derive(Debug)]
pub enum Error {
    HtmlTemplateCreationError,
    OpenClipboardError,
    EmptyClipboardError,
    SetClipboardError,
    CloseClipboardError,
    MemoryAllocationError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HtmlTemplateCreationError => write!(f, "Failed to create HTML template."),
            Error::OpenClipboardError => write!(f, "Failed to open clipboard."),
            Error::EmptyClipboardError => write!(f, "Failed to empty clipboard."),
            Error::SetClipboardError => write!(f, "Failed to set clipboard."),
            Error::CloseClipboardError => write!(f, "Failed to close clipboard."),
            Error::MemoryAllocationError => write!(f, "Failed to allocate memory."),
        }
    }
}

/// Set HTML to the clipboard on Windows.
///
/// https://docs.microsoft.com/en-us/windows/win32/dataxchg/html-clipboard-format
///
/// Uses 50 characters for the offsets.
pub fn set_clipboard_html(html: String) {
    // Create the HTML document to set to clipboard
    let fragment = html;

    let start_html = 391;
    let start_fragment = 454;
    let start_selection = Some(start_fragment);

    let end_fragment = start_fragment + fragment.len() - 1;
    let end_selection = Some(end_fragment);
    // 37 is the `<!-- EndFragment -->\n...`
    let end_html = end_fragment + 37;

    let mut document = String::new();

    // Description

    // Version
    document.push_str(format!("Version:{}\n", "0.9").as_str());

    // StartHTML
    document.push_str(format!("StartHTML:{:0>50}\n", start_html).as_str());
    // EndHTML
    document.push_str(format!("EndHTML:{:0>50}\n", end_html).as_str());

    // StartFragment
    document.push_str(format!("StartFragment:{:0>50}\n", start_fragment).as_str());
    // EndFragment
    document.push_str(format!("EndFragment:{:0>50}\n", end_fragment).as_str());

    if let (Some(start_selection), Some(end_selection)) = (start_selection, end_selection) {
        // StartSelection
        document.push_str(format!("StartSelection:{:0>50}\n", start_selection).as_str());
        // EndSelection
        document.push_str(format!("EndSelection:{:0>50}\n", end_selection).as_str());
    }

    // Context

    document.push_str(
        r#"<!DOCTYPE>
<HTML>
<HEAD>
</HEAD>
<BODY>
<!-- StartFragment -->
"#,
    );
    document.push_str(&fragment);
    document.push_str(
        r#"
<!-- EndFragment -->
</BODY>
</HTML>"#,
    );

    let cstring = CString::new(document).expect("Failed to convert to CString.");
    let cstring = cstring.as_bytes_with_nul();

    // Register Format
    #[allow(non_snake_case)]
    let CF_HTML;
    unsafe {
        // [Where they tell us the official name](https://docs.microsoft.com/en-us/windows/win32/dataxchg/html-clipboard-format)
        //
        // The official name of the clipboard (the string used by RegisterClipboardFormat) is HTML Format.
        let format_name: Vec<u16> = "HTML Format\0".encode_utf16().collect();
        let pcwstr = windows::core::PCWSTR(format_name.as_ptr());
        // RegisterClipboardFormatW: <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclipboardformatw>
        let uint = windows::Win32::System::DataExchange::RegisterClipboardFormatW(pcwstr);
        CF_HTML = uint;
    }


    // 1. Open Clipboard
    // 2. Empty Clipboard
    // 3. Set Clipboard
    // 4. Close Clipboard

    // 1. Open Clipboard
    unsafe {
        windows::Win32::System::DataExchange::OpenClipboard(None)
            .expect("Failed to open clipboard.");
    }

    // 2. Empty Clipboard
    unsafe {
        windows::Win32::System::DataExchange::EmptyClipboard().expect("Failed to empty clipboard.");
    }

    // 3. Set Clipboard
    unsafe {
        let mem_alloc: HGLOBAL =
            GlobalAlloc(GMEM_MOVEABLE, cstring.len() * std::mem::size_of::<u16>())
                .expect("Failed to allocate memory.");
        let mem_lock = GlobalLock(mem_alloc);
        std::ptr::copy_nonoverlapping(cstring.as_ptr(), mem_lock as *mut u8, cstring.len());
        let _ = GlobalUnlock(mem_alloc);
        let handle = HANDLE(mem_alloc.0 as isize);

        if windows::Win32::System::DataExchange::SetClipboardData(CF_HTML, handle).is_err() {
            panic!("Failed to set clipboard.");
        }
    }

    // 4. Close Clipboard
    unsafe {
        windows::Win32::System::DataExchange::CloseClipboard().expect("Failed to close clipboard.");
    }
}
