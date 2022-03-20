use std::{
    ffi::{
        c_void,
        CString,
    },
    os::raw::{
        c_char,
    },
    mem::transmute_copy,
    ptr::self,
};

type FarProc = ptr::NonNull<c_void>;
type HModule = ptr::NonNull<c_void>;

extern "stdcall" {
    fn LoadLibraryA(name: *const c_char) -> Option<HModule>;
    fn GetProcAddress(module: HModule, name: *const c_char) -> Option<FarProc>;
}

#[derive(Debug)]
pub struct Library {
    module: HModule,
}

#[derive(Debug, PartialEq)]
pub enum LoadLibErr {
    LibNotFound,
    InvalidLibName,
}

#[derive(Debug, PartialEq)]
pub enum GetProcErr {
    ProcNotFound,
    InvalidProcName,
}

impl Library {
    pub fn new(name: &str) -> Result<Self, LoadLibErr> {
        let c_name = CString::new(name).or(Err(LoadLibErr::InvalidLibName))?;
        let l = unsafe { LoadLibraryA(c_name.as_ptr()) };

        l.map(|module| Library { module })
         .ok_or(LoadLibErr::LibNotFound)
    }

    pub fn get_proc<T>(&self, name: &str) -> Result<T, GetProcErr> {
        let c_name = CString::new(name).or(Err(GetProcErr::InvalidProcName))?;
        let proc = unsafe { GetProcAddress(self.module, c_name.as_ptr()) };

        proc.map(|p| unsafe { transmute_copy(&p) })
            .ok_or(GetProcErr::ProcNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DLL_NAME: &str = "IPHLPAPI.dll";

    #[test]
    fn load_lib_invalid_name() {
        let l = Library::new("name\0abcd");
        matches!(l, Err(LoadLibErr::InvalidLibName));
    }

    #[test]
    fn load_lib_non_existing_lib() {
        let l = Library::new("non_existing_lib_name");
        matches!(l, Err(LoadLibErr::LibNotFound));
    }

    #[test]
    fn load_lib_ok() {
        assert!(Library::new(TEST_DLL_NAME).is_ok());
    }

    #[test]
    fn get_proc_non_existent() {
        let l = Library::new(TEST_DLL_NAME).unwrap();
        let p = l.get_proc::<fn() -> ()>("NoSuchProcName");
        matches!(p, Err(GetProcErr::ProcNotFound));
    }
    
    #[test]
    fn get_proc_invalid_name() {
        let l = Library::new(TEST_DLL_NAME).unwrap();
        let p = l.get_proc::<fn() -> ()>("proc\0name");
        matches!(p, Err(GetProcErr::InvalidProcName));
    }

    #[test]
    fn get_proc_ok() {
        let l = Library::new("IPHLPAPI.dll").unwrap();
        let p = l.get_proc::<extern "stdcall" fn() -> *const c_void>("IcmpCreateFile");
        assert!(p.is_ok());
    }
}