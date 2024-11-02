fn main()
{

}

struct UserData
{
    name: String,
    email: String,
    active: bool,
    address: String,
}

impl UserData
{
    fn new(&self) -> String
    {
        self.name
    }
}