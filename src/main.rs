use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTree, ParseTreeListener, Tree};

mod grammar;
mod nfa;

use grammar::regexlexer::*;
use antlr_rust::tree::{ParseTreeVisitor, Visitable};
use crate::grammar::regexparser::{RegexParser, RegexParserContextType};
use crate::grammar::regexvisitor::RegexVisitor;
use crate::nfa::NFA;

pub struct CustomParseTreeVisitor {
    pub nfa: NFA
}

impl<'node> ParseTreeVisitor<'node, RegexParserContextType> for CustomParseTreeVisitor {

}


impl<'input> RegexVisitor<'input> for CustomParseTreeVisitor {


    // fn visit_atom(&mut self, ctx: &AtomContext<'node>) {
    //
    //     println!("atom: {}", ctx.get_text());
    //
    //     self.nfa.add_transition(0,'a',1);
    //     self.nfa.add_transition(0,'b',1);
    //
    //
    //
    //     self.visit_children(ctx);
    // }
    //
    // fn visit_regExp(&mut self, ctx: &RegExpContext<'node>) {
    //     println!("regexp: {}", ctx.get_text());
    //
    //     self.visit_children(ctx);
    // }
    //
    // fn visit_root(&mut self, ctx: &RootContext<'node>) {
    //     println!("visit root");
    //     println!("{}", ctx.get_text());
    //
    //     self.visit_children(ctx);
    // }

    

}


fn main(){
    let input="a|b";
    let lexer = RegexLexer::new(InputStream::new(input));
    let source = CommonTokenStream::new(lexer);
    let mut parser = RegexParser::new(source);

    let parse_tree_root = parser.root().expect("test");
    //assert!(parse_tree_root.is_ok());

    // // Accept states: 2
    // let accept_states: HashSet<u32> = [2].into_iter().collect();
    //
    // // Create the NFA
    // let nfa = NFA::new(transitions, start_state, accept_states);
    //
    // // Test the NFA with different inputs
    // let test_inputs = ["ab", "a", "b", "", "abc"];
    //


    let mut visitor = CustomParseTreeVisitor{nfa: NFA::new()};

    visitor.nfa.add_accept_state(1);

    parse_tree_root.accept(&mut visitor);

    let test_inputs = ["ab", "a", "b", "", "abc"];

    for input in test_inputs.iter() {
        let is_accepted = visitor.nfa.is_accepted(input);
        println!("Input '{}' is accepted: {}", input, is_accepted);
    }

    print!("finished");
}