#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_long, c_void, CStr};

use crate::types::Boolean;

enum FILE {}

const DIR_SEPARATOR: c_char = b'/' as c_char;
const DIR_SEPARATOR_S: &[u8] = b"/\0";

use crate::doom::z_zone::PU_STATIC;

const SEEK_END: c_int = 2;
const SEEK_SET: c_int = 0;

const EISDIR: c_int = 21;

extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn mkdir(path: *const c_char, mode: u32) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn vsnprintf(s: *mut c_char, n: usize, format: *const c_char, arg: ...) -> c_int;
}

extern "C" {
    fn I_Error(fmt: *const c_char, ...);
}

extern "C" {
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

#[no_mangle]
pub extern "C" fn M_MakeDirectory(path: *mut c_char) {
    unsafe {
        mkdir(path, 0o755);
    }
}

#[no_mangle]
pub extern "C" fn M_FileExists(filename: *mut c_char) -> c_int {
    unsafe {
        let fstream = fopen(filename as *const c_char, b"r\0".as_ptr() as *const c_char);
        if !fstream.is_null() {
            fclose(fstream);
            return 1;
        }
        (errno() == EISDIR) as c_int
    }
}

#[no_mangle]
pub extern "C" fn M_FileLength(handle: *mut FILE) -> c_long {
    unsafe {
        let savedpos = ftell(handle);
        fseek(handle, 0, SEEK_END);
        let length = ftell(handle);
        fseek(handle, savedpos, SEEK_SET);
        length
    }
}

#[no_mangle]
pub extern "C" fn M_WriteFile(name: *mut c_char, source: *mut c_void, length: c_int) -> c_int {
    unsafe {
        let handle = fopen(name as *const c_char, b"wb\0".as_ptr() as *const c_char);
        if handle.is_null() {
            return 0;
        }
        let count = fwrite(source, 1, length as usize, handle);
        fclose(handle);
        if count < length as usize {
            return 0;
        }
        1
    }
}

#[no_mangle]
pub extern "C" fn M_ReadFile(name: *mut c_char, buffer: *mut *mut c_char) -> c_int {
    unsafe {
        let handle = fopen(name as *const c_char, b"rb\0".as_ptr() as *const c_char);
        if handle.is_null() {
            I_Error(b"Couldn't read file %s\0".as_ptr() as *const c_char, name);
        }
        let length = M_FileLength(handle);
        let buf = Z_Malloc(length as c_int, PU_STATIC, std::ptr::null_mut());
        let count = fread(buf, 1, length as usize, handle);
        fclose(handle);
        if count < length as usize {
            I_Error(b"Couldn't read file %s\0".as_ptr() as *const c_char, name);
        }
        *buffer = buf as *mut c_char;
        length as c_int
    }
}

#[no_mangle]
pub extern "C" fn M_TempFile(s: *mut c_char) -> *mut c_char {
    unsafe {
        let tempdir = b"/tmp\0".as_ptr() as *const c_char;
        let sep = DIR_SEPARATOR_S.as_ptr() as *const c_char;
        let strs: [*const c_char; 4] = [tempdir, sep, s as *const c_char, std::ptr::null()];
        M_StringJoinA(strs.as_ptr())
    }
}

#[no_mangle]
pub extern "C" fn M_StrToInt(str: *const c_char, result: *mut c_int) -> c_int {
    unsafe {
        if sscanf(str, b" 0x%x\0".as_ptr() as *const c_char, result) == 1 {
            return 1;
        }
        if sscanf(str, b" 0X%x\0".as_ptr() as *const c_char, result) == 1 {
            return 1;
        }
        if sscanf(str, b" 0%o\0".as_ptr() as *const c_char, result) == 1 {
            return 1;
        }
        if sscanf(str, b" %d\0".as_ptr() as *const c_char, result) == 1 {
            return 1;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn M_ExtractFileBase(path: *mut c_char, dest: *mut c_char) {
    unsafe {
        let len = strlen(path);
        let mut src = path.add(len).sub(1);
        while src != path && *src.sub(1) != DIR_SEPARATOR {
            src = src.sub(1);
        }
        let filename = src;
        let mut length: usize = 0;
        std::ptr::write_bytes(dest, 0, 8);
        while *src != 0 && *src != b'.' as c_char {
            if length >= 8 {
                let filename_cstr = CStr::from_ptr(filename);
                let dest_cstr = CStr::from_ptr(dest);
                eprintln!(
                    "Warning: Truncated '{}.????????' lump name to '{}'.",
                    filename_cstr.to_string_lossy(),
                    dest_cstr.to_string_lossy()
                );
                break;
            }
            *dest.add(length) = toupper(*src as c_int) as c_char;
            src = src.add(1);
            length += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn M_ForceUppercase(text: *mut c_char) {
    unsafe {
        let mut p = text;
        while *p != 0 {
            *p = toupper(*p as c_int) as c_char;
            p = p.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn M_ForceLowercase(text: *mut c_char) {
    unsafe {
        let mut p = text;
        while *p != 0 {
            *p = tolower(*p as c_int) as c_char;
            p = p.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn M_StrCaseStr(haystack: *mut c_char, needle: *mut c_char) -> *mut c_char {
    unsafe {
        let haystack_len = strlen(haystack);
        let needle_len = strlen(needle);
        if haystack_len < needle_len {
            return std::ptr::null_mut();
        }
        let len = haystack_len - needle_len;
        for i in 0..=len {
            if strncasecmp(haystack.add(i), needle, needle_len) == 0 {
                return haystack.add(i);
            }
        }
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn M_StringDuplicate(orig: *const c_char) -> *mut c_char {
    unsafe {
        let result = strdup(orig);
        if result.is_null() {
            I_Error(
                b"Failed to duplicate string (length %i)\n\0".as_ptr() as *const c_char,
                strlen(orig) as c_int,
            );
        }
        result
    }
}

#[no_mangle]
pub extern "C" fn M_StringReplace(
    haystack: *const c_char,
    needle: *const c_char,
    replacement: *const c_char,
) -> *mut c_char {
    unsafe {
        let needle_len = strlen(needle);
        let mut result_len = strlen(haystack) + 1;
        let mut p = haystack;
        loop {
            p = strstr(p, needle);
            if p.is_null() {
                break;
            }
            p = p.add(needle_len);
            result_len = result_len
                .wrapping_add(strlen(replacement))
                .wrapping_sub(needle_len);
        }
        let result = malloc(result_len) as *mut c_char;
        if result.is_null() {
            I_Error(b"M_StringReplace: Failed to allocate new string\0".as_ptr() as *const c_char);
            return std::ptr::null_mut();
        }
        let mut dst = result;
        let mut dst_len = result_len;
        p = haystack;
        while *p != 0 {
            if strncmp(p, needle, needle_len) == 0 {
                M_StringCopy(dst, replacement, dst_len);
                p = p.add(needle_len);
                let rep_len = strlen(replacement);
                dst = dst.add(rep_len);
                dst_len = dst_len.wrapping_sub(rep_len);
            } else {
                *dst = *p;
                dst = dst.add(1);
                dst_len -= 1;
                p = p.add(1);
            }
        }
        *dst = 0;
        result
    }
}

#[no_mangle]
pub extern "C" fn M_StringCopy(dest: *mut c_char, src: *const c_char, dest_size: usize) -> Boolean {
    unsafe {
        if dest_size >= 1 {
            *dest.add(dest_size - 1) = 0;
            strncpy(dest, src, dest_size - 1);
        } else {
            return Boolean::FALSE;
        }
        let len = strlen(dest);
        Boolean::from(*src.add(len) == 0)
    }
}

#[no_mangle]
pub extern "C" fn M_StringConcat(
    dest: *mut c_char,
    src: *const c_char,
    dest_size: usize,
) -> Boolean {
    unsafe {
        let mut offset = strlen(dest);
        if offset > dest_size {
            offset = dest_size;
        }
        M_StringCopy(dest.add(offset), src, dest_size - offset)
    }
}

#[no_mangle]
pub extern "C" fn M_StringStartsWith(s: *const c_char, prefix: *const c_char) -> Boolean {
    unsafe {
        let s_len = strlen(s);
        let prefix_len = strlen(prefix);
        Boolean::from(s_len > prefix_len && strncmp(s, prefix, prefix_len) == 0)
    }
}

#[no_mangle]
pub extern "C" fn M_StringEndsWith(s: *const c_char, suffix: *const c_char) -> Boolean {
    unsafe {
        let s_len = strlen(s);
        let suffix_len = strlen(suffix);
        Boolean::from(s_len >= suffix_len && strcmp(s.add(s_len - suffix_len), suffix) == 0)
    }
}

extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn M_StringJoinA(strs: *const *const c_char) -> *mut c_char {
    unsafe {
        let mut result_len: usize = 1;
        let mut p = strs;
        while !(*p).is_null() {
            result_len += strlen(*p);
            p = p.add(1);
        }

        let result = malloc(result_len) as *mut c_char;
        if result.is_null() {
            I_Error(b"M_StringJoinA: Failed to allocate new string\0".as_ptr() as *const c_char);
            return std::ptr::null_mut();
        }

        let mut dst = result;
        p = strs;
        while !(*p).is_null() {
            let src = *p;
            let len = strlen(src);
            std::ptr::copy_nonoverlapping(src, dst, len);
            dst = dst.add(len);
            p = p.add(1);
        }
        *dst = 0;
        result
    }
}

#[no_mangle]
pub extern "C" fn M_snprintf_clamp(buf: *mut c_char, len: usize, result: c_int) -> c_int {
    if len == 0 {
        return 0;
    }
    if result < 0 || result >= len as c_int {
        unsafe {
            *buf.add(len - 1) = 0;
        }
        (len as c_int) - 1
    } else {
        result
    }
}

pub(crate) fn m_snprintf_clamp(buf: *mut c_char, len: usize, result: c_int) -> c_int {
    if len == 0 {
        return 0;
    }
    if result < 0 || result >= len as c_int {
        unsafe {
            *buf.add(len - 1) = 0;
        }
        (len as c_int) - 1
    } else {
        result
    }
}

#[no_mangle]
pub extern "C" fn M_HomeDir() -> *const c_char {
    unsafe { getenv(b"HOME\0".as_ptr() as *const c_char) }
}

#[no_mangle]
pub extern "C" fn M_DefaultConfigDir() -> *const c_char {
    unsafe {
        let home = M_HomeDir();
        if home.is_null() {
            return b".\0".as_ptr() as *const c_char;
        }
        let xdg = getenv(b"XDG_CONFIG_HOME\0".as_ptr() as *const c_char);
        if !xdg.is_null() {
            let strs: [*const c_char; 3] =
                [xdg, b"/doom\0".as_ptr() as *const c_char, std::ptr::null()];
            return M_StringJoinA(strs.as_ptr());
        }
        let strs: [*const c_char; 3] = [
            home,
            b"/.config/doom\0".as_ptr() as *const c_char,
            std::ptr::null(),
        ];
        M_StringJoinA(strs.as_ptr())
    }
}

#[no_mangle]
pub extern "C" fn M_OEMToUTF8(_oem: *const c_char) -> *mut c_char {
    std::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_string_copy_truncation() {
        let src = CString::new("Hello, World!").unwrap();
        let mut dest: Vec<c_char> = vec![0; 6];
        let result = unsafe { M_StringCopy(dest.as_mut_ptr(), src.as_ptr(), dest.len()) };
        assert_eq!(result, Boolean::FALSE);
        let cstr = unsafe { CStr::from_ptr(dest.as_ptr()) };
        assert_eq!(cstr.to_str().unwrap(), "Hello");
    }

    #[test]
    fn test_string_concat_truncation() {
        let mut buf: Vec<c_char> = CString::new("Hello, ")
            .unwrap()
            .into_bytes_with_nul()
            .into_iter()
            .map(|b| b as c_char)
            .collect();
        buf.resize(11, 0);
        let suffix = CString::new("World!").unwrap();
        let result = unsafe { M_StringConcat(buf.as_mut_ptr(), suffix.as_ptr(), buf.len()) };
        // "Hello, " (7) + "World!" (6) = 13 chars, but buffer is 11 => truncated to "Hello, Wor"
        assert_eq!(result, Boolean::FALSE);
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
        assert_eq!(cstr.to_str().unwrap(), "Hello, Wor");
    }
}
