use com::{ComInterface, ComPtr, IUnknown};

use winapi::shared::guiddef::IID;

pub const IID_IEXAMPLE: IID = IID {
    Data1: 0xC5F45CBC,
    Data2: 0x4439,
    Data3: 0x418C,
    Data4: [0xA9, 0xF9, 0x05, 0xAC, 0x67, 0x52, 0x5E, 0x43],
};

pub trait IExample: IUnknown {}

unsafe impl ComInterface for dyn IExample {
    type VTable = IExampleVTable;
    const IID: IID = IID_IEXAMPLE;
}

pub type IExampleVPtr = *const IExampleVTable;

impl<T: IExample + ComInterface + ?Sized> IExample for ComPtr<T> {}

#[repr(C)]
pub struct IExampleVTable {
    pub base: <dyn IUnknown as ComInterface>::VTable,
}
