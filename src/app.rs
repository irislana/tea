use harfbuzz_sys::*;
use std::ffi::CString;
use std::path::Path;
use ui::text;
use ui::window;

pub fn run() {
    text::with_new_freetype(|lib| {
        let mut font = text::Font::new(lib, Path::new("src/ui/fonts/Monoid-Regular.ttf"));
        font.set_pixel_size(20); // Changed font size to 20 pixels

        let en = CString::new("en").unwrap();

        let text = text::Text {
            string: "This is a modified test string =>".to_string(), // Modified test string
            direction: HB_DIRECTION_LTR,
            script: HB_SCRIPT_LATIN,
            language: unsafe { hb_language_from_string(en.as_ptr(), en.as_bytes().len() as i32) },
        };

        text::render_text_to_glyphs(text, font);

        // Additional changes
        let custom_variable = 42; // Example of adding a custom variable

        // Placeholder for future functionality
        if custom_variable == 42 {
            println!("Custom variable is 42");
        }
    })
    .unwrap();
}
