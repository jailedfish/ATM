-- Your SQL goes here
CREATE TABLE "driver"(
	"id" SERIAL PRIMARY KEY,
	"first_name" VARCHAR NOT NULL,
	"second_name" VARCHAR NOT NULL
);

CREATE TABLE "position"(
	"id" SERIAL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"coords" JSON
);

CREATE TABLE "barrel"(
	"id" SERIAL PRIMARY KEY,
	"position_id" INTEGER NOT NULL REFERENCES position(id),
	"driver_id" INTEGER REFERENCES driver(id),
	"going_to_position_id" INTEGER REFERENCES position(id),
	"contains" VARCHAR NOT NULL
);

