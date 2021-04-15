#![cfg(target_os="android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};

#[no_mangle]
pub unsafe extern fn Java_rsonnicdebris_usty_droid_MainActivity_hello(
    env: JNIEnv, _: JObject, j_recipient: JString
) -> jstring {

    let recipient = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );

    let output = env.new_string("Ciao ".to_owned() + recipient.to_str().unwrap()).unwrap();
    output.into_inner()
}