#![no_std]
#![no_main]
//#![feature(start)]
#![allow(warnings)]
#![feature(static_nobundle)]
#[no_mangle]
//#[link(name="stdc++", kind="static")]
extern {}
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//pros::Motor

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}


//Initialization code
#[no_mangle]
pub unsafe extern "C" fn initialize() {
    motor_set_gearing(1, motor_gearset_e_E_MOTOR_GEARSET_18);
    motor_set_reversed(1, true);
    motor_set_encoder_units(1, motor_encoder_units_e_E_MOTOR_ENCODER_DEGREES);
    lcd_initialize();
}


//Autonomous routine
#[no_mangle]
pub unsafe extern "C" fn autonomous() {

}


//Operator Control code
#[no_mangle]
pub unsafe extern "C" fn opcontrol() {
    loop {
        motor_move(1, controller_get_analog(controller_id_e_t_E_CONTROLLER_MASTER, controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_Y));
        lcd_print(0, "Buttons Bitmap: %d\n".as_ptr(), lcd_read_buttons() as cty::c_uint);
        delay(20);
    }
}
