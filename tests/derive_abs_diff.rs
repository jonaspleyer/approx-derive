#[cfg(feature = "infer_name")]
use approxim as approx;

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

#[test]
fn derive_abs_diff_option() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct ContainsOptional {
        value: f64,
        #[approx(equal)]
        opt: Option<i32>,
    }
    let c1 = ContainsOptional {
        value: 1.1,
        opt: Some(1),
    };
    let c2 = ContainsOptional {
        value: 1.0,
        opt: Some(1),
    };
    let c3 = ContainsOptional {
        value: 1.0,
        opt: Some(2),
    };
    approx::assert_abs_diff_eq!(c1, c2, epsilon = 0.2);
    approx::assert_abs_diff_ne!(c2, c3);
}

#[test]
fn derive_abs_diff_eq_equal_1() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Prediction {
        confidence: f64,
        #[approx(equal)]
        category: String,
    }
    let p1 = Prediction {
        confidence: -1.0,
        category: "horses".into(),
    };
    let p2 = Prediction {
        confidence: -1.2,
        category: "horses".into(),
    };
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.3);
}

#[test]
fn derive_abs_diff_eq_equal_2() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
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
    approx::assert_abs_diff_ne!(p1, p2, epsilon = 0.3);
}

#[test]
fn derive_abs_diff_option_2() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
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
    approx::assert_abs_diff_eq!(c1, c2, epsilon = 0.15);
    approx::assert_abs_diff_ne!(c1, c3, epsilon = 0.15);
    approx::assert_abs_diff_ne!(c4, c3, epsilon = 10.0);
    approx::assert_abs_diff_eq!(c4, c5, epsilon = 6.0);
}

#[test]
fn derive_abs_diff_mapping() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
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
    approx::assert_abs_diff_eq!(c1, c2);
}

#[test]
fn derive_abs_diff_mapping_enum() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    enum Pet {
        Cat {
            weight: f32,
            #[approx(map = |_| {Some(&0f32)})]
            birthday: String,
        },
    }

    let c1 = Pet::Cat {
        weight: 5.3,
        birthday: "19th of April 2022".into(),
    };
    let c2 = Pet::Cat {
        weight: 5.3,
        birthday: "19/04/2022".into(),
    };
    approx::assert_abs_diff_eq!(c1, c2);
}

#[test]
fn derive_abs_diff_mapping_function() {
    #[derive(PartialEq, Debug)]
    enum Time {
        Days(u16),
        Weeks(u16),
    }
    fn time_to_days(t: &Time) -> Option<u16> {
        match t {
            Time::Days(d) => Some(*d),
            Time::Weeks(w) => Some(7 * w),
        }
    }
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Dogo {
        age_in_weeks: u16,
        #[approx(map = time_to_days)]
        next_doctors_appointment: Time,
    }
    let d1 = Dogo {
        age_in_weeks: 52,
        next_doctors_appointment: Time::Days(35),
    };
    let d2 = Dogo {
        age_in_weeks: 52,
        next_doctors_appointment: Time::Weeks(5),
    };
    approx::assert_abs_diff_eq!(d1, d2, epsilon = 0);
}

#[test]
fn derive_abs_diff_equal_higher_priority_than_mapping() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Length {
        #[approx(equal)]
        #[approx(map = |x: &f32| Some(2.0*x))]
        meters: f32,
    }
    let l1 = Length { meters: 3.0 };
    let l2 = Length { meters: 3.0001 };
    approx::assert_abs_diff_ne!(l1, l2, epsilon = 0.001);
}

#[test]
fn derive_abs_diff_equal_enum() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    enum Parameter {
        Fixed(f32),
        Range(f32, f32),
    }

    let l1 = Parameter::Fixed(1.0);
    let l2 = Parameter::Fixed(20.0);
    approx::assert_abs_diff_ne!(l1, l2);
    approx::assert_abs_diff_eq!(l1, l2, epsilon = 20.0);

    let l3 = Parameter::Range(1.0, 3.0);
    let l4 = Parameter::Range(1.0, 3.1);
    approx::assert_abs_diff_ne!(l2, l3);
    approx::assert_abs_diff_ne!(l3, l4);
    approx::assert_abs_diff_eq!(l3, l4, epsilon = 0.11);
}

#[test]
fn derive_abs_diff_equal_enum_struct() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    enum Parameter {
        Fixed { value: f32 },
        Range { lower: f32, upper: f32 },
    }

    let l1 = Parameter::Fixed { value: 1.0 };
    let l2 = Parameter::Fixed { value: 20.0 };
    approx::assert_abs_diff_ne!(l1, l2);
    approx::assert_abs_diff_eq!(l1, l2, epsilon = 20.0);

    let l3 = Parameter::Range {
        lower: 1.0,
        upper: 3.0,
    };
    let l4 = Parameter::Range {
        lower: 1.0,
        upper: 3.1,
    };
    approx::assert_abs_diff_ne!(l2, l3);
    approx::assert_abs_diff_ne!(l3, l4);
    approx::assert_abs_diff_eq!(l3, l4, epsilon = 0.11);
}

