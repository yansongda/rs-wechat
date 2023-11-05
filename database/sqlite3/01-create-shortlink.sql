-- auto-generated definition
create table shortlink
(
    id         integer           not null
        constraint id
            primary key autoincrement,
    user_id    integer           not null,
    short      text              not null
        constraint uk_short
            unique,
    link       text              not null,
    visit      integer default 0 not null,
    created_at text,
    updated_at text
);

create index idx_shortlink_user_id
    on shortlink (user_id);
