#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

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
    println!("\n\n\n\n\n\n")
}

#[no_mangle]
pub extern fn rust_main() {
    startup();

    loop{}
}


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
