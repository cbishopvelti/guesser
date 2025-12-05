
use std::error::Error;
use std::io::{self, BufRead}; // Import BufRead for lines()

use std::sync::Arc;
use arrow_array::{RecordBatch, RecordBatchIterator, StringArray, ArrayRef, Int32Array};
use guesser::db::get_table;
use lancedb::arrow::arrow_schema::{Field, DataType, Schema};
use guesser::db;

// cargo run --bin gb
#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {
    println!("Hello gb");

    let input = io::read_to_string(io::stdin())?;

    // The "Direct" Way
    let batch = RecordBatch::try_from_iter(vec![
        ("id", Arc::new(Int32Array::from(vec![1])) as ArrayRef),
        ("command_context", Arc::new(StringArray::from(vec![
            format!("This was the output of a previous command which might be relevent:\n{}", input)
        ])) as ArrayRef),
    ])?;

    // FIX: Wrap it in RecordBatchIterator before passing to .add()
    let schema = batch.schema();
    let input_stream = RecordBatchIterator::new(vec![Ok(batch)], schema);


    // 1. Create the builder and store it in a mutable variable
    let mut builder = get_table().await?.merge_insert(&["id"]);

    // 2. Configure it (using the mutable variable)
    builder.when_matched_update_all(None);
    builder.when_not_matched_insert_all();

    // 3. Execute it (consumes the builder)
    // Note: Remember to wrap input_stream in Box::new() as per the previous error
    builder.execute(Box::new(input_stream)).await?;


    // table.add(input_stream).execute().await?;

    Ok(())
}
