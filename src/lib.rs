#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/*
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits {
    pub _address: u8,
}

#[doc = r" If Bindgen could only determine the size and alignment of a"]
#[doc = r" type, it is represented like this."]
#[derive(PartialEq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___normal_iterator<_Iterator> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    pub _M_current: _Iterator,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reverse_iterator<_Iterator> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
    pub current: _Iterator,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_string_view<_CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    pub _M_len: usize,
    pub _M_str: *const _CharT,
}

pub type std_enable_if_t = u8;


#[repr(C)]
pub struct std_basic_string<_CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    pub _M_dataplus: std_basic_string__Alloc_hider,
    pub _M_string_length: std_basic_string_size_type,
    pub __bindgen_anon_1: std_basic_string__bindgen_ty_2<_CharT>,
}
pub type std_basic_string__Char_alloc_type = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string__Alloc_traits = __gnu_cxx___alloc_traits;
pub type std_basic_string_traits_type<_Traits> = _Traits;
pub type std_basic_string_value_type = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_allocator_type = std_basic_string__Char_alloc_type;
pub type std_basic_string_size_type = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_difference_type = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_reference = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_const_reference = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_pointer = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_const_pointer = __BindgenOpaqueArray<u8, 0usize>;
pub type std_basic_string_iterator = __gnu_cxx___normal_iterator<std_basic_string_pointer>;
pub type std_basic_string_const_iterator =
    __gnu_cxx___normal_iterator<std_basic_string_const_pointer>;
pub type std_basic_string_const_reverse_iterator =
    std_reverse_iterator<std_basic_string_const_iterator>;
pub type std_basic_string_reverse_iterator = std_reverse_iterator<std_basic_string_iterator>;
pub type std_basic_string___const_iterator = std_basic_string_const_iterator;
pub type std_basic_string___sv_type<_CharT> = std_basic_string_view<_CharT>;
pub type std_basic_string__If_sv = std_enable_if_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_basic_string___sv_wrapper<_CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    pub _M_sv: std_basic_string___sv_type<_CharT>,
}
#[repr(C)]
pub struct std_basic_string__Alloc_hider {
    pub _M_p: std_basic_string_pointer,
}
pub const std_basic_string__S_local_capacity: std_basic_string__bindgen_ty_1 = 0;
pub type std_basic_string__bindgen_ty_1 = i32;
#[repr(C)]
pub struct std_basic_string__bindgen_ty_2<_CharT> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
    pub _M_local_buf: __BindgenUnionField<*mut _CharT>,
    pub _M_allocated_capacity: __BindgenUnionField<std_basic_string_size_type>,
    pub bindgen_union_field: u64,
}

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}

pub type std_integral_constant_value_type<_Tp> = _Tp;
pub type std_integral_constant_type = u8;
pub type std_true_type = u8;
pub type std_false_type = u8;
pub type std___enable_if_t = u8;

pub type std_string = std_basic_string<::std::os::raw::c_char>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libusb_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libusb_device_handle {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct LibSeek_SeekDevice {
    pub m_vendor_id: ::std::os::raw::c_int,
    pub m_product_id: ::std::os::raw::c_int,
    pub m_dev_filename: std_string,
    pub m_timeout: ::std::os::raw::c_int,
    pub m_is_opened: bool,
    pub m_ctx: *mut libusb_context,
    pub m_handle: *mut libusb_device_handle,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of blah"][::std::mem::size_of::<std_string>() - 64usize];
    ["Size of LibSeek_SeekDevice"][::std::mem::size_of::<LibSeek_SeekDevice>() - 64usize];
    ["Alignment of LibSeek_SeekDevice"][::std::mem::align_of::<LibSeek_SeekDevice>() - 8usize];
    ["Offset of field: LibSeek_SeekDevice::m_vendor_id"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_vendor_id) - 0usize];
    ["Offset of field: LibSeek_SeekDevice::m_product_id"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_product_id) - 4usize];
    ["Offset of field: LibSeek_SeekDevice::m_dev_filename"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_dev_filename) - 8usize];
    ["Offset of field: LibSeek_SeekDevice::m_timeout"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_timeout) - 40usize];
    ["Offset of field: LibSeek_SeekDevice::m_is_opened"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_is_opened) - 44usize];
    ["Offset of field: LibSeek_SeekDevice::m_ctx"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_ctx) - 48usize];
    ["Offset of field: LibSeek_SeekDevice::m_handle"]
        [::std::mem::offset_of!(LibSeek_SeekDevice, m_handle) - 56usize];
};
*/