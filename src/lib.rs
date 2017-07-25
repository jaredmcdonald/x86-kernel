#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

fn startup() {
    vga_buffer::clear_screen();
    println!("       $$$$$$$\\  $$\\        $$$$$$\\  $$$$$$$\\   $$$$$$\\  $$\\   $$\\  $$\\ ");
    println!("       $$  __$$\\ $$ |      $$  __$$\\ $$  __$$\\ $$  __$$\\ $$ |  $$ | $$ |");
    println!("       $$ |  $$ |$$ |      $$ /  $$ |$$ |  $$ |$$ /  \\__|$$ |  $$ | $$ |");
    println!("       $$$$$$$\\ |$$ |      $$$$$$$$ |$$$$$$$  |$$ |$$$$\\ $$$$$$$$ | $$ |");
    println!("       $$  __$$\\ $$ |      $$  __$$ |$$  __$$< $$ |\\_$$ |$$  __$$ | \\__|");
    println!("       $$ |  $$ |$$ |      $$ |  $$ |$$ |  $$ |$$ |  $$ |$$ |  $$ |     ");
    println!("       $$$$$$$  |$$$$$$$$\\ $$ |  $$ |$$ |  $$ |\\$$$$$$  |$$ |  $$ | $$\\ ");
    println!("       \\_______/ \\________|\\__|  \\__|\\__|  \\__| \\______/ \\__|  \\__| \\__|");
    println!("\n\n\n\n\n\n");
    // vga_buffer::animate_bounce("BLARGH", 1);
}

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    startup();
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    let colors = [
        vga_buffer::Color::Blue,
        vga_buffer::Color::Green,
        vga_buffer::Color::Yellow,
    ];

    loop {
        for color in colors.iter() {
            vga_buffer::print_color_line(*color);
        }
    }
}


#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
