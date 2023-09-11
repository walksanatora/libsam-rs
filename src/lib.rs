use std::ffi::NulError;

use libc::c_void;

pub mod sys;


/// A enum containg all errors that TTS can return
pub enum TTSError {
    /// the string *you* passed contains a null, dont do that
    ContainsNull,
    /// error id from the libSAM, will mabey split this into values l8r
    Code(i32)
}

/// quick impl so i dont have to catch it and it can just be questioned
impl From<NulError> for TTSError {
    fn from(_value: NulError) -> Self {
        TTSError::ContainsNull
    }
}

/// set SAM tts values (0/None sets value to default)
pub fn set_speech_values(
    pitch: Option<u8>,
    speed: Option<u8>,
    throat: Option<u8>,
    mouth: Option<u8>,
) {
    unsafe {
        sys::setupSpeak(
            pitch.unwrap_or(0),
            speed.unwrap_or(0),
            throat.unwrap_or(0),
            mouth.unwrap_or(0),
        )
    }
}

/// internal function to render a string into PCM audio
/// SAFTEY: chunk must be at most 255 bytes long
unsafe fn render_chunk(chunk: &str) -> Result<Vec<u8>,TTSError> {
    let mut bytes: Vec<i8> = chunk.bytes().map(|b|{std::mem::transmute(b)}).collect();
    bytes.push(0);
    let ptr = sys::speakText(bytes.as_mut_ptr());
    let res = ptr.read();
    if res.res != 1 {
        libc::free(ptr as *mut c_void);
        return Err(TTSError::Code(res.res))
    }
    let buf = std::slice::from_raw_parts(res.buf, res.buf_size as usize);
    buf.into_iter().map(|b|std::mem::transmute(b)).collect()
}

/// Speaks the chosen text as a message
pub fn speak_words(tospeak: &str) -> Result<Vec<u8>,TTSError> {
    let bytes: Vec<u8> = if tospeak.len()<=255 {
        unsafe {render_chunk(tospeak)?}
    } else {
        let words = tospeak.split(' ');
        let mut small = vec![];
        let mut result: Vec<u8> = vec![];
        for word in words {
            if small.iter().map(|x:&&str| {x.len() }).fold(0,|acc, x| acc + x)+word.len() <= 255 {
                small.push(word);
            } else {
                result.append(&mut unsafe {render_chunk(small.join(" ").as_str())?})
            }
        };
        result.append(&mut unsafe {render_chunk(small.join(" ").as_str())?});
        result
    };
    Ok(bytes)
}