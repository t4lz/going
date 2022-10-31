/* automatically generated by rust-bindgen 0.61.0 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 0;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 35;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub type wchar_t = ::std::os::raw::c_uint;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GoString_ {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}
#[test]
fn bindgen_test_layout__GoString_() {
    const UNINIT: ::std::mem::MaybeUninit<_GoString_> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_GoString_>(),
        16usize,
        concat!("Size of: ", stringify!(_GoString_))
    );
    assert_eq!(
        ::std::mem::align_of::<_GoString_>(),
        8usize,
        concat!("Alignment of ", stringify!(_GoString_))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(n)
        )
    );
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_uint;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    const UNINIT: ::std::mem::MaybeUninit<__fsid_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_int;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct raw_syscall_result {
    pub res1: usize,
    pub res2: usize,
    pub errno: usize,
}
#[test]
fn bindgen_test_layout_raw_syscall_result() {
    const UNINIT: ::std::mem::MaybeUninit<raw_syscall_result> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<raw_syscall_result>(),
        24usize,
        concat!("Size of: ", stringify!(raw_syscall_result))
    );
    assert_eq!(
        ::std::mem::align_of::<raw_syscall_result>(),
        8usize,
        concat!("Alignment of ", stringify!(raw_syscall_result))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).res1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(raw_syscall_result),
            "::",
            stringify!(res1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).res2) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(raw_syscall_result),
            "::",
            stringify!(res2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).errno) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(raw_syscall_result),
            "::",
            stringify!(errno)
        )
    );
}
pub type syscall_callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> raw_syscall_result,
>;
extern "C" {
    pub fn syscall_bridge(
        arg1: syscall_callback,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> raw_syscall_result;
}
pub type GoInt8 = ::std::os::raw::c_schar;
pub type GoUint8 = ::std::os::raw::c_uchar;
pub type GoInt16 = ::std::os::raw::c_short;
pub type GoUint16 = ::std::os::raw::c_ushort;
pub type GoInt32 = ::std::os::raw::c_int;
pub type GoUint32 = ::std::os::raw::c_uint;
pub type GoInt64 = ::std::os::raw::c_longlong;
pub type GoUint64 = ::std::os::raw::c_ulonglong;
pub type GoInt = GoInt64;
pub type GoUint = GoUint64;
pub type GoUintptr = usize;
pub type GoFloat32 = f32;
pub type GoFloat64 = f64;
pub type GoComplex64 = __BindgenComplex<f32>;
pub type GoComplex128 = __BindgenComplex<f64>;
pub type _check_for_64_bit_pointer_matching_GoInt = [::std::os::raw::c_char; 1usize];
pub type GoString = _GoString_;
pub type GoMap = *mut ::std::os::raw::c_void;
pub type GoChan = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GoInterface() {
    const UNINIT: ::std::mem::MaybeUninit<GoInterface> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GoInterface>(),
        16usize,
        concat!("Size of: ", stringify!(GoInterface))
    );
    assert_eq!(
        ::std::mem::align_of::<GoInterface>(),
        8usize,
        concat!("Alignment of ", stringify!(GoInterface))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).t) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(v)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: GoInt,
    pub cap: GoInt,
}
#[test]
fn bindgen_test_layout_GoSlice() {
    const UNINIT: ::std::mem::MaybeUninit<GoSlice> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GoSlice>(),
        24usize,
        concat!("Size of: ", stringify!(GoSlice))
    );
    assert_eq!(
        ::std::mem::align_of::<GoSlice>(),
        8usize,
        concat!("Alignment of ", stringify!(GoSlice))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cap) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(cap)
        )
    );
}
extern "C" {
    pub fn Initialize(rustDetour: syscall_callback) -> GoUintptr;
}
