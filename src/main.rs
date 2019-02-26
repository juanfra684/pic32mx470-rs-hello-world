
#![allow(unused_variables)]
#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]

use core::panic::PanicInfo;
use mips_rt;
//use volatile_register::{RW, RO};
use pic32mx4xxfxxxh;


fn set_yellow_led(on: bool){
    let bit = 1 << 1;
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    p.PORTD.anseldclr.write(|w| unsafe { w.bits(bit)});
    p.PORTD.trisdclr.write(|w| unsafe { w.bits(bit) });
    if on {
        p.PORTD.latdset.write(|w| unsafe { w.bits(bit)});
    }else{
        p.PORTD.latdclr.write(|w| unsafe {w.bits(bit)});
    }
}

fn set_green_led(on: bool){
    let bit = 1 << 6;
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    let port = &p.PORTG;
    port.anselgclr.write(|w| unsafe { w.bits(bit)});
    port.trisgclr.write(|w| unsafe { w.bits(bit) });
    if on {
        port.latgset.write(|w| unsafe { w.bits(bit)});
    }else{
        port.latgclr.write(|w| unsafe {w.bits(bit)});
    }
}

fn uart2_init() {
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};

    let pps = p.PPS;
    pps.rpf5r.write(|w| unsafe { w.rpf5r().bits(0b0001) }); // UART 2 on RPF5

    let sys_clock : u32 = 96000000;
    let pb_clock : u32 = sys_clock; // TODO: consider PBDIV
    let baud_rate : u32 = 115200;
    let brg = (pb_clock/(4*baud_rate)-1) as u16;

    let uart2 = p.UART2;
    uart2.u2mode.write(|w|  { w.brgh().bit(true) });
    uart2.u2sta.write(|w| unsafe { w.utxen().bit(true).utxisel().bits(0b10) });
    uart2.u2brg.write(|w| unsafe { w.brg().bits(brg) });
    uart2.u2modeset.write(|w| w.on().bit(true));
}

fn uart2_write(byte: u8){

    let  uart2 = unsafe { pic32mx4xxfxxxh::Peripherals::steal()}.UART2;

    while uart2.u2sta.read().utxbf().bit() { }
    uart2.u2txreg.write(|w| unsafe { w.u2txreg().bits(byte as u16) });
}


#[no_mangle]
pub unsafe extern "C" fn main() -> ! {

    let txt = "Hello World!\n";
    uart2_init();
    let mut state = false;
    loop {
        for ch in txt.bytes() {
            uart2_write(ch);
        }
        set_yellow_led(state);
        set_green_led(!state);
        let mut i = 1000000;
        while i > 0 {
            i-= 1;
            asm!("nop");
        }
        state = !state;
    }
}



#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    let x = 42;
    loop {}
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}

