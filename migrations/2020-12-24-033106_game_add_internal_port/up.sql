-- Your SQL goes here
ALTER TABLE games
	ADD COLUMN internal_port INT,
	ADD CONSTRAINT internal_port_uniq UNIQUE (internal_port);
