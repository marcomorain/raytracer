#[macro_use]
extern crate cucumber_rust;

use raytracer::tuple;
use std::str::FromStr;
use std::error::Error;
use std::string::String;

fn build_tuple(s: &[String]) -> Result<tuple::Tuple, Box<Error>> {
    let mut result = try!(tuple::Tuple(
            s[1].parse::<f32>()?,
            s[2].parse::<f32>()?,
            s[3].parse::<f32>()?,
            s[4].parse::<f32>()?));
            //return result;
}

#[derive(Debug)]
pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    foo: String
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            foo: "a default string".to_string()
        }
    }
}

mod example_steps {

    use super::*;

    // Any type that implements cucumber_rust::World + Default can be the world
    steps!(MyWorld => {

         given regex r"(\w*) ← tuple\((\S+) (\S+) (\S+) (\S+)\)" |world, matches, step| {
             // 8(4.3, -4.2, 3.1, 0.0)

             let name = matches[1];
             let t = tuple(
                 matches[2].parse::<f32>()?,
                 matches[3].parse::<f32>()?,
                 matches[4].parse::<f32>()?,
                 matches[5].parse::<f32>()?,
             );


             println!("the tuple {:?} is {:?}", name, t);

         };
         given "I am trying out Cucumber" |world, step| {
             world.foo = "Some string".to_string();
             format!("step = {0}", "foo");
             // Set up your context in given steps
         };

        // when "I consider what I am doing" |world, step| {
        //     // Take actions
        //     let new_string = format!("{}.", &world.foo);
        //     world.foo = new_string;
        // };

        // then "I am interested in ATDD" |world, step| {
        //     // Check that the outcomes to be observed have occurred
        //     assert_eq!(world.foo, "Some string.");
        // };

        // then regex r"^we can (.*) rules with regex$" |world, matches, step| {
        //     // And access them as an array
        //     assert_eq!(matches[1], "implement");
        // };

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
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {

}

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
