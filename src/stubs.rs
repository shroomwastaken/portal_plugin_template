use core::ffi::c_void;

// function stubs for ones we don't actually need
pub extern "thiscall" fn pause(_: *mut c_void) {}
pub extern "thiscall" fn unpause(_: *mut c_void) {}
pub extern "thiscall" fn level_init(_: *mut c_void, _: *const i8) {}
pub extern "thiscall" fn server_activate(_: *mut c_void, _: *mut i8, _: i32, _: i32) {}
pub extern "thiscall" fn game_frame(_: *mut c_void, _: bool) {}
pub extern "thiscall" fn level_shutdown(_: *mut c_void) {}
pub extern "thiscall" fn client_active(_: *mut c_void, _: *mut c_void) {}
pub extern "thiscall" fn client_disconnect(_: *mut c_void, _: *mut c_void) {}
pub extern "thiscall" fn client_put_in_server(_: *mut c_void, _: *mut c_void, _: *const i8) {}
pub extern "thiscall" fn set_command_client(_: *mut c_void, _: i32) {}
pub extern "thiscall" fn client_settings_changed(_: *mut c_void, _: *mut c_void) {}
pub extern "thiscall" fn client_connect(_: *mut c_void, _: *mut bool, _: *mut c_void, _: *const i8, _: *const i8, _: *mut char, _: i32) -> i32 { 0 }
pub extern "thiscall" fn client_command(_: *mut c_void, _: *mut c_void, _: *const c_void) -> i32 { 0 }
pub extern "thiscall" fn network_id_validated(_: *mut c_void, _: *const char, _: *const char) -> i32 { 0 }
pub extern "thiscall" fn on_query_cvar_value_finished(_: *mut c_void, _: i32, _: *mut c_void, _: i32, _: *const i8, _: *const i8) {}

// add these if you adapt this to a game that uses version 003
// dont forget to add them to the VTABLE
// pub extern "thiscall" fn on_edict_allocated(_: *mut c_void, _: *mut c_void) {}
// pub extern "thiscall" fn on_edict_freed(_: *mut c_void, _: *const c_void) {}
