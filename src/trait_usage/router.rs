use std::sync::Arc;
use std::collections::HashMap;

pub fn run(){
    println!("开始运行trait示例...");
    // let defaultAuth = AuthDefault::new();
    // let e = Engine{name: "".to_string(), auth: defaultAuth};
    // let e = Engine{name: "".to_string(), auth: defaultAuth};
    let mut eg: Engine<&dyn Fn(String) -> Result<String, EngineError>> = Engine::new("".to_string());
    eg.addRouter(1, &SendKey);
    eg.processRouter(1);
    // eg.authPro();

    
}

// trait Fn(String) -> Result<String, EngineError>;
// type Future: Fn(String) -> Result<String, EngineError>;

struct Engine<F>
{
    name: String,
    router: HashMap<u16, Box<F>>,//  Arc<F>,
    // auth: AuthTool<S>,
}


impl <F> Engine<F>
where F: Fn(String) -> Result<String, EngineError>,
{
    fn new(name: String) -> Self{
        // let pf = Arc::new(process_fn);
        // let defaultAuth = AuthDefault::new();
        // let authTool = AuthTool(defaultAuth);
        // let tool = AuthTool::from(defaultAuth);
        let m = HashMap::<u16, Box<F>>::new();
        // let am = Arc::new(m);
        Engine{
            name: name,
            router: m,
        }
    }
    fn addRouter(&mut self, msgid: u16, process_fn: F){
        self.router.insert(msgid, Box::new(process_fn));
    }

    fn processRouter(self, msgid: u16){
        let value = self.router.get(&msgid);
        let func = value.as_ref();
        match func{
            Some(f) => {
                f("nihao".to_string());
            }
            None => {}
        }
    }

    // fn authPro(self){
    //     // self.auth.RecvKey("hello".to_string());
    //     let pf = self.auth.as_ref();
    //     pf(self.name);
    // }
}

struct AuthTool<S: Auth>(S);


impl<S> From<S> for AuthTool<S>
where
    S: Auth,
{
    fn from(scheme: S) -> AuthTool<S> {
        AuthTool(scheme)
    }
}

impl<S> AsRef<S> for AuthTool<S>
where
    S: Auth,
{
    fn as_ref(&self) -> &S {
        &self.0
    }
}

impl<S> AsMut<S> for AuthTool<S>
where
    S: Auth,
{
    fn as_mut(&mut self) -> &mut S {
        &mut self.0
    }
}


pub trait Auth: Clone{
    fn SendKey(self, name: String) -> Result<String, EngineError>;
	fn RecvKey(self, name: String) -> Result<String, EngineError>;
}

pub enum EngineError{
    AuthDefault //验证默认错误
}


pub struct AuthDefault{
}

impl AuthDefault{
    pub fn new() -> AuthDefault {
        AuthDefault{}
    }
}

impl Auth for AuthDefault {
    fn SendKey(self, name: String) -> Result<String, EngineError>{
        println!("SendKeyDefault");
        Ok("".to_string())
    }

    fn RecvKey(self, name: String) -> Result<String, EngineError> {
        println!("RecvKeyDefault");
        Ok("".to_string())
    }
}
impl Clone for AuthDefault {
    fn clone(&self) -> Self {
        Self {}
    }
}


fn SendKey(name: String) -> Result<String, EngineError>{
    println!("SendKeyDefault");
    Ok("".to_string())
}

fn RecvKey(name: String) -> Result<String, EngineError> {
    println!("RecvKeyDefault");
    Ok("".to_string())
}
