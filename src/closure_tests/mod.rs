use std::collections::HashMap;

fn call_a_closure<F> (closure: F, arg: u8) -> u8 where F: Fn(u8) -> u8 {
    closure(arg)
}

fn call_a_closure_without_args<F> (closure: F) where F: Fn() -> () {
    closure();
}

struct ClosureCaller {
    closures: HashMap< String,Box<Fn() -> ()> >,
}

impl ClosureCaller {
    fn new () -> ClosureCaller {
        ClosureCaller {
            closures: HashMap::new(),
        }
    }
    fn register_closure ( &mut self, s: &str, closure: Box<Fn() -> ()> ) {
        self.closures.insert(s.to_string(),closure);
    }
    fn take_action (&self, s: &str) {
        if let Some(closure) = self.closures.get(s) {
            (*closure)();
        } else {
            println!("404 Not Found");
        }
    }
}

struct ClosureCallerPartDeux<'a> {
    closures: HashMap<&'a str,&'a Fn() -> ()>,
}

impl<'a> ClosureCallerPartDeux<'a> {
    fn new () -> ClosureCallerPartDeux<'a> {
        ClosureCallerPartDeux { closures: HashMap::new() }
    }
    fn register_action ( &mut self, s: &'a str, f: &'a Fn() -> () ) {
        self.closures.insert(s,f.clone());
    }
    fn take_action ( &self, s: &str ) {
        if let Some(closure) = self.closures.get(s) {
            closure();
        } else {
            println!("no action for {}",s);
        }
    }
}

struct ClosureCallerPart3 {
    closures: HashMap< String,Box<Fn() -> ()> >,
}

impl ClosureCallerPart3 {
    fn new () -> ClosureCallerPart3 {
        ClosureCallerPart3 { closures: HashMap::new() }
    }
    fn register_action<F> ( &mut self, s: &str, f: &'static F ) where F: Fn() -> () {
        self.closures.insert( s.to_string(),Box::new(f) );
    }
    fn take_action ( &self, s: &str ) {
        if let Some(closure) = self.closures.get(s) {
            (*closure)();
        } else {
            println!("no action for {}",s);
        }
    }
}

pub fn execute () {
    let times_two = |x: u8| x * 2;
    assert_eq!(4,times_two(2));

    let mut number: u8 = 12;
    let clos1 = |x: u8| x + number;
    // let a: &mut u8 = &mut number; CANNOT DO THIS. THE CLOSURE BORROWS NUMBER.
    assert_eq!(20,clos1(8));

    let vec = vec![1,2,3];
    let take_ownership = || vec;
    // println!("{:?}",vec); CANNOT DO THIS. THE CLOSURE TAKES OWNERSHIP.
    
    let mut number: u8 = 16;
    {
        let mut add_number = |x: u8| number+=x;
        add_number(8);
    }
    assert_eq!(24,number);

    let mut number: u8 = 32;
    {
        let mut add_number = move |x: u8| number+=x;
        add_number(8);
    }
    assert_eq!(32,number);

    assert_eq!(call_a_closure(|x: u8| x + 1, 1), 2);

    call_a_closure_without_args(|| println!("closure"));
    
    println!("------------------------------");

    let mut caller = ClosureCaller::new();
    let vec: Vec<&str> = vec![
        "GET /",
        "GET /home",
        "GET /login",
        "GET /",
        "GET /",
        "GET /home"
    ];
    caller.register_closure("GET /",Box::new( || {
        println!("(/): 200 OK");
    }));
    caller.register_closure("GET /home",Box::new( || {
        println!("(/home): 200 OK");
    }));
    for request_paths in &vec {
        caller.take_action(request_paths);
    }

    println!("------------------------------");

    let a = || println!("called 'GET /'");
    let b = || println!("called 'GET /home'");
    let mut caller = ClosureCallerPartDeux::new();
    caller.register_action("GET /",&a);
    caller.register_action("GET /home",&b);

    for request_paths in &vec {
        caller.take_action(request_paths);
    }
    
    println!("------------------------------");

    //let mut caller = ClosureCallerPart3::new();
    //caller.register_action( "GET /", &'static (|| println!("getting /")) );

    //for request_path in &vec {
    //    caller.take_action(request_path);
    //}
}
