use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ErrorsInfo {
    code: String,
    message: String,
    detail: Option<String>,
}
impl ErrorsInfo {
    pub fn new(code: &str, message: &str, detail: Option<&str>) -> ErrorsInfo {
        ErrorsInfo {
            code: code.to_string(),
            message: message.to_string(),
            detail: match detail {
                None => None,
                Some(s) => Some(s.to_string()),
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Errs {
    pub(crate) errors: Option<Vec<ErrorsInfo>>,
}
// Errs {
//     errors: Some(vec![ErrorsInfo::new(
//         "UNAUTHORIZED",
//         "authentication required",
//         None,
//     )]),
// },
impl Errs {
    #[allow(dead_code)]
    pub fn new(code: &str, message: &str, detail: Option<&str>) -> Errs {
        Errs {
            errors: Some(vec![ErrorsInfo::new(code, message, detail)]),
        }
    }
}
#[allow(dead_code)]
pub enum ErrsInto {
    BlobUploadUnknown,
    FileCreateFailed,
    RedisFailed,
    DigestFailed,
    FileRenameFailed,
    DataBasePoolFailed,
    ManifestInvalid, // manifests
}

impl ErrsInto {
    #[allow(dead_code)]
    pub fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ErrsInto::BlobUploadUnknown => {
                HttpResponse::NotFound().json(
                    Errs::new("BLOB_UPLOAD_UNKNOWN",
                              "blob upload unknown to registry",
                              Some("If a blob upload has been cancelled or was never started, this error code may be returned.")
                    )
                )
            }
            ErrsInto::FileCreateFailed=>{
                HttpResponse::InternalServerError().json(
                    Errs::new("File_CREATE_FAILED",
                    "file created failed",
                        None
                    )
                )
            }
            ErrsInto::RedisFailed => {
                HttpResponse::InternalServerError().json(
                    Errs::new("Redis_Buffer_FAILED",
                    "redis created buffer failed",
                        None
                    )
                )
            }
            ErrsInto::DigestFailed => {
                HttpResponse::InternalServerError().json(
                    Errs::new("Digest_file_cache_failed",
                    "sha256 digest file cache failed",
                        None
                    )
                )
            }
            ErrsInto::FileRenameFailed => {
                HttpResponse::InternalServerError().json(
                    Errs::new("Cache_File_Rename_Failed",
                    "Cached_File rename to digest failed",
                        None
                    )
                )
            }
            ErrsInto::DataBasePoolFailed => {
                HttpResponse::InternalServerError().json(
                    Errs::new("DataBase_Pool_get_connect_failed",
                    "Get connection from database pool failed",
                        None
                    )
                )
            }

            ErrsInto::ManifestInvalid => {
                HttpResponse::BadRequest().json({
                    Errs::new("MANIFEST_INVALID",
                    "manifest invalid",
                        None
                    )
                })
            }
        }
    }
}
// #[test]
// pub fn test1() {
//     let response = ErrsInto::BlobUploadUnknown.error_response();
// }
