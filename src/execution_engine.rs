//! Runtime code generation and execution.

use super::miri::*;
use super::prelude::*;
use super::target::LLVMTargetDataRef;
use super::target_machine::{LLVMCodeModel, LLVMTargetMachineRef};

#[derive(Debug)]
pub enum LLVMOpaqueGenericValueArrayRef {}

#[derive(Debug)]
pub enum LLVMOpaqueGenericValue {}

#[derive(Debug)]
pub enum LLVMOpaqueExecutionEngine {}

#[derive(Debug)]
pub enum LLVMOpaqueMCJITMemoryManager {}

pub type LLVMGenericValueArrayRef = *mut LLVMOpaqueGenericValueArrayRef;
pub type LLVMGenericValueRef = *mut LLVMOpaqueGenericValue;
pub type LLVMExecutionEngineRef = *mut LLVMOpaqueExecutionEngine;
pub type LLVMMCJITMemoryManagerRef = *mut LLVMOpaqueMCJITMemoryManager;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct LLVMMCJITCompilerOptions {
    pub OptLevel: ::libc::c_uint,
    pub CodeModel: LLVMCodeModel,
    pub NoFramePointerElim: LLVMBool,
    pub EnableFastISel: LLVMBool,
    pub MCJMM: LLVMMCJITMemoryManagerRef,
}

pub type LLVMMemoryManagerAllocateCodeSectionCallback = extern "C" fn(
    Opaque: *mut ::libc::c_void,
    Size: ::libc::uintptr_t,
    Alignment: ::libc::c_uint,
    SectionID: ::libc::c_uint,
    SectionName: *const ::libc::c_char,
) -> *mut u8;
pub type LLVMMemoryManagerAllocateDataSectionCallback = extern "C" fn(
    Opaque: *mut ::libc::c_void,
    Size: ::libc::uintptr_t,
    Alignment: ::libc::c_uint,
    SectionID: ::libc::c_uint,
    SectionName: *const ::libc::c_char,
    IsReadOnly: LLVMBool,
) -> *mut u8;
pub type LLVMMemoryManagerFinalizeMemoryCallback =
    extern "C" fn(Opaque: *mut ::libc::c_void, ErrMsg: *mut *mut ::libc::c_char) -> LLVMBool;
pub type LLVMMemoryManagerDestroyCallback = Option<extern "C" fn(Opaque: *mut ::libc::c_void)>;

