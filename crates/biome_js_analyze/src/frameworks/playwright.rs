use biome_js_syntax::AnyJsExpression;
use biome_rowan::TokenText;

/// Extracts the object name from an expression.
///
/// For identifier expressions (e.g., `page`), returns the identifier name.
/// For member expressions (e.g., `context.page`), returns the member name.
pub(crate) fn get_object_name(expr: &AnyJsExpression) -> Option<TokenText> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            Some(id.name().ok()?.value_token().ok()?.token_text_trimmed())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => Some(
            member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        _ => None,
    }
}

/// Checks if a name represents a Playwright page or frame.
///
/// Returns true if the name:
/// - Is exactly "page" or "frame"
/// - Ends with "Page" or "Frame" (e.g., "myPage", "childFrame")
pub(crate) fn is_page_or_frame_name(name: &str) -> bool {
    name == "page" || name == "frame" || name.ends_with("Page") || name.ends_with("Frame")
}

/// Gets the page/frame name from an expression if it represents a page or frame.
///
/// Combines `get_object_name` and `is_page_or_frame_name` for convenience.
pub(crate) fn get_page_or_frame_name(expr: &AnyJsExpression) -> Option<TokenText> {
    let name = get_object_name(expr)?;
    if is_page_or_frame_name(name.text()) {
        Some(name)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_page_or_frame_name_valid() {
        assert!(is_page_or_frame_name("page"));
        assert!(is_page_or_frame_name("frame"));
        assert!(is_page_or_frame_name("myPage"));
        assert!(is_page_or_frame_name("childFrame"));
        assert!(is_page_or_frame_name("newPage"));
        assert!(is_page_or_frame_name("mainFrame"));
    }

    #[test]
    fn test_is_page_or_frame_name_invalid() {
        assert!(!is_page_or_frame_name("locator"));
        assert!(!is_page_or_frame_name("browser"));
        assert!(!is_page_or_frame_name("context"));
        assert!(!is_page_or_frame_name("element"));
        assert!(!is_page_or_frame_name("pageBuilder")); // "Page" must be at the end
        assert!(!is_page_or_frame_name("frameWork")); // "Frame" must be at the end
    }
}
