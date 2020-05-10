-- Your SQL goes here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    firstname text NOT NULL,
    lastname text NOT NULL,
    email text UNIQUE NOT NULL,
    pass text NOT NULL,
    tokens text[]
);

/*HASHING ALGORITHM: SHA256*/
INSERT INTO users(firstname, lastname, email, pass) VALUES 
('Wesley', 'Craig', 'wcraig@yahoo.com', '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8'), /*PASS: password*/
('Minnie', 'Patel', 'minnie.patel@gmail.com', '8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92'), /*PASS: 123456*/
('Connie', 'Madsen', 'connie97@hotmail.com', '6aba5d6a47aec9eceda40c9de1d2a7d4632e24ec67f4368972f37ad486af6d7c'), /*PASS: conniemadsen*/
('Harry', 'Goldstein', 'harry@goldstein.com', 'cc796b2eb0b410567ece72904666db66dc6a8693b59dac424241355584a8b151'); /*PASS: h4rry*/