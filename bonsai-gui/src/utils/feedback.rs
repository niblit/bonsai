use wasm_bindgen::JsValue;
use web_sys::HtmlAudioElement;

use bonsai_chess::prelude::{Ply, SpecialMove};

// preload assets for better latency
thread_local! {
    // No RefCell needed! HtmlAudioElement is already a reference-counted handle.
    static MOVE_SOUND: HtmlAudioElement = create_audio("/static/sounds/Move.ogg");
    static CAPTURE_SOUND: HtmlAudioElement = create_audio("/static/sounds/Capture.ogg");
}

/// Plays sound and haptic feedback based on the move type.
pub fn provide_feedback(ply: &Ply) {
    let is_capture = ply.piece_captured().is_some()
        || matches!(ply.special_move(), Some(SpecialMove::EnPassant(_)));

    play_sound(is_capture);
    vibrate(is_capture);
}

fn play_sound(is_capture: bool) {
    let _ = if is_capture {
        CAPTURE_SOUND.with(play_preloaded_sound)
    } else {
        MOVE_SOUND.with(play_preloaded_sound)
    };
}

fn vibrate(is_capture: bool) {
    if let Some(window) = web_sys::window() {
        let nav = window.navigator();

        let pattern = if is_capture {
            JsValue::from(30)
        } else {
            JsValue::from(10)
        };

        let _ = nav.vibrate_with_pattern(&pattern);
    }
}

// Helper to configure the audio element correctly
fn create_audio(src: &str) -> HtmlAudioElement {
    let audio = HtmlAudioElement::new_with_src(src).unwrap();
    audio.set_preload("auto");
    audio
}

fn play_preloaded_sound(audio: &HtmlAudioElement) -> Result<(), JsValue> {
    audio.set_current_time(0.0);
    audio.play().map(|_| ())
}
