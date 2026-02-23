-- delete then re insert with updated date

BEGIN;

UPDATE billing
SET
    customer_id = ?, -- customer_id
    date        = ?  -- date
WHERE
    billing_id  = ?; -- billing_id

DELETE
FROM billing_retrieve_data
WHERE billing_id = ?; -- billing_id

WITH updatingBilling AS
(
    SELECT
        billing_id
    FROM billing
    WHERE billing_id = ? -- billing_id
    LIMIT 1
)
INSERT INTO billing_retrieve_data(billing_id, retrieve_data_id)
SELECT updatingBilling.billing_id, retrieve_data_id
FROM retrieve_data
    CROSS JOIN updatingBilling
WHERE
    datetime(date) >= datetime(?) -- from
  AND
    datetime(date) <= datetime(?); -- to

COMMIT;