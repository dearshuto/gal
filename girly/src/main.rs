use std::borrow::Cow;

use winit::{
    application::ApplicationHandler,
    raw_window_handle::{self, HasDisplayHandle, HasWindowHandle},
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    event_loop.run_app(&mut App::new()).unwrap();
}

struct Graphics {
    entry: ash::Entry,
    instance: ash::Instance,

    // デバッグ
    debug_util_instance: ash::ext::debug_utils::Instance,
    debug_utils_messenger: ash::vk::DebugUtilsMessengerEXT,

    // サーフェイス
    surface_instance: ash::khr::surface::Instance,
    surface: ash::vk::SurfaceKHR,
}

struct App {
    window: Option<winit::window::Window>,
    graphics: Option<Graphics>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            graphics: None,
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(graphics) = &self.graphics {
            unsafe {
                graphics
                    .surface_instance
                    .destroy_surface(graphics.surface, None)
            }

            unsafe {
                graphics
                    .debug_util_instance
                    .destroy_debug_utils_messenger(graphics.debug_utils_messenger, None)
            }

            unsafe { graphics.instance.destroy_instance(None) }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default();
        let window = event_loop.create_window(window_attributes).unwrap();

        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let raw_display_handle = window.display_handle().unwrap().as_raw();

        let entry = ash::Entry::linked();
        let instance = {
            let app_info = ash::vk::ApplicationInfo::default()
                .application_name(c"Girly")
                .engine_name(c"Girly")
                .application_version(0)
                .api_version(ash::vk::API_VERSION_1_3);

            let extension_names: Vec<_> = [
                ash::ext::debug_utils::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::get_physical_device_properties2::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::portability_enumeration::NAME.as_ptr(),
            ]
            .iter()
            .chain(ash_window::enumerate_required_extensions(raw_display_handle).unwrap())
            .map(|x| *x)
            .collect();

            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                ash::vk::InstanceCreateFlags::default()
            };

            let layer_names = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
            let create_info = ash::vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&layer_names)
                .enabled_extension_names(&extension_names)
                .flags(create_flags);

            unsafe { entry.create_instance(&create_info, None) }.unwrap()
        };

        // デバッグ
        let debug_util_instance = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let debug_utils_messenger = unsafe {
            let create_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
                )
                .message_type(
                    ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));
            debug_util_instance.create_debug_utils_messenger(&create_info, None)
        }
        .unwrap();

        // サーフェイス
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                raw_display_handle,
                raw_window_handle,
                None,
            )
        }
        .unwrap();
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);

        // 物理デバイスの検索
        let (_physical_device, _queue_family_index) =
            unsafe { instance.enumerate_physical_devices() }
                .unwrap()
                .iter()
                .find_map(|physical_device| {
                    unsafe {
                        instance.get_physical_device_queue_family_properties(*physical_device)
                    }
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        if !info.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) {
                            return None;
                        }

                        if !unsafe {
                            surface_instance.get_physical_device_surface_support(
                                *physical_device,
                                index as u32,
                                surface,
                            )
                        }
                        .unwrap()
                        {
                            return None;
                        }

                        Some((*physical_device, index))
                    })
                })
                .unwrap();

        self.graphics = Some(Graphics {
            entry,
            instance,
            debug_util_instance,
            debug_utils_messenger,
            surface_instance,
            surface,
        });

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let winit::event::WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    ash::vk::FALSE
}
