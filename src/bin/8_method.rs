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
    tuple();
}


fn instance()
{
    let mut user_field_a = User
    {
        username: String::from("User"),
        email: String::from("user@outlook.kr"),
        password: String::from("********"),
        active: true,
    };

    //// Instance Value update
    user_field_a.username = String::from("User2");        // Update User -> User2
    user_field_a.email = String::from("user2@gmail.com"); // Update outlook.kr -> gmail.com

    println!("Field A");
    println!("User Name     : {:?}", user_field_a.username); // Result User -> User2
    println!("User Email    : {:?}", user_field_a.email);    // Result outlook.kr -> gmail.com
    println!("User Password : {:?}", user_field_a.password); // Result : Not Update
    println!("User Active   : {:?}", user_field_a.active);   // Result : Not Update


    let mut user_field_b = User
    {
        username: String::from("User3"),
        email: String::from("user3@github.com"),
            ..user_field_a // user_field_a의 나머지 Instance Field를 가져온다.
    };

    println!("Field B");
    println!("User Name     : {:?}", user_field_b.username);
    println!("User Email    : {:?}", user_field_b.email);
    println!("User Password : {:?}", user_field_b.password);  // Result : Not Update
    println!("User Active   : {:?}", user_field_b.active);    // Result : Not Update
}

fn short(username: String, email: String, password: String) -> User
{
    User
    {
        username: String::from(username),
        email: String::from(email),
        password: String::from(password),
        active: true,
    }
}

struct RGB(i32, i32, i32); // x, y, z
impl RGB
{
    fn get_hex(&self) -> String
    {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}
// Note : {:02X} : 16진수 형식으로 변환하는 포멧 지정자이다.
//        [:] : 포멧 지정자의 시작을 나타낸다.
//        02  : 2자리로 제한하여 출력하지만, 빈 자리는 0으로 자동으로 채운다.
//        X   : 16진수로 출력하고 문자열을 대문자로 변환 후 출력

fn tuple()
{
    let rgb = RGB(255, 255, 255);
    let hex = rgb.get_hex(); // rgb의 값을 가져와 get_hex의 포멧팅 형식으로 변환하여 값을 저장한다.

    println!("{}", hex);
}