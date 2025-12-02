-- Create schema if not exists
CREATE SCHEMA IF NOT EXISTS global_visa_mgmt;

-- Create enum types in the new schema
CREATE TYPE global_visa_mgmt.sex_enum AS ENUM ('Male', 'Female', 'Other');
CREATE TYPE global_visa_mgmt.marital_status_enum AS ENUM ('Single', 'Married', 'Divorced');
CREATE TYPE global_visa_mgmt.h1b_status_enum AS ENUM ('Active', 'Inactive');

-- Create the table in the new schema with UUID as primary key
CREATE TABLE global_visa_mgmt.h1bcustomer (
    customer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- unique ID for each entry
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    dob DATE NOT NULL,
    sex global_visa_mgmt.sex_enum NOT NULL,
    marital_status global_visa_mgmt.marital_status_enum NOT NULL,
    phone VARCHAR(25) NOT NULL,
    emergency_contact_name VARCHAR(100) NOT NULL,
    emergency_contact_phone VARCHAR(25) NOT NULL,
    employment_start_date DATE NOT NULL,
    street_name VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    zip VARCHAR(20) NOT NULL,
    client_name VARCHAR(255) NOT NULL,
    client_street_name VARCHAR(255) NOT NULL,
    client_city VARCHAR(100) NOT NULL,
    client_state VARCHAR(100) NOT NULL,
    client_zip VARCHAR(20) NOT NULL,
    lca_title VARCHAR(255) NOT NULL,
    lca_salary DECIMAL(15,2) NOT NULL,
    lca_code VARCHAR(50) NOT NULL,
    receipt_number VARCHAR(100) NOT NULL,
    h1b_start_date DATE NOT NULL,
    h1b_end_date DATE NOT NULL,
    login_email VARCHAR(255) NOT NULL,
    h1b_status global_visa_mgmt.h1b_status_enum DEFAULT 'Active'
);

-- Example insert statement
INSERT INTO global_visa_mgmt.h1bcustomer (
    email,
    first_name,
    last_name,
    dob,
    sex,
    marital_status,
    phone,
    emergency_contact_name,
    emergency_contact_phone,
    employment_start_date,
    street_name,
    city,
    state,
    zip,
    client_name,
    client_street_name,
    client_city,
    client_state,
    client_zip,
    lca_title,
    lca_salary,
    lca_code,
    receipt_number,
    h1b_start_date,
    h1b_end_date,
    login_email,
    h1b_status
)
VALUES (
    'rajesh.kumar@techcorp.com',
    'Rajesh',
    'Kumar',
    '1988-03-15',
    'Male',
    'Married',
    '+1-408-555-2468',
    'Priya Kumar',
    '+1-408-555-1357',
    '2024-01-15',
    '567 Innovation Drive',
    'San Francisco',
    'CA',
    '94105',
    'TechCorp Solutions LLC',
    '1200 Market Street',
    'San Francisco',
    'CA',
    '94102',
    'Senior Software Engineer',
    135000.00,
    'LCA98765',
    'R456789123',
    '2024-04-01',
    '2027-04-01',
    'rajesh.login@techcorp.com',
    'Active'
);
