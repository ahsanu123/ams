SELECT
    b.billing_id,
    b.customer_id,
    b.date,

    MIN(rd.date) as "from",
    MAX(rd.date) as "to",

    SUM(rd.amount) AS "amount",
    SUM(rd.amount * p.value) AS "bill"

FROM billing_retrieve_data brd
JOIN billing b ON b.billing_id = brd.billing_id
JOIN retrieve_data rd ON brd.retrieve_data_id = rd.retrieve_data_id
JOIN price p ON p.price_id = rd.price_id
WHERE
    b.date >= datetime(?) -- start_date
    AND
    b.date < datetime(?) -- end_date
ORDER BY
    rd.date ASC;
