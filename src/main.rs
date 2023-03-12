use machine::Machine;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut machine = Machine::new();
    machine.run()
}
