#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

use panic_halt as _;
use core::cell::Cell;
use avr_device::interrupt::Mutex;


static LED_INDEX: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));

const PATTERN_LEN: u8 = 51;
const PATTERN: [u8; PATTERN_LEN as usize] = [
    0b11111111,
    0b11111111,
    0b00001110,
    0b00111100,
    0b11110000,
    0b11110000,
    0b00111100,
    0b00001110,
    0b11111111,
    0b11111111,
    0b00000000,
    0b01100000,
    0b11110100,
    0b10010100,
    0b10010100,
    0b11111100,
    0b11111000,
    0b10000000,
    0b00000000,
    0b11111111,
    0b11111111,
    0b00110000,
    0b01111000,
    0b11111100,
    0b11001100,
    0b10000000,
    0b00000000,
    0b01111000,
    0b11111100,
    0b11010100,
    0b11010100,
    0b11011100,
    0b01011000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00001110,
    0b11011111,
    0b11011111,
    0b00001110,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
];


#[avr_device::interrupt(attiny2313)]
fn TIMER1_COMPA() {
    let mut index = 0;
    avr_device::interrupt::free(|cr| {
        let cell = LED_INDEX.borrow(cr);
        index = cell.get();
        let new_index = if index >= PATTERN_LEN - 1 { 0 } 
            else { index + 1 };
        cell.set(new_index);
    });
    let dp = unsafe { avr_device::attiny2313::Peripherals::steal() };
    dp.PORTB.portb.write(|r| r.bits(PATTERN[index as usize]));

}

#[avr_device::entry]
fn main() -> ! {
    {
        let dp = avr_device::attiny2313::Peripherals::take().unwrap();

        dp.PORTB.ddrb.write(|r| r.bits(0xFF));

        dp.TC1.tccr1b.write(|r| r.cs1().direct().wgm1().bits(0b1));
        dp.TC1.ocr1a.write(|r| r.bits(1250));
        dp.TC1.timsk.write(|r| r.ocie1a().set_bit());
    }
    

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::asm::sleep();
    }
}
