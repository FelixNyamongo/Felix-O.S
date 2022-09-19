#![no_std]  // Disables the standard library
#![no_main] // Disables main as the start point

use core::panic::PanicInfo; // Creating our own panic implementation now that there is no STD LIB


/*We need to ovewrite the _start entry point to the program
 Main wont work since we have no runtime*/
 #[no_mangle] // disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
 pub extern "C" fn _start()->!{
    loop{}
 }


/*
This function will be called on panicing
The PanicInfo parameter contains the file and line where the panic happened and the optional panic message
The function should never return. It is running on bare metal so where is it going to return to
So it loops indefinetly
->! means that the function never returns
The appendix describes it as:
    ! Always empty bottom type for diverging functions
    where "diverging" means "never returns".
*/
#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    loop{}
}
/*
The eh_personality language item marks a function that is used for implementing stack unwinding.
Stack unwinding is what makes panics in Rust work
Rust uses unwinding to run the destructors of all live stack variables in case of a panic
This ensures that all used memory is freed and allows the parent thread to catch the panic and continue execution

We should disable unwinding and intead abort on panic. Bare metal so... no libs
This is done on the toml file
*/
