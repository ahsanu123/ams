BEGIN;

INSERT INTO billing (customer_id, date)
VALUES
    (?, ?); -- customer_id, date

WITH inserted AS
(
    SELECT last_insert_rowid() as billing_id
)

INSERT INTO billing_retrieve_data(billing_id, retrieve_data_id)
SELECT inserted.billing_id, retrieve_data_id
FROM retrieve_data
CROSS JOIN inserted
WHERE
    datetime(date) >= datetime(?) -- from
    AND
    datetime(date) <= datetime(?) -- to
    AND
    is_paid = false;

UPDATE retrieve_data
SET is_paid = true
WHERE
    is_paid = false
    AND
    datetime(date) >= datetime(?) -- from
    AND
    datetime(date) <= datetime(?); -- to

COMMIT;