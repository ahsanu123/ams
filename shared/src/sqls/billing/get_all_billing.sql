SELECT
    b.billing_id,
    c.customer_id,
    blc.balance_id,
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
    SUM(rd.amount) AS amount,

    MIN(datetime(rd.date)) as "from",
    MAX(datetime(rd.date)) as "to"
FROM billing b
JOIN billing_retrieve_data brd ON brd.billing_id = b.billing_id
JOIN retrieve_data rd ON rd.retrieve_data_id = brd.retrieve_data_id
JOIN price p ON rd.price_id = p.price_id
JOIN balance_billing bb ON bb.billing_id = b.billing_id
JOIN balance blc ON blc.balance_id = bb.balance_id
JOIN customer c ON b.customer_id = c.customer_id
GROUP BY
    c.customer_id;