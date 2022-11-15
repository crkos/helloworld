use std::error::Error;

///Logic for what the program will be mainly doing
pub fn run(person: Person) -> Result<(), Box<dyn Error>>{
    let name = person.name;
    let dob = person.dob;
    let nac = person.nac_state;
    println!("The name of the person is {}, dob is {} and birth state is {}", name, dob, nac);
    Ok(())
}

/// Struct person, 
pub struct Person {
    pub name: String,
    pub dob: String,
    pub nac_state: String
}

/// Implementation to a Struct "Person" goes here, all methods n' stuff
impl Person {
    /// Method new, Just constructs a new Object Person, and returns Self, Error handling is done
    /// inside the method in case something fails
    /// # Errors
    /// New panics if the provides args are less or equal to 3 this because if len is 3 but index 3
    /// will panic
    /// As well it will be panic if there are more than 4 args
    pub fn new(args: &[String]) -> Result<Self, &'static str>{
        if args.len() <= 3 {
            return Err("Not enought parameters");
        }

        if args.len() > 4 {
            return Err("Unnecesary parameters added, Run with exact parameters");
        }

        let name : String = args[1].clone();
        let dob : String = args[2].clone();
        let nac_state : String = args[3].clone();

        Ok(Self { name, dob, nac_state })
    }
}
