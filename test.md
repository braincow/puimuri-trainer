```
curl -i -H "Accept: application/json" \
    --request GET \
    http://localhost:8000/api/equation
```

```
curl -i -H "Accept: application/json" -H "Content-Type: application/json" \
    --request POST \
    --data '{"exercise_type":"Power","missing_variable":"Power","given_variables":[["Voltage",0.86],["Current",0.1]]}' \
    http://localhost:8000/api/equation/answer/0.086
```
