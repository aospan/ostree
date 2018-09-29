// This file was generated by gir (https://github.com/gtk-rs/gir @ c385982)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Error;
use SePolicyRestoreconFlags;
use ffi;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SePolicy(Object<ffi::OstreeSePolicy>);

    match fn {
        get_type => || ffi::ostree_sepolicy_get_type(),
    }
}

impl SePolicy {
    /// ## `path`
    /// Path to a root directory
    /// ## `cancellable`
    /// Cancellable
    ///
    /// # Returns
    ///
    /// An accessor object for SELinux policy in root located at `path`
    pub fn new<'a, P: IsA<gio::File>, Q: Into<Option<&'a gio::Cancellable>>>(path: &P, cancellable: Q) -> Result<SePolicy, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sepolicy_new(path.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// ## `rootfs_dfd`
    /// Directory fd for rootfs (will not be cloned)
    /// ## `cancellable`
    /// Cancellable
    ///
    /// # Returns
    ///
    /// An accessor object for SELinux policy in root located at `rootfs_dfd`
    pub fn new_at<'a, P: Into<Option<&'a gio::Cancellable>>>(rootfs_dfd: i32, cancellable: P) -> Result<SePolicy, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_sepolicy_new_at(rootfs_dfd, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn fscreatecon_cleanup<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(unused: P) {
    //    unsafe { TODO: call ffi::ostree_sepolicy_fscreatecon_cleanup() }
    //}
}

/// Trait containing all `SePolicy` methods.
///
/// # Implementors
///
/// [`SePolicy`](struct.SePolicy.html)
pub trait SePolicyExt {
    ///
    /// # Returns
    ///
    /// Checksum of current policy
    fn get_csum(&self) -> Option<String>;

    /// Store in `out_label` the security context for the given `relpath` and
    /// mode `unix_mode`. If the policy does not specify a label, `None`
    /// will be returned.
    /// ## `relpath`
    /// Path
    /// ## `unix_mode`
    /// Unix mode
    /// ## `out_label`
    /// Return location for security context
    /// ## `cancellable`
    /// Cancellable
    fn get_label<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, relpath: &str, unix_mode: u32, cancellable: P) -> Result<String, Error>;

    ///
    /// # Returns
    ///
    /// Type of current policy
    fn get_name(&self) -> Option<String>;

    ///
    /// # Returns
    ///
    /// Path to rootfs
    fn get_path(&self) -> Option<gio::File>;

    /// Reset the security context of `target` based on the SELinux policy.
    /// ## `path`
    /// Path string to use for policy lookup
    /// ## `info`
    /// File attributes
    /// ## `target`
    /// Physical path to target file
    /// ## `flags`
    /// Flags controlling behavior
    /// ## `out_new_label`
    /// New label, or `None` if unchanged
    /// ## `cancellable`
    /// Cancellable
    fn restorecon<'a, 'b, P: Into<Option<&'a gio::FileInfo>>, Q: IsA<gio::File>, R: Into<Option<&'b gio::Cancellable>>>(&self, path: &str, info: P, target: &Q, flags: SePolicyRestoreconFlags, cancellable: R) -> Result<String, Error>;

    /// ## `path`
    /// Use this path to determine a label
    /// ## `mode`
    /// Used along with `path`
    fn setfscreatecon(&self, path: &str, mode: u32) -> Result<(), Error>;

    fn get_property_rootfs_dfd(&self) -> i32;
}

impl<O: IsA<SePolicy> + IsA<glib::object::Object>> SePolicyExt for O {
    fn get_csum(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_csum(self.to_glib_none().0))
        }
    }

    fn get_label<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, relpath: &str, unix_mode: u32, cancellable: P) -> Result<String, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut out_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sepolicy_get_label(self.to_glib_none().0, relpath.to_glib_none().0, unix_mode, &mut out_label, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_label)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_name(self.to_glib_none().0))
        }
    }

    fn get_path(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::ostree_sepolicy_get_path(self.to_glib_none().0))
        }
    }

    fn restorecon<'a, 'b, P: Into<Option<&'a gio::FileInfo>>, Q: IsA<gio::File>, R: Into<Option<&'b gio::Cancellable>>>(&self, path: &str, info: P, target: &Q, flags: SePolicyRestoreconFlags, cancellable: R) -> Result<String, Error> {
        let info = info.into();
        let info = info.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut out_new_label = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sepolicy_restorecon(self.to_glib_none().0, path.to_glib_none().0, info.0, target.to_glib_none().0, flags.to_glib(), &mut out_new_label, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(out_new_label)) } else { Err(from_glib_full(error)) }
        }
    }

    fn setfscreatecon(&self, path: &str, mode: u32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_sepolicy_setfscreatecon(self.to_glib_none().0, path.to_glib_none().0, mode, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_rootfs_dfd(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "rootfs-dfd".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }
}
