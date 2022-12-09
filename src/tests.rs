#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use crate::um::UniversalMachine;
    use crate::instructions;

    #[test]
    fn add_test() {
        let mut UM = UniversalMachine::new();
        let val1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;
        let val2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
        instructions::load_value(&mut UM, val1);
        instructions::load_value(&mut UM, val2);
        instructions::add(&mut UM, 3, 1, 2);
        assert_eq!(4, UM.r[3]);
    }

    #[test]
    fn load_value_test() {
        let mut UM = UniversalMachine::new();
        let val1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;
        instructions::load_value(&mut UM, val1);
        assert_eq!(1, UM.r[1]);
    }

    #[test]
    fn mult_test() {
        let mut UM = UniversalMachine::new();
        let val1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;
        let val2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;
        instructions::load_value(&mut UM, val1);
        instructions::load_value(&mut UM, val2);
        instructions::mul(&mut UM, 3, 1, 2);
        assert_eq!(3, UM.r[3]);
    }

    #[test]
    fn mult_overflow_check_test() {

    }
}
