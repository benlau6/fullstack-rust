-- Add up migration script here
CREATE TABLE pokemon (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  height SMALLINT NOT NULL,
  weight SMALLINT NOT NULL,
  types VARCHAR(255)[] NOT NULL,
  image_url VARCHAR(255),
  image_url_game_front VARCHAR(255),
  image_url_game_back VARCHAR(255),
  image_url_game_front_shiny VARCHAR(255),
  image_url_game_back_shiny VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
