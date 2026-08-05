# rate_limiter

```ps
 1..20 | ForEach-Object {
>>     try {
>>         (Invoke-WebRequest -Uri "http://localhost:8080/health" -Method Get).StatusCode
>>     } catch {
>>         $_.Exception.Response.StatusCode.Value__
>>     }
>> }
```