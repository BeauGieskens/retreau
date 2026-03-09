-- Migration number: 0001 	 2025-11-13T09:15:11.261Z
CREATE TABLE user (
    id TEXT PRIMARY KEY,         -- UUID
    name TEXT NOT NULL,
    avatar_url TEXT,
    created_at BIGINT NOT NULL,  -- i64 for Unix timestamp milliseconds
    updated_at BIGINT            -- i64 for Unix timestamp milliseconds
);

CREATE TABLE user_identity (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    subject TEXT NOT NULL,  -- unique identifier from OIDC
    UNIQUE (provider, subject)
);

CREATE TABLE team (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at BIGINT NOT NULL,  -- i64 for Unix timestamp milliseconds
    updated_at BIGINT,           -- i64 for Unix timestamp milliseconds
    deleted_at BIGINT            -- i64 for Unix timestamp milliseconds
);

CREATE TABLE team_member (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES team(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    created_at BIGINT NOT NULL,  -- i64 for Unix timestamp milliseconds
    updated_at BIGINT,           -- i64 for Unix timestamp milliseconds
    UNIQUE (team_id, user_id)
);

CREATE TABLE board (
    id TEXT PRIMARY KEY,
    team_id TEXT REFERENCES team(id) ON DELETE SET NULL,
    admin_id TEXT REFERENCES user(id) ON DELETE SET NULL,
    name TEXT NOT NULL DEFAULT 'Untitled Board',
    status TEXT NOT NULL DEFAULT 'thinking',  -- thinking, grouping, voting, completed
    created_at BIGINT NOT NULL,               -- i64 for Unix timestamp milliseconds
    updated_at BIGINT,                        -- i64 for Unix timestamp milliseconds
    deleted_at BIGINT                         -- i64 for Unix timestamp milliseconds
);

CREATE TABLE thought (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL REFERENCES board(id) ON DELETE CASCADE,
    author_id TEXT REFERENCES user(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    emoji TEXT,
    category TEXT NOT NULL,      -- went_well, to_improve, action_item
    votes INTEGER NOT NULL DEFAULT 0,
    idx INTEGER NOT NULL,        -- position within its category for ordering
    connected_thought_id TEXT REFERENCES thought(id) ON DELETE SET NULL,
    created_at BIGINT NOT NULL,  -- i64 for Unix timestamp milliseconds
    updated_at BIGINT,           -- i64 for Unix timestamp milliseconds
    deleted_at BIGINT            -- i64 for Unix timestamp milliseconds
);
