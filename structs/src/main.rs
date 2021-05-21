#[derive(Debug)]
struct Account {
    username : String,
    password : String
}

impl Account {
    fn new(username : &str, password : &str) -> Account {
        return Account {
            username: String::from(username),
            password: String::from(password)
        };
    }

    fn print(&self) {
        println!("Username: {}\nPassword: {}", self.username, self.password);
    }
}

fn main() {
    let acc1 = Account {
        username: String::from("John"),
        password: String::from("Hunter1")
    };

    print_struct_1(&acc1);
    print_struct_2(&acc1);
    acc1.print();


    let acc2 = make_account("Jane", "Princess2");

    print_struct_1(&acc2);
    print_struct_2(&acc2);
    acc2.print();


    let acc3 = Account {
        username: String::from("Carl"),
        ..acc1
    };

    print_struct_1(&acc3);
    print_struct_2(&acc3);
    acc3.print();

    let acc4 = Account::new("Anna", "Conda17");

    print_struct_1(&acc4);
    print_struct_2(&acc4);
    acc4.print();
}

fn make_account(username : &str, password : &str) -> Account {
    return Account {
        username: String::from(username),
        password: String::from(password)
    };
}

fn print_struct_1(acc : &Account) {
    println!("Username: {}\nPassword: {}", acc.username, acc.password);
}

fn print_struct_2(acc : &Account) {
    println!("{:?}", acc);
}
