-- Your SQL goes here
ALTER TABLE games
	ADD COLUMN thrones_t1 INT NOT NULL DEFAULT 5;
ALTER TABLE games
	ADD COLUMN thrones_t2 INT NOT NULL DEFAULT 0;
ALTER TABLE games
	ADD COLUMN thrones_t3 INT NOT NULL DEFAULT 0;

ALTER TABLE games
	ADD COLUMN throne_points_required INT NOT NULL DEFAULT 5;