'''
studying this link
https://wikidocs.net/120388
'''

import ccxt

### check markets in Binance
binance = ccxt.binance()
markets = binance.load_markets()

print(markets.keys())
print(len(markets))
print(type(markets))





### check current price
import pprint
btc = binance.fetch_ticker("BTC/USDT")
pprint.pprint(btc)




### check candle
import pandas as pd

# # default of fetch_ohlcv: 1minute
# # return value: 2nd dimension list
# btc_ohlcv = binance.fetch_ohlcv("BTC/USDT")

# df = pd.DataFrame(btc_ohlcv, columns=['datetime', 'open', 'high', 'low', 'close', 'volume'])
# df['datetime'] = pd.to_datetime(df['datetime'], unit='ms')
# df.set_index('datetime', inplace=True)
# print(df)

# # 1day data
# btc_ohlcv_1d = binance.fetch_ohlcv("BTC/USDT", '1d')
# df = pd.DataFrame(btc_ohlcv_1d, columns=['datetime', 'open', 'high', 'low', 'close', 'volume'])
# df['datetime'] = pd.to_datetime(df['datetime'], unit='ms')
# df.set_index('datetime', inplace=True)
# print(df)

# specific amount of data
btc_ohlcv_n = binance.fetch_ohlcv(symbol="BTC/USDT", timeframe='1d', limit=10)
df = pd.DataFrame(btc_ohlcv_n, columns=['datetime', 'open', 'high', 'low', 'close', 'volume'])
df['datetime'] = pd.to_datetime(df['datetime'], unit='ms')
df.set_index('datetime', inplace=True)
print(df)




### check orderbook
orderbook = binance.fetch_order_book('ETH/USDT')
print(orderbook['asks'])
print(orderbook['bids'])




### check balance(api needed)
with open('api.txt') as f:
  lines = f.readlines()
  api_key = lines[0].strip()
  secret = lines[1].strip()

binance_ = ccxt.binance(config={
  'apiKey': api_key, 
  'secret': secret
})

balance = binance_.fetch_balance()
print(balance['USDT'])
# format in print
# free: current balance of coin which is not using in trade
# used: current balance of coin which is using in trade
# total: free + used