CREATE TABLE "public"."repo" (
    "namespace" varchar NOT NULL,
    "name" varchar NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY ("namespace", "name")
);

SELECT diesel_manage_updated_at('public.repo');

CREATE INDEX "idx_created_at" ON "public"."repo" USING btree(created_at);