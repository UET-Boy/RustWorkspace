fn main() {
    println!("Using git without fear!");
    let student1 = Student::Public(String::from("Hamza"));
    let student2 = Student::Private(String::from("Faisal"));
    let student3 = Student::Overseas(String::from("Imran"));

    println!("{:?}", student1);
    println!("{:?}", student2);
    println!("{:?}", student3);
}

#[derive(Debug)]

enum Student{
    Private(String),
    Public(String),
    Overseas(String),
}