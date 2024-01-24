CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  slack_id VARCHAR(20) NOT NULL,
  slack_team_id VARCHAR(60) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  points INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT users_slack_id_unique UNIQUE(slack_id, slack_team_id)
);

SELECT diesel_manage_updated_at('users');
