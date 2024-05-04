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
