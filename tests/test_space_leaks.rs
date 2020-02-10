#[test]
fn size_leak() -> sled::Result<()> {
    let tree = sled::Config::new()
        .temporary(true)
        .segment_size(2048)
        .flush_every_ms(None)
        .snapshot_after_ops(100_000_000)
        .open()?;

    for _ in 0..10_000 {
        tree.insert(b"", b"")?;
    }

    tree.flush()?;

    let sz = tree.size_on_disk()?;
    assert!(
        sz <= 16384,
        "expected system to use less than or equal to \
            16486 bytes, but actually used {}",
        sz
    );

    Ok(())
}
