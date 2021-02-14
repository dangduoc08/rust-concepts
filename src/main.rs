use traits::fn_accepts_traits_as_params;
use traits::traits::{American, Vietnamese};

fn main() {
    let nam: Vietnamese = Vietnamese {
        name: String::from("Nguyễn Thành Nam"),
        age: 20,
    };

    let john: American = American {
        name: "John Cena".to_string(),
        age: 48,
    };

    fn_accepts_traits_as_params::notify3(&nam);
    fn_accepts_traits_as_params::notify2(&john);
}
