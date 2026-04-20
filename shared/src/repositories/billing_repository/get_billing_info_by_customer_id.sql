SELECT
    c.customer_id,
    c.customer_name,
    c.is_active,
    c.is_admin,
    c.created_date,
    c.updated_date,

    p.price_id,
    p.date AS price_date,
    p.value,

    SUM(
        CASE WHEN rd.is_paid = true
             THEN rd.amount * p.value
        ELSE
             0.0
        END
    ) AS paid_bill,
    SUM(
        CASE WHEN rd.is_paid = true
             THEN rd.amount
        ELSE
             0
        END
    ) AS paid_total_amount,

    SUM(
        CASE WHEN rd.is_paid = false
             THEN rd.amount * p.value
        ELSE
             0.0
        END
    ) AS unpaid_bill,
    SUM(
        CASE WHEN rd.is_paid = false
             THEN rd.amount
        ELSE
             0
        END
    ) AS unpaid_total_amount,

    SUM(rd.amount * p.value) AS bill,
    SUM(rd.amount) AS total_amount,

    MIN(datetime(rd.date)) as "from",
    MAX(datetime(rd.date)) as "to"

FROM retrieve_data rd
JOIN customer c on c.customer_id = rd.customer_id
JOIN price p on p.price_id = rd.price_id
WHERE
    rd.customer_id = ? -- customer_id
GROUP BY
    rd.customer_id
ORDER BY
    rd.date DESC

