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
    let mut input_string = String::new();
    let input_int = 0;
    let mut name_in = String::from("");
    let mut color_in = String::from("");
    let mut size_in = String::from("");
    /////////////////////////////////////////////////////////////////////////

    
    println!("Welcome to Window Shopping Simulator!");

    while input_int != 3
    {
        println!("Select an option:");
        println!("1: Add item to cart.");
        println!("2: View cart.");
        println!("3: Quit");
        std::io::stdin().read_line( &mut input_string).expect("Didn't receive input.");
        let input_int: i32 = input_string.trim().parse().expect("invalid input");
        /////////////////////////////////////////////////////////////////////////
        if input_int == 1 
        {
            println!("What is the name of your item?");
            std::io::stdin().read_line( &mut name_in).expect("Didn't receive input.");
            println!("What is the color of your item?");
            std::io::stdin().read_line( &mut color_in).expect("Didn't receive input.");
            println!("What is the size of your item?");
            std::io::stdin().read_line( &mut size_in).expect("Didn't receive input.");

            
            let temp_item: Item = Item{
                name: String::from("test"),
                color: String::from("test"),
                size: String::from("test")
            };
            cart.push(temp_item);
        }
        /////////////////////////////////////////////////////////////////////////
        else if input_int == 2
        {
            println!("Cart:");
            for item in &cart
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
