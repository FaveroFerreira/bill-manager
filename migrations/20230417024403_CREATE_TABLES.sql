-- Add migration script here
CREATE TABLE bill
(
    id               UUID PRIMARY KEY NOT NULL,
    description      VARCHAR(50)        NOT NULL,
    original_amount  DECIMAL(10, 2)     NOT NULL,
    corrected_amount DECIMAL(10, 2)     NULL,
    due_date         DATE               NOT NULL,
    payment_date     DATE               NOT NULL
);

CREATE TABLE interest_configuration
(
    id          UUID PRIMARY KEY NOT NULL,
    start_range INT                NOT NULL,
    end_range   INT                NOT NULL,
    fine        DECIMAL(10, 4)     NOT NULL,
    interest    DECIMAL(10, 4)     NOT NULL
);

INSERT INTO interest_configuration (id, start_range, end_range, fine, interest)
VALUES (gen_random_uuid(), 1, 3, 0.02, 0.001);

INSERT INTO interest_configuration (id, start_range, end_range, fine, interest)
VALUES (gen_random_uuid(), 4, 5, 0.03, 0.002);

INSERT INTO interest_configuration (id, start_range, end_range, fine, interest)
VALUES (gen_random_uuid(), 6, 999999, 0.05, 0.003);