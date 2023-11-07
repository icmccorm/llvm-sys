use crate::execution_engine::{LLVMGenericValueArrayRef, LLVMGenericValueRef};
use crate::prelude::LLVMTypeRef;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MiriProvenance {
    pub alloc_id: u64,
    pub tag: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MiriPointer {
    pub addr: u64,
    pub prov: MiriProvenance,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct APIntPointer {
    pub data: *const u64,
    pub words: u8
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MiriErrorTrace {
    pub directory: *const ::std::os::raw::c_char,
    pub directory_len: u64,
    pub file: *const ::std::os::raw::c_char,
    pub file_len: u64,
    pub line: u32,
    pub column: u32,
}

pub type MiriInterpCxOpaque = ::std::os::raw::c_void;

pub type MiriMemset = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: MiriPointer,
        arg3: u8,
        arg4: u64,
    ) -> bool,
>;

pub type MiriIntToPtr = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: u64) -> MiriPointer,
>;

pub type MiriPtrToInt = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: MiriPointer) -> u64,
>;

pub type MiriMemcpy = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: MiriPointer,
        arg3: *const u8,
        arg4: u64,
    ) -> bool,
>;

pub type MiriAllocationHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: u64, arg3: u64, arg4: bool) -> MiriPointer,
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
        arg5: u64,
        arg6: u64,
    ) -> bool,
>;
pub type MiriCallByNameHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        args_ref: LLVMGenericValueArrayRef,
        name: *const ::std::os::raw::c_char,
        name_length: u64,
        tref: LLVMTypeRef,
    ) -> bool,
>;
pub type MiriCallByPointerHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        fn_ref: MiriPointer,
        args_ref: LLVMGenericValueArrayRef,
        tref: LLVMTypeRef,
    ) -> bool,
>;

pub type MiriStackTraceRecorderHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        traces: *const MiriErrorTrace,
        num_traces: u64,
        instruction_ptr: *const ::std::os::raw::c_char,
        instruction_len: u64,
    ),
>;
pub type MiriRegisterGlobalHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        name: *const ::std::os::raw::c_char,
        name_length: u64,
        pointer: MiriPointer,
    ) -> bool
>;