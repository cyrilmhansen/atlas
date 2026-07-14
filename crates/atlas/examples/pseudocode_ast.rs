use atlas::ast::{insertion_sort_ast, merge_sort_ast, partition_ast};

fn main() {
    for ast in [merge_sort_ast(), partition_ast(), insertion_sort_ast()] {
        println!("AST {} version {}", ast.id, ast.ast_version);
        println!("{}", ast.render());
    }
}
