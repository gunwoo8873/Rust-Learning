#[derive(Debug)]
struct User
{
    username: String,
    email: String,
    password: String,
    active: bool,
}

fn main()
{
    instance();
}


fn instance()
{
    let user_field = User
    {
        username: String::from("User"),
        email: String::from("user@outlook.kr"),
        password: String::from("********"),
        active: true,
    };

    println!("User Name : {:?}", user_field.username);
    println!("User Email : {:?}", user_field.email);
    println!("User Password : {:?}", user_field.password);
    println!("User Active : {:?}", user_field.active);
}