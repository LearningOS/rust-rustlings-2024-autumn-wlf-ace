// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// struct ShoppingList<T> {
//     items: Vec<T>,
// }
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
// enum Item {
//     StringItem(String),
//     IntItem(i32),
//     FloatItem(f32),
// }
// fn main(){
//     let mut shopping_list: Vec<Item> = Vec::new();
//     shopping_list.push(Item::StringItem("milk".to_string()));
//     shopping_list.push(Item::IntItem(2));
//     shopping_list.push(Item::FloatItem(3.14));

//     for item in shopping_list{
//         match item{
//             Item::StringItem(s) =>println!("String: {}", s),
//             Item::IntItem(i) =>println!("Int: {}", i),
//             Item::FloatItem(f) =>println!("Float: {}", f),
//         }
//     }
// }
