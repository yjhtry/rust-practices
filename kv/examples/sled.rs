use anyhow::Result;

fn main() -> Result<()> {
    let config = sled::Config::new().temporary(true);
    let db = config.open()?;

    db.insert("key1", "value1")?;
    db.insert("key2", "value2")?;
    db.insert("key3", "value3")?;

    let res = db.range("key1"..="key3");

    for r in res {
        let (k, v) = r?;
        println!(
            "k: {:?}, v: {:?}",
            String::from_utf8(k.to_vec()),
            String::from_utf8(v.to_vec())
        );
    }
    Ok(())
}
