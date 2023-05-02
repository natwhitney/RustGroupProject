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
    let mut input_string;
    let mut input_int = 0;
    //let mut name_in = String::new();
    //let mut color_in = String::new();
    //let mut size_in = String::new();
    /////////////////////////////////////////////////////////////////////////

    println!("");
    println!("Welcome to Window Shopping Simulator!");
    println!("-----------------------------------------------");

    while input_int != 3
    {   
        println!("Select an option:");
        println!("1: Add item to cart.");
        println!("2: View cart.");
        println!("3: Quit");
        input_string = ("").to_string();
        std::io::stdin().read_line( &mut input_string).expect("Didn't receive input.");
        input_int = input_string.trim().parse().expect("invalid input");
        println!("");

        /////////////////////////////////////////////////////////////////////////
        if input_int == 1 
        {
            let mut name_in = ("").to_string();
            let mut color_in = ("").to_string();
            let mut size_in = ("").to_string();
            println!("What item did you want to add to cart? (EX: Car, House, Bike, etc)");
            std::io::stdin().read_line( &mut name_in).expect("Didn't receive input.");
            println!("What is the color of your item?");
            std::io::stdin().read_line( &mut color_in).expect("Didn't receive input.");
            println!("What is the size of your item?");
            std::io::stdin().read_line( &mut size_in).expect("Didn't receive input.");

            
            let temp_item: Item = Item{
                name: name_in.to_string(),
                color: color_in.to_string(),
                size: size_in.to_string()
            };
            cart.push(temp_item);
        }
        /////////////////////////////////////////////////////////////////////////
        else if input_int == 2
        {
            println!("Cart:");
            println!("--------------------");
            for item in &cart
            {
                //println!("Item {?:} in cart:", item);
                println!("Item : {}", item.name);
                println!("Color : {}", item.color);
                println!("Size : {}", item.size);
                println!("--------------------");
            }
        }
        else if input_int != 3
        {
            println!("Invalid Input");
        }
        println!("-----------------------------------------------");

    }

     println!("Ending program. Goodbye!");

}
