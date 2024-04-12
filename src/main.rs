use std::{
    cmp::Ordering,
    io::{self, Write},
};

#[derive(Clone)]
struct Scientist<'a> {
    first_name: &'a str,
    last_name: &'a str,
}
fn ingest() -> String {
    let _ = io::stdout().flush();
    let mut input: String = String::new();
    input.clear();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => println!("could not parse input: {}", err),
    }
    return input.trim().to_string();
}

fn binary_search(arr: &[Scientist], target: &str) -> Option<usize> {
    let target = target.to_lowercase();
    let mut min: usize = 0;
    let mut max: usize = arr.len() - 1;
    while max >= min {
        let mid: usize = (min + max) / 2;
        match arr[mid].last_name.to_lowercase().cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => min = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    break;
                } else {
                    max = mid - 1
                }
            }
        }
    }
    return None;
}

fn main() {
    let scientist_names: [Scientist; 10] = [
        Scientist {
            first_name: "Tim",
            last_name: "Berners-Lee",
        },
        Scientist {
            first_name: "Brendan",
            last_name: "Eich",
        },
        Scientist {
            first_name: "Bill",
            last_name: "Gates",
        },
        Scientist {
            first_name: "Hedy",
            last_name: "Lamarr",
        },
        Scientist {
            first_name: "Barbara",
            last_name: "Liskov",
        },
        Scientist {
            first_name: "Larry",
            last_name: "Page",
        },
        Scientist {
            first_name: "Steve",
            last_name: "Wozniak",
        },
        Scientist {
            first_name: "Carl",
            last_name: "Sassenrath",
        },
        Scientist {
            first_name: "Guido",
            last_name: "Van-Rassum",
        },
        Scientist {
            first_name: "Linus",
            last_name: "Torvalds",
        },
    ];
    let mut scientists_vec: Vec<Scientist> = scientist_names.iter().cloned().collect();
    scientists_vec.sort_by(|a, b| a.last_name.cmp(&b.last_name));
    let sorted_scientists: [Scientist; 10] =
        scientists_vec
            .try_into()
            .unwrap_or_else(|v: Vec<Scientist>| {
                panic!("Expected a Vec of length {} but it was {}", 10, v.len())
            });

    print!("Please enter the name of a computer scientist: ");
    let input_fullname: String = ingest().parse().expect("could not parse input");
    let lowercase_name: String = input_fullname.to_lowercase();
    let last_name: &str = lowercase_name.split_whitespace().last().unwrap();
    let found_index: Option<usize> = binary_search(&sorted_scientists, &last_name);
    match found_index {
        Some(index) => {
            println!(
                "{} {} Is a Computer Scientist and is in the list",
                sorted_scientists[index].first_name, sorted_scientists[index].last_name
            );
        }
        None => {
            println!("{} Is not a Computer Scientist in the list", input_fullname);
        }
    }
}
