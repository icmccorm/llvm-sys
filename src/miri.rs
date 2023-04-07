/* automatically generated by rust-bindgen 0.64.0 */
use crate::execution_engine::{LLVMGenericValueRef};
use crate::prelude::LLVMTypeRef;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MiriPointer {
    pub addr: ::std::os::raw::c_ulonglong,
    pub alloc_id: ::std::os::raw::c_ulonglong,
    pub tag: ::std::os::raw::c_ulonglong,
    pub offset: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MiriErrorTrace {
    pub directory: *const ::std::os::raw::c_char,
    pub directory_len: ::libc::size_t,
    pub file: *const ::std::os::raw::c_char,
    pub file_len: ::libc::size_t,
    pub line: u32,
    pub column: u32,
}

pub type MiriInterpCxOpaque = ::std::os::raw::c_void;

pub type MiriAllocationHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: ::libc::size_t) -> MiriPointer,
>;
pub type MiriFreeHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: MiriPointer) -> bool,
>;
pub type MiriLoadStoreHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: LLVMGenericValueRef,
        arg3: MiriPointer,
        arg4: LLVMTypeRef,
        arg5: ::libc::size_t,
    ) -> bool,
>;
pub type MiriCallbackHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        ret_ref: LLVMGenericValueRef,
        args_ref: LLVMGenericValueRef,
        num_args: ::libc::size_t,
        name: *const ::libc::c_char,
        name_length: ::libc::size_t,
        tref: LLVMTypeRef,
    ) -> bool,
>;
pub type MiriStackTraceRecorderHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        traces: *const MiriErrorTrace,
        num_traces: ::libc::size_t,
    ),
>;
