#[derive(Debug)]
struct User
{
    username: String,
    email: String,
    password: String,
    active: bool,
}
// Note : struct를 구조체라는 Field 명칭이 있다. 한 가지의 Data type을 가지는 것이 아닌 각 다른 type을 명시가 가능하다.

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
            ..user_field_a // user_field_a의 나머지 Instance Field를 가져온다. [단 Value가 동일하다 라는 전제하에 권장]
    };

    println!("Field B");
    println!("User Name     : {:?}", user_field_b.username);
    println!("User Email    : {:?}", user_field_b.email);
    println!("User Password : {:?}", user_field_b.password);  // Result : Not Update
    println!("User Active   : {:?}", user_field_b.active);    // Result : Not Update
}
// Note : 이 코드는 Instance라고 칭하며 각 필드에 대한 구체적인 값을 명시하여 Key:Value 형식으로 추가해야 한다.
//        하지만 반드시 struct를 정의했을 때 동일하지 않아도 된다.

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

enum Iptype
{
    V4,
    V6,
}

struct Ipv4
{
    kind: Iptype,
    address: String,
    port: u16,
}

struct Ipv6
{
    kind: Iptype,
    address: String,
}

fn enum_fn()
{
    let ipv4 = Ipv4
    {
        kind: Iptype::V4,
        address: String::from("127.0.0.1"),
        port: 8888,
    };

    let ipv6 = Ipv6
    {
        kind: Iptype::V6,
        address: String::from("::1"),
    };
}

fn enum_match(ip: Iptype) -> String
{
    match ip
    {
        Iptype::V4 => String::from("V4"),
        Iptype::V6 => String::from("V6"),
    }
}