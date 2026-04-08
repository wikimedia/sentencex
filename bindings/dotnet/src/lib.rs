use sentencex::{get_sentence_boundaries, segment};

#[repr(C)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct SegmentResult {
    pub ptr: *mut ByteRange,
    pub len: usize,
}

#[repr(C)]
pub struct BoundaryEntry {
    pub start_byte: usize,
    pub end_byte: usize,
    pub boundary_symbol: [u8; 8],
    pub boundary_symbol_len: u8,
    pub is_paragraph_break: u8,
}

#[repr(C)]
pub struct BoundaryResult {
    pub ptr: *mut BoundaryEntry,
    pub len: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sentencex_segment(
    language_ptr: *const u8,
    language_len: usize,
    text_ptr: *const u8,
    text_len: usize,
) -> SegmentResult {
    if language_ptr.is_null() || text_ptr.is_null() {
        return SegmentResult {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
    }

    let language = unsafe {
        match std::str::from_utf8(std::slice::from_raw_parts(language_ptr, language_len)) {
            Ok(s) => s,
            Err(_) => {
                return SegmentResult {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                }
            }
        }
    };

    let text = unsafe {
        match std::str::from_utf8(std::slice::from_raw_parts(text_ptr, text_len)) {
            Ok(s) => s,
            Err(_) => {
                return SegmentResult {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                }
            }
        }
    };

    let segments = segment(language, text);

    let mut ranges: Vec<ByteRange> = segments
        .iter()
        .map(|s| {
            let start = s.as_ptr() as usize - text.as_ptr() as usize;
            let end = start + s.len();
            ByteRange { start, end }
        })
        .collect();

    ranges.shrink_to_fit();
    let len = ranges.len();
    let ptr = ranges.as_mut_ptr();
    std::mem::forget(ranges);

    SegmentResult { ptr, len }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sentencex_free_segment_result(result: SegmentResult) {
    if !result.ptr.is_null() {
        unsafe {
            drop(Vec::from_raw_parts(result.ptr, result.len, result.len));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sentencex_get_boundaries(
    language_ptr: *const u8,
    language_len: usize,
    text_ptr: *const u8,
    text_len: usize,
) -> BoundaryResult {
    if language_ptr.is_null() || text_ptr.is_null() {
        return BoundaryResult {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
    }

    let language = unsafe {
        match std::str::from_utf8(std::slice::from_raw_parts(language_ptr, language_len)) {
            Ok(s) => s,
            Err(_) => {
                return BoundaryResult {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                }
            }
        }
    };

    let text = unsafe {
        match std::str::from_utf8(std::slice::from_raw_parts(text_ptr, text_len)) {
            Ok(s) => s,
            Err(_) => {
                return BoundaryResult {
                    ptr: std::ptr::null_mut(),
                    len: 0,
                }
            }
        }
    };

    let boundaries = get_sentence_boundaries(language, text);

    let mut entries: Vec<BoundaryEntry> = boundaries
        .iter()
        .map(|b| {
            let mut boundary_symbol = [0u8; 8];
            let boundary_symbol_len = if let Some(ref sym) = b.boundary_symbol {
                let bytes = sym.as_bytes();
                let len = bytes.len().min(8);
                boundary_symbol[..len].copy_from_slice(&bytes[..len]);
                len as u8
            } else {
                0
            };

            BoundaryEntry {
                start_byte: b.start_byte,
                end_byte: b.end_byte,
                boundary_symbol,
                boundary_symbol_len,
                is_paragraph_break: b.is_paragraph_break as u8,
            }
        })
        .collect();

    entries.shrink_to_fit();
    let len = entries.len();
    let ptr = entries.as_mut_ptr();
    std::mem::forget(entries);

    BoundaryResult { ptr, len }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sentencex_free_boundary_result(result: BoundaryResult) {
    if !result.ptr.is_null() {
        unsafe {
            drop(Vec::from_raw_parts(result.ptr, result.len, result.len));
        }
    }
}
