-- Your SQL goes here
CREATE TABLE artical (
id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
title VARCHAR NOT NULL,
desc TEXT NOT NULL,
image TEXT NOT NULL,
post_type VARCHAR NOT NULL,
createdAt DATETIME NOT NULL,
updatedAt DATETIME NOT NULL,
deletedAt DATETIME NOT NULL,
status BOOLEAN NOT NULL DEFAULT t,
creator INTEGER NOT NULL,
FOREIGN KEY(creator) REFERENCES user(id));

CREATE TABLE user (
id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
name VARCHAR NOT NULL,
name_ar VARCHAR NOT NULL,
email VARCHAR NOT NULL UNIQUE,
password VARCHAR NOT NULL,
image VARCHAR NOT NULL,
createdAt DATETIME NOT NULL,
updatedAt DATETIME NOT NULL,
deletedAt DATETIME NOT NULL,
status BOOLEAN NOT NULL DEFAULT t,
active BOOLEAN NOT NULL DEFAULT t);

CREATE TABLE service (
id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
title VARCHAR NOT NULL,
desc TEXT NOT NULL,
image VARCHAR NOT NULL,
createdAt DATETIME NOT NULL,
updatedAt DATETIME NOT NULL,
deletedAt DATETIME NOT NULL,
status BOOLEAN NOT NULL DEFAULT t,
creator INTEGER NOT NULL,
FOREIGN KEY(creator) REFERENCES user(id));