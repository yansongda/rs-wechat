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
    created_at datetime,
    updated_at datetime
);

-- auto-generated definition
create table totp
(
    id         integer not null
        constraint pk_id
            primary key autoincrement,
    user_id    integer,
    username   text    not null,
    issuer     text    not null,
    secret     text    not null,
    created_at datetime,
    updated_at datetime
);

create index idx_user_id
    on totp (user_id);
