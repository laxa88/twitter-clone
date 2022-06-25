CREATE TABLE account (
    id INT GENERATED ALWAYS AS IDENTITY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO
    account (email, password)
values
    ('gashzack@hotmail.com', 'abc'),
    ('waiyin@hotmail.my', '123');

CREATE TABLE tweet (
    id INT GENERATED ALWAYS AS IDENTITY,
    account_id INT,
    body TEXT,
    created_at TIMESTAMP,
    PRIMARY KEY(id),
    CONSTRAINT fk_account FOREIGN KEY(account_id) REFERENCES account(id)
);

CREATE TABLE follower (
    id INT GENERATED ALWAYS AS IDENTITY,
    account_id INT,
    follower_account_id INT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY(id),
    CONSTRAINT fk_account FOREIGN KEY(account_id) REFERENCES account(id),
    CONSTRAINT fk_follower_account FOREIGN KEY(follower_account_id) REFERENCES account(id)
);