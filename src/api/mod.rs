pub mod clientcompany;
pub mod cv;
pub mod fileupload;
pub mod jobfunction;
pub mod keyword;
pub mod user;

use salvo::Depot;

pub fn sanitize_query_string(raw_string: String) -> String {
    raw_string
        .chars()
        .map(|x| match x.is_alphanumeric() {
            true => x,
            false => '_',
        })
        .collect()
}
