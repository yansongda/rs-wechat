-- auto-generated definition
create table user
(
    id         integer not null
        constraint pk_id
            primary key autoincrement,
    open_id    text    not null
        constraint uk_open_id
            unique,
    avatar     text,
    nickname   text,
    slogan     text,
    created_at text,
    updated_at text
);

-- auto-generated definition
create table totp
(
    id         integer not null
        constraint pk_id
            primary key autoincrement,
    user_id    integer not null,
    username   text    not null,
    issuer     text        null,
    secret     text    not null,
    created_at text,
    updated_at text
);

create index idx_totp_user_id
    on totp (user_id);
