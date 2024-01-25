CREATE TABLE teams (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  slack_id VARCHAR(20) NOT NULL,
  api_key VARCHAR NOT NULL, -- This is TEMPORARY, should get encrypted or pulled from vault instead
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT teams_slack_id_unique UNIQUE(slack_id)
);

SELECT diesel_manage_updated_at('teams');
