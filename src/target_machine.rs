//! Target machine information, to generate assembly or object files.

use super::prelude::*;
use super::target::LLVMTargetDataRef;

#[derive(Debug)]
pub enum LLVMOpaqueTargetMachine {}

pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;

#[derive(Debug)]
pub enum LLVMOpaqueTargetMachineOptions {}

pub type LLVMTargetMachineOptionsRef = *mut LLVMOpaqueTargetMachineOptions;

#[derive(Debug)]
pub enum LLVMTarget {}

pub type LLVMTargetRef = *mut LLVMTarget;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeGenOptLevel {
    LLVMCodeGenLevelNone = 0,
    LLVMCodeGenLevelLess = 1,
    LLVMCodeGenLevelDefault = 2,
    LLVMCodeGenLevelAggressive = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMRelocMode {
    LLVMRelocDefault = 0,
    LLVMRelocStatic = 1,
    LLVMRelocPIC = 2,
    LLVMRelocDynamicNoPic = 3,
    LLVMRelocROPI = 4,
    LLVMRelocRWPI = 5,
    LLVMRelocROPI_RWPI = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeModel {
    LLVMCodeModelDefault = 0,
    LLVMCodeModelJITDefault = 1,
    LLVMCodeModelTiny = 2,
    LLVMCodeModelSmall = 3,
    LLVMCodeModelKernel = 4,
    LLVMCodeModelMedium = 5,
    LLVMCodeModelLarge = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeGenFileType {
    LLVMAssemblyFile = 0,
    LLVMObjectFile = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMGlobalISelAbortMode {
    LLVMGlobalISelAbortEnable,
    LLVMGlobalISelAbortDisable,
    LLVMGlobalISelAbortDisableWithDiag,
}

extern "C" {
    pub fn LLVMGetFirstTarget() -> LLVMTargetRef;
    pub fn LLVMGetNextTarget(T: LLVMTargetRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromName(Name: *const ::libc::c_char) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromTriple(
        Triple: *const ::libc::c_char,
        T: *mut LLVMTargetRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMGetTargetName(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMGetTargetDescription(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMTargetHasJIT(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasTargetMachine(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasAsmBackend(T: LLVMTargetRef) -> LLVMBool;

    /// Create a new set of options for an llvm::TargetMachine.
    ///
    /// The returned option structure must be released with
    /// LLVMDisposeTargetMachineOptions() after the call to
    /// LLVMCreateTargetMachineWithOptions().
    pub fn LLVMCreateTargetMachineOptions() -> LLVMTargetMachineOptionsRef;
    /// Dispose of an LLVMTargetMachineOptionsRef instance.
    pub fn LLVMDisposeTargetMachineOptions(Options: LLVMTargetMachineOptionsRef);
    pub fn LLVMTargetMachineOptionsSetCPU(
        Options: LLVMTargetMachineOptionsRef,
        CPU: *const ::libc::c_char,
    );
    /// Set the list of features for the target machine.
    ///
    /// `Features` is a comma-separated list of features.
    pub fn LLVMTargetMachineOptionsSetFeatures(
        Options: LLVMTargetMachineOptionsRef,
        Features: *const ::libc::c_char,
    );
    pub fn LLVMTargetMachineOptionsSetABI(
        Options: LLVMTargetMachineOptionsRef,
        ABI: *const ::libc::c_char,
    );
    pub fn LLVMTargetMachineOptionsSetCodeGenOptLevel(
        Options: LLVMTargetMachineOptionsRef,
        Level: LLVMCodeGenOptLevel,
    );
    pub fn LLVMTargetMachineOptionsSetRelocMode(
        Options: LLVMTargetMachineOptionsRef,
        Reloc: LLVMRelocMode,
    );
    pub fn LLVMTargetMachineOptionsSetCodeModel(
        Options: LLVMTargetMachineOptionsRef,
        CodeModel: LLVMCodeModel,
    );
    pub fn LLVMCreateTargetMachineWithOptions(
        T: LLVMTargetRef,
        Triple: *const ::libc::c_char,
        Options: LLVMTargetMachineOptionsRef,
    ) -> LLVMTargetMachineRef;

    pub fn LLVMCreateTargetMachine(
        T: LLVMTargetRef,
        Triple: *const ::libc::c_char,
        CPU: *const ::libc::c_char,
        Features: *const ::libc::c_char,
        Level: LLVMCodeGenOptLevel,
        Reloc: LLVMRelocMode,
        CodeModel: LLVMCodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);
    pub fn LLVMGetTargetMachineTarget(T: LLVMTargetMachineRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetMachineTriple(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineCPU(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineFeatureString(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    /// Create a DataLayout based on the target machine.
    pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;
    pub fn LLVMSetTargetMachineAsmVerbosity(T: LLVMTargetMachineRef, VerboseAsm: LLVMBool);

    /// Enable fast-path instruction selection.
    pub fn LLVMSetTargetMachineFastISel(T: LLVMTargetMachineRef, Enable: LLVMBool);
    /// Enable global instruction selection.
    pub fn LLVMSetTargetMachineGlobalISel(T: LLVMTargetMachineRef, Enable: LLVMBool);
    /// Set abort behaviour when global instruction selection fails to lower/select an instruction.
    pub fn LLVMSetTargetMachineGlobalISelAbort(
        T: LLVMTargetMachineRef,
        Mode: LLVMGlobalISelAbortMode,
    );
    /// Enable the MachineOutliner pass.
    pub fn LLVMSetTargetMachineMachineOutliner(T: LLVMTargetMachineRef, Enable: LLVMBool);

    pub fn LLVMTargetMachineEmitToFile(
        T: LLVMTargetMachineRef,
        M: LLVMModuleRef,
        Filename: *const ::libc::c_char,
        codegen: LLVMCodeGenFileType,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMTargetMachineEmitToMemoryBuffer(
        T: LLVMTargetMachineRef,
        M: LLVMModuleRef,
        codegen: LLVMCodeGenFileType,
        ErrorMessage: *mut *mut ::libc::c_char,
        OutMemBuf: *mut LLVMMemoryBufferRef,
    ) -> LLVMBool;

    pub fn LLVMGetDefaultTargetTriple() -> *mut ::libc::c_char;
    /// Normalize a target triple. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMNormalizeTargetTriple(triple: *const ::libc::c_char) -> *mut ::libc::c_char;
    /// Get the host CPU as a string. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMGetHostCPUName() -> *mut ::libc::c_char;
    /// Get the host CPU's features as a string. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMGetHostCPUFeatures() -> *mut ::libc::c_char;

    pub fn LLVMAddAnalysisPasses(T: LLVMTargetMachineRef, PM: LLVMPassManagerRef);
}
