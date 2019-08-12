//! Thin but safe wrappers for the URID mapping features.
use crate::URID;
use core::feature::Feature;
use core::UriBound;
use std::ffi::CStr;

/// Host feature to map URIs to integers
pub struct Map<'a> {
    internal: &'a sys::LV2_URID_Map,
}

unsafe impl<'a> UriBound for Map<'a> {
    const URI: &'static [u8] = sys::LV2_URID_MAP_URI;
}

impl<'a> Feature<'a> for Map<'a> {
    type RawDataType = sys::LV2_URID_Map;

    fn from_raw_data(data: Option<&'a mut sys::LV2_URID_Map>) -> Option<Self> {
        if let Some(internal) = data {
            Some(Self { internal })
        } else {
            None
        }
    }
}

impl<'a> Map<'a> {
    /// Return the URID of the given URI.
    ///
    /// This method capsules the raw mapping method provided by the host. Therefore, it may not be very fast or even capable of running in a real-time environment. Instead of calling this method every time you need a URID, you should call it once and cache it.
    pub fn map(&self, uri: &CStr) -> URID {
        let handle = self.internal.handle;
        let uri = uri.as_ptr();
        unsafe { (self.internal.map.unwrap())(handle, uri) }
    }
}

/// Host feature to revert the URI -> URID mapping.
pub struct Unmap<'a> {
    internal: &'a sys::LV2_URID_Unmap,
}

unsafe impl<'a> UriBound for Unmap<'a> {
    const URI: &'static [u8] = sys::LV2_URID_UNMAP_URI;
}

impl<'a> Feature<'a> for Unmap<'a> {
    type RawDataType = sys::LV2_URID_Unmap;

    fn from_raw_data(data: Option<&'a mut sys::LV2_URID_Unmap>) -> Option<Self> {
        if let Some(internal) = data {
            Some(Self { internal })
        } else {
            None
        }
    }
}

impl<'a> Unmap<'a> {
    /// Return the URI of the given URID.
    ///
    /// This method capsules the raw mapping method provided by the host. Therefore, it may not be very fast or even capable of running in a real-time environment. Instead of calling this method every time you need a URID, you should call it once and cache it.
    pub fn unmap(&self, urid: URID) -> &CStr {
        let handle = self.internal.handle;
        unsafe {
            let uri = (self.internal.unmap.unwrap())(handle, urid);
            CStr::from_ptr(uri)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::*;

    #[test]
    fn test_map_unmap() {
        let mut test_bench = crate::test_bench::TestBench::new();

        let uri_a = CStr::from_bytes_with_nul(b"urn:my-uri-a\0").unwrap();
        let uri_b = CStr::from_bytes_with_nul(b"urn:my_uri-b\0").unwrap();
        {
            let map = test_bench.make_map();

            assert_eq!(0, map.map(uri_a));
            assert_eq!(1, map.map(uri_b));
            assert_eq!(0, map.map(uri_a));
        }
        {
            let unmap = test_bench.make_unmap();

            assert_eq!(uri_a, unmap.unmap(0));
            assert_eq!(uri_b, unmap.unmap(1));
        }
    }
}
