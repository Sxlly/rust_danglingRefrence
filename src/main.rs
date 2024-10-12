fn main() { //declare main method

    fn dangling_ref() -> &String { // decalre new function -> dangling_ref that returns string refrence

        let new_string = String::from("hello"); //declare news_string has new string variable with contents "hello"

        &new_string;

    } // at this position new_String is out of the scope and is deallocated from memory

    let new_string: &String = dangling_ref();
    println!("{}", new_string);
    
}
