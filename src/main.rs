use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeListener, TerminalNode, Tree};
mod grammar;
use grammar::regexlexer::regexLexer;
use grammar::regexparser::*;
use grammar::regexparserlistener::*;
use grammar::regexparservisitor::*;
use antlr_rust::tree::{ParseTreeVisitor, Visitable};

pub struct NFA_row{
    start: Vec<String>,
    action: String,
    end: Vec<String>,
} 


pub struct CustomParseTreeVisitor {
    pub nfa: Vec<NFA_row>,
}

impl<'node> ParseTreeVisitor<'node, regexParserContextType> for CustomParseTreeVisitor {

}


impl<'node> regexParserVisitor<'node> for CustomParseTreeVisitor {


    fn visit_atom(&mut self, ctx: &AtomContext<'node>) {
        
        println!("{}", ctx.get_text());
        

        self.visit_children(ctx);
    }

    fn visit_regExp(&mut self, ctx: &RegExpContext<'node>) {
        println!("regexp: {}", ctx.get_text());

        self.visit_children(ctx);
    }

    fn visit_root(&mut self, ctx: &RootContext<'node>) {
        println!("visit root");
        println!("{}", ctx.get_text());
       
        self.visit_children(ctx);
    }

    

}


fn main(){
    let input="ab";
    let lexer = regexLexer::new(InputStream::new(input));
    let source = CommonTokenStream::new(lexer);
    let mut parser = regexParser::new(source);

    let parse_tree_root = parser.root().expect("test");
    //assert!(parse_tree_root.is_ok());

    let mut visitor = CustomParseTreeVisitor{nfa: Vec::new()};

    parse_tree_root.accept(&mut visitor);

    

    print!("finished");
}