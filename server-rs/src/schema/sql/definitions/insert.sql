insert into definitions 
(word_group_id, cluster_id, pronounciation, word, prefixes, suffixes, definition)
values ($1, $2, $3, $4, $5, $6, $7) returning id;