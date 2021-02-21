use lifetimes::struct_with_lifetimes::{Product, CMS};

fn main() {
    let mut iphone_12_pro_max = Product {
        name: "iPhone 12 Pro Max",
        price: &1390000,
        is_public: &false,
    };

    iphone_12_pro_max.public_product();

    println!("{:#?}", iphone_12_pro_max);

    iphone_12_pro_max.unpublic_product();

    println!("{:#?}", iphone_12_pro_max);
}
