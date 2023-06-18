mod pizza_order {
    pub struct Pizza {
        pub size: String,
        pub toppings: Vec<String>,
        pub crust: String,
        pub cheese: String,
        pub dough: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                size: String::from("small"),
                toppings: vec![String::from(topping)],
                crust: String::from("thin"),
                cheese: String::from("mozzarella"),
                dough: String::from("regular"),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Please have a seat at the table.");
        }

        pub fn take_order() {
            seat_at_table();
            println!("What would you like to order?");
            let cust_pizza: super::Pizza = super::Pizza::lunch("pepperoni");
            serve_order(cust_pizza);
        }

        fn serve_order(pizza: super::Pizza) {
            println!("Here is your order!");
            println!(
                "{} {} pizza with {} crust, {} cheese, and {} dough.",
                pizza.size,
                pizza.toppings.join(", "),
                pizza.crust,
                pizza.cheese,
                pizza.dough
            );
        }

        pub fn take_payment() {
            println!("That will be $10.00");
        }

        pub fn thank_customer() {
            println!("Thank you for your business!");
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
    crate::restaurant::pizza_order::help_customer::take_payment();
    crate::restaurant::pizza_order::help_customer::thank_customer();
}
