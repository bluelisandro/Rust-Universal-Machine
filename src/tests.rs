#![allow(unused_imports)]

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

        let val_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011; // 3
        let val_reg_2: u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0011; // 3

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

    #[test]
    fn map_seg_test() {
        let mut UM = UniversalMachine::new();

        let three_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011;

        // Load a value into a register      
        // Load 3 into r[1]  
        instructions::load_value(&mut UM, three_reg_1);

        // Map the segment we want to access
        // B is the register index where we want map_seg to return the newly mapped segment index
        instructions::map_seg(&mut UM, 2, 1);

        // So now, the index for our newly mapped segment should be in r[2]

        // Now store the value in r[1] = 3, into the segment index in r[2]
        instructions::seg_store(&mut UM, 2, 0, 1);

        // Load that segment's value into a register
        // Load segments[r[2]][0] into r[5]
        instructions::seg_load(&mut UM, 3, 2, 0);

        assert_eq!(UM.r[3], 3);
    }

    #[test]
    fn map_seg_offset_test() {
        let mut UM = UniversalMachine::new();
             
        // Load r[1] with 3
        let three_reg_1: u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0011;
        instructions::load_value(&mut UM, three_reg_1);

        // Load r[3] with 0
        let zero_reg_3: u32 = 0b_0000_0110_0000_0000_0000_0000_0000_0000;
        instructions::load_value(&mut UM, zero_reg_3);

        // Load r[4] with 1
        let one_reg_4: u32 = 0b_0000_1000_0000_0000_0000_0000_0000_0001;
        instructions::load_value(&mut UM, one_reg_4);

        // Load r[5] with 2
        let two_reg_5: u32 = 0b_0000_1010_0000_0000_0000_0000_0000_0010;
        instructions::load_value(&mut UM, two_reg_5);

        // Map the segment we want to access
        // r[B] is the register where we want map_seg to return the newly mapped segment index
        // r[C] is the capacity of the new segment
        instructions::map_seg(&mut UM, 2, 5);

        // Value we want to check for:                r[1] = 3 
        // Index of newly mapped segment:             r[2]
        // Value of 0th segment offset:               r[3] = 0
        // Value of 1st segment offset:               r[4] = 1
        // Capacity to give map_seg for new segment : r[5] = 2

        // Store the value in r[1] = 3, in segment[r[2]][r[3] = 0]
        instructions::seg_store(&mut UM, 2, 3, 1);

        // Store the value in r[1] = 3, in segment[r[2]][r[4] = 1]
        instructions::seg_store(&mut UM, 2, 4, 1);

        // Load seg[2][1] into r[6]
        instructions::seg_load(&mut UM, 6, 2, 4);

        assert_eq!(UM.r[6], 3);
    }

}
