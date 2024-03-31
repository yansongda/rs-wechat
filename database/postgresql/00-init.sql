-- auto-generated definition
create table "user"
(
    id         bigserial
        constraint pk_user_id
            primary key,
    open_id    varchar(128)                           not null,
    avatar     text,
    nickname   varchar(128)             default '微信用户'::character varying,
    slogan     varchar(246),
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table "user"
    owner to miniprogram;

create unique index uk_user_open_id
    on "user" (open_id);

-- auto-generated definition
create table totp
(
    id         bigserial
        constraint pk_totp_id
            primary key,
    user_id    bigint                                 not null,
    username   varchar(128)                           not null,
    issuer     varchar(128),
    period     smallint                 default 30    not null,
    secret     varchar(256)                           not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table totp
    owner to miniprogram;

create index idx_totp_user
    on totp (user_id);

-- auto-generated definition
create table shortlink
(
    id         bigserial
        constraint pk_shorlink_id
            primary key,
    user_id    bigint                                 not null,
    short      varchar(64)                            not null
        constraint uk_shortlink_short
            unique,
    link       text,
    visit      integer                  default 0     not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

alter table shortlink
    owner to miniprogram;

create index idx_shortlink_user
    on shortlink (user_id);


