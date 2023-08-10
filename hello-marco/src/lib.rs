/* 
Marco Polo Game
A command-line tool to play Marco Polo. 
If user enter Marco, the program will return Polo. 
If user any other name, the program will return the name back.
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        name.to_string()
    }
}
