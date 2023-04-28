//Program 2 Mock Store
/*
    David Sullivan
    4/25/23
    An extremely simple store front that lets the user add items to a cart.
*/
use std::vec::Vec;

fn main() {
    #[derive(Debug)]
    struct Item{
        name: String,
        color: String,
        size: String,
    }

    let mut cart: Vec<Item> = Vec::new();
    let mut input = 0;
    let mut nameIn;
    let mut colorIn;
    let mut sizeIn;

  //  let mut test: Item = Item{
  //      name: String::from("test"),
  //      color: String::from("test"),
  //      size: String::from("test")
  //  };
    
    println!("Welcome to the Mock Store!");

    while input != 3 
    {
        println!("Select an option:");
        println!("1: Add item to cart.");
        println!("2: View cart.");
        println!("3: Quit");
        std::io::stdin().read_line( &mut input).expect("Didn't receive input.");

        if input == 1 
        {
            println!("What is the name of your item?");
            std::io::stdin().read_line( &mut nameIn).expect("Didn't receive input.");
            println!("What is the color of your item?");
            std::io::stdin().read_line( &mut colorIn).expect("Didn't receive input.");
            println!("What is the size of your item?");
            std::io::stdin().read_line( &mut sizeIn).expect("Didn't receive input.");

            
            let tempItem: Item = Item{
                name: String::from("test"),
                color: String::from("test"),
                size: String::from("test")
            };
            cart.push(tempItem);
        }
        else if input == 2
        {
            for item in cart
            {
                println!("Item {:?} in cart:", item);
                println!("Item : {}", item.name);
                println!("Color : {}", item.color);
                println!("Size : {}", item.size);
            }
        }
    }

     println!("Ending program. Goodbye!");

}
