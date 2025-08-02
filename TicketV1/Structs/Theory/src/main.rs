fn main() {
    // put your code here to launch it
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn is_open(self)-> bool {
            self.status == "Open"
        }

        fn default() -> Ticket {
            Ticket {title: "".to_string(), description: "".to_string(), status: "".to_string()}
        }


    }


}
