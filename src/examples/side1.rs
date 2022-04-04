struct Color {
    red: u8, green: u8, blue: u8
}

struct Person {
    first_name: String, last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    } 

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

//Enum (types with definite values (?))
enum Movement {
    //Variants
    Up(bool),
    Down
}

fn moving(m: Movement) {
    // switch
    match m {
        Movement::Up(boolean)    => println!("This is up: {}", boolean),
        Movement::Down  => println!("This is dn")
    }
}


pub fn run() {

    moving(Movement::Up(true));
    moving(Movement::Down);
    
    //function
    println!("This is a side1 function! Double: {}", double(3));

    // so-called "closure" function (allows outside variables?)
    let one: u8 = 1;
    let double_plus_one = |num: u8| num + num + one;
    println!("This is a side1 function! Double+1 {}", double_plus_one(3));

    // traditional struct
    let mut some_color = Color {
        red: 0, green: 255, blue: 0
    };
    some_color.green -= 1;
    println!("RGB: {} {} {}", some_color.red, some_color.green, some_color.blue);

    // tuple struct
    struct Color2(u8, u8, u8);
    let mut some_color2 = Color2(255, 0, 0);
    some_color2.0 -= 1;
    println!("RGB2: {} {} {}", some_color2.0, some_color2.1, some_color2.2);

    // struct with functions
    let mut some_guy = Person::new("John", "Doe");
    println!("{} {}", some_guy.first_name, some_guy.last_name);
    some_guy.set_last_name("Doe2");
    println!("{}", some_guy.name());
    some_guy.set_last_name("Doe3");
    println!("{:?}", some_guy.to_tuple());
    
}

// function
fn double(num : u8) -> u8 {
    num + num
}






