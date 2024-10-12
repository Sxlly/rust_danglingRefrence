



fn main() { //declare main method

    fn dangling_ref() -> &String { // decalre new function -> dangling_ref that returns string refrence

        let new_string = String::from("hello"); //declare news_string has new string variable with contents "hello"

        return &new_string; //return a refrence -> to String -> new_string

    } // at this position new_String is out of the scope and is deallocated from memory

    let new_string: &String = dangling_ref(); // declare new variable new_string -> that is a refrence to a string -> which comes from function -> which returns string refrence that does not exist due to deallocation
    println!("{}", new_string); //the scope of dangling_ref has ended, but we still have the refrence -> error
    
}
