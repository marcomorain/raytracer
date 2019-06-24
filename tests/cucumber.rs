extern crate raytracer;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::string::String;

use raytracer::tuple;

#[macro_use]
extern crate cucumber_rust;

#[derive(Debug)]
pub struct MyWorld {
    tuples: HashMap<String, tuple::Tuple>,
}

fn parse_float(input: &str) -> f32 {
    let re = Regex::new(r"√(\d+)").unwrap();
    return match re.captures(input) {
        None => input.parse::<f32>().unwrap(),
        Some(m) => m[1].parse::<f32>().unwrap().sqrt(),
    };
}

fn parse_rational(input: &str) -> f32 {
    let parts = input
        .split("/")
        .map(|s| parse_float(s))
        .take(2)
        .collect::<Vec<f32>>();

    if let [num, denom] = parts[..] {
        return num / denom;
    } else if let [val] = parts[..] {
        return val;
    }

    panic!("failed to parse {:?}", input)
}

fn parse_floats(input: &str) -> Vec<f32> {
    let nums = input
        .split(", ")
        .map(|s| parse_rational(s))
        .collect::<Vec<_>>();
    return nums;
}

fn tuple_field(t: &tuple::Tuple, f: &String) -> Result<f32, Box<Error>> {
    return match f.as_ref() {
        "x" => Ok(t.0),
        "y" => Ok(t.1),
        "z" => Ok(t.2),
        "w" => Ok(t.3),
        "red"   => Ok(t.0),
        "green" => Ok(t.1),
        "blue"  => Ok(t.2),
        _ => return Err("No matching field".into()),
    };
}

fn build_tuple(class: &String, data: &String) -> tuple::Tuple {
    let vals = parse_floats(data);
    return match class.as_ref() {
        "tuple" => return tuple::Tuple(vals[0], vals[1], vals[2], vals[3]),
        "point" => tuple::point(vals[0], vals[1], vals[2]),
        "vector" => tuple::vector(vals[0], vals[1], vals[2]),
        "color" => return tuple::color(vals[0], vals[1], vals[2]),
        _ => panic!("failed to parse tuple {:?} {:?}", class, vals),
    };
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            tuples: HashMap::new(),
        }
    }
}

fn approx_equal(expected: f32, actual: f32) {
    let epslison = 0.00001;
    assert!(
        (expected - actual).abs() < epslison,
        "expected: {} actual: {}",
        expected,
        actual
    );
}

mod example_steps {

    use super::*;

