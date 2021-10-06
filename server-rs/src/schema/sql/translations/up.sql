create table if not exists translations (
    id int auto_increment primary key,
    lang_id int not null,
    definition varchar(256) not null,
    literal varchar(256)
)