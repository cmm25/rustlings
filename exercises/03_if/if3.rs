pub fn animal_habitat(animal: &str) -> &'static str {
    // Determine the identifier for the animal
    let identifier = if animal == "crab" {
        "1"
    } else if animal == "gopher" {
        "2"
    } else if animal == "snake" {
        "3"
    } else {
        "Unknown"
    };

    // Determine the habitat based on the identifier
    let habitat = if identifier == "1" {
        "Beach"
    } else if identifier == "2" {
        "Burrow"
    } else if identifier == "3" {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

fn main() {
    // Print out the habitats for specific animals
    println!("{}", animal_habitat("crab"));
    println!("{}", animal_habitat("gopher"));
    println!("{}", animal_habitat("snake"));
    println!("{}", animal_habitat("dinosaur"));
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
