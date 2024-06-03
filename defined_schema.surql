DEFINE NAMESPACE IF NOT EXISTS koteka_gym;
-- Specify the namespace for the database
USE NS koteka_gym;
-- Define database
DEFINE DATABASE IF NOT EXISTS dev;

-- define table user
-- Create schemafull user table.
-- Define user fields.
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD user_type ON TABLE user TYPE string VALUE string::lowercase($value);;
DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE user  TYPE datetime  DEFAULT time::now();
DEFINE FIELD password ON TABLE user TYPE string;
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
DEFINE INDEX usernamerEmailIndex ON TABLE user COLUMNS username UNIQUE;

-- define table gym
-- Create schemafull gym table.
-- Define gym fields.
DEFINE TABLE gym SCHEMAFULL;
DEFINE FIELD user_id ON TABLE gym TYPE option<record<user>>;
DEFINE FIELD address ON TABLE gym TYPE string DEFAULT None;
DEFINE FIELD owner_name ON TABLE gym TYPE string DEFAULT None;
DEFINE FIELD phone ON TABLE gym TYPE int DEFAULT 0;
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE user  TYPE datetime  DEFAULT time::now();

-- define table Gymnast
-- Create schemafull Gymnast table.
-- Define Gymnast fields.
DEFINE TABLE gymnast SCHEMAFULL;
DEFINE FIELD user_id ON TABLE gymnast TYPE option<record<user>>;
DEFINE FIELD address ON TABLE gymnast TYPE string;
DEFINE FIELD sex ON TABLE gymnast TYPE string;
DEFINE FIELD birth_date ON TABLE gymnast TYPE datetime  DEFAULT time::now();

-- define table Feed
-- Create schemafull Feed table.
-- Define Feed fields.
DEFINE TABLE feed SCHEMAFULL;
DEFINE FIELD user_id ON TABLE feed TYPE option<record<user>>;
DEFINE FIELD post ON TABLE feed TYPE string;
DEFINE FIELD created_at ON TABLE feed TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE feed  TYPE datetime  DEFAULT time::now();

-- define table PersonalTrainer
-- Create schemafull PersonalTrainer table.
-- Define PersonalTrainer fields.
DEFINE TABLE personal_trainer SCHEMAFULL;
DEFINE FIELD user_id ON TABLE feed TYPE option<record<user>>;