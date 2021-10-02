create table if not exists translations (
    id int auto_increment,
    langId int not null,
    definition varchar(256) not null,
    literal varchar(256),
    primary key (id)
)