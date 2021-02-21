#[derive(Debug)]
pub struct Product<'a, 'b, 'c> {
    pub name: &'a str,
    pub price: &'b u128,
    pub is_public: &'c bool,
}

pub trait CMS {
    fn public_product(&mut self);
    fn unpublic_product(&mut self);
}

impl<'a, 'b, 'c> CMS for Product<'a, 'b, 'c> {
    fn public_product(&mut self) {
        self.is_public = &true;
    }

    fn unpublic_product(&mut self) {
        self.is_public = &false;
    }
}
