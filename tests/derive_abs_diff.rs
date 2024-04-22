use approx_derive::AbsDiffEq_f64;

#[test]
fn _derive_approx_diff() {
    #[derive(AbsDiffEq_f64, Debug, PartialEq)]
    struct MyStruct {
        value: f64,
    }
    let my_struct_1 = MyStruct {
        value: 3.0,
    };
    let my_struct_2 = MyStruct {
        value: 2.0,
    };
    approx::assert_abs_diff_ne!(my_struct_1, my_struct_2);
    approx::assert_abs_diff_eq!(my_struct_1, my_struct_2, epsilon = 1.2);
}

#[test]
fn _derive_approx_diff_skip() {
    #[derive(AbsDiffEq_f64, Debug, PartialEq)]
    struct Ms {
        value1: f64,
        value2: f64,
        #[approx(skip)]
        id: u8,
    }
    let ms1 = Ms {
        value1: 0.0,
        value2: 10.0,
        id: 7,
    };
    let ms2 = Ms {
        value1: 0.001,
        value2: 10.003,
        id: 88,
    };
    approx::assert_abs_diff_eq!(ms1, ms2, epsilon = 0.1);
    approx::assert_abs_diff_ne!(ms1, ms2, epsilon = 0.002);
}

#[test]
fn _derive_approx_diff_with_u8() {
    #[derive(AbbsDiffEq_f64, Debug, PartialEq)]
    struct Ms {
        value: f64,
        #[approx(u8)]
        id: u8,
    }
    let ms1 = Ms {
        value: 1.0,
        id: 7,
    };
    let ms2 = Ms {
        value: 1.1,
        id: 7,
    };
}
