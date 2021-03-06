use std::sync::Arc;

pub fn run(){
    println!("开始运行trait示例...");
    // let defaultAuth = AuthDefault::new();
    // let e = Engine{name: "".to_string(), auth: defaultAuth};
    // let e = Engine{name: "".to_string(), auth: defaultAuth};
    let eg = Engine::new("".to_string(), SendKey);
    eg.authPro();

    
}

struct Engine <F>
{
    name: String,
    auth: Arc<F>,
    // auth: AuthTool<S>,
}


impl <F> Engine<F>
where F: Fn(String) -> Result<String, EngineError>,
{
    fn new(name: String, process_fn: F) -> Self{
        let pf = Arc::new(SendKey);
        // let defaultAuth = AuthDefault::new();
        // let authTool = AuthTool(defaultAuth);
        // let tool = AuthTool::from(defaultAuth);

        Engine{
            name: name,
            auth: pf,
        }
    }
    fn authPro(self){
        // self.auth.RecvKey("hello".to_string());
        let pf = self.auth.as_ref();
        pf(self.name);
    }
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