#[test]
fn derive_abs_diff_equal_enum_cast() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    enum Location {
        Smooth {
            x: f32,
            y: f32,
        },
        Lattice {
            #[approx(cast_value)]
            x: isize,
            #[approx(equal)]
            y: isize,
        },
    }

    let l1 = Location::Smooth { x: 1.0, y: 2.0 };
    let l2 = Location::Smooth { x: 1.1, y: 1.9 };
    approx::assert_abs_diff_ne!(l1, l2);
    approx::assert_abs_diff_eq!(l1, l2, epsilon = 0.11);

    let l3 = Location::Lattice { x: 1, y: 2 };
    let l4 = Location::Lattice { x: 1, y: 3 };
    let l5 = Location::Lattice { x: 1, y: 2 };
    approx::assert_abs_diff_ne!(l2, l3);
    approx::assert_abs_diff_ne!(l3, l4);
    approx::assert_abs_diff_eq!(l3, l5);

    let l6 = Location::Lattice { x: 2, y: 2 };
    let l7 = Location::Lattice { x: 1, y: 2 };
    approx::assert_abs_diff_eq!(l6, l7, epsilon = 1.0);
    approx::assert_abs_diff_ne!(l6, l7, epsilon = 0.01);
}

#[test]
fn derive_abs_diff_eq_nested_struct() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct Player {
        health: f32,
        position: Point,
    }

    let p1 = Player {
        health: 0.5,
        position: Point { x: 1.0, y: 2.0 },
    };
    let p2 = Player {
        health: 0.6,
        position: Point { x: 1.1, y: 2.1 },
    };
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.2);
}

#[test]
fn derive_abs_diff_equal_enum_nested() {
    #[derive(AbsDiffEq, PartialEq, Debug)]
    struct SampledFloat {
        min: f32,
        max: f32,
        initial: f32,
        #[approx(equal)]
        individual: bool,
    }

    #[derive(AbsDiffEq, PartialEq, Debug)]
    #[approx(epsilon_type = f32)]
    enum Parameter {
        Fix(f32),
        Sampled(SampledFloat),
    }

    let p1 = Parameter::Fix(1.0);
    let p2 = Parameter::Fix(1.1);
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.2);

    let p3 = Parameter::Sampled(SampledFloat {
        min: 1.0,
        max: 1.2,
        initial: 1.1,
        individual: false,
    });
    let p4 = Parameter::Sampled(SampledFloat {
        min: 1.1,
        max: 1.3,
        initial: 1.2,
        individual: false,
    });
    approx::assert_abs_diff_eq!(p3, p4, epsilon = 0.2);
    approx::assert_abs_diff_ne!(p3, p4, epsilon = 0.01);
}

#[test]
fn iterator() {
    #[derive(PartialEq, Debug, AbsDiffEq)]
    #[approx(epsilon_type = f32)]
    struct Agent {
        #[approx(into_iter)]
        pos: [f32; 2],
    }

    let a1 = Agent { pos: [0f32; 2] };
    let a2 = Agent { pos: [1f32; 2] };

    approx::assert_abs_diff_ne!(a1, a2);
    approx::assert_abs_diff_eq!(a1, a2, epsilon = 1.2);
}

#[test]
fn iterator_not_equal_length() {
    #[derive(PartialEq, Debug, AbsDiffEq)]
    struct Interaction {
        strength: f64,
        #[approx(into_iter)]
        polynomial_coefficients: Vec<f64>,
    }

    let i1 = Interaction {
        strength: 1.0,
        polynomial_coefficients: vec![0.1, 0.003, 0.234],
    };
    let i2 = Interaction {
        strength: 1.0,
        polynomial_coefficients: vec![0.1, 0.003],
    };

    approx::assert_abs_diff_ne!(i1, i2, epsilon = 1000.0);
    approx::assert_abs_diff_ne!(i1, i2, epsilon = 10_000.0);
}

#[test]
fn iterator_enum() {
    #[derive(PartialEq, Debug, AbsDiffEq)]
    enum Parameter {
        Sampled {
            value: f32,
            #[approx(into_iter)]
            bounds: [f32; 2],
        },
        Fixed(f32),
    }

    let p1 = Parameter::Sampled {
        value: -1.0,
        bounds: [0.0, 3.0],
    };
    let p2 = Parameter::Sampled {
        value: -0.9,
        bounds: [-0.1, 2.9],
    };
    let p3 = Parameter::Fixed(1.0);
    approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.11);
    approx::assert_abs_diff_ne!(p1, p2);
    approx::assert_abs_diff_ne!(p1, p3);
    approx::assert_abs_diff_ne!(p2, p3);
}

#[cfg(feature = "infer_name")]
#[test]
fn epsilon_mapping() {
    #[derive(Clone, PartialEq, Debug, AbsDiffEq)]
    #[approx(epsilon_type = f32)]
    struct A {
        a: f32,
        #[approx(epsilon_map = |x| (x as usize, x))]
        b: (usize, f32),
    }

    let a1 = A {
        a: 0.0,
        b: (1, 3.0),
    };
    let a2 = A {
        a: 0.1,
        b: (2, 3.1),
    };

    approx::assert_abs_diff_ne!(a1, a2);
    approx::assert_abs_diff_eq!(a1, a2, epsilon = 1.1);
}
