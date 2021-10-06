select *
from clusters
where in 
    select distinct cluster_id 
    from definitions
    where word like `%$1%`
