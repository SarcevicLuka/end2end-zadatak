CREATE TABLE employees
(
    id                 VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    first_name         VARCHAR(255) NOT NULL,
    last_name          VARCHAR(255) NOT NULL,
    sex                VARCHAR(1) NOT NULL,
    "image"            BYTEA NOT NULL,
    birth_year         INTEGER NOT NULL,
    start_of_work      VARCHAR(20) NOT NULL,
    type_of_contract   VARCHAR(10) NOT NULL,
    length_of_contract VARCHAR(20) NOT NULL,
    department         VARCHAR(50) NOT NULL,
    days_of_holiday    INTEGER DEFAULT 25,
    free_days          INTEGER DEFAULT 10,
    days_of_paid_leave INTEGER DEFAULT 10,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_employee_id PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('employees');

CREATE INDEX IF NOT EXISTS employees_first_name         ON employees USING BTREE(first_name);
CREATE INDEX IF NOT EXISTS employees_last_name          ON employees USING BTREE(last_name);
CREATE INDEX IF NOT EXISTS employees_sex                ON employees USING BTREE(sex);
CREATE INDEX IF NOT EXISTS employees_image              ON employees USING BTREE("image");
CREATE INDEX IF NOT EXISTS employees_birth_year         ON employees USING BTREE(birth_year);
CREATE INDEX IF NOT EXISTS employees_start_of_work      ON employees USING BTREE(start_of_work);
CREATE INDEX IF NOT EXISTS employees_type_of_contract   ON employees USING BTREE(type_of_contract);
CREATE INDEX IF NOT EXISTS employees_length_of_contract ON employees USING BTREE(length_of_contract);
CREATE INDEX IF NOT EXISTS employees_department         ON employees USING BTREE(department);
CREATE INDEX IF NOT EXISTS employees_days_of_paid_leave ON employees USING BTREE(days_of_paid_leave);
CREATE INDEX IF NOT EXISTS employees_free_days          ON employees USING BTREE(free_days);
CREATE INDEX IF NOT EXISTS employees_days_of_paid_leave ON employees USING BTREE(days_of_paid_leave);