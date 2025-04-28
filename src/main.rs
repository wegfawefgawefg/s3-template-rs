use log::{debug, error};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;

pub const VULTR_S3_ENDPOINT: &str = "https://sjc1.vultrobjects.com";
pub const REGION_STR: &str = "sjc1";
pub const BUCKET_NAME: &str = "urbucketname";
pub const ACCESS_KEY: &str = "AAAAAAAAAAAAAAAAAAAA";
pub const SECRET_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

/// Uploads an in-memory byte slice to Vultr's S3-compatible storage and returns the object key.
pub async fn upload_file(data: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Generate a random object key.
    let object_key = Uuid::new_v4().to_string();

    // Set up the custom region configuration.
    let region = Region::Custom {
        region: REGION_STR.to_owned(),
        endpoint: VULTR_S3_ENDPOINT.to_owned(),
    };

    // Create credentials.
    let credentials = Credentials::new(Some(ACCESS_KEY), Some(SECRET_KEY), None, None, None)?;

    // Set up the bucket with path-style addressing and a request timeout.
    let bucket = Bucket::new(BUCKET_NAME, region.clone(), credentials.clone())?
        .with_path_style()
        .with_request_timeout(Duration::from_secs(30))?;

    debug!("Attempting to upload to bucket: {}", BUCKET_NAME);
    debug!("Using endpoint: {}", VULTR_S3_ENDPOINT);
    debug!("Object key: {}", object_key);

    let response = bucket.put_object(&object_key, data).await?;
    if response.status_code() != 200 && response.status_code() != 204 {
        return Err(format!("Upload failed with code: {}", response.status_code()).into());
    }

    Ok(object_key)
}

pub async fn get_presigned_url(object_key: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let region = Region::Custom {
        region: REGION_STR.to_owned(),
        endpoint: VULTR_S3_ENDPOINT.to_owned(),
    };

    let credentials = Credentials::new(Some(ACCESS_KEY), Some(SECRET_KEY), None, None, None)?;

    let bucket = Bucket::new(BUCKET_NAME, region.clone(), credentials.clone())?
        .with_path_style()
        .with_request_timeout(Duration::from_secs(30))?;

    let presigned_url = match bucket.presign_get(&object_key, 86400, None).await {
        Ok(presigned_url) => presigned_url,
        Err(e) => {
            error!("Error getting presigned URL: {:?}", e);
            return Err(e.into());
        }
    };

    Ok(presigned_url)
}

pub async fn get_file_contents(object_key: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let region = Region::Custom {
        region: REGION_STR.to_owned(),
        endpoint: VULTR_S3_ENDPOINT.to_owned(),
    };

    let credentials = Credentials::new(Some(ACCESS_KEY), Some(SECRET_KEY), None, None, None)?;

    let bucket = Bucket::new(BUCKET_NAME, region.clone(), credentials.clone())?
        .with_path_style()
        .with_request_timeout(Duration::from_secs(30))?;

    debug!("Attempting to download file from bucket: {}", BUCKET_NAME);
    debug!("Object key: {}", object_key);

    let response = bucket.get_object(&object_key).await?;
    if response.status_code() != 200 {
        return Err(format!("Download failed with code: {}", response.status_code()).into());
    }

    Ok(response.bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_data_to_s3() {
        let data = b"Sample content for S3 upload test";
        let object_key = upload_file(data).await.expect("Upload failed");
        println!("Uploaded object key: {}", object_key);
        assert!(!object_key.is_empty());
    }

    #[tokio::test]
    async fn test_get_file_contents() {
        // First upload a test file
        let test_content = b"Test content for S3 download test";
        let object_key = upload_file(test_content).await.expect("Upload failed");

        // Then try to download it
        let downloaded_content = get_file_contents(&object_key)
            .await
            .expect("Download failed");
        assert_eq!(downloaded_content, test_content);
    }
}

#[tokio::main]
async fn main() {}
