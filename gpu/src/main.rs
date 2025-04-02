use anyhow::Result;
use ash::{Entry, vk};
use std::ffi::CString;

const REQUIRED_INSTANCE_EXTENSIONS: &[&str] = &["VK_KHR_portability_enumeration"];
const REQUIRED_DEVICE_EXTENSIONS: &[&str] =
    &["VK_KHR_8bit_storage", "VK_KHR_storage_buffer_storage_class"];

fn main() -> Result<()> {
    env_logger::init();

    let entry = unsafe { Entry::load()? };
    let instance_extensions = unsafe { entry.enumerate_instance_extension_properties(None)? };
    let instance_extension_names = instance_extensions
        .iter()
        .filter_map(|ext| {
            if let Ok(name) = ext.extension_name_as_c_str() {
                if REQUIRED_INSTANCE_EXTENSIONS.contains(&name.to_str().unwrap()) {
                    return Some(name.as_ptr());
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let app_name = CString::new("Compute Shader Example")?;
    let app_info = vk::ApplicationInfo::default()
        .application_name(&app_name)
        .api_version(vk::make_api_version(0, 1, 2, 0));

    let flags = vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(instance_extension_names.as_slice())
        .flags(flags);
    let instance = unsafe { entry.create_instance(&create_info, None)? };

    let devices = unsafe { instance.enumerate_physical_devices()? };
    if devices.len() < 1 {
        panic!("No Vulkan devices found");
    }
    let device_extensions = unsafe { instance.enumerate_device_extension_properties(devices[0])? };
    let device_extension_names = device_extensions
        .iter()
        .filter_map(|ext| {
            if let Ok(name) = ext.extension_name_as_c_str() {
                if REQUIRED_DEVICE_EXTENSIONS.contains(&name.to_str().unwrap()) {
                    return Some(name.as_ptr());
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let shader_bytes: &[u8] = include_bytes!(env!("shader.spv"));
    let shader_module_create_info = vk::ShaderModuleCreateInfo::default().code(shader_bytes);

    println!("Shader module loaded successfully.");

    // unsafe {
    //     instance.destroy_shader_module(shader_module, None);
    //     instance.destroy_instance(None);
    // }

    Ok(())
}
