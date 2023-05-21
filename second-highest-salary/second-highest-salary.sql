# Write your MySQL query statement below
SELECT 
    (
        SELECT 
            DISTINCT e.salary 
        FROM 
            Employee AS e 
        ORDER BY 
            e.salary DESC 
        LIMIT 
        1 OFFSET 1
    ) AS SecondHighestSalary;