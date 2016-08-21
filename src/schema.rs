infer_schema!(dotenv!("DATABASE_URL"));


// or, to be more explicit:
// infer_table_from_schema!(dotenv!("DATABASE_URL"), "track");