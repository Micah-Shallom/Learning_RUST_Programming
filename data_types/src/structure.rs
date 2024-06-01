use std::fmt::format;

/*
        STRUCTURES
        a collection of key-valaue pairs
        synonymous to a class in python or structs in golang
    */
fn main() {
    structure();
}


pub fn structure() {
    #[derive(Debug)]
    struct Employee {
        name: String,
        company: String,
        age: u32
    }

    let emp = Employee{
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    println!("{:?}", emp);
    println!("{}", emp.name);

    //adding a method to a structure
    impl Employee{
        fn fn_details(&self) -> String {
            format!("name:{}, age:{}, company:{}", &self.name, &self.age, &self.company)
        }
        //a structure can have a static method which doesnt take the self parameter

        fn static_fn_detail() -> String{
            return String::from("Details of a person");
        }
    }
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn_detail())



}