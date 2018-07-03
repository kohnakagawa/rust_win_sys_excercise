#![allow(bad_style)]

use winapi::shared::ntdef::{ULONG, USHORT, PVOID};
use winapi::shared::ntdef::{NTSTATUS, HANDLE};
use winapi::shared::ntdef::{UNICODE_STRING};
use winapi::um::winnt::{ACCESS_MASK, GENERIC_MAPPING, BOOLEAN};
use winapi::shared::minwindef::{BYTE};
use winapi::shared::ntstatus::{STATUS_SUCCESS};

use std::ptr::null_mut;

pub const STATUS_INFO_LENGTH_MISMATCH: ULONG = 0xC0000004;

ENUM!{enum POOL_TYPE {
    NonPagedPool,
    PagedPool,
    NonPagedPoolMustSucceed,
    DontUseThisType,
    NonPagedPoolCacheAligned,
    PagedPoolCacheAligned,
    NonPagedPoolCacheAlignedMustS,
}}

#[repr(ULONG)] // underlying type is set to ULONG
ENUM!{enum SYSTEM_INFORMATION_CLASS {
    SystemBasicInformation = 0,
    SystemProcessorInformation = 1,
    SystemPerformanceInformation = 2,
    SystemTimeOfDayInformation = 3,
    SystemPathInformation = 4,
    SystemProcessInformation = 5,
    SystemCallCountInformation = 6,
    SystemDeviceInformation = 7,
    SystemProcessorPerformanceInformation = 8,
    SystemFlagsInformation = 9,
    SystemCallTimeInformation = 10,
    SystemModuleInformation = 11,
    SystemLocksInformation = 12,
    SystemStackTraceInformation = 13,
    SystemPagedPoolInformation = 14,
    SystemNonPagedPoolInformation = 15,
    SystemHandleInformation = 16,
    SystemObjectInformation = 17,
    SystemPageFileInformation = 18,
    SystemVdmInstemulInformation = 19,
    SystemVdmBopInformation = 20,
    SystemFileCacheInformation = 21,
    SystemPoolTagInformation = 22,
    SystemInterruptInformation = 23,
    SystemDpcBehaviorInformation = 24,
    SystemFullMemoryInformation = 25,
    SystemLoadGdiDriverInformation = 26,
    SystemUnloadGdiDriverInformation = 27,
    SystemTimeAdjustmentInformation = 28,
    SystemSummaryMemoryInformation = 29,
    SystemMirrorMemoryInformation = 30,
    SystemPerformanceTraceInformation = 31,
    SystemObsolete0 = 32,
    SystemExceptionInformation = 33,
    SystemCrashDumpStateInformation = 34,
    SystemKernelDebuggerInformation = 35,
    SystemContextSwitchInformation = 36,
    SystemRegistryQuotaInformation = 37,
    SystemExtendServiceTableInformation = 38,
    SystemPrioritySeperation = 39,
    SystemVerifierAddDriverInformation = 40,
    SystemVerifierRemoveDriverInformation = 41,
    SystemProcessorIdleInformation = 42,
    SystemLegacyDriverInformation = 43,
    SystemCurrentTimeZoneInformation = 44,
    SystemLookasideInformation = 45,
    SystemTimeSlipNotification = 46,
    SystemSessionCreate = 47,
    SystemSessionDetach = 48,
    SystemSessionInformation = 49,
    SystemRangeStartInformation = 50,
    SystemVerifierInformation = 51,
    SystemVerifierThunkExtend = 52,
    SystemSessionProcessInformation = 53,
    SystemLoadGdiDriverInSystemSpace = 54,
    SystemNumaProcessorMap = 55,
    SystemPrefetcherInformation = 56,
    SystemExtendedProcessInformation = 57,
    SystemRecommendedSharedDataAlignment = 58,
    SystemComPlusPackage = 59,
    SystemNumaAvailableMemory = 60,
    SystemProcessorPowerInformation = 61,
    SystemEmulationBasicInformation = 62,
    SystemEmulationProcessorInformation = 63,
    SystemExtendedHandleInformation = 64,
    SystemLostDelayedWriteInformation = 65,
    SystemBigPoolInformation = 66,
    SystemSessionPoolTagInformation = 67,
    SystemSessionMappedViewInformation = 68,
    SystemHotpatchInformation = 69,
    SystemObjectSecurityMode = 70,
    SystemWatchdogTimerHandler = 71,
    SystemWatchdogTimerInformation = 72,
    SystemLogicalProcessorInformation = 73,
    SystemWow64SharedInformation = 74,
    SystemRegisterFirmwareTableInformationHandler = 75,
    SystemFirmwareTableInformation = 76,
    SystemModuleInformationEx = 77,
    SystemVerifierTriageInformation = 78,
    SystemSuperfetchInformation = 79,
    SystemMemoryListInformation = 80,
    SystemFileCacheInformationEx = 81,
    MaxSystemInfoClass = 82,
}}

