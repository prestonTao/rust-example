

trait Parser { }

struct CommentParser { }
impl Parser for CommentParser { }

const ALL_PARSERS: [&dyn Parser; 1] = [&CommentParser {}];

pub fn run() {
    for &parser in &ALL_PARSERS {
        // do something with parser
    }
}

//-------------------

trait ParserEnv {}
struct ParserResult{}

// alternative 1, with trait generic type
trait Parser1<E: ParserEnv> {
    fn parse(&mut self, env: E) -> ParserResult;
}

// alternative 2, with trait reference
trait Parser2 {
    fn parser(&mut self, env: &dyn ParserEnv) -> ParserResult;
    // may need &dyn mut ParserEnv if you want to modify env as well
}

