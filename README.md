# s3-template-rs

Small Rust playground template for S3-compatible object storage (currently configured for Vultr Object Storage).

Original commit date: `2025-04-28`.

## What this repo currently does

`src/main.rs` provides three async helpers:

- `upload_file(data: &[u8]) -> Result<String, _>`: uploads bytes and returns a UUID object key.
- `get_presigned_url(object_key: &str) -> Result<String, _>`: creates a signed GET URL (24h expiry).
- `get_file_contents(object_key: &str) -> Result<Vec<u8>, _>`: downloads object bytes.

The crate compiles, and tests exercise live upload/download flows.

## Quick start

1. Set storage values in `src/main.rs`:
   - `VULTR_S3_ENDPOINT`
   - `REGION_STR`
   - `BUCKET_NAME`
   - `ACCESS_KEY`
   - `SECRET_KEY`
2. Build:

```bash
cargo check
```

3. Run tests:

```bash
cargo test
```

## Current limitations / issues

- Credentials are hardcoded in source (`src/main.rs`), which is unsafe for real usage.
- Tests are integration-style and require real network + valid S3 credentials; they fail with placeholders.
- Library-style functions are in `main.rs` with an empty `main()`. Moving helpers to `src/lib.rs` would make reuse cleaner.
- Uploaded test objects are not cleaned up, so buckets can accumulate test files.

## Recommended next improvements

1. Load config from environment variables (`std::env`) instead of constants.
2. Split into `lib.rs` (API) + `main.rs` (example CLI or no binary).
3. Gate live tests behind an env flag (for example, skip unless credentials are set).
4. Add `delete_object` helper and clean up test uploads.

## License

See `LICENSE`.
