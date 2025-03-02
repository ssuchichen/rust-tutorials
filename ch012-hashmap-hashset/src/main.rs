use std::collections::{HashMap, HashSet};

fn printhm(hm: &HashMap<String, f32>) {
    println!("length: {}", hm.len());
    println!("capacity: {}", hm.capacity());
    println!("stock_list: {:#?}", hm);
}

fn prinths(hs: &HashSet<String>) {
    println!("length: {}", hs.len());
    println!("capacity: {}", hs.capacity());
    println!("stock_list: {:#?}", hs);
}

fn main() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    // let stock_list= HashMap::<String, f32>::new();

    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AAPL".to_string(), 232.92);
    stock_list.insert("AMSC".to_string(), 50.78);

    // 覆盖
    stock_list.insert("AAPL".to_string(), 233.47);

    // k存在就不覆盖，k不存在就插入新值
    stock_list.entry("META".to_string()).or_insert(346.34);
    stock_list.entry("META".to_string()).or_insert(358.34);

    printhm(&stock_list);
    stock_list.remove("AAPL");
    printhm(&stock_list);

    for (ticker, current_value) in stock_list {
        println!("{} is trading at {}", ticker, current_value);
    }

    println!("--------------------------------------------");

    let mut planets_list = HashSet::from([
        "Mercury".to_string(),
        "Venus".to_string(),
        "Earth".to_string(),
    ]);
    let planets_list_more = HashSet::from([
        "Earth".to_string(),
        "Mars".to_string(),
        "Jupiter".to_string(),
    ]);

    // 返回planets_list中与planets_list_more的差异部分
    let planets_diff = planets_list.difference(&planets_list_more);
    for planet in planets_diff {
        println!("Thanks for adding {}", planet);
    }

    // 将两个集合合并，在删除相同的元素
    let planets_sym_diff = planets_list.symmetric_difference(&planets_list_more);
    for planet in planets_sym_diff {
        println!("Thanks for adding {}", planet);
    }

    // 插入
    planets_list.insert("Saturn".to_string());
    planets_list.insert("Uranus".to_string());
    planets_list.insert("Neptune".to_string());
    planets_list.insert("Pluto".to_string());
    planets_list.insert("Pluto".to_string());
    planets_list.insert("Pluto".to_string());
    // 合并
    planets_list.extend(planets_list_more);

    prinths(&planets_list);
}
