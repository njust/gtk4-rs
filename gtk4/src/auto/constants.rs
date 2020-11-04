// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gtk_sys;
use std::ffi::CStr;

pub static IM_MODULE_EXTENSION_POINT_NAME: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_IM_MODULE_EXTENSION_POINT_NAME)
            .to_str()
            .unwrap()
    });
pub static LEVEL_BAR_OFFSET_FULL: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_LEVEL_BAR_OFFSET_FULL)
            .to_str()
            .unwrap()
    });
pub static LEVEL_BAR_OFFSET_HIGH: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_LEVEL_BAR_OFFSET_HIGH)
            .to_str()
            .unwrap()
    });
pub static LEVEL_BAR_OFFSET_LOW: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_LEVEL_BAR_OFFSET_LOW)
            .to_str()
            .unwrap()
    });
pub static MEDIA_FILE_EXTENSION_POINT_NAME: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_MEDIA_FILE_EXTENSION_POINT_NAME)
            .to_str()
            .unwrap()
    });
pub static PAPER_NAME_A3: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_A3).to_str().unwrap()
    });
pub static PAPER_NAME_A4: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_A4).to_str().unwrap()
    });
pub static PAPER_NAME_A5: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_A5).to_str().unwrap()
    });
pub static PAPER_NAME_B5: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_B5).to_str().unwrap()
    });
pub static PAPER_NAME_EXECUTIVE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_EXECUTIVE)
            .to_str()
            .unwrap()
    });
pub static PAPER_NAME_LEGAL: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_LEGAL)
            .to_str()
            .unwrap()
    });
pub static PAPER_NAME_LETTER: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PAPER_NAME_LETTER)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_COLLATE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_COLLATE)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_DEFAULT_SOURCE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_DEFAULT_SOURCE)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_DITHER: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_DITHER)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_DUPLEX: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_DUPLEX)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_FINISHINGS: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_FINISHINGS)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_MEDIA_TYPE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_MEDIA_TYPE)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_NUMBER_UP: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_NUMBER_UP)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_NUMBER_UP_LAYOUT: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_NUMBER_UP_LAYOUT)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_N_COPIES: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_N_COPIES)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_ORIENTATION: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_ORIENTATION)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_OUTPUT_BASENAME: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_OUTPUT_BASENAME)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_OUTPUT_BIN: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_OUTPUT_BIN)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_OUTPUT_DIR: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_OUTPUT_DIR)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_OUTPUT_FILE_FORMAT: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_OUTPUT_FILE_FORMAT)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_OUTPUT_URI: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_OUTPUT_URI)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PAGE_RANGES: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PAGE_RANGES)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PAGE_SET: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PAGE_SET)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PAPER_FORMAT: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PAPER_FORMAT)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PAPER_HEIGHT: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PAPER_HEIGHT)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PAPER_WIDTH: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PAPER_WIDTH)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PRINTER: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PRINTER)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PRINTER_LPI: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PRINTER_LPI)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_PRINT_PAGES: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_PRINT_PAGES)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_QUALITY: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_QUALITY)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_RESOLUTION: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_RESOLUTION)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_RESOLUTION_X: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_RESOLUTION_X)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_RESOLUTION_Y: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_RESOLUTION_Y)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_REVERSE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_REVERSE)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_SCALE: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_SCALE)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_USE_COLOR: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_USE_COLOR)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_WIN32_DRIVER_EXTRA: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_WIN32_DRIVER_EXTRA)
            .to_str()
            .unwrap()
    });
pub static PRINT_SETTINGS_WIN32_DRIVER_VERSION: once_cell::sync::Lazy<&'static str> =
    once_cell::sync::Lazy::new(|| unsafe {
        CStr::from_ptr(gtk_sys::GTK_PRINT_SETTINGS_WIN32_DRIVER_VERSION)
            .to_str()
            .unwrap()
    });
