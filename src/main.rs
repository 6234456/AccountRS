fn main() {
}

struct Account{
    id: u16,
    name:String,
    value:u32,
    subs: Option<Vec<Account>>,
}

impl Account {
    fn new(id:u16, name:String, value:u32) -> Account {
        Account {
            id : id,
            name: name,
            value: value,
            subs: None,
        }
    }

    fn val(&mut self) -> u32 {
        match &mut self.subs {
          None      =>  self.value,
          Some(x)   =>  {
              if x.len() == 0 {
                self.value
              } else {
                let mut res : u32 = 0;

                for e in x {
                    res += e.val();
                }
                res
            }
          }
        }
    }

    fn add(&mut self, account:Account) -> &mut Account {
        match &mut self.subs {
          None      =>  self.subs = Some(vec!(account)),
          Some(x)   =>  x.push(account)
        }
        self
    }

    fn remove(&mut self, id:u16) -> &mut Account{
        match &mut self.subs {
            None    => (),
            Some(x) => {
               for i in 0..x.len() {
                   if x[i].id == id {
                       x.swap_remove(i);
                       if x.len() == 0 {
                       }
                   }
               }
            }
        }

        self
    }

    fn find(&mut self, id:u16) -> Option<&mut Account> {
        if self.id == id {
           return Some(self)
        }

        match &mut self.subs {
            None => None,
            Some(x) => {
                for e in x {
                    let tmp = e.find(id);

                    if let Some(_) = tmp {
                        return tmp;
                    }
                }

                return None;            
            }
        }
    }
}

#[test]
fn test_account( ){
    let mut a = Account::new(123, String::from("Demo"), 0);
    assert_eq!(a.value, 0);

    let mut b = Account::new(1232, String::from("name: String"), 0);
    b.add(Account::new(12321, String::from("demo2"), 12));

    a.add(Account::new(1231, String::from("s: Box<str>"), 13));
    a.add(b);

    assert_eq!(a.find(12321).unwrap().name, String::from("demo2"));
    println!("{}", a.val());
    assert_eq!(a.val(), 25);

    a.find(12321).unwrap().add(Account::new(1234, String::from("1234"), 13));
    assert_eq!(a.val(), 26);

    a.find(1234).unwrap().value = 1;
    assert_eq!(a.val(), 14);

    println!("{}", a.val());
}
