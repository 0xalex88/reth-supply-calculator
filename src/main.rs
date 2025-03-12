use alloy_primitives::U256;
use reth::utils::open_db_read_only;
use reth_db::cursor::DbCursorRO;
use reth_db::transaction::DbTx;
use reth_db::{mdbx::DatabaseArguments, tables, ClientVersion, Database};
use std::{path::Path, sync::Arc};

fn main() -> eyre::Result<()> {
    let db_path_str = std::env::args()
        .nth(1)
        .expect("usage: ./reth-supply-calculator <db-path>");

    let db_path = Path::new(&db_path_str);
    let db = Arc::new(open_db_read_only(
        db_path.join("db").as_path(),
        DatabaseArguments::new(ClientVersion::default()),
    )?);

    let tx = db.tx()?;
    let mut c = tx.cursor_read::<tables::PlainAccountState>()?;

    let mut total_balance = U256::ZERO;
    let mut account_count = 0;

    while let Some((_, acc)) = c.next()? {
        account_count += 1;
        total_balance += acc.balance;

        if account_count % 10_000 == 0 {
            println!("Processed {} accounts", account_count);
        }
    }

    println!("Completed processing {} accounts", account_count);
    println!("Total wei balance: {}", total_balance);

    Ok(())
}
