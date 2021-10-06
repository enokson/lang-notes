create table if not exists definitions (
    id int auto_increment primary key,
    word_group_id int not null,
    cluster_id int not null,
    pronounciation varchar(256),
    word varchar(256) not null,
    prefixes varchar(256),
    suffixes varchar(256),
    definition varchar(256) not null
);