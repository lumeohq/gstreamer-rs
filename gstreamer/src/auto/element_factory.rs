// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Caps;
use crate::Element;
use crate::ElementFactoryListType;
use crate::Object;
use crate::PadDirection;
use crate::PluginFeature;
use crate::Rank;
use crate::StaticPadTemplate;
use crate::URIType;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstElementFactory")]
    pub struct ElementFactory(Object<ffi::GstElementFactory, ffi::GstElementFactoryClass>) @extends PluginFeature, Object;

    match fn {
        type_ => || ffi::gst_element_factory_get_type(),
    }
}

impl ElementFactory {
    #[doc(alias = "gst_element_factory_can_sink_all_caps")]
    pub fn can_sink_all_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_all_caps(
                self.to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_sink_any_caps")]
    pub fn can_sink_any_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_any_caps(
                self.to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_all_caps")]
    pub fn can_src_all_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_all_caps(
                self.to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_any_caps")]
    pub fn can_src_any_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_any_caps(
                self.to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_create")]
    pub fn create(&self, name: Option<&str>) -> Result<Element, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_element_factory_create(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create element from factory"))
        }
    }

    #[doc(alias = "gst_element_factory_get_element_type")]
    #[doc(alias = "get_element_type")]
    pub fn element_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_element_factory_get_element_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_element_factory_get_metadata(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_metadata_keys")]
    #[doc(alias = "get_metadata_keys")]
    pub fn metadata_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_get_metadata_keys(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_num_pad_templates")]
    #[doc(alias = "get_num_pad_templates")]
    pub fn num_pad_templates(&self) -> u32 {
        unsafe { ffi::gst_element_factory_get_num_pad_templates(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_element_factory_get_static_pad_templates")]
    #[doc(alias = "get_static_pad_templates")]
    pub fn static_pad_templates(&self) -> Vec<StaticPadTemplate> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_element_factory_get_static_pad_templates(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_uri_protocols")]
    #[doc(alias = "get_uri_protocols")]
    pub fn uri_protocols(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_element_factory_get_uri_protocols(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_uri_type")]
    #[doc(alias = "get_uri_type")]
    pub fn uri_type(&self) -> URIType {
        unsafe { from_glib(ffi::gst_element_factory_get_uri_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_element_factory_has_interface")]
    pub fn has_interface(&self, interfacename: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_has_interface(
                self.to_glib_none().0,
                interfacename.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_list_is_type")]
    pub fn list_is_type(&self, type_: ElementFactoryListType) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_list_is_type(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_find")]
    pub fn find(name: &str) -> Option<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_element_factory_find(name.to_glib_none().0)) }
    }

    #[doc(alias = "gst_element_factory_list_filter")]
    pub fn list_filter(
        list: &[ElementFactory],
        caps: &Caps,
        direction: PadDirection,
        subsetonly: bool,
    ) -> Vec<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_list_filter(
                list.to_glib_none().0,
                caps.to_glib_none().0,
                direction.into_glib(),
                subsetonly.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_list_get_elements")]
    pub fn list_get_elements(type_: ElementFactoryListType, minrank: Rank) -> Vec<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_list_get_elements(
                type_.into_glib(),
                minrank.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_make")]
    pub fn make(factoryname: &str, name: Option<&str>) -> Result<Element, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_element_factory_make(
                factoryname.to_glib_none().0,
                name.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create element from factory name"))
        }
    }
}

unsafe impl Send for ElementFactory {}
unsafe impl Sync for ElementFactory {}
