CREATE TABLE users (
    id integer NOT NULL PRIMARY KEY,
    username varchar NOT NULL UNIQUE,
    firstname varchar,
    lastname varchar NOT NULL,
    email varchar,
    pass varchar,
    cv_id_list integer[]
);

CREATE TABLE cvs (
    id integer NOT NULL PRIMARY KEY,
    cv_name text,
    file_name text,
    cv_list integer[],
    keyword_list varchar[],
    target_job_functions integer[]
);

CREATE TABLE jobfunctions (
    id integer NOT NULL PRIMARY KEY,
    job_function_name text,
    keyword_list integer[]
);

CREATE TABLE Keyword (
    id integer NOT NULL PRIMARY KEY,
    keyword_name text,
    pass text,
    cv_list integer[]
);

CREATE TABLE ClientCompany (
    id integer NOT NULL PRIMARY KEY,
    keyword_name text
);