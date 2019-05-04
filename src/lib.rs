use std::ffi::{CString, CStr};

#[derive(Debug)]
pub enum Error {
//    StringConversionFailed,
    FfiNullPtr(std::ffi::NulError),
    NullPtr,
}

impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Error {
        return Error::FfiNullPtr(other);
    }
}

type Result<T> = std::result::Result<T, Error>;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub struct PathInfo {
    pub path: String,
    pub deriver: String,
    pub nar_hash: String,
    pub references: String,
    pub registration_time: i64,
    pub nar_size: u64,
    pub ultimate: bool,
    pub signatures: String,
    pub ca: String,
}

impl PathInfo {
    fn from_cpathinfo(cpi: &ffi::CPathInfo) -> Self {
        Self {
            path: unsafe { CStr::from_ptr(cpi.path).to_string_lossy().to_string() },
            deriver: unsafe { CStr::from_ptr(cpi.deriver).to_string_lossy().to_string() },
            nar_hash: unsafe { CStr::from_ptr(cpi.narHash).to_string_lossy().to_string() },
            references: unsafe { CStr::from_ptr(cpi.references).to_string_lossy().to_string() },
            registration_time: cpi.registrationTime as i64,
            nar_size: cpi.narSize,
            ultimate: cpi.ultimate != 0,
            signatures: unsafe { CStr::from_ptr(cpi.signatures).to_string_lossy().to_string() },
            ca: unsafe { CStr::from_ptr(cpi.ca).to_string_lossy().to_string() },
        }
    }
}

#[derive(Debug)]
pub struct Instance {
    instp: *mut ffi::nixstorec_instance,
}

unsafe impl Send for Instance {}
impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { ffi::nixstorec_free_instance(self.instp) };
    }
}

impl Instance {
    pub fn new() -> Result<Instance> {
        let instp = unsafe {
            ffi::nixstorec_new_instance()
        };

        if instp == 0 as _ {
            return Err(Error::NullPtr);
        }

        Ok(Self{
            instp
        })
    }

    pub fn is_valid_path<T: AsRef<str>>(&mut self, path: T) -> Result<bool> {
        let path = std::ffi::CString::new(path.as_ref())?;

        let c_path = path.as_ptr();

        unsafe {
            let ret : bool = ffi::nixstorec_is_valid_path(self.instp, c_path) != 0;
            return Ok(ret);
        }
    }

    pub fn query_path_info<T: AsRef<str>>(&mut self, path: T) -> Result<Option<PathInfo>> {
        let path : CString = std::ffi::CString::new(path.as_ref())?;
        let c_path = path.as_ptr();

        let c_pathinfo_ptr = unsafe {
            ffi::nixstorec_query_path_info(self.instp, c_path)
        };

        if c_pathinfo_ptr == 0 as _ {
            return Ok(None);
        }

        let pathinfo = {
            let c_pathinfo = unsafe { *c_pathinfo_ptr };
            PathInfo::from_cpathinfo(&c_pathinfo)
        };

        unsafe { ffi::nixstorec_free_path_info(c_pathinfo_ptr); }

        return Ok(Some(pathinfo));
    }

    pub fn query_path_from_hash_part<T: AsRef<str>>(&mut self, hash_part: T) -> Result<Option<String>> {

        let hash_path_c = CString::new(hash_part.as_ref())?;

        let path_c = unsafe { ffi::nixstorec_query_path_from_hash_part(self.instp, hash_path_c.as_ptr()) };

        if path_c == 0 as _ {
            return Err(Error::NullPtr);
        }

        let path = unsafe {
            CStr::from_ptr(path_c).to_string_lossy().to_string()
        };

        unsafe { ffi::nixstorec_free(path_c as _); }

        if path.is_empty() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    pub fn query_path_from_file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> Result<Option<String>> {

        let file_hash_c = CString::new(file_hash.as_ref())?;

        let path_c = unsafe { ffi::nixstorec_query_path_from_file_hash(self.instp, file_hash_c.as_ptr()) };

        if path_c == 0 as _ {
            return Err(Error::NullPtr);
        }

        let path = unsafe {
            CStr::from_ptr(path_c).to_string_lossy().to_string()
        };

        let ptr = path_c as _;

        unsafe {
            ffi::nixstorec_free(ptr);
        }

        if path.is_empty() {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
//     fn query_path_from_nar_hash() {
//         let v = super::query_path_from_nar_hash("fooooo");
//         println!("{:?}", v);
//         assert!(v.is_ok());
//     }
//
//     #[test]
//     fn query_path_from_hash_part() {
//         let v = super::query_path_from_hash_part("foooooooooo");
//         println!("{:?}", v);
//         assert!(v.is_ok());
//     }

    #[test]
    fn query_path_info() {
        for i in 0..100 {
            let mut instance = super::Instance::new().unwrap();
            let v = instance.query_path_info("testtesttest");
            println!("{:?}", v);
            assert!(v.unwrap().is_none())
        }
    }
}
