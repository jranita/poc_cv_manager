CREATE TABLE users (
    id serial PRIMARY KEY,
    firstname varchar NOT NULL,
    lastname varchar NOT NULL,
    email varchar NOT NULL UNIQUE,
    password varchar NOT NULL,
    cv_id_list integer[],
    role varchar NOT NULL,
    -- create_dt datetime not null default (datetime('now')),
    date_created timestamp
);

CREATE TABLE cvs (
    id serial PRIMARY KEY,
    cv_name text NOT NULL,
    file_name text NOT NULL,
    target_companies integer[],
    keyword_list integer[],
    target_job_function integer[],
    date_created timestamp
);

CREATE TABLE jobfunctions (
    id serial PRIMARY KEY,
    job_function_name text NOT NULL,
    keyword_list integer[],
    date_created timestamp
);

CREATE TABLE keywords (
    id serial PRIMARY KEY,
    keyword_name text NOT NULL,
    date_created timestamp
);

CREATE TABLE clientcompanies (
    id serial PRIMARY KEY,
    company_name text NOT NULL,
    date_created timestamp
);