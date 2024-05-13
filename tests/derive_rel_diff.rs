use approx_derive::*;

#[test]
fn derive_rel_diff_eq() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
    }

    let ms1 = MyStruct { value: 20.0 };
    let ms2 = MyStruct { value: 20.1 };
    approx::assert_relative_eq!(ms1, ms2, max_relative = 0.1);
}

#[test]
fn derive_rel_diff_eq_skip() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
        #[approx(skip)]
        id: usize,
    }

    let ms1 = MyStruct {
        value: 20.0,
        id: 99,
    };
    let ms2 = MyStruct {
        value: 20.1,
        id: 39,
    };
    approx::assert_relative_eq!(ms1, ms2, max_relative = 0.1);
}

#[test]
fn derive_rel_diff_eq_skip_infer_epsilon_type() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct MyStruct {
        #[approx(skip)]
        id: u8,
        value: f32,
    }
    let ms1 = MyStruct { id: 1, value: 1.0 };
    let ms2 = MyStruct { id: 9, value: 1.1 };
    approx::assert_relative_eq!(ms1, ms2, max_relative = 0.2);
}

#[test]
fn derive_rel_diff_eq_cast_field() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
        #[approx(cast_field)]
        v2: f32,
    }

    let ms1 = MyStruct {
        value: 20.0,
        v2: 2.0,
    };
    let ms2 = MyStruct {
        value: 20.1,
        v2: 1.9,
    };
    approx::assert_relative_eq!(ms1, ms2, max_relative = 0.1);
}

#[test]
fn derive_rel_diff_eq_cast_value() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct MyStruct {
        value: f64,
        #[approx(cast_value)]
        v2: f32,
    }

    let ms1 = MyStruct {
        value: 20.0,
        v2: 1.0,
    };
    let ms2 = MyStruct {
        value: 20.0,
        v2: 1.0 + f32::MIN,
    };
    approx::assert_relative_ne!(ms1, ms2, max_relative = f32::MIN as f64 / 2.0);
}

#[test]
fn derive_rel_diff_eq_tuple_struct() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Position(f32, f32);
    let p1 = Position(34.58, 906.1);
    let p2 = Position(34.57, 906.2);
    approx::assert_relative_ne!(p1, p2, max_relative = f32::MIN);
    approx::assert_relative_eq!(p1, p2, max_relative = 0.01);
}

#[test]
fn derive_rel_diff_eq_generics() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct GenericPosition<F> {
        x: F,
        y: F,
    }
    let p1 = GenericPosition { x: 34.58, y: 906.1 };
    let p2 = GenericPosition { x: 34.57, y: 906.2 };
    approx::assert_relative_ne!(p1, p2, max_relative = f64::MIN);
    approx::assert_relative_eq!(p1, p2, max_relative = 0.01);
}
