#[derive(Debug)]
pub struct TransactionPoolCompatibility {
    is_compatible: bool,
    reason: String,
    statement_types: Vec<String>
}

impl TransactionPoolCompatibility {
    pub fn new(is_compatible: bool, reason: String, statement_types: Vec<String>) -> TransactionPoolCompatibility {
        TransactionPoolCompatibility {
            is_compatible,
            reason,
            statement_types
        }
    }

    // static method on the struct
    pub fn parse(subject: String) -> TransactionPoolCompatibility {
        let result = pg_query::parse(&subject).unwrap();
        for (node, depth, parent) in result.protobuf.nodes() {
            // println!("{} node:{:?} parent:{:?}", " ".repeat((depth as usize) * 2), node, parent);
            match node {
                pg_query::NodeRef::VariableSetStmt(stmt) => println!("got em! {:?} {:?}", stmt, stmt.is_local),
                _ => println!("nope"),
            }
        }

        TransactionPoolCompatibility {
            is_compatible: true,
            reason: "it's compatible".to_string(),
            statement_types: vec!["SELECT".to_string(), "INSERT".to_string()]
        }
    }
}

pub fn is_pgbouncer_compatible(subject: String) -> String {
    let result = pg_query::parse(&subject.to_string());
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:?}", result.statement_types());
    result.statement_types()[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_pgbouncer_compatible() {
        // println!(
        //     "{:?}",
        //     pg_query::parse("SELECT * FROM contacts; SET statement_timeout TO '1s';").unwrap().statement_types()
        // );
        // println!(
        //     "{:?}",
        //     pg_query::parse("SELECT * FROM contacts; SET statement_timeout TO '1s';").unwrap().protobuf.nodes()
        // );

        let result = TransactionPoolCompatibility::parse(
            "SELECT * FROM contacts; SET statement_timeout TO '1s';".to_string()
        );
        println!("{:?}", result);

        // for (node, depth, parent) in result.protobuf.nodes() {
        //     println!("{} node:{:?} parent:{:?}", " ".repeat((depth as usize) * 2), node, parent);
        //     match node {
        //         pg_query::NodeRef::VariableSetStmt(stmt) => println!("got em! {:?} {:?}", stmt, stmt.is_local),
        //         _ => println!("nope"),
        //     }
        // }

        // println!(
        //     "{:?}",
        //     pg_query::parse("SELECT * FROM contacts; SET statement_timeout TO '1s';").unwrap().protobuf.stmts
        // );
        // pg_query::parse("SELECT * FROM contacts; SET statement_timeout TO '1s';").unwrap();
        // println!("{:?}", pg_query::split_with_parser("SELECT * FROM contacts; SET statement_timeout TO '1s';"));

        // let result = is_pgbouncer_compatible("SELECT * FROM contacts".to_string());
        // assert_eq!(result, "SELECT");
        // let mut i = 0;
        // while i < 100000 {
        //     i += 1;
        //     pg_query::parse("SET LOCAL lock_timeout TO '10s'").unwrap();
        // }

        assert_eq!(is_pgbouncer_compatible("SET LOCAL lock_timeout TO '10s';".to_string()), "VariableSetStmt");
        assert_eq!(is_pgbouncer_compatible("LISTEN channel_name;".to_string()), "ListenStmt");
        assert_eq!(is_pgbouncer_compatible("UNLISTEN channel_name;".to_string()), "UnlistenStmt");
        assert_eq!(is_pgbouncer_compatible("BEGIN;".to_string()), "TransactionStmt");
        assert_eq!(is_pgbouncer_compatible("COMMIT;".to_string()), "TransactionStmt");
        assert_eq!(is_pgbouncer_compatible("ROLLBACK;".to_string()), "TransactionStmt");
        assert_eq!(is_pgbouncer_compatible("PREPARE prepared_statement AS SELECT * FROM my_table;".to_string()), "PrepareStmt");
        assert_eq!(is_pgbouncer_compatible("EXECUTE prepared_statement;".to_string()), "ExecuteStmt");
        assert_eq!(is_pgbouncer_compatible("DEALLOCATE prepared_statement;".to_string()), "DeallocateStmt");
    }
}
