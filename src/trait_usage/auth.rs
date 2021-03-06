use std::sync::Arc;

/*
    这种方式支持直接在struct中带加trait约束，缺点是不能使用new方法给带trait属性赋值。
    只能通过其它方式创建实例。
*/


pub fn run(){
    println!("开始运行trait示例...");
    let defaultAuth = AuthDefault::new();
    let e = Engine{name: "".to_string(), auth: defaultAuth};
    // let e = Engine{name: "".to_string(), auth: ()};
    // let e: Engine<&dyn Auth> = Engine::new("".to_string());
    e.authPro();

    
}

struct Engine <S>
{
    name: String,
    auth: S,
    // auth: Option<Box<S>>,// Arc<Box<S>>,
    // auth: AuthTool<S>,
}


impl <S: Auth> Engine<S>{
    // fn new(name: String) -> Self{
    //     // let defaultAuth = AuthDefault::new();
    //     // let bauth = Box::new(defaultAuth);
    //     // // let authTool = AuthTool(defaultAuth);
    //     // // let tool = AuthTool::from(defaultAuth);
    //     // let abauth = Arc::new(bauth);
    //     Engine{
    //         name: name,
    //         auth: Option::None,
    //     }
    // }
    // fn setAuth(&mut self, auth: S){
    //     self.auth = Option::Some(auth);
    // }
    fn authPro(self){
        // match self.auth {
        //     Some(auth) => {
        //         auth.RecvKey("".to_string());
        //     }
        //     None => {
        //         let defaultAuth = AuthDefault::new();
        //         defaultAuth.RecvKey("".to_string());
        //     }
        // }

        self.auth.RecvKey("hello".to_string());
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

