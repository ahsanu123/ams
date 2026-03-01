SELECT
    rd.retrieve_data_id,
    rd.price_id,
    rd.amount,
    rd.date,
    rd.is_paid,

    c.customer_id,
    c.customer_name,
    c.is_active,
    c.is_admin,
    c.created_date,
    c.updated_date,

    p.price_id,
    p.date AS price_date,
    p.value,

    SUM(rd.amount * p.value) AS bill,
    SUM(rd.amount) AS total_amount,

    MIN(date(rd.date)) as "from",
    MAX(date(rd.date)) as "to"

FROM retrieve_data rd
JOIN customer c on c.customer_id = rd.customer_id
JOIN price p on p.price_id = rd.price_id
WHERE
    rd.date >= date(?) -- start_year
    AND
    rd.date < date(?) -- end_year
    AND 
    rd.customer_id = ? -- customer_id
GROUP BY
    rd.customer_id
ORDER BY
    rd.date

