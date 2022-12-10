#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use crate::um::UniversalMachine;
    use crate::instructions;

    #[test]
    fn conditional_move_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_1000_0001_0000_1100_0001; 
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_1111; // 15

        instructions::load_value(&mut UM, val_reg_1);
        instructions::load_value(&mut UM, val_reg_2);

        instructions::cmov(&mut UM, 5, 2, 1);
        assert_eq!(15, UM.r[2]);
    }

    #[test]
    fn invalid_conditional_move_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_6: u32 = 0b_0000_1100_0000_0000_0000_0000_0000_0001;
        let val_reg_1: u32 = 0b_0000_0010_0001_0010_1000_1000_0100_0001; // 68323393
        
        instructions::load_value(&mut UM, val_reg_6);
        instructions::load_value(&mut UM, val_reg_1);
        instructions::cmov(&mut UM, 6, 1, 2);
        assert_eq!(1, UM.r[6]);
    }

    #[test]
    fn add_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0001_0000_0000_0001;
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;

        instructions::load_value(&mut UM, val_reg_1);
        instructions::load_value(&mut UM, val_reg_2);
        instructions::add(&mut UM, 3, 1, 2);
        assert_eq!(4100, UM.r[3]);
    }

    #[test]
    fn add_overflow_test() {
        let mut UM = UniversalMachine::new();

        UM.r[1] = u32::MAX;
        UM.r[2] = 1;

        instructions::add(&mut UM, 3, 1, 2);
        assert_eq!(0, UM.r[3]);
    }

    #[test]
    fn load_value_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0001;

        instructions::load_value(&mut UM, val_reg_1);
        assert_eq!(1, UM.r[1]);
    }

    #[test]
    fn mult_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011;
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011;

        instructions::load_value(&mut UM, val_reg_1);
        instructions::load_value(&mut UM, val_reg_2);
        instructions::mul(&mut UM, 3, 1, 2);
        assert_eq!(9, UM.r[3]);
    }

    #[test]
    fn mult_overflow_check_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;
        let val_reg_2: u32 = 0b_0000_0100_1111_1111_1111_1111_1111_1111;

        instructions::load_value(&mut UM, val_reg_1);
        instructions::load_value(&mut UM, val_reg_2);
        instructions::mul(&mut UM, 3, 1, 2);
        assert_eq!(4261412865, UM.r[3]);
    }

    #[test]
    fn div_test() {
        let mut UM = UniversalMachine::new();

        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;
        let val_reg_2: u32 = 0b_0000_0100_1111_1111_1111_1111_1111_1111;

        instructions::load_value(&mut UM, val_reg_1);
        instructions::load_value(&mut UM, val_reg_2);
        instructions::div(&mut UM, 3, 1, 2);
        assert_eq!(1, UM.r[3]);
    }

    #[test]
    #[should_panic]
    fn div_by_zero_test() {
        let mut UM = UniversalMachine::new();
        let val_reg_1: u32 = 0b_0000_0010_1111_1111_1111_1111_1111_1111;

        instructions::load_value(&mut UM, val_reg_1);
        instructions::div(&mut UM, 6, 1, 2);
    }

}
