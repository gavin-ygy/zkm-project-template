extern crate libc;
use libc::c_int;
use std::os::raw::c_char;
use std::path::Path;
use anyhow::Context;

extern "C" {
    fn Stark2Snark(
        keypath: *const c_char,
        inputdir: *const c_char,
        outputdir: *const c_char,
         result: *mut *mut libc::c_char
    ) -> c_int;
    fn SetupAndGenerateSolVerifier(inputdir: *const c_char,  result: *mut *mut libc::c_char) -> c_int;
}

#[cfg(feature = "snark")]
pub fn prove_snark(keypath: &str, inputdir: &str, outputdir: &str) -> anyhow::Result<()> {
    let path = Path::new(keypath);
    let pk_file = path.join("proving.key");
    let vk_file = path.join("verifying.key");

    if !pk_file.exists() || !vk_file.exists() {
        panic!(
            "The vk or pk doesn't exist in the path: {}. Please first set the SETUP_FLAG=true to run setup_and_generate_sol_verifier.",inputdir
        );
    }

    let keypath = std::ffi::CString::new(keypath).unwrap();
    let inputdir = std::ffi::CString::new(inputdir).unwrap();
    let outputdir = std::ffi::CString::new(outputdir).unwrap();

    let mut result: *mut libc::c_char = std::ptr::null_mut();

    let ret = unsafe { Stark2Snark(keypath.as_ptr(), inputdir.as_ptr(), outputdir.as_ptr(), &mut result,) };
    if ret == 0 {
        Ok(())
    } else {
        let error_str = std::ffi::CStr::from_ptr(result).to_string_lossy();
        // Free the allocated C string
        libc::free(result as *mut libc::c_void);
        //Ok(false)
        bail!("prove_snark error: {}", error_str)
    }
}

#[cfg(not(feature = "snark"))]
pub fn prove_snark(keypath: &str, inputdir: &str, outputdir: &str) -> anyhow::Result<()> {
    panic!("not support snark");
}

#[cfg(feature = "snark")]
pub fn setup_and_generate_sol_verifier(inputdir: &str, &mut result,) -> anyhow::Result<()> {
    let inputdir = std::ffi::CString::new(inputdir).unwrap();

    let ret = unsafe { SetupAndGenerateSolVerifier(inputdir.as_ptr()) };
    if ret == 0 {
        Ok(true)
    } else {
        let error_str = std::ffi::CStr::from_ptr(result).to_string_lossy();
        // Free the allocated C string
        libc::free(result as *mut libc::c_void);
        //Ok(false)
        bail!("prove_snark error: {}", error_str)
    }
}

#[cfg(not(feature = "snark"))]
pub fn setup_and_generate_sol_verifier(inputdir: &str) -> anyhow::Result<()> {
    panic!("not support setup");
}
