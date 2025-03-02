fn main() {
    let fruit_list = vec![
        "Strawberry",
        "Blueberry",
        "Banana",
        "Mango",
        "Orange",
        "Apple",
    ];
    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brazil Nuts"];

    let mut fruit_iter = fruit_list.iter();
    // for fruit in fruit_iter {
    //     println!("{}", fruit);
    // }

    // 迭代器的状态会随着元素的获取而发生变化，因此需要迭代器是可变的
    // next()方法用于返回迭代器指向的下一个元素，并将迭代器推进到下一个位置。
    // 这意味着每次调用next()都会修改迭代器内部的指针或索引，以确保下次调用next()时能够返回正确的下一个元素。
    let item0 = fruit_iter.next();
    println!("First item in iterator: {:?}", item0.unwrap());
    println!("--------------------------------------------");
    fruit_iter.next();
    println!("Third item in iterator: {:?}", fruit_iter.next());
    println!("--------------------------------------------");
    let aggregate_foods = fruit_list.iter().chain(nut_list.iter());
    let all_foods = aggregate_foods.clone().collect::<Vec<_>>();
    println!("{:?}", all_foods);
    println!("--------------------------------------------");
    for food in aggregate_foods {
        println!("Eating {}", food);
    }
    println!("--------------------------------------------");
    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));
    let new_fruits = fruit_list_strings.map(|mut e| {
        e.push_str(" fruit");
        e
    });
    // for fruit in new_fruits {
    //     println!("{}", fruit);
    // }
    new_fruits.clone().for_each(|e| println!("{}", e));
    println!("Last fruit is: {}", new_fruits.clone().last().unwrap());
    println!("--------------------------------------------");
    let mut stepby = new_fruits.clone().step_by(2);
    println!("{}", stepby.next().unwrap());
    println!("{}", stepby.next().unwrap());
    println!("{}", stepby.next().unwrap());
    println!("--------------------------------------------");
    let first_names = vec!["Alice", "Bob", "Charlie", "Dave"];
    let first_names_strings = first_names.iter().map(|e| String::from(*e));
    let last_names = vec!["Lee", "Wong", "Chang", "Hung", "Chu"];
    let last_names_strings = last_names.iter().map(|e| String::from(*e));
    let full_names = first_names_strings.zip(last_names_strings);
    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));
    println!("--------------------------------------------");
    for item in full_names.clone().enumerate() {
        println!("Index: {}, value: {} {}", item.0, item.1.0, item.1.1);
    }
    println!("--------------------------------------------");
    full_names
        .skip(1) // 跳过1个元素
        .take(2) // 取2个元素
        .for_each(|e| println!("Did not skip: {} {}", e.0, e.1));
    println!("--------------------------------------------");
    let foods = vec![("potatoes", 10), ("strawberries", 25), ("burgers", 31)];
    // 对迭代器中的所有元素执行累加操作，并返回一个最终的结果
    let food_quantity = foods.clone().iter().fold(0u32, |acc, e| acc + e.1);
    println!("Your total food quantity is: {}", food_quantity);

    let mut mypeekable = foods.iter().peekable();
    mypeekable.next();
    let food = mypeekable.peek();
    println!("Peeking at {}", food.unwrap().1);
}
