use approx_derive::*;

#[test]
fn derive_abs_diff_eq() {
    /// Struct definition
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
    }

    let s1 = MyStruct { value: 0.0 };
    let s2 = MyStruct { value: 0.003 };
    approx::assert_abs_diff_ne!(s1, s2);
    approx::assert_abs_diff_eq!(s1, s2, epsilon = 0.004);
}

#[test]
fn derive_abs_diff_eq_cast_field() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
        #[approx(cast_field)]
        v2: f32,
    }

    let s1 = MyStruct {
        value: 0.0,
        v2: 1.0,
    };
    let s2 = MyStruct {
        value: 1.0,
        v2: 1.0,
    };
    approx::assert_abs_diff_ne!(s1, s2);
}

#[test]
fn derive_abs_diff_eq_cast_field_2() {
    /// Struct definition
    #[derive(AbsDiffEq, PartialEq, Debug)]
    #[approx(epsilon_type = f32)]
    struct MyStructCast2 {
        #[approx(cast_field)]
        value: f64,
        v2: f32,
    }

    let s1 = MyStructCast2 {
        value: 0.0,
        v2: 1.0,
    };
    let s2 = MyStructCast2 {
        value: 1.0,
        v2: 1.0,
    };
    approx::assert_abs_diff_ne!(s1, s2);
}

#[test]
fn derive_abs_diff_eq_cast_value() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct MyStructCastValue {
        v1: f32,
        #[approx(cast_value)]
        v2: f64,
    }
    let ms1 = MyStructCastValue { v1: 1.0, v2: 333.0 };
    let ms2 = MyStructCastValue { v1: 3.0, v2: 331.3 };
    approx::assert_abs_diff_eq!(ms1, ms2, epsilon = 2.3001);
}

#[test]
fn derive_abs_diff_eq_static_epsilon() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct MyStructStatic {
        v1: f32,
        #[approx(cast_field)]
        #[approx(static_epsilon = 0.002)]
        v2: f64,
    }
    let ms1 = MyStructStatic { v1: 1.0, v2: 1.0 };
    let ms2 = MyStructStatic { v1: 1.0, v2: 1.001 };
    approx::assert_abs_diff_eq!(ms1, ms2);
}

#[test]
fn derive_abs_diff_eq_skip() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct MyStruct2 {
        v1: f64,
        v3: f64,
        #[approx(skip)]
        id: u8,
    }
    let my_struct_1 = MyStruct2 {
        v1: 3.0,
        v3: 1.0,
        id: 21,
    };
    let my_struct_2 = MyStruct2 {
        v1: 2.0,
        v3: 2.0,
        id: 33,
    };
    approx::assert_abs_diff_ne!(my_struct_1, my_struct_2);
    approx::assert_abs_diff_eq!(my_struct_1, my_struct_2, epsilon = 1.2);
}

#[test]
fn derive_abs_diff_eq_tuple_struct() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Position(f32, f32);
    let p1 = Position(1.0, 0.2);
    let p2 = Position(0.0, 0.0);
    approx::assert_abs_diff_ne!(p1, p2);
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 1.0);
}

#[test]
fn derive_abs_diff_eq_generics() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct GenericPosition<F> {
        x: F,
        y: F,
    }
    let p1 = GenericPosition { x: 1.0, y: 2.0 };
    let p2 = GenericPosition {
        x: 1.00001,
        y: 1.99999,
    };
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.00002);
}

#[test]
fn derive_abs_diff_eq_generics_tuple() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct GenericPos<F>(F, F);
    let p1 = GenericPos(1_f32, 33_f32);
    let p2 = GenericPos(1_f32, 32_f32);
    approx::assert_abs_diff_ne!(p1, p2);
}
