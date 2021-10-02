create table if not exists examples (
    id int auto_increment,
    definitionId int,
    translationId int,
    example varchar(256) not null,
    primary key (id)
)