use std::error::Error;
use lancedb::Table;
use lancedb::arrow::{IntoArrow, arrow_schema::{DataType, Field, Schema}};
use arrow_array::RecordBatchIterator;
use std::sync::Arc;
use std::io;


pub async fn get_table() -> Result<Table, Box<dyn Error>> {
    let db = lancedb::connect("data/dev").execute().await.unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new(
            "command_context",
            DataType::Utf8,
            true,
        ),
    ]));

    let batches = RecordBatchIterator::new(vec![], schema.clone());

    // 2. Try to create the table. If it fails (likely because it exists), open it.
    let table = match db.create_table("contexts_2", batches).execute().await {
        Ok(t) => t,
        Err(_) => {
            // If creation failed, assume it exists and try to open
            db.open_table("contexts_2")
                .execute()
                .await
                .expect("Failed to create OR open the table")
        }
    };

    Ok(table)
}
