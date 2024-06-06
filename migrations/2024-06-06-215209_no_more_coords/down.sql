-- This file should undo anything in `up.sql`


ALTER TABLE "position" ADD COLUMN "coords" JSON;

