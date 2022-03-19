use santiago::grammar::grammar_builder::GrammarBuilder;
use santiago::grammar::rule::Rule;
use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::lexer_builder::LexerBuilder;
use santiago::lexer::lexer_rule::LexerRule;
use santiago::parser::parse::parse;

fn main() -> Result<(), String> {
    // Let's define an ambiguous grammar for adding integer numbers:
    //
    //   Sum = Sum Plus Sum
    //       | "int"
    //
    //   Plus = "plus"
    //
    let grammar: Vec<Rule> = GrammarBuilder::new()
        .map_to_rules("Sum", &["Sum", "Plus", "Sum"])
        .map_to_lexemes("Sum", &["int"])
        .map_to_lexemes("Plus", &["plus"])
        .finish();

    // A lexer splits the input string into units
    // of related characters called "Lexemes"
    //
    // For this calculator we are interested in the "+" operator
    // and the digits of the integer numbers:
    //
    //   "plus" := "+" (a character)
    //   "int"  := \d+ (regular expression for 1 or more digits)
    //     ∅    := " " (ignore whitespace)
    //
    let lexing_rules: Vec<LexerRule> = LexerBuilder::new()
        .string(&["initial"], "+", |lexer| lexer.consume_as("plus"))
        .pattern(&["initial"], r"\d+", |lexer| lexer.consume_as("int"))
        .string(&["initial"], " ", |lexer| lexer.ignore())
        .finish();

    // Let's start by tokenizing the input
    let input = "1 + 2 + 3";
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, input);

    // Now parse!
    let abstract_syntax_trees = parse(&grammar, &lexemes)?;

    // And print the results:
    println!("input: {:?}", input.chars());

    println!("lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme:?}");
    }

    println!("Grammar:");
    for rule in &grammar {
        println!("  {rule}");
    }

    println!("Forest:");
    for ast in abstract_syntax_trees {
        println!("{ast}");
    }

    Ok(())
}
