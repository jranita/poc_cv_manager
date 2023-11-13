use std::{env, path::Path};

use salvo::prelude::*;
use tokio::fs::create_dir_all;

#[handler]
pub async fn upload(req: &mut Request, res: &mut Response) {
    let file_path: String = env::var("FILE_PATH").expect("FILE_PATH env variable not available");
    // TODO replace UserID in file path
    let complete_file_path = vec![file_path, "user_id".to_string()].join("/");
    println!("going to create dir: {}", complete_file_path);
    create_dir_all(&complete_file_path)
        .await
        .expect("Failed to create folder");
    println!("created dir");

    let file = req.file("files").await;
    if let Some(file) = file {
        let dest = vec![
            complete_file_path,
            file.name().unwrap_or("file").to_string(),
        ]
        .join("/");
        let info = if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            format!("file not found in request: {}", e)
        } else {
            format!("File uploaded to {}", dest)
        };
        res.render(Text::Plain(info));
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("file not found in request"));
    };
}

#[handler]
pub async fn uploader(res: &mut Response) {
    res.render(Text::Html(UPLOAD_HTML));
}

static UPLOAD_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Upload files</title>
    </head>
    <body>
        <h1>Upload files</h1>
        <form action="/api/cvs/files/" method="post" enctype="multipart/form-data">
            <input type="file" name="files"/>
            <input type="submit" value="upload" />
        </form>
    </body>
</html>
"#;
