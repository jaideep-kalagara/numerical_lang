use numerical_lang::{cli::Cli, lexer::Lexer};

fn main() {
    let cli = Cli::new();

    let input = cli.read_input();
    let mut lexer = Lexer::new(input.as_str());

    if cli.output_tokens {
        cli.output_tokens_file(&mut lexer);
    }
}
