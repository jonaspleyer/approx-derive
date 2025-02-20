use approx_derive::*;

#[test]
fn derive_rel_diff_eq() {
    /// Struct definition
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

#[test]
fn derive_rel_diff_eq_equal_1() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Prediction {
        confidence: f64,
        #[approx(equal)]
        category: String,
    }
    let p1 = Prediction {
        confidence: -10.0,
        category: "horses".into(),
    };
    let p2 = Prediction {
        confidence: -10.2,
        category: "horses".into(),
    };
    approx::assert_relative_eq!(p1, p2, max_relative = 0.021);
}

#[test]
fn derive_rel_diff_eq_equal_2() {
    #[derive(RelativeEq, PartialEq, Debug)]
    #[approx(epsilon_type = f64)]
    struct Prediction {
        #[approx(equal)]
        category: String,
        confidence: f64,
    }
    let p1 = Prediction {
        confidence: 0.00002,
        category: "my_horses".into(),
    };
    let p2 = Prediction {
        confidence: -0.0001,
        category: "horses".into(),
    };
    approx::assert_relative_ne!(p1, p2, max_relative = 0.1);
}

#[test]
fn derive_relative_eq_equal_2() {
    #[derive(RelativeEq, PartialEq, Debug)]
    #[approx(epsilon_type = f64)]
    struct Prediction {
        #[approx(equal)]
        category: String,
        confidence: f64,
    }
    let p1 = Prediction {
        confidence: -1.0,
        category: "my_horses".into(),
    };
    let p2 = Prediction {
        confidence: -1.2,
        category: "horses".into(),
    };
    approx::assert_relative_ne!(p1, p2, epsilon = 0.3);
}

#[test]
fn derive_relative_option_2() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Car {
        max_speed: f32,
        #[approx(map = |x| x)]
        battery: Option<f32>,
    }

    let c1 = Car {
        max_speed: 180.0,
        battery: Some(1.0),
    };
    let c2 = Car {
        max_speed: 180.1,
        battery: Some(0.99),
    };
    let c3 = Car {
        max_speed: 180.0,
        battery: None,
    };
    let c4 = Car {
        max_speed: 182.0,
        battery: Some(1.1),
    };
    let c5 = Car {
        max_speed: 177.0,
        battery: Some(0.9),
    };
    approx::assert_relative_eq!(c1, c2, epsilon = 0.15);
    approx::assert_relative_ne!(c1, c3, epsilon = 0.15);
    approx::assert_relative_ne!(c4, c3, epsilon = 10.0);
    approx::assert_relative_eq!(c4, c5, epsilon = 6.0);
}

#[test]
fn derive_relative_mapping() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Cat {
        weight: f32,
        #[approx(map = |_| {Some(&0f32)})]
        birthday: String,
    }
    let c1 = Cat {
        weight: 5.3,
        birthday: "19th of April 2022".into(),
    };
    let c2 = Cat {
        weight: 5.3,
        birthday: "19/04/2022".into(),
    };
    approx::assert_relative_eq!(c1, c2);
}

#[test]
fn derive_relative_mapping_function() {
    #[derive(PartialEq, Debug)]
    enum Time {
        Days(f64),
        Weeks(f64),
    }
    fn time_to_days(t: &Time) -> Option<f64> {
        match t {
            Time::Days(d) => Some(*d),
            Time::Weeks(w) => Some(7.0 * w),
        }
    }
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Dogo {
        age_in_weeks: f64,
        #[approx(map = time_to_days)]
        next_doctors_appointment: Time,
    }
    let d1 = Dogo {
        age_in_weeks: 52.0,
        next_doctors_appointment: Time::Days(35.0),
    };
    let d2 = Dogo {
        age_in_weeks: 52.0,
        next_doctors_appointment: Time::Weeks(5.0),
    };
    approx::assert_relative_eq!(d1, d2, epsilon = 0.0);
}

#[test]
fn derive_relative_equal_higher_priority_than_mapping() {
    #[derive(RelativeEq, PartialEq, Debug)]
    struct Length {
        #[approx(equal)]
        #[approx(map = |x: &f32| Some(2.0*x))]
        meters: f32,
    }
    let l1 = Length { meters: 3.0 };
    let l2 = Length { meters: 3.0001 };
    approx::assert_relative_ne!(l1, l2, epsilon = 0.001);
}