extern "C" {
    pub fn LLVMLinkInMCJIT();
    pub fn LLVMLinkInInterpreter();
    // Operations on generic values
    pub fn LLVMCreateGenericValueOfData(Data: *const u8, Len: u32) -> LLVMGenericValueRef;
    pub fn LLVMCreateGenericValueOfInt(
        Ty: LLVMTypeRef,
        N: ::libc::c_ulonglong,
        IsSigned: LLVMBool,
    ) -> LLVMGenericValueRef;
    pub fn LLVMCreateGenericValueOfPointer(P: *mut ::libc::c_void) -> LLVMGenericValueRef;
    pub fn LLVMCreateGenericValueOfMiriPointer(meta: MiriPointer) -> LLVMGenericValueRef;
    pub fn LLVMCreateGenericValueOfFloat(
        Ty: LLVMTypeRef,
        N: ::libc::c_double,
    ) -> LLVMGenericValueRef;
    pub fn LLVMGenericValueIntWidth(GenValRef: LLVMGenericValueRef) -> ::libc::c_uint;
    pub fn LLVMGenericValueToInt(
        GenVal: LLVMGenericValueRef,
        IsSigned: LLVMBool,
    ) -> ::libc::c_ulonglong;
    pub fn LLVMGenericValueToPointer(GenVal: LLVMGenericValueRef) -> *mut ::libc::c_void;
    pub fn LLVMGenericValueToMiriPointer(GenVal: LLVMGenericValueRef) -> MiriPointer;
    pub fn LLVMGenericValueToFloat(
        TyRef: LLVMTypeRef,
        GenVal: LLVMGenericValueRef,
    ) -> ::libc::c_double;
    pub fn LLVMGenericValueToFloatSingle(GenVal: LLVMGenericValueRef) -> ::libc::c_float;
    pub fn LLVMGenericValueToFloatDouble(GenVal: LLVMGenericValueRef) -> ::libc::c_double;
    pub fn LLVMGenericValueSetDoubleValue(GenVal: LLVMGenericValueRef, DoubleVal: ::libc::c_double);
    pub fn LLVMGenericValueSetFloatValue(GenVal: LLVMGenericValueRef, FloatVal: ::libc::c_float);
    pub fn LLVMGenericValueSetIntValue(GenVal: LLVMGenericValueRef, Src: u64, LoadBytes: u32);
    pub fn LLVMGenericValueSetMiriPointerValue(GenVal: LLVMGenericValueRef, Ptr: MiriPointer);
    pub fn LLVMGenericValueSetMiriParentPointerValue(
        GenVal: LLVMGenericValueRef,
        PointerMetaVal: MiriPointer,
    );
    pub fn LLVMCreateAggregateGenericValue(NumMembers: u64) -> LLVMGenericValueRef;
    pub fn LLVMGenericValueAppendAggregateValue(
        GenVal: LLVMGenericValueRef,
        GenValElement: LLVMGenericValueRef,
    );
    pub fn LLVMGenericValueEnsureCapacity(GenVal: LLVMGenericValueRef, Size: u64);
    pub fn LLVMGetPointerToAggregateGenericValue(
        GenValRef: LLVMGenericValueRef,
        Index: u64,
    ) -> LLVMGenericValueRef;
    pub fn LLVMGetAggregateGenericValueLength(GenValRef: LLVMGenericValueRef) -> ::libc::size_t;
    pub fn LLVMGenericValueArrayRefGetElementAt(
        GenArray: LLVMGenericValueArrayRef,
        Index: u64,
    ) -> LLVMGenericValueRef;

    pub fn LLVMGenericValueArrayRefLength(GenArray: LLVMGenericValueArrayRef) -> u64;

    pub fn LLVMDisposeGenericValue(GenVal: LLVMGenericValueRef);

    pub fn LLVMGenericValueCopy(Src: LLVMGenericValueRef, Dest: LLVMGenericValueRef);
    // Operations on execution engines
    pub fn LLVMCreateExecutionEngineForModule(
        OutEE: *mut LLVMExecutionEngineRef,
        M: LLVMModuleRef,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateInterpreterForModule(
        OutInterp: *mut LLVMExecutionEngineRef,
        M: LLVMModuleRef,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateJITCompilerForModule(
        OutJIT: *mut LLVMExecutionEngineRef,
        M: LLVMModuleRef,
        OptLevel: ::libc::c_uint,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMInitializeMCJITCompilerOptions(
        Options: *mut LLVMMCJITCompilerOptions,
        SizeOfOptions: ::libc::size_t,
    );

    // hooks into Miri
    pub fn LLVMExecutionEngineSetMiriInterpCxWrapper(
        EE: LLVMExecutionEngineRef,
        MiriWrapper: *mut ::std::os::raw::c_void,
    );
    pub fn LLVMExecutionEngineSetMiriStackTraceRecorderHook(
        EE: LLVMExecutionEngineRef,
        IncomingStackTraceRecorderHook: MiriStackTraceRecorderHook,
    );
    pub fn LLVMExecutionEngineSetMiriCallByNameHook(
        EE: LLVMExecutionEngineRef,
        IncomingCallbackHook: MiriCallByNameHook,
    );
    pub fn LLVMExecutionEngineSetMiriCallByPointerHook(
        EE: LLVMExecutionEngineRef,
        IncomingCallbackHook: MiriCallByPointerHook,
    );
    pub fn LLVMExecutionEngineSetMiriLoadHook(
        EE: LLVMExecutionEngineRef,
        IncomingLoadHook: MiriLoadStoreHook,
    );
    pub fn LLVMExecutionEngineSetMiriStoreHook(
        EE: LLVMExecutionEngineRef,
        IncomingStoreHook: MiriLoadStoreHook,
    );
    pub fn LLVMExecutionEngineSetMiriMalloc(
        EE: LLVMExecutionEngineRef,
        IncomingMallocHook: MiriAllocationHook,
    );
    pub fn LLVMExecutionEngineSetMiriFree(
        EE: LLVMExecutionEngineRef,
        IncomingFreeHook: MiriFreeHook,
    );
    pub fn LLVMExecutionEngineSetMiriMemset(EE: LLVMExecutionEngineRef, IncomingMemset: MiriMemset);
    pub fn LLVMExecutionEngineSetMiriMemcpy(EE: LLVMExecutionEngineRef, IncomingMemcpy: MiriMemcpy);

    pub fn LLVMExecutionEngineSetMiriIntToPtr(
        EE: LLVMExecutionEngineRef,
        IncomingIntToPtr: MiriIntToPtr,
    );

    pub fn LLVMExecutionEngineSetMiriPtrToInt(
        EE: LLVMExecutionEngineRef,
        IncomingPtrToInt: MiriPtrToInt,
    );

    pub fn LLVMExecutionEngineSetMiriRegisterGlobalHook(EE: LLVMExecutionEngineRef, GlobalHook: MiriRegisterGlobalHook);

    pub fn LLVMExecutionEngineStepThread(EE: LLVMExecutionEngineRef, ThreadID: u64) -> LLVMBool;

    pub fn LLVMExecutionEngineCreateThread(
        EE: LLVMExecutionEngineRef,
        ThreadID: u64,
        F: LLVMValueRef,
        NumArgs: u32,
        Args: *mut LLVMGenericValueRef,
    ) -> LLVMGenericValueRef;

    pub fn LLVMExecutionEngineHasThread(EE: LLVMExecutionEngineRef, ThreadID: u64) -> LLVMBool;

    pub fn LLVMExecutionEngineTerminateThread(EE: LLVMExecutionEngineRef, ThreadID: u64);
    /// Create an MCJIT execution engine for a module, with the given options.
    ///
    /// It is
    /// the responsibility of the caller to ensure that all fields in Options up to
    /// the given SizeOfOptions are initialized. It is correct to pass a smaller
    /// value of SizeOfOptions that omits some fields. The canonical way of using
    /// this is:
    ///
    /// ```c++
    /// LLVMMCJITCompilerOptions options;
    /// LLVMInitializeMCJITCompilerOptions(&options, sizeof(options));
    /// // ... fill in those options you care about
    /// LLVMCreateMCJITCompilerForModule(&jit, mod, &options, sizeof(options),
    ///                                  &error);
    /// ```
    ///
    /// Note that this is also correct, though possibly suboptimal:
    ///
    /// ```c++
    /// LLVMCreateMCJITCompilerForModule(&jit, mod, 0, 0, &error);
    /// ```
    ///
    /// 0 is returned on success, or 1 on failure.
    pub fn LLVMCreateMCJITCompilerForModule(
        OutJIT: *mut LLVMExecutionEngineRef,
        M: LLVMModuleRef,
        Options: *mut LLVMMCJITCompilerOptions,
        SizeOfOptions: ::libc::size_t,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    pub fn LLVMDisposeExecutionEngine(EE: LLVMExecutionEngineRef);
    pub fn LLVMRunStaticConstructors(EE: LLVMExecutionEngineRef);
    pub fn LLVMRunStaticDestructors(EE: LLVMExecutionEngineRef);
    pub fn LLVMRunFunctionAsMain(
        EE: LLVMExecutionEngineRef,
        F: LLVMValueRef,
        ArgC: ::libc::c_uint,
        ArgV: *const *const ::libc::c_char,
        EnvP: *const *const ::libc::c_char,
    ) -> ::libc::c_int;
    pub fn LLVMRunFunction(
        EE: LLVMExecutionEngineRef,
        F: LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Args: *mut LLVMGenericValueRef,
    ) -> LLVMGenericValueRef;
    pub fn LLVMFreeMachineCodeForFunction(EE: LLVMExecutionEngineRef, F: LLVMValueRef);
    pub fn LLVMAddModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef);
    pub fn LLVMRemoveModule(
        EE: LLVMExecutionEngineRef,
        M: LLVMModuleRef,
        OutMod: *mut LLVMModuleRef,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMFindFunction(
        EE: LLVMExecutionEngineRef,
        Name: *const ::libc::c_char,
        OutFn: *mut LLVMValueRef,
    ) -> LLVMBool;
    pub fn LLVMRecompileAndRelinkFunction(
        EE: LLVMExecutionEngineRef,
        Fn: LLVMValueRef,
    ) -> *mut ::libc::c_void;
    pub fn LLVMGetExecutionEngineTargetData(EE: LLVMExecutionEngineRef) -> LLVMTargetDataRef;
    pub fn LLVMGetExecutionEngineTargetMachine(EE: LLVMExecutionEngineRef) -> LLVMTargetMachineRef;
    pub fn LLVMAddGlobalMapping(
        EE: LLVMExecutionEngineRef,
        Global: LLVMValueRef,
        Addr: *mut ::libc::c_void,
    );
    pub fn LLVMGetPointerToGlobal(
        EE: LLVMExecutionEngineRef,
        Global: LLVMValueRef,
    ) -> *mut ::libc::c_void;
    pub fn LLVMGetGlobalValueAddress(
        EE: LLVMExecutionEngineRef,
        Name: *const ::libc::c_char,
    ) -> u64;
    pub fn LLVMGetFunctionAddress(EE: LLVMExecutionEngineRef, Name: *const ::libc::c_char) -> u64;

    pub fn LLVMExecutionEngineGetErrMsg(
        EE: LLVMExecutionEngineRef,
        OutError: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    // Operations on memory managers
    // Create a simple custom MCJIT memory manager.
    //
    // This memory manager can intercept allocations in a module-oblivious way. It will
    // return NULL if any of the passed functions are NULL.
    //
    // `AllocateCodeSection` and `AllocateDataSection` are called to allocate blocks
    // of memory for executable code and data, respectively. `FinalizeMemory` is called
    // to set page permissions and flush caches, returning 0 on success and 1 on error.
    //
    // `Opaque` will be passed to the callbacks.
    pub fn LLVMCreateSimpleMCJITMemoryManager(
        Opaque: *mut ::libc::c_void,
        AllocateCodeSection: LLVMMemoryManagerAllocateCodeSectionCallback,
        AllocateDataSection: LLVMMemoryManagerAllocateDataSectionCallback,
        FinalizeMemory: LLVMMemoryManagerFinalizeMemoryCallback,
        Destroy: LLVMMemoryManagerDestroyCallback,
    ) -> LLVMMCJITMemoryManagerRef;

    pub fn LLVMDisposeMCJITMemoryManager(MM: LLVMMCJITMemoryManagerRef);

    // JIT event listener functions
    pub fn LLVMCreateGDBRegistrationListener() -> LLVMJITEventListenerRef;
    pub fn LLVMCreateIntelJITEventListener() -> LLVMJITEventListenerRef;
    pub fn LLVMCreateOProfileJITEventListener() -> LLVMJITEventListenerRef;
    pub fn LLVMCreatePerfJITEventListener() -> LLVMJITEventListenerRef;

}
