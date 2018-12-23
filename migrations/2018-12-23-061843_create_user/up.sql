CREATE TABLE "public"."user" (
    "id" SERIAL PRIMARY KEY,
    "name" varchar NOT NULL,
    "nickname" varchar,
    "email" varchar NOT NULL,
    "password_hash" varchar NOT NULL,
    "token" varchar,
    "token_secret" varchar,
    "token_expiry" varchar,
    "avatar" varchar,
    "created_at" timestamp NOT NULL DEFAULT current_timestamp,
    "updated_at" timestamp NOT NULL DEFAULT current_timestamp
);

SELECT diesel_manage_updated_at('public.user');
