use std::ops::Add;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Add for Person {
    type Output = Marriage;

    fn add(self, rhs: Self) -> Self::Output {
        Marriage {
            husband: self,
            wife: rhs,
            location: "Arizona".to_string(),
            date: chrono::offset::Local::now().date_naive(),
        }
    }
}

#[derive(Debug)]
struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::NaiveDate,
}

#[derive(Debug)]
struct GroceryItem {
    name: String,
    price: f32,
}

#[derive(Debug)]
struct GroceryBill {
    items: Vec<GroceryItem>,
    tax_rate: f32,
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;

    fn add(self, rhs: GroceryItem) -> Self::Output {
        let mut bill = self;
        bill.items.push(rhs);
        bill
    }
}

impl GroceryBill {
    fn calculate_total(&self) -> f32 {
        let items_total = self.items.iter().fold(0f32, |acc, item| acc + item.price);
        let tax = items_total * self.tax_rate;
        items_total + tax
    }
}

fn main() {
    let person01 = Person {
        first_name: "James".to_string(),
        last_name: "William".to_string(),
    };
    let person02 = Person {
        first_name: "Nancy".to_string(),
        last_name: "Smith".to_string(),
    };
    let marriage = person01 + person02;
    println!(
        "{} got married to {} on {}",
        marriage.husband.first_name, marriage.wife.first_name, marriage.date
    );

    let mut bill = GroceryBill {
        items: Vec::new(),
        tax_rate: 0.027,
    };

    let carrots = GroceryItem {
        name: "Bag of Carrots 1 pound".to_string(),
        price: 2.2,
    };

    let cheese = GroceryItem {
        name: "Cottage Cheese 12oz".to_string(),
        price: 3.4,
    };

    bill = bill + carrots + cheese;
    println!("The total of your grocery bill is {}", bill.calculate_total());
}
