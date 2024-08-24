#![cfg(target_arch = "x86")] // so vscode doesnt scream about thiscall
#![cfg(panic = "abort")] // so vscode doesnt scream about the panic handler

// remove no_std and the panic_handler if you dont care about the dll being 720kb :skull:
// also if you remove this dont forget to remove the panic = "abort" bit in Cargo.toml
#![no_std]

#[panic_handler]
fn _panic(_: &core::panic::PanicInfo) -> ! {
	loop {}
}

use core::ffi::c_void; // void* my beloved

use crate::utils::strcmp;

mod utils;
mod stubs;

type CreateInterfaceFn = extern "thiscall" fn(*const i8, *mut i32) -> *mut c_void;

const VTABLE: [*const c_void; 18] = [
	load                                as *const c_void,
	unload                              as *const c_void,
	stubs::pause                        as *const c_void,
	stubs::unpause                      as *const c_void,
	get_plugin_description              as *const c_void,
	stubs::level_init                   as *const c_void,
	stubs::server_activate              as *const c_void,
	stubs::game_frame                   as *const c_void,
	stubs::level_shutdown               as *const c_void,
	stubs::client_active                as *const c_void,
	stubs::client_disconnect            as *const c_void,
	stubs::client_put_in_server         as *const c_void,
	stubs::set_command_client           as *const c_void,
	stubs::client_settings_changed      as *const c_void,
	stubs::client_connect               as *const c_void,
	stubs::client_command               as *const c_void,
	stubs::network_id_validated         as *const c_void,
	stubs::on_query_cvar_value_finished as *const c_void,
];

#[repr(C)]
struct RustPlugin {
	pub vtable: *const *const c_void
}

const PLUGIN_OBJECT: RustPlugin = RustPlugin { vtable: VTABLE.as_ptr() };

#[no_mangle]
#[allow(non_snake_case)]
// this has to not be snake_case since we need source to see and call it
// this also has to be extern C not thiscall (thank you jukspaaaaaaaa)
unsafe extern "C" fn CreateInterface(name: *const i8, ret: *mut i32) -> *mut c_void {
	// using version 002 since we want to load this in portal 1
	if strcmp(name, "ISERVERPLUGINCALLBACKS002") {
		if !ret.is_null() { *ret = 0; }
		return &PLUGIN_OBJECT as *const RustPlugin as *mut c_void; // crazy double cast lol
	}
	if !ret.is_null() { *ret = 1; }
	return 0 as *mut c_void; // just a null pointer
}

extern "thiscall" fn get_plugin_description(_: *mut c_void) -> *const i8 {
	// c prefix because we return a c-string
	return c"shroom's rust-based plugin".as_ptr();
}

extern "thiscall" fn load(_: *mut c_void, _: CreateInterfaceFn, _: CreateInterfaceFn) -> bool {
	return true;
}

extern "thiscall" fn unload(_: *mut c_void) {

}