#[repr(ULONG)]
ENUM!{enum OBJECT_INFORMATION_CLASS {
    ObjectBasicInformation,
    ObjectNameInformation,
    ObjectTypeInformation,
    ObjectAllInformation,
    ObjectDataInformation,
}}

STRUCT!{struct OBJECT_TYPE_INFORMATION {
    Name: UNICODE_STRING,
    TotalNumberOfObjects: ULONG,
    TotalNumberOfHandles: ULONG,
    TotalPagedPoolUsage: ULONG,
    TotalNonPagedPoolUsage: ULONG,
    TotalNamePoolUsage: ULONG,
    TotalHandleTableUsage: ULONG,
    HighWaterNumberOfObjects: ULONG,
    HighWaterNumberOfHandles: ULONG,
    HighWaterPagedPoolUsage: ULONG,
    HighWaterNonPagedPoolUsage: ULONG,
    HighWaterNamePoolUsage: ULONG,
    HighWaterHandleTableUsage: ULONG,
    InvalidAttributes: ULONG,
    GenericMapping: GENERIC_MAPPING,
    ValidAccess: ULONG,
    SecurityRequired: BOOLEAN,
    MaintainHandleCount: BOOLEAN,
    MaintainTypeList: USHORT,
    PoolType: POOL_TYPE,
    PagedPoolUsage: ULONG,
    NonPagedPoolUsage: ULONG,
}}

STRUCT!{struct SYSTEM_HANDLE {
    ProcessId: ULONG,
    ObjectTypeNumber: BYTE,
    Flags: BYTE,
    Handle: USHORT,
    Object: PVOID,
    GrantedAccess: ACCESS_MASK,
}}

STRUCT!{struct SYSTEM_HANDLE_INFORMATION {
    HandleCount: ULONG,
    Handles: [SYSTEM_HANDLE; 1],
}}

#[link(name = "ntdll")]
extern "system" { // NOTE: "system" により呼び出し規約がstdcallに
    pub fn NtQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: *mut ULONG
    ) -> NTSTATUS;

    pub fn NtDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        Attributes: ULONG,
        Options: ULONG
    ) -> NTSTATUS;

    pub fn NtQueryObject(
        ObjectHandle: HANDLE,
        ObjectInformationCLass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ReturnLength: *mut ULONG
    ) -> NTSTATUS;
}

pub struct SYSTEM_HANDLE_HOLDER {
    buffer: Vec<u8>,
}

impl SYSTEM_HANDLE_HOLDER {
    pub fn new() -> SYSTEM_HANDLE_HOLDER {
        SYSTEM_HANDLE_HOLDER {
            buffer: vec![0;0x1000],
        }
    }

    fn expand_buffer(&mut self, new_size: usize) {
        self.buffer.resize(new_size, 0);
    }

    fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    pub fn raw_ptr(&self) -> *const SYSTEM_HANDLE_INFORMATION {
        self.buffer.as_slice().as_ptr() as *const SYSTEM_HANDLE_INFORMATION
    }

    pub fn raw_mut_ptr(&mut self) -> *mut SYSTEM_HANDLE_INFORMATION {
        self.buffer.as_mut_slice().as_mut_ptr() as *mut SYSTEM_HANDLE_INFORMATION
    }

    pub fn query_system_handle(&mut self) -> Result<(), NTSTATUS> {
        unsafe {
            let mut info_size: ULONG = 0;
            NtQuerySystemInformation(
                SystemHandleInformation,
                self.raw_mut_ptr() as *mut _ as PVOID,
                self.buffer_size() as ULONG,
                &mut info_size
            );
            self.expand_buffer(info_size as usize);

            let err_code = NtQuerySystemInformation(
                SystemHandleInformation,
                self.raw_mut_ptr() as *mut _ as PVOID,
                self.buffer_size() as ULONG,
                null_mut()
            );
            match err_code {
                STATUS_SUCCESS => { Ok(()) },
                _ => { Err(err_code) },
            }
        }
    }
}