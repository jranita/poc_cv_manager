use chrono::NaiveDateTime;
use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub cv_id_list: Vec<i32>,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CV {
    pub id: i32,
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<Keyword>,
    pub target_job_function: Vec<JobFunction>,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewCV {
    pub cv_name: String,
    pub file_name: String,
    pub keyword_list: Vec<Keyword>,
    pub target_job_function: Vec<JobFunction>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct JobFunction {
    pub id: i32,
    pub job_function_name: String,
    pub keyword_list: Vec<Keyword>,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NewJobFunction {
    pub job_function_name: String,
    pub keyword_list: Vec<Keyword>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Keyword {
    pub id: i32,
    pub keyword_name: String,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewKeyword {
    pub keyword_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientCompany {
    pub id: i32,
    pub client_name: String,
    pub client_email: String,
    pub date_created: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewClientCompany {
    pub client_name: String,
    pub client_email: String,
}
