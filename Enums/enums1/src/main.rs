fn main() {
    println!("Using git without fear!");
    let student1 = Student::Public(String::from("Hamza"));
    let student2 = Student::Private(String::from("Faisal"));
    let student3 = Student::Overseas(String::from("Imran"));
    let fee = Student::FeeSlab(20, 40, 80);

    println!("{:?}", student1);
    println!("{:?}", student2);
    println!("{:?}", student3);
    println!("{:?}", fee);
}

#[derive(Debug)]

enum Student{
    Private(String),
    Public(String),
    Overseas(String),
    FeeSlab(u8,u8,u8),
}