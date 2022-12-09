#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use crate::um::UniversalMachine;
    use crate::instructions;

    #[test]
    fn add() {
        let mut UM = UniversalMachine::new();
        let val1: u32 = 0b_0000_0001_0000_0000_0000_0000_0000_0001;
        let val2: u32 = 0b_0000_0010_000000000000000000000011;
        instructions::load_value(&mut UM, val1);
        instructions::load_value(&mut UM, val2);
        instructions::add(&mut UM, 3, 1, 2);
        assert_eq!(5, UM.r[3]);
    }
}
