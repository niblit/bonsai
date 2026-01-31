use bonsai_chess::prelude::{Ply, SpecialMove};
use wasm_bindgen::JsValue;

/// Plays sound and haptic feedback based on the move type.
pub fn provide_feedback(ply: &Ply) {
    let is_capture = ply.piece_captured().is_some()
        || matches!(ply.special_move(), Some(SpecialMove::EnPassant(_)));

    // 1. Audio Feedback
    let audio_src = if is_capture {
        "/static/sounds/Capture.ogg"
    } else {
        "/static/sounds/Move.ogg"
    };

    if let Ok(audio) = web_sys::HtmlAudioElement::new_with_src(audio_src) {
        let _ = audio.play();
    }

    // 2. Haptic Feedback
    if let Some(window) = web_sys::window() {
        let nav = window.navigator();

        // Vibrate pattern: 50ms for move, [50, 30, 50] for capture
        let pattern = if is_capture {
            JsValue::from(100)
        } else {
            JsValue::from(50)
        };

        // We ignore the result of vibrate as it is not critical
        let _ = nav.vibrate_with_pattern(&pattern);
    }
}
