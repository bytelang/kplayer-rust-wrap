// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/prompt/prompt_plugin.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct PromptPlugin {
    // message fields
    pub path: ::std::string::String,
    pub content: ::std::vec::Vec<u8>,
    pub unique: ::std::string::String,
    pub params: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PromptPlugin {
    fn default() -> &'a PromptPlugin {
        <PromptPlugin as ::protobuf::Message>::default_instance()
    }
}

impl PromptPlugin {
    pub fn new() -> PromptPlugin {
        ::std::default::Default::default()
    }

    // string path = 1;


    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    // bytes content = 2;


    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }

    // string unique = 3;


    pub fn get_unique(&self) -> &str {
        &self.unique
    }
    pub fn clear_unique(&mut self) {
        self.unique.clear();
    }

    // Param is passed by value, moved
    pub fn set_unique(&mut self, v: ::std::string::String) {
        self.unique = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unique(&mut self) -> &mut ::std::string::String {
        &mut self.unique
    }

    // Take field
    pub fn take_unique(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.unique, ::std::string::String::new())
    }

    // repeated .KPProto.Prompt.PromptPlugin.ParamsEntry params = 4;


    pub fn get_params(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.params
    }
    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.params, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for PromptPlugin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.unique)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.params)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.content);
        }
        if !self.unique.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.unique);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.params);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        }
        if !self.content.is_empty() {
            os.write_bytes(2, &self.content)?;
        }
        if !self.unique.is_empty() {
            os.write_string(3, &self.unique)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.params, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PromptPlugin {
        PromptPlugin::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "path",
                |m: &PromptPlugin| { &m.path },
                |m: &mut PromptPlugin| { &mut m.path },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "content",
                |m: &PromptPlugin| { &m.content },
                |m: &mut PromptPlugin| { &mut m.content },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "unique",
                |m: &PromptPlugin| { &m.unique },
                |m: &mut PromptPlugin| { &mut m.unique },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "params",
                |m: &PromptPlugin| { &m.params },
                |m: &mut PromptPlugin| { &mut m.params },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PromptPlugin>(
                "PromptPlugin",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PromptPlugin {
        static instance: ::protobuf::rt::LazyV2<PromptPlugin> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PromptPlugin::new)
    }
}

impl ::protobuf::Clear for PromptPlugin {
    fn clear(&mut self) {
        self.path.clear();
        self.content.clear();
        self.unique.clear();
        self.params.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PromptPlugin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PromptPlugin {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventPromptPluginAdd {
    // message fields
    pub plugin: ::protobuf::SingularPtrField<PromptPlugin>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventPromptPluginAdd {
    fn default() -> &'a EventPromptPluginAdd {
        <EventPromptPluginAdd as ::protobuf::Message>::default_instance()
    }
}

impl EventPromptPluginAdd {
    pub fn new() -> EventPromptPluginAdd {
        ::std::default::Default::default()
    }

    // .KPProto.Prompt.PromptPlugin plugin = 1;


    pub fn get_plugin(&self) -> &PromptPlugin {
        self.plugin.as_ref().unwrap_or_else(|| <PromptPlugin as ::protobuf::Message>::default_instance())
    }
    pub fn clear_plugin(&mut self) {
        self.plugin.clear();
    }

    pub fn has_plugin(&self) -> bool {
        self.plugin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plugin(&mut self, v: PromptPlugin) {
        self.plugin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plugin(&mut self) -> &mut PromptPlugin {
        if self.plugin.is_none() {
            self.plugin.set_default();
        }
        self.plugin.as_mut().unwrap()
    }

    // Take field
    pub fn take_plugin(&mut self) -> PromptPlugin {
        self.plugin.take().unwrap_or_else(|| PromptPlugin::new())
    }
}

impl ::protobuf::Message for EventPromptPluginAdd {
    fn is_initialized(&self) -> bool {
        for v in &self.plugin {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.plugin)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.plugin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.plugin.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EventPromptPluginAdd {
        EventPromptPluginAdd::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PromptPlugin>>(
                "plugin",
                |m: &EventPromptPluginAdd| { &m.plugin },
                |m: &mut EventPromptPluginAdd| { &mut m.plugin },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventPromptPluginAdd>(
                "EventPromptPluginAdd",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventPromptPluginAdd {
        static instance: ::protobuf::rt::LazyV2<EventPromptPluginAdd> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventPromptPluginAdd::new)
    }
}

impl ::protobuf::Clear for EventPromptPluginAdd {
    fn clear(&mut self) {
        self.plugin.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventPromptPluginAdd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventPromptPluginAdd {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventPromptPluginRemove {
    // message fields
    pub unique: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventPromptPluginRemove {
    fn default() -> &'a EventPromptPluginRemove {
        <EventPromptPluginRemove as ::protobuf::Message>::default_instance()
    }
}

impl EventPromptPluginRemove {
    pub fn new() -> EventPromptPluginRemove {
        ::std::default::Default::default()
    }

    // string unique = 1;


    pub fn get_unique(&self) -> &str {
        &self.unique
    }
    pub fn clear_unique(&mut self) {
        self.unique.clear();
    }

    // Param is passed by value, moved
    pub fn set_unique(&mut self, v: ::std::string::String) {
        self.unique = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unique(&mut self) -> &mut ::std::string::String {
        &mut self.unique
    }

    // Take field
    pub fn take_unique(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.unique, ::std::string::String::new())
    }
}

impl ::protobuf::Message for EventPromptPluginRemove {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.unique)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.unique.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.unique);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.unique.is_empty() {
            os.write_string(1, &self.unique)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EventPromptPluginRemove {
        EventPromptPluginRemove::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "unique",
                |m: &EventPromptPluginRemove| { &m.unique },
                |m: &mut EventPromptPluginRemove| { &mut m.unique },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventPromptPluginRemove>(
                "EventPromptPluginRemove",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventPromptPluginRemove {
        static instance: ::protobuf::rt::LazyV2<EventPromptPluginRemove> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventPromptPluginRemove::new)
    }
}

impl ::protobuf::Clear for EventPromptPluginRemove {
    fn clear(&mut self) {
        self.unique.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventPromptPluginRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventPromptPluginRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventPromptPluginList {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventPromptPluginList {
    fn default() -> &'a EventPromptPluginList {
        <EventPromptPluginList as ::protobuf::Message>::default_instance()
    }
}

impl EventPromptPluginList {
    pub fn new() -> EventPromptPluginList {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for EventPromptPluginList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EventPromptPluginList {
        EventPromptPluginList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventPromptPluginList>(
                "EventPromptPluginList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventPromptPluginList {
        static instance: ::protobuf::rt::LazyV2<EventPromptPluginList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventPromptPluginList::new)
    }
}

impl ::protobuf::Clear for EventPromptPluginList {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventPromptPluginList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventPromptPluginList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventPromptPluginUpdate {
    // message fields
    pub unique: ::std::string::String,
    pub params: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventPromptPluginUpdate {
    fn default() -> &'a EventPromptPluginUpdate {
        <EventPromptPluginUpdate as ::protobuf::Message>::default_instance()
    }
}

impl EventPromptPluginUpdate {
    pub fn new() -> EventPromptPluginUpdate {
        ::std::default::Default::default()
    }

    // string unique = 1;


    pub fn get_unique(&self) -> &str {
        &self.unique
    }
    pub fn clear_unique(&mut self) {
        self.unique.clear();
    }

    // Param is passed by value, moved
    pub fn set_unique(&mut self, v: ::std::string::String) {
        self.unique = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unique(&mut self) -> &mut ::std::string::String {
        &mut self.unique
    }

    // Take field
    pub fn take_unique(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.unique, ::std::string::String::new())
    }

    // repeated .KPProto.Prompt.EventPromptPluginUpdate.ParamsEntry params = 2;


    pub fn get_params(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.params
    }
    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.params, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for EventPromptPluginUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.unique)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.params)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.unique.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.unique);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.params);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.unique.is_empty() {
            os.write_string(1, &self.unique)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.params, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EventPromptPluginUpdate {
        EventPromptPluginUpdate::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "unique",
                |m: &EventPromptPluginUpdate| { &m.unique },
                |m: &mut EventPromptPluginUpdate| { &mut m.unique },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "params",
                |m: &EventPromptPluginUpdate| { &m.params },
                |m: &mut EventPromptPluginUpdate| { &mut m.params },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventPromptPluginUpdate>(
                "EventPromptPluginUpdate",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventPromptPluginUpdate {
        static instance: ::protobuf::rt::LazyV2<EventPromptPluginUpdate> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventPromptPluginUpdate::new)
    }
}

impl ::protobuf::Clear for EventPromptPluginUpdate {
    fn clear(&mut self) {
        self.unique.clear();
        self.params.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventPromptPluginUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventPromptPluginUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20proto/prompt/prompt_plugin.proto\x12\x0eKPProto.Prompt\"\xd1\x01\n\
    \x0cPromptPlugin\x12\x12\n\x04path\x18\x01\x20\x01(\tR\x04path\x12\x18\n\
    \x07content\x18\x02\x20\x01(\x0cR\x07content\x12\x16\n\x06unique\x18\x03\
    \x20\x01(\tR\x06unique\x12@\n\x06params\x18\x04\x20\x03(\x0b2(.KPProto.P\
    rompt.PromptPlugin.ParamsEntryR\x06params\x1a9\n\x0bParamsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value:\x028\x01\"L\n\x14EventPromptPluginAdd\x124\n\x06plugin\x18\
    \x01\x20\x01(\x0b2\x1c.KPProto.Prompt.PromptPluginR\x06plugin\"1\n\x17Ev\
    entPromptPluginRemove\x12\x16\n\x06unique\x18\x01\x20\x01(\tR\x06unique\
    \"\x17\n\x15EventPromptPluginList\"\xb9\x01\n\x17EventPromptPluginUpdate\
    \x12\x16\n\x06unique\x18\x01\x20\x01(\tR\x06unique\x12K\n\x06params\x18\
    \x02\x20\x03(\x0b23.KPProto.Prompt.EventPromptPluginUpdate.ParamsEntryR\
    \x06params\x1a9\n\x0bParamsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01B5Z3git\
    hub.com/bytelang/kplayer/types/core/proto/promptJ\xe1\x04\n\x06\x12\x04\
    \0\0\x1b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\
    \0\x17\n\x08\n\x01\x08\x12\x03\x04\0J\n\t\n\x02\x08\x0b\x12\x03\x04\0J\n\
    \n\n\x02\x04\0\x12\x04\x06\0\x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\
    \x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\x02\x12\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\t\r\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x10\x11\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x08\x02\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x02\x07\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\x08\x0f\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x08\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\t\x02\x14\
    \n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\t\t\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\t\x12\x13\
    \n\x0b\n\x04\x04\0\x02\x03\x12\x03\n\x02!\n\x0c\n\x05\x04\0\x02\x03\x06\
    \x12\x03\n\x02\x15\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\n\x16\x1c\n\x0c\
    \n\x05\x04\0\x02\x03\x03\x12\x03\n\x1f\x20\n\n\n\x02\x04\x01\x12\x04\r\0\
    \x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\r\x08\x1c\n\x0b\n\x04\x04\x01\x02\
    \0\x12\x03\x0e\x02\x1a\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0e\x02\x0e\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0e\x0f\x15\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\x0e\x18\x19\n\n\n\x02\x04\x02\x12\x04\x11\0\x13\x01\n\
    \n\n\x03\x04\x02\x01\x12\x03\x11\x08\x1f\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x12\x02\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x12\x02\x08\n\x0c\
    \n\x05\x04\x02\x02\0\x01\x12\x03\x12\t\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x12\x12\x13\n\n\n\x02\x04\x03\x12\x04\x15\0\x16\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\x15\x08\x1d\n\n\n\x02\x04\x04\x12\x04\x18\0\x1b\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03\x18\x08\x1f\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03\x19\x02\x14\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\x19\x02\x08\n\x0c\
    \n\x05\x04\x04\x02\0\x01\x12\x03\x19\t\x0f\n\x0c\n\x05\x04\x04\x02\0\x03\
    \x12\x03\x19\x12\x13\n\x0b\n\x04\x04\x04\x02\x01\x12\x03\x1a\x02!\n\x0c\
    \n\x05\x04\x04\x02\x01\x06\x12\x03\x1a\x02\x15\n\x0c\n\x05\x04\x04\x02\
    \x01\x01\x12\x03\x1a\x16\x1c\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03\x1a\
    \x1f\x20b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
