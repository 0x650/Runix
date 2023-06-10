/*
 * Copyright 2023 Runix Project Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[path = "../io/ports.rs"]
mod ports;

const PORT: u16 = 0x3f8;

pub fn init() {
    ports::outb(PORT + 1, 0x00); // Disable all interrupts
    ports::outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
    ports::outb(PORT + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
    ports::outb(PORT + 1, 0x00); //                  (hi byte)
    ports::outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
    ports::outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
    ports::outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
    ports::outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
    ports::outb(PORT + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

    // If serial is not faulty set it in normal operation mode
    // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
    ports::outb(PORT + 4, 0x0F);
}

fn is_transmit_empty() -> u8 {
    return ports::inb(PORT + 5) & 0x20;
}

fn transmit_single_byte(c: u8) {
    while is_transmit_empty() == 0 {}
    ports::outb(PORT, c);
}

pub fn print(string: &str) {
    for c in string.bytes() {
        if c == b'\n' {
            transmit_single_byte(b'\r');
        }
        transmit_single_byte(c);
    }
}
