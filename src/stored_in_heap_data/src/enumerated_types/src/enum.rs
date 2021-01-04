#[derive(Debug)]
struct Address {
    street: String,
    district: String,
    ward: i8,
    city: String,
}

#[derive(Debug)]
enum Person {
    Age(i8),
    Name(String),
    Lat(f32),
    Long(f32),
    Address(Address),
}

#[derive(Debug)]
enum SyncStatus {
    Connected,
    Disconnected,
}

// bind value
enum ProductVariant {
    SyncPrice(SyncStatus),
    SyncStock(SyncStatus),
}

fn main() {
    let my_age = Person::Age(26);
    let my_name = Person::Name(String::from("Dang Duoc"));
    let lat = Person::Lat(14.369860);
    let long = Person::Long(108.006020);
    let addr = Person::Address(Address {
        street: String::from("Nhat Chi Mai"),
        district: String::from("Tan Binh"),
        ward: 13,
        city: String::from("Ho Chi Minh"),
    });

    println!("ads{:?}", Person::Age(123));
    println!("What the hell {}", defind_status(SyncStatus::Disconnected));

    println!(
        "asda, {:?}",
        get_status(ProductVariant::SyncPrice(SyncStatus::Disconnected))
    );

    println!("plus_one: {:?}", plus_one(Option::Some(10)));

    let n: i32 = 20;
    println!("hix{}", n);
    match n {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let syntax

    if let Person::Age(20) = my_age {
        println!("age = 20")
    } else {
        println!("{:#?}", my_age)
    }
}

fn defind_status(syncStatus: SyncStatus) -> i32 {
    match syncStatus {
        SyncStatus::Disconnected => {
            println!("Test");
            0
        }
        SyncStatus::Connected => 1,
    }
}

fn get_status(productVariant: ProductVariant) -> i32 {
    match productVariant {
        ProductVariant::SyncPrice(syncStatus) => {
            println!("{:?}", syncStatus);
            32
        }

        // bind value
        ProductVariant::SyncStock(syncStatus) => {
            println!("{:?}", syncStatus);
            32
        }
    }
}

fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        None => None,
        Some(i) => Some(i + 10),
    }
}