    steps!(MyWorld => {

         given regex r"(\w+\d?) ← (tuple|point|vector|color)\((.+)\)" (String, String, String) |world, name, class, data, _step| {
             let tuple = build_tuple(&class, &data);
             world.tuples.insert(name.clone(), tuple);
         };

         // When norm ← normalize(v)
         when regex r"(\w+\d?) ← normalize\((\w+\d?)\)" (String, String) |world, dst, src, _step| {
             let tuple = world.tuples[&src].normalize();
             world.tuples.insert(dst.clone(), tuple);
         };

        // a.x = 4.3
        then regex r"(\w+\d?)\.([w-z]|red|green|blue) = (.+)" (String, String, String)|world, var, field, data, _step| {
            let val = parse_rational(&data);
            match tuple_field(&world.tuples[&var], &field) {
                Ok(e) => assert_eq!(e, val),
                Err(e) => panic!("field error {:?}", e)
            }
        };

        then regex r"(\w+\d?) - (\w+\d?) = (tuple|point|vector|color)\((.*)\)" |world, matches, _step| {
            let first = &world.tuples[&matches[1]];
            let second = &world.tuples[&matches[2]];
            let tuple = build_tuple(&matches[3], &matches[4]);
            assert_eq!(tuple, first - second);
        };
        then regex r"(\w+\d?) \+ (\w+\d?) = (tuple|point|vector|color)\((.*)\)" |world, matches, _step| {
            let first = &world.tuples[&matches[1]];
            let second = &world.tuples[&matches[2]];
            let tuple = build_tuple(&matches[3], &matches[4]);
            assert_eq!(tuple, first + second);
        };
        // `Then p = tuple(4, -4, 3, 1)`
        then regex r"^(\w+\d?) = (tuple|point|vector)\((.*)\)" |world, matches, _step| {
            let name = &matches[1];
            let tuple = build_tuple(&matches[2], &matches[3]);
            assert_eq!(world.tuples[name], tuple);
        };

        // Then -a = tuple(-1, 2, -3, 4)
        then regex r"^-(\w*\d?) = (tuple|point|vector)\((.*)\)" |world, matches, _step| {
            let name = &matches[1];
            let tuple = build_tuple(&matches[2], &matches[3]);
            assert_eq!(-&world.tuples[name], tuple);
        };

        //  And a is a not point
        then regex r"(\w+\d?) is a point" |world, matches, _step| {
            assert!(world.tuples[&matches[1]].is_point())
        };

        //  And a is a not point
        then regex r"(\w+\d?) is not a point" |world, matches, _step| {
            assert!(!world.tuples[&matches[1]].is_point())
        };

        then regex r"(\w+\d?) is a vector" |world, matches, _step| {
            assert!(world.tuples[&matches[1]].is_vector())
        };

        then regex r"(\w+\d?) is not a vector" |world, matches, _step| {
            assert!(!world.tuples[&matches[1]].is_vector())
        };

        // Then a * 3.5 = tuple(3.5, -7, 10.5, -14)
        then regex r"^(\w+\d?) \* (\d+(\.\d+)?) = (tuple|point|vector|color)\((.+)\)" |world, matches, _step| {
            let name = &matches[1];
            let scalar = parse_float(&matches[2]);
            let tuple = build_tuple(&matches[4], &matches[5]);
            assert_eq!(tuple, &world.tuples[name] * scalar);
        };

        // Then c1 * c2 = color(0.9, 0.2, 0.04)
        then regex r"^(\w+\d?) \* (\D+\d?) = (tuple|point|vector|color)\((.+)\)" (String, String, String, String) |world, c1, c2, class, data, _step| {
            let tuple = build_tuple(&class, &data);
            assert_eq!(tuple, &world.tuples[&c1] * &world.tuples[&c2]);
        };

        // Then a / 3.5 = tuple(3.5, -7, 10.5, -14)
        then regex r"^(\w+\d?) / (\d+(\.\d+)?) = (tuple|point|vector)\((.+)\)" |world, matches, _step| {
            let name = &matches[1];
            let scalar = parse_float(&matches[2]);
            let tuple = build_tuple(&matches[4], &matches[5]);
            assert_eq!(tuple, &world.tuples[name] / scalar);
        };

        // Then magnitude(v) = 1
        then regex r"^magnitude\((\w+\d?)\) = (\S+)" (String, String) |world, name, mag, _step| {
            approx_equal(parse_float(&mag), world.tuples[&name].magnitude());
        };

        // Then normalize(v) = vector(1, 0, 0)
        then regex r"^normalize\((\w+\d?)\) = (?:approximately\s)?(vector)\((.+)\)" (String, String, String) |world, name, class, data, _step| {
            let tuple = build_tuple(&class, &data);
            let n = world.tuples[&name].normalize();
            assert_eq!(n, tuple);
        };


        // Then dot(a, b) = 20
        then regex r"^dot\((\w+\d?), (\w+\d?)\) = (\S+)" (String, String, String) |world, a, b, mag, _step| {
            approx_equal(parse_float(&mag), world.tuples[&a].dot(&world.tuples[&b]));
        };

        // Then cross(a, b) = vector(-1, 2, -1)
        then regex r"^cross\((\w+\d?), (\w+\d?)\) = (tuple|point|vector)\((.+)\)" (String, String, String, String) |world, a, b, class, data, _step| {
            let expected = build_tuple(&class, &data);
            let actual = world.tuples[&a].cross(&world.tuples[&b]);
            assert_eq!(expected, actual);
        };

        // then regex r"^we can also match (\d+) (.+) types$" (usize, String) |world, num, word, step| {
        //     // `num` will be of type usize, `word` of type String
        //     assert_eq!(num, 42);
        //     assert_eq!(word, "olika");
        // };

        // then "we can use data tables to provide more parameters" |world, step| {
        //     let table = step.table().unwrap().clone();

        //     assert_eq!(table.header, vec!["key", "value"]);

        //     let expected_keys: Vec<String> = table.rows.into_iter().map(|x| x[0].to_owned()).collect();
        //     let expected_values: Vec<String> = table.rows.into_iter().map(|x| x[1].to_owned()).collect();

        //     assert_eq!(expected_keys, vec!["a", "b"]);
        //     assert_eq!(expected_values, vec!["fizz", "buzz"]);
        // }
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./features", // Path to our feature files
    world: MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}
