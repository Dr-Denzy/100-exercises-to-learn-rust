// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn valid_product_name(name: &String) -> bool {
        name.len() > 0 && name.as_bytes().len() <= 300
    }

    fn valid_quantity(quantity: &u32) -> bool {
        quantity > &0
    }

    fn valid_quantity2(quantity: &u32) -> bool {
        quantity > &0
    }

    fn valid_unit_price(price: &u32) -> bool {
        price > &0
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if !Self::valid_product_name(&product_name) {
            panic!("invalid product name")
        }

        if !Self::valid_quantity(&quantity) {
            panic!("invalid quantity")
        }

        if !Self::valid_unit_price(&unit_price) {
            panic!("invalid unit price")
        }

        Self {
            product_name,
            quantity,
            unit_price,

        }
    }

    // accessors

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // mutators (mut self option)
    pub fn set_product_name1(mut self, new_product_name: String) -> Self {
        if !Self::valid_product_name(&new_product_name) {
            panic!("invalid product name")
        }
        self.product_name = new_product_name;
        self
    }

    pub fn set_quantity1(mut self, new_quantity: u32) -> Self {
        if !Self::valid_quantity(&new_quantity) {
            panic!("invalid quantity")
        }
        self.quantity = new_quantity;
        self
    }

    pub fn set_unit_price1(mut self, new_unit_price: u32) -> Self {
        if !Self::valid_unit_price(&new_unit_price) {
            panic!("invalid unit price")
        }
        self.unit_price = new_unit_price;
        self
    }

    // mutators (&mut self option)
    pub fn set_product_name(&mut self, new_product_name: String) {
        if !Self::valid_product_name(&new_product_name) {
            panic!("invalid product name")
        }
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        if !Self::valid_quantity(&new_quantity) {
            panic!("invalid quantity")
        }
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        if !Self::valid_unit_price(&new_unit_price) {
            panic!("invalid unit price")
        }
        self.unit_price = new_unit_price;
    }

    pub fn total(&self) -> u32 {
        &self.unit_price * &self.quantity
    }
}

