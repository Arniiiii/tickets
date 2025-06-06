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
    quantity: u64,
    unit_price: u64,
}

impl Order {
    fn validate_product_name(product_name: &String) -> Result<(), &'static str> {
        if product_name.is_empty() {
            return Err("The product name can't be empty");
        }
        if product_name.as_bytes().len() > 300 {
            return Err("The product name can't be longer than 300 bytes");
        }

        Ok(())
    }

    fn validate_quantity(quantity: &u64) -> Result<(), &'static str> {
        if *quantity <= 0 {
            return Err("The quantity must be strictly greater than zero.");
        }
        Ok(())
    }

    fn validate_unit_price(unit_price: &u64) -> Result<(), &'static str> {
        if *unit_price <= 0 {
            return Err("The unit price is in cents and must be strictly greater than zero.");
        }
        Ok(())
    }

    pub fn new(product_name: String, quantity: u64, unit_price: u64) -> Order {
        {
            Self::validate_product_name(&product_name).unwrap_or_else(|error| {
                panic!("{error}");
            });
            Self::validate_quantity(&quantity).unwrap_or_else(|error| {
                panic!("{error}");
            });
            Self::validate_unit_price(&unit_price).unwrap_or_else(|error| {
                panic!("{error}");
            });
            Order {
                product_name,
                quantity,
                unit_price,
            }
        }
    }
    pub fn total(&self) -> u64 {
        self.unit_price * self.quantity
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u64 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }
    pub fn set_product_name(&mut self, product_name: String) -> &mut Self {
        Self::validate_product_name(&product_name).unwrap_or_else(|error| {
            panic!("{error}");
        });
        self.product_name = product_name;
        self
    }
    pub fn set_quantity(&mut self, quantity: u64) -> &mut Self {
        Self::validate_quantity(&quantity).unwrap_or_else(|error| {
            panic!("{error}");
        });
        self.quantity = quantity;
        self
    }
    pub fn set_unit_price(&mut self, unit_price: u64) -> &mut Self {
        Self::validate_unit_price(&unit_price).unwrap_or_else(|error| {
            panic!("{error}");
        });
        self.unit_price = unit_price;
        self
    }
}
