-- Your SQL goes here
-- =========================================
-- Drop unused auth-related tables
-- =========================================

DROP TABLE IF EXISTS "session" CASCADE;
DROP TABLE IF EXISTS "account" CASCADE;
DROP TABLE IF EXISTS "verification" CASCADE;

-- =========================================
-- Simplify the user table
-- =========================================

-- Remove constraints that depend on removed columns (if any)
DROP INDEX IF EXISTS "user_email_key";

-- Alter the user table:
ALTER TABLE "user"
    -- Drop unused columns
    DROP COLUMN IF EXISTS "emailVerified",
    DROP COLUMN IF EXISTS "image",
    DROP COLUMN IF EXISTS "createdAt",
    DROP COLUMN IF EXISTS "updatedAt",

    -- Add password column
    ADD COLUMN IF NOT EXISTS "password" TEXT NOT NULL,

    -- Make email optional
    ALTER COLUMN "email" DROP NOT NULL;

-- =========================================
-- Optional: add constraints / indexes
-- =========================================

-- Optional unique email (allows multiple NULLs in Postgres)
CREATE UNIQUE INDEX IF NOT EXISTS "user_email_unique"
ON "user"("email")
WHERE "email" IS NOT NULL;
