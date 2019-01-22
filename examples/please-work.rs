#![allow(non_snake_case)]

extern crate vkffi;

use std::ptr;
use std::ffi::{CString};

use vkffi::*;

fn main() {
    let application_info = VkApplicationInfo {
        sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: ptr::null(),
        pApplicationName: CString::new("my app").unwrap().as_ptr(),
        applicationVersion: version(1, 0, 0),
        pEngineName: CString::new("some engine").unwrap().as_ptr(),
        engineVersion: version(0, 0, 0),
        apiVersion: version(1, 0, 3)
    };

    let instance_create_info = VkInstanceCreateInfo {
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        pApplicationInfo: &application_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: ptr::null()
    };

    unsafe {
        let mut instance = 0;
        let result = vkCreateInstance(&instance_create_info, ptr::null(), &mut instance);
        println!("got result: {:#?}", result);
    }
}