#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

use panic_halt as _;
use avr_device::interrupt::Mutex;
use attiny_hal::pac::PORTB;
use core::cell::{Cell, RefCell};

static LED_INDEX: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));

const PATTERN_LEN: usize = 51;
const PATTERN: [u8; PATTERN_LEN] = [0b11111111, 0b11111111, 0b00001110, 0b00111100, 0b11110000, 0b11110000, 0b00111100, 0b00001110, 0b11111111, 0b11111111, 0b00000000, 0b01100000, 0b11110100, 0b10010100, 0b10010100, 0b11111100, 0b11111000, 0b10000000, 0b00000000, 0b11111111, 0b11111111, 0b00110000, 0b01111000, 0b11111100, 0b11001100, 0b10000000, 0b00000000, 0b01111000, 0b11111100, 0b11010100, 0b11010100, 0b11011100, 0b01011000, 0b00000000, 0b00000000, 0b00000000, 0b00001110, 0b11011111, 0b11011111, 0b00001110, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000];


static PORTB: Mutex<RefCell<Option<PORTB>>> = Mutex::new(RefCell::new(None));

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();

    dp.PORTB.ddrb.write(|r| r.bits(0xff)); 

    dp.TC1.tccr1b.write(|r| r.cs1().direct().wgm1().bits(0b1));
    dp.TC1.ocr1a.write(|r| r.bits(1250));
    dp.TC1.timsk.write(|r| r.ocie1a().set_bit());

    avr_device::interrupt::free(|cr| 
        PORTB.borrow(cr).replace(Some(dp.PORTB))
    );

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::asm::sleep();
    }
}

#[avr_device::interrupt(attiny2313)]
fn TIMER1_COMPA() {
    avr_device::interrupt::free(|cr| {
        let cell = LED_INDEX.borrow(cr);
        let index = cell.get();

        let new_index = if index >= PATTERN_LEN - 1 { 0 } 
            else { index + 1 };
        cell.set(new_index);
        
        let portb_borrow = PORTB.borrow(cr)
            .borrow_mut();
        let portb = portb_borrow.as_ref()
            .unwrap();

        portb.portb.write(|r| r.bits(PATTERN[index]))
    });
}
