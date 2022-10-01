use crate::{
    extensions::khr_surface,
    loader::tables::InstanceTable,
    util::result::VulkanResult,
    vk::{
        EXT_METAL_SURFACE_EXTENSION_NAME, KHR_ANDROID_SURFACE_EXTENSION_NAME,
        KHR_WAYLAND_SURFACE_EXTENSION_NAME, KHR_WIN32_SURFACE_EXTENSION_NAME,
        KHR_XCB_SURFACE_EXTENSION_NAME, KHR_XLIB_SURFACE_EXTENSION_NAME,
    },
    vk10, vkcall,
};
use khr_surface::KHR_SURFACE_EXTENSION_NAME;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use std::ffi::CStr;

pub unsafe fn create_surface(
    table: &InstanceTable,
    instance: vk10::Instance,
    window: &(impl HasRawWindowHandle + HasRawDisplayHandle),
    allocation_callbacks: *const vk10::AllocationCallbacks,
) -> VulkanResult<khr_surface::SurfaceKHR> {
    use raw_window_handle::{RawDisplayHandle as Rdh, RawWindowHandle as Rwh};

    // ripped out from https://github.com/wyatt-herkamp/wgpu/blob/35b19feb3c355f59f7e79808c7f9a0d4cec78b78/wgpu-hal/src/vulkan/instance.rs#L592
    // erupt has not yet updated to raw-window-handle 0.5
    match (window.raw_window_handle(), window.raw_display_handle()) {
        (Rwh::Wayland(handle), Rdh::Wayland(display)) => {
            use crate::extensions::khr_wayland_surface::WaylandSurfaceCreateInfoKHR;

            let create_info = WaylandSurfaceCreateInfoKHR {
                display: display.display,
                surface: handle.surface,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_wayland_surface_khr.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        (Rwh::Xlib(handle), Rdh::Xlib(display)) => {
            use crate::extensions::khr_xlib_surface::XlibSurfaceCreateInfoKHR;

            let create_info = XlibSurfaceCreateInfoKHR {
                dpy: display.display,
                window: handle.window,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_xlib_surface_khr.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        (Rwh::Xcb(handle), Rdh::Xcb(display)) => {
            use crate::extensions::khr_xcb_surface::XcbSurfaceCreateInfoKHR;

            let create_info = XcbSurfaceCreateInfoKHR {
                connection: display.connection,
                window: handle.window,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_xcb_surface_khr.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        (Rwh::AndroidNdk(handle), _) => {
            use crate::extensions::khr_android_surface::AndroidSurfaceCreateInfoKHR;

            let create_info = AndroidSurfaceCreateInfoKHR {
                window: handle.a_native_window,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_android_surface_khr.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        (Rwh::Win32(handle), _) => {
            use crate::extensions::khr_win32_surface::Win32SurfaceCreateInfoKHR;

            let create_info = Win32SurfaceCreateInfoKHR {
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_win_32_surface_khr.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        #[cfg(target_os = "mac")]
        (Rwh::AppKit(handle), _) | RawWindowHandle::UiKit(handle) => {
            use crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT;
            use raw_window_metal::{appkit, Layer};

            let layer = match appkit::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer,
                Layer::None => {
                    return VulkanResult::new_err(vk10::Result::ERROR_INITIALIZATION_FAILED)
                }
            };

            let create_info = MetalSurfaceCreateInfoEXT {
                p_layer: layer,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_metal_surface_ext.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        #[cfg(target_os = "ios")]
        (Rwh::AppKit(handle), _) | RawWindowHandle::UiKit(handle) => {
            use crate::extensions::ext_metal_surface::MetalSurfaceCreateInfoEXT;
            use raw_window_metal::{appkit, Layer};

            let layer = match appkit::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer,
                Layer::None => {
                    return VulkanResult::new_err(vk10::Result::ERROR_INITIALIZATION_FAILED)
                }
            };

            let create_info = MetalSurfaceCreateInfoEXT {
                p_layer: layer,
                ..Default::default()
            };

            vkcall!(
                surface,
                (table.create_metal_surface_ext.unwrap())(
                    instance,
                    &create_info,
                    allocation_callbacks,
                    surface
                )
            )
        }
        _ => VulkanResult::new_err(vk10::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(
    window_handle: &impl HasRawWindowHandle,
) -> VulkanResult<&'static [&'static CStr]> {
    const WAYLAND: &[&CStr] = &[
        KHR_SURFACE_EXTENSION_NAME,
        KHR_WAYLAND_SURFACE_EXTENSION_NAME,
    ];
    const XLIB: &[&CStr] = &[KHR_SURFACE_EXTENSION_NAME, KHR_XLIB_SURFACE_EXTENSION_NAME];
    const XCB: &[&CStr] = &[KHR_SURFACE_EXTENSION_NAME, KHR_XCB_SURFACE_EXTENSION_NAME];
    const ANDROID: &[&CStr] = &[
        KHR_SURFACE_EXTENSION_NAME,
        KHR_ANDROID_SURFACE_EXTENSION_NAME,
    ];
    const WIN: &[&CStr] = &[KHR_SURFACE_EXTENSION_NAME, KHR_WIN32_SURFACE_EXTENSION_NAME];
    const METAL: &[&CStr] = &[KHR_SURFACE_EXTENSION_NAME, EXT_METAL_SURFACE_EXTENSION_NAME];

    let extensions = match window_handle.raw_window_handle() {
        RawWindowHandle::Wayland(_) => WAYLAND,
        RawWindowHandle::Xlib(_) => XLIB,
        RawWindowHandle::Xcb(_) => XCB,
        RawWindowHandle::AndroidNdk(_) => ANDROID,
        RawWindowHandle::Win32(_) => WIN,
        RawWindowHandle::AppKit(_) | RawWindowHandle::UiKit(_) => METAL,
        _ => return VulkanResult::new_err(vk10::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    VulkanResult::new_ok(extensions)
}