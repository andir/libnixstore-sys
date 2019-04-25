use std::ffi::{CString, CStr};

pub enum Error {
//    StringConversionFailed,
    NullPtr
}

impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Error {
        return Error::NullPtr;
    }
}

type Result<T> = std::result::Result<T, Error>;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct PathInfo {
    path: String,
    deriver: String,
    nar_hash: String,
    references: String,
    registration_time: i64,
    nar_size: u64,
    ultimate: bool,
    signatures: String,
    ca: String,
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

pub fn init() {
    unsafe {
        ffi::libnixstorec_init();
    }
}

pub fn is_valid_path<T: AsRef<str>>(path: T) -> Result<bool> {
    let path = std::ffi::CString::new(path.as_ref())?;

    unsafe {
        let c_path = path.as_ptr();
        let ret : bool = ffi::libnixstorec_is_valid_path(c_path) != 0;
        return Ok(ret);
    }
}

pub fn query_path_info<T: AsRef<str>>(path: T) -> Result<PathInfo> {
    let path : CString = std::ffi::CString::new(path.as_ref())?;
    let c_path = path.as_ptr();

    let c_pathinfo_ptr = unsafe {
        ffi::libnixstorec_query_path_info(c_path)
    };

    if c_pathinfo_ptr == 0 as _ {
        return Err(Error::NullPtr);
    }

    let pathinfo = {
        let c_pathinfo = unsafe { *c_pathinfo_ptr };
        PathInfo::from_cpathinfo(&c_pathinfo)
    };

    unsafe { ffi::libnixstorec_free_path_info(c_pathinfo_ptr); }

    return Ok(pathinfo);
}

pub fn query_path_from_hash_part<T: AsRef<str>>(hash_part: T) -> Result<String> {

    let hash_path_c = CString::new(hash_part.as_ref())?;

    let path_c = unsafe { ffi::libnixstorec_query_path_from_hash_part(hash_path_c.as_ptr()) };

    if path_c == 0 as _ {
        return Err(Error::NullPtr);
    }

    let path = unsafe {
        CStr::from_ptr(path_c).to_string_lossy().to_string()
    };

    unsafe { ffi::libnixstorec_free(path_c as _); }

    Ok(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
