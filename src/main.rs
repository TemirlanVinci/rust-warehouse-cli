use std::io;

#[derive(Debug)]
struct Item {
    name: String,
    count: u32,
    price: u32,
}

impl Item {
    fn new(name: String, count: u32, price: u32) -> Self {
        Self { name, count, price }
    }
}

fn main() {
    let mut warehouse = Vec::new();

    loop {
        // Добавил отступы для красоты
        println!("\n---------------------------------");
        println!("WAREHOUSE MANAGER CLI");
        println!("1. Add Item");
        println!("2. View Warehouse");
        println!("3. Item Details");
        println!("4. Delete Item");
        println!("---------------------------------");
        println!("Enter command:");

        let mut state: String = String::new();
        io::stdin()
            .read_line(&mut state)
            .expect("Failed to read line");

        // Добавил проверку на пустой ввод, чтобы не крашилось сразу,
        // но оставил твой unwrap() как основной метод
        let state: u32 = match state.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please enter a number!");
                continue;
            }
        };

        match state {
            1 => {
                println!("\n[ADD ITEM]");

                println!("Enter item name:");
                let mut state_name: String = String::new();
                io::stdin()
                    .read_line(&mut state_name)
                    .expect("Failed to read name");

                println!("Enter quantity:");
                let mut state_count: String = String::new();
                io::stdin()
                    .read_line(&mut state_count)
                    .expect("Failed to read count");
                let state_count: u32 = state_count.trim().parse().unwrap();

                println!("Enter price:");
                let mut state_price: String = String::new();
                io::stdin()
                    .read_line(&mut state_price)
                    .expect("Failed to read price");
                let state_price: u32 = state_price.trim().parse().unwrap();

                let new_item = Item::new(state_name.trim().to_string(), state_count, state_price);
                warehouse.push(new_item);

                println!("Item added successfully!");
            }

            2 => {
                println!("\n[WAREHOUSE LIST]");
                // Вынес заголовок ДО цикла, чтобы он не дублировался
                if warehouse.is_empty() {
                    println!("The warehouse is empty.");
                } else {
                    for (index, item) in warehouse.iter().enumerate() {
                        // Добавил вывод индекса, чтобы было удобно удалять/смотреть детали
                        println!("{}. {}", index, item.name);
                    }
                }
            }

            3 => {
                println!("\n[ITEM DETAILS]");
                println!("Enter item index:");
                let mut state_info: String = String::new();
                io::stdin()
                    .read_line(&mut state_info)
                    .expect("Failed to read line");
                let state_info: usize = state_info.trim().parse().unwrap();

                if state_info < warehouse.len() {
                    let item = &warehouse[state_info];
                    println!("----------------");
                    println!("Name:     {}", item.name);
                    println!("Quantity: {}", item.count);
                    println!("Price:    {}", item.price);
                    println!("----------------");
                } else {
                    println!("Error: Item with index {} does not exist!", state_info);
                }
            }

            4 => {
                println!("\n[DELETE ITEM]");
                println!("Enter item index to delete:");
                let mut state_info: String = String::new();
                io::stdin()
                    .read_line(&mut state_info)
                    .expect("Failed to read line");
                let state_info: usize = state_info.trim().parse().unwrap();

                if state_info < warehouse.len() {
                    warehouse.remove(state_info);
                    println!("Item at index {} deleted.", state_info);
                } else {
                    println!("Error: Item with index {} does not exist!", state_info);
                }
            }

            _ => println!("Invalid command! Please enter 1-4."),
        }
    }
}
