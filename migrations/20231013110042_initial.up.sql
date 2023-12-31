CREATE TABLE users (
    id serial PRIMARY KEY,
    firstname varchar NOT NULL,
    lastname varchar NOT NULL,
    email varchar NOT NULL UNIQUE,
    password varchar NOT NULL,
    cv_id_list integer[] DEFAULT array[]::integer[],
    role varchar NOT NULL,
    date_created timestamp without time zone default (now() at time zone('utc'))
);

CREATE TABLE cvs (
    id serial PRIMARY KEY,
    cv_name text NOT NULL,
    file_name text NOT NULL,
    user_id integer,
    target_companies integer[] DEFAULT array[]::integer[],
    keyword_list integer[] DEFAULT array[]::integer[],
    target_job_functions integer[] DEFAULT array[]::integer[],
    date_created timestamp without time zone default (now() at time zone('utc'))
);

CREATE TABLE jobfunctions (
    id serial PRIMARY KEY,
    job_function_name text NOT NULL UNIQUE,
    keyword_list integer[] DEFAULT array[]::integer[],
    date_created timestamp without time zone default (now() at time zone('utc'))
);

CREATE TABLE keywords (
    id serial PRIMARY KEY,
    keyword_name text NOT NULL UNIQUE,
    date_created timestamp without time zone default (now() at time zone('utc'))
);

CREATE TABLE clientcompanies (
    id serial PRIMARY KEY,
    company_name text NOT NULL UNIQUE,
    date_created timestamp without time zone default (now() at time zone('utc'))
);
