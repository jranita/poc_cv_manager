CREATE TABLE users (
    id integer NOT NULL PRIMARY KEY,
    firstname varchar NOT NULL,
    lastname varchar NOT NULL,
    email varchar NOT NULL UNIQUE,
    pass varchar,
    cv_id_list integer[],
    date_created timestamp
);

CREATE TABLE cvs (
    id integer NOT NULL PRIMARY KEY,
    cv_name text,
    file_name text,
    cv_list integer[],
    keyword_list varchar[],
    target_job_function integer[],
    date_created timestamp
);

CREATE TABLE jobfunctions (
    id integer NOT NULL PRIMARY KEY,
    job_function_name text,
    keyword_list integer[],
    date_created timestamp
);

CREATE TABLE keywords (
    id integer NOT NULL PRIMARY KEY,
    keyword_name text,
    date_created timestamp
);

CREATE TABLE clientcompanies (
    id integer NOT NULL PRIMARY KEY,
    company_name text,
    date_created timestamp
);
