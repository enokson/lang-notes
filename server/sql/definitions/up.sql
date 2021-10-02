create table if not exists definitions (
    id int auto_increment,
    wordGroupId int not null,
    clusterId int not null,
    pronounciation varchar(256),
    word varchar(256) not null,
    prefixes varchar(256),
    suffixes varchar(256),
    definition varchar(256) not null,
    primary key (id)
)