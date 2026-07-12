use atlas::ast::{merge_sort_ast, partition_ast};

fn main() {
    for ast in [merge_sort_ast(), partition_ast()] {
        println!("AST {} version {}", ast.id, ast.ast_version);
        println!("{}", ast.render());
    }
}
