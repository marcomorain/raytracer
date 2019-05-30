extern crate raytracer;

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

fn parse_floats(input: &str) -> Vec<f32> {
    let nums = input
        .split(", ")
        .filter_map(|s| s.parse::<f32>().ok())
        .collect::<Vec<_>>();
    return nums;
}

fn tuple_field(t: &tuple::Tuple, f: &String) -> Result<f32, Box<Error>> {
  return match f.as_ref() {
      "x" => Ok(t.0),
      "y" => Ok(t.1),
      "z" => Ok(t.2),
      "w" => Ok(t.3),
      _ => return Err("No matching field".into())
  }
}

fn build_point(world: &mut MyWorld, s: &[String]) -> Result<(), Box<Error>> {
    world.tuples.insert(
        s[1].clone(),
        tuple::point(
            s[2].parse::<f32>()?,
            s[3].parse::<f32>()?,
            s[4].parse::<f32>()?,
        ),
    );
    return Ok(());
}

fn build_tuple(world: &mut MyWorld, s: &[String]) -> Result<(), Box<Error>> {
    let name = &s[1];
    let class = &s[2];
    let vals = parse_floats(&s[3]);

    let t = match class.as_ref() {
        "tuple" => tuple::Tuple(vals[0], vals[1], vals[2], vals[3]),
        "point" => tuple::point(vals[0], vals[1], vals[2]),
        "vector" => tuple::vector(vals[0], vals[1], vals[2]),
        &_ => tuple::Tuple(0.0, 0.0, 0.0, 0.0)
    };

    world.tuples.insert(name.clone(), t);

    return Ok(());
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

mod example_steps {

    use super::*;

    steps!(MyWorld => {

         given regex r"(\w*) ← (tuple|point|vector)\((.*)\)" |world, matches, _step| {
             let parsed = build_tuple(world, matches);
             assert!(parsed.is_ok());
         };

        // a.x = 4.3
        then regex r"(\w*)\.([w-z]) = ([\-\+]?[0-9]*(\.[0-9]+)?)" |world, matches, _step| {
            let t =  &world.tuples[&matches[1]];

            let val = matches[3].parse::<f32>();

            if val.is_err() {
                panic!("parse error");
            } else {
                match tuple_field(&t, &matches[2]) {
                    Ok(e) => assert_eq!(e, val.unwrap()),
                    Err(e) => panic!("field error {:?}", e)
                }
            }
        };

        // `Then p = tuple(4, -4, 3, 1)`
        then regex r"(\w*) = tuple(4, -4, 3, 1)" |world, matches, _step| {
            let t =  &world.tuples[&matches[1]];
        };

        then regex r"(\w*) is a point" |world, matches, _step| {
            let name = &matches[1];
            match world.tuples.get(name) {
              Some(t) => assert!(tuple::is_point(t)),
              None => panic!("no tuple named {:?}", name)
            }
        };

        then regex r"(\w*) is not a point" |world, matches, _step| {
            let name = &matches[1];
            match world.tuples.get(name) {
              Some(t) => assert!(!tuple::is_point(t)),
              None => panic!("no tuple named {:?}", name)
            }
        };

        then regex r"(\w*) is a vector" |world, matches, _step| {
            let name = &matches[1];
            match world.tuples.get(name) {
              Some(t) => assert!(tuple::is_vector(t)),
              None => panic!("no tuple named {:?}", name)
            }
        };

        then regex r"(\w*) is not a vector" |world, matches, _step| {
            let name = &matches[1];
            match world.tuples.get(name) {
              Some(t) => assert!(!tuple::is_vector(t)),
              None => panic!("no tuple named {:?}", name)
            }
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
