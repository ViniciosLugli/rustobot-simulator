-- CreateEnum
CREATE TYPE "CommandType" AS ENUM ('MANUAL', 'CODE', 'UNKNOWN');

-- CreateTable
CREATE TABLE "Command" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "action" TEXT NOT NULL,
    "type" "CommandType" NOT NULL DEFAULT 'UNKNOWN',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Command_pkey" PRIMARY KEY ("id")
);
