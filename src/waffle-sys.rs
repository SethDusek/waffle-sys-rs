/* automatically generated by rust-bindgen */

pub type __int32_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_config {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_window {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum waffle_error {
    WAFFLE_NO_ERROR = 0,
    WAFFLE_ERROR_FATAL = 1,
    WAFFLE_ERROR_UNKNOWN = 2,
    WAFFLE_ERROR_INTERNAL = 3,
    WAFFLE_ERROR_BAD_ALLOC = 4,
    WAFFLE_ERROR_NOT_INITIALIZED = 5,
    WAFFLE_ERROR_ALREADY_INITIALIZED = 6,
    WAFFLE_ERROR_BAD_ATTRIBUTE = 8,
    WAFFLE_ERROR_BAD_PARAMETER = 16,
    WAFFLE_ERROR_BAD_DISPLAY_MATCH = 17,
    WAFFLE_ERROR_UNSUPPORTED_ON_PLATFORM = 18,
    WAFFLE_ERROR_BUILT_WITHOUT_SUPPORT = 19,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_error_info {
    pub code: waffle_error,
    pub message: *const ::std::os::raw::c_char,
    pub message_length: usize,
}
#[test]
fn bindgen_test_layout_waffle_error_info() {
    assert_eq!(
        ::std::mem::size_of::<waffle_error_info>(),
        24usize,
        concat!("Size of: ", stringify!(waffle_error_info))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_error_info>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_error_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_error_info>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_error_info),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_error_info>())).message as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_error_info),
            "::",
            stringify!(message)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<waffle_error_info>())).message_length as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_error_info),
            "::",
            stringify!(message_length)
        )
    );
}
extern "C" {
    pub fn waffle_error_get_code() -> waffle_error;
}
extern "C" {
    pub fn waffle_error_get_info() -> *const waffle_error_info;
}
extern "C" {
    pub fn waffle_error_to_string(e: waffle_error) -> *const ::std::os::raw::c_char;
}
pub const waffle_enum_WAFFLE_DONT_CARE: waffle_enum = -1;
pub const waffle_enum_WAFFLE_NONE: waffle_enum = 0;
pub const waffle_enum_WAFFLE_PLATFORM: waffle_enum = 16;
pub const waffle_enum_WAFFLE_PLATFORM_ANDROID: waffle_enum = 17;
pub const waffle_enum_WAFFLE_PLATFORM_CGL: waffle_enum = 18;
pub const waffle_enum_WAFFLE_PLATFORM_GLX: waffle_enum = 19;
pub const waffle_enum_WAFFLE_PLATFORM_WAYLAND: waffle_enum = 20;
pub const waffle_enum_WAFFLE_PLATFORM_X11_EGL: waffle_enum = 21;
pub const waffle_enum_WAFFLE_PLATFORM_GBM: waffle_enum = 22;
pub const waffle_enum_WAFFLE_PLATFORM_WGL: waffle_enum = 23;
pub const waffle_enum_WAFFLE_PLATFORM_NACL: waffle_enum = 24;
pub const waffle_enum_WAFFLE_PLATFORM_SURFACELESS_EGL: waffle_enum = 25;
pub const waffle_enum_WAFFLE_CONTEXT_API: waffle_enum = 522;
pub const waffle_enum_WAFFLE_CONTEXT_OPENGL: waffle_enum = 523;
pub const waffle_enum_WAFFLE_CONTEXT_OPENGL_ES1: waffle_enum = 524;
pub const waffle_enum_WAFFLE_CONTEXT_OPENGL_ES2: waffle_enum = 525;
pub const waffle_enum_WAFFLE_CONTEXT_OPENGL_ES3: waffle_enum = 532;
pub const waffle_enum_WAFFLE_CONTEXT_MAJOR_VERSION: waffle_enum = 526;
pub const waffle_enum_WAFFLE_CONTEXT_MINOR_VERSION: waffle_enum = 527;
pub const waffle_enum_WAFFLE_CONTEXT_PROFILE: waffle_enum = 528;
pub const waffle_enum_WAFFLE_CONTEXT_CORE_PROFILE: waffle_enum = 529;
pub const waffle_enum_WAFFLE_CONTEXT_COMPATIBILITY_PROFILE: waffle_enum = 530;
pub const waffle_enum_WAFFLE_CONTEXT_FORWARD_COMPATIBLE: waffle_enum = 533;
pub const waffle_enum_WAFFLE_CONTEXT_DEBUG: waffle_enum = 534;
pub const waffle_enum_WAFFLE_CONTEXT_ROBUST_ACCESS: waffle_enum = 535;
pub const waffle_enum_WAFFLE_RED_SIZE: waffle_enum = 513;
pub const waffle_enum_WAFFLE_GREEN_SIZE: waffle_enum = 514;
pub const waffle_enum_WAFFLE_BLUE_SIZE: waffle_enum = 515;
pub const waffle_enum_WAFFLE_ALPHA_SIZE: waffle_enum = 516;
pub const waffle_enum_WAFFLE_DEPTH_SIZE: waffle_enum = 517;
pub const waffle_enum_WAFFLE_STENCIL_SIZE: waffle_enum = 518;
pub const waffle_enum_WAFFLE_SAMPLE_BUFFERS: waffle_enum = 519;
pub const waffle_enum_WAFFLE_SAMPLES: waffle_enum = 520;
pub const waffle_enum_WAFFLE_DOUBLE_BUFFERED: waffle_enum = 521;
pub const waffle_enum_WAFFLE_ACCUM_BUFFER: waffle_enum = 531;
pub const waffle_enum_WAFFLE_DL_OPENGL: waffle_enum = 769;
pub const waffle_enum_WAFFLE_DL_OPENGL_ES1: waffle_enum = 770;
pub const waffle_enum_WAFFLE_DL_OPENGL_ES2: waffle_enum = 771;
pub const waffle_enum_WAFFLE_DL_OPENGL_ES3: waffle_enum = 772;
pub const waffle_enum_WAFFLE_WINDOW_WIDTH: waffle_enum = 784;
pub const waffle_enum_WAFFLE_WINDOW_HEIGHT: waffle_enum = 785;
pub const waffle_enum_WAFFLE_WINDOW_FULLSCREEN: waffle_enum = 786;
pub type waffle_enum = i32;
extern "C" {
    pub fn waffle_enum_to_string(e: i32) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn waffle_init(attrib_list: *const i32) -> bool;
}
extern "C" {
    pub fn waffle_make_current(
        dpy: *mut waffle_display,
        window: *mut waffle_window,
        ctx: *mut waffle_context,
    ) -> bool;
}
extern "C" {
    pub fn waffle_get_proc_address(
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn waffle_is_extension_in_string(
        extension_string: *const ::std::os::raw::c_char,
        extension_name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn waffle_display_connect(name: *const ::std::os::raw::c_char) -> *mut waffle_display;
}
extern "C" {
    pub fn waffle_display_disconnect(self_: *mut waffle_display) -> bool;
}
extern "C" {
    pub fn waffle_display_supports_context_api(
        self_: *mut waffle_display,
        context_api: i32,
    ) -> bool;
}
extern "C" {
    pub fn waffle_display_get_native(self_: *mut waffle_display) -> *mut waffle_native_display;
}
extern "C" {
    pub fn waffle_config_choose(
        dpy: *mut waffle_display,
        attrib_list: *const i32,
    ) -> *mut waffle_config;
}
extern "C" {
    pub fn waffle_config_destroy(self_: *mut waffle_config) -> bool;
}
extern "C" {
    pub fn waffle_config_get_native(self_: *mut waffle_config) -> *mut waffle_native_config;
}
extern "C" {
    pub fn waffle_context_create(
        config: *mut waffle_config,
        shared_ctx: *mut waffle_context,
    ) -> *mut waffle_context;
}
extern "C" {
    pub fn waffle_context_destroy(self_: *mut waffle_context) -> bool;
}
extern "C" {
    pub fn waffle_context_get_native(self_: *mut waffle_context) -> *mut waffle_native_context;
}
extern "C" {
    pub fn waffle_window_create(
        config: *mut waffle_config,
        width: i32,
        height: i32,
    ) -> *mut waffle_window;
}
extern "C" {
    pub fn waffle_window_destroy(self_: *mut waffle_window) -> bool;
}
extern "C" {
    pub fn waffle_window_show(self_: *mut waffle_window) -> bool;
}
extern "C" {
    pub fn waffle_window_swap_buffers(self_: *mut waffle_window) -> bool;
}
extern "C" {
    pub fn waffle_window_get_native(self_: *mut waffle_window) -> *mut waffle_native_window;
}
extern "C" {
    pub fn waffle_dl_can_open(dl: i32) -> bool;
}
extern "C" {
    pub fn waffle_dl_sym(
        dl: i32,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_glx_config {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_glx_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_glx_display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_glx_window {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_wayland_config {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_wayland_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_wayland_display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_wayland_window {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_x11_egl_config {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_x11_egl_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_x11_egl_display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_x11_egl_window {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union waffle_native_display {
    pub gbm: *mut waffle_gbm_display,
    pub glx: *mut waffle_glx_display,
    pub x11_egl: *mut waffle_x11_egl_display,
    pub wayland: *mut waffle_wayland_display,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_waffle_native_display() {
    assert_eq!(
        ::std::mem::size_of::<waffle_native_display>(),
        8usize,
        concat!("Size of: ", stringify!(waffle_native_display))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_native_display>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_native_display))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_display>())).gbm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_display),
            "::",
            stringify!(gbm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_display>())).glx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_display),
            "::",
            stringify!(glx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_display>())).x11_egl as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_display),
            "::",
            stringify!(x11_egl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_display>())).wayland as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_display),
            "::",
            stringify!(wayland)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union waffle_native_config {
    pub gbm: *mut waffle_gbm_config,
    pub glx: *mut waffle_glx_config,
    pub x11_egl: *mut waffle_x11_egl_config,
    pub wayland: *mut waffle_wayland_config,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_waffle_native_config() {
    assert_eq!(
        ::std::mem::size_of::<waffle_native_config>(),
        8usize,
        concat!("Size of: ", stringify!(waffle_native_config))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_native_config>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_native_config))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_config>())).gbm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_config),
            "::",
            stringify!(gbm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_config>())).glx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_config),
            "::",
            stringify!(glx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_config>())).x11_egl as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_config),
            "::",
            stringify!(x11_egl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_config>())).wayland as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_config),
            "::",
            stringify!(wayland)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union waffle_native_context {
    pub gbm: *mut waffle_gbm_context,
    pub glx: *mut waffle_glx_context,
    pub x11_egl: *mut waffle_x11_egl_context,
    pub wayland: *mut waffle_wayland_context,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_waffle_native_context() {
    assert_eq!(
        ::std::mem::size_of::<waffle_native_context>(),
        8usize,
        concat!("Size of: ", stringify!(waffle_native_context))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_native_context>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_native_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_context>())).gbm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_context),
            "::",
            stringify!(gbm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_context>())).glx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_context),
            "::",
            stringify!(glx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_context>())).x11_egl as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_context),
            "::",
            stringify!(x11_egl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_context>())).wayland as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_context),
            "::",
            stringify!(wayland)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union waffle_native_window {
    pub gbm: *mut waffle_gbm_window,
    pub glx: *mut waffle_glx_window,
    pub x11_egl: *mut waffle_x11_egl_window,
    pub wayland: *mut waffle_wayland_window,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_waffle_native_window() {
    assert_eq!(
        ::std::mem::size_of::<waffle_native_window>(),
        8usize,
        concat!("Size of: ", stringify!(waffle_native_window))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_native_window>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_native_window))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_window>())).gbm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_window),
            "::",
            stringify!(gbm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_window>())).glx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_window),
            "::",
            stringify!(glx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_window>())).x11_egl as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_window),
            "::",
            stringify!(x11_egl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_native_window>())).wayland as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_native_window),
            "::",
            stringify!(wayland)
        )
    );
}
extern "C" {
    pub fn waffle_attrib_list_length(attrib_list: *const i32) -> i32;
}
extern "C" {
    pub fn waffle_attrib_list_get(attrib_list: *const i32, key: i32, value: *mut i32) -> bool;
}
extern "C" {
    pub fn waffle_attrib_list_get_with_default(
        attrib_list: *const i32,
        key: i32,
        value: *mut i32,
        default_value: i32,
    ) -> bool;
}
extern "C" {
    pub fn waffle_attrib_list_update(attrib_list: *mut i32, key: i32, value: i32) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_device {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_bo {
    _unused: [u8; 0],
}
pub type EGLDisplay = *mut ::std::os::raw::c_void;
pub type EGLConfig = *mut ::std::os::raw::c_void;
pub type EGLSurface = *mut ::std::os::raw::c_void;
pub type EGLContext = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_gbm_display {
    pub gbm_device: *mut gbm_device,
    pub egl_display: EGLDisplay,
}
#[test]
fn bindgen_test_layout_waffle_gbm_display() {
    assert_eq!(
        ::std::mem::size_of::<waffle_gbm_display>(),
        16usize,
        concat!("Size of: ", stringify!(waffle_gbm_display))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_gbm_display>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_gbm_display))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_display>())).gbm_device as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_display),
            "::",
            stringify!(gbm_device)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_display>())).egl_display as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_display),
            "::",
            stringify!(egl_display)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_gbm_config {
    pub display: waffle_gbm_display,
    pub egl_config: EGLConfig,
}
#[test]
fn bindgen_test_layout_waffle_gbm_config() {
    assert_eq!(
        ::std::mem::size_of::<waffle_gbm_config>(),
        24usize,
        concat!("Size of: ", stringify!(waffle_gbm_config))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_gbm_config>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_gbm_config))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_config>())).display as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_config),
            "::",
            stringify!(display)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_config>())).egl_config as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_config),
            "::",
            stringify!(egl_config)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_gbm_context {
    pub display: waffle_gbm_display,
    pub egl_context: EGLContext,
}
#[test]
fn bindgen_test_layout_waffle_gbm_context() {
    assert_eq!(
        ::std::mem::size_of::<waffle_gbm_context>(),
        24usize,
        concat!("Size of: ", stringify!(waffle_gbm_context))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_gbm_context>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_gbm_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_context>())).display as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_context),
            "::",
            stringify!(display)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_context>())).egl_context as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_context),
            "::",
            stringify!(egl_context)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct waffle_gbm_window {
    pub display: waffle_gbm_display,
    pub gbm_surface: *mut gbm_surface,
    pub egl_surface: EGLSurface,
}
#[test]
fn bindgen_test_layout_waffle_gbm_window() {
    assert_eq!(
        ::std::mem::size_of::<waffle_gbm_window>(),
        32usize,
        concat!("Size of: ", stringify!(waffle_gbm_window))
    );
    assert_eq!(
        ::std::mem::align_of::<waffle_gbm_window>(),
        8usize,
        concat!("Alignment of ", stringify!(waffle_gbm_window))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_window>())).display as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_window),
            "::",
            stringify!(display)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_window>())).gbm_surface as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_window),
            "::",
            stringify!(gbm_surface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<waffle_gbm_window>())).egl_surface as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(waffle_gbm_window),
            "::",
            stringify!(egl_surface)
        )
    );
}