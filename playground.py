import requests

endpoint = "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=500"
response = requests.get(endpoint)
trading_data = response.json()
print(trading_data)