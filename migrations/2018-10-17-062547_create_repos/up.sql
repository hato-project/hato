CREATE TABLE "public"."repo" (
    "id" varchar NOT NULL,
    "name" varchar NOT NULL DEFAULT '',
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('public.repo');

CREATE INDEX "idx_created_at" ON "public"."repo" USING btree(created_at);