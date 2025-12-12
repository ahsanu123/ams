let user_ids = [1 2 3 4 5 6]
let all_year = (seq date --begin-date '2025-01-01' --end-date '2028-01-10' --increment 1day -o '%Y-%m-%dT09:15:12.11')

$all_year | each {|day|
    $user_ids | each {|user_id|
        let amount = (random int 1..11)

        let payload = {
            amount: $amount
            date: $day
            userId: $user_id
        }
        (
          curl -X 'POST' 'http://localhost:9090/taking-record/upsert-taking-record-by-date' 
            -H 'accept: */*' 
            -H 'Content-Type: application/json' 
            -d ($payload | to json) 
        )
    }
}
