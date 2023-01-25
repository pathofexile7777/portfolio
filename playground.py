'''
binance spot data payload
{
  "e": "kline",     // Event type
  "E": 123456789,   // Event time
  "s": "BNBBTC",    // Symbol
  "k": {
    "t": 123400000, // Kline start time
    "T": 123460000, // Kline close time
    "s": "BNBBTC",  // Symbol
    "i": "1m",      // Interval
    "f": 100,       // First trade ID
    "L": 200,       // Last trade ID
    "o": "0.0010",  // Open price
    "c": "0.0020",  // Close price
    "h": "0.0025",  // High price
    "l": "0.0015",  // Low price
    "v": "1000",    // Base asset volume                        total volume of the base asset(e.g. BTC, ETH, etc)
    "n": 100,       // Number of trades
    "x": false,     // Is this kline closed?
    "q": "1.0000",  // Quote asset volume                       total volume of the quote asset(e.g. USD, USDT, etc)
    "V": "500",     // Taker buy base asset volume              total volume of the base asset bought by takers(market makers)
    "Q": "0.500",   // Taker buy quote asset volume             total volume of the quote asset bought by takers(market makers)
    "B": "123456"   // Ignore
  }
}
'''

import websocket, json

symbol = 'btcusdt'
interval = '1s'

url = f'wss://stream.binance.com:9443/ws/{symbol}@kline_{interval}'
closes = []
highs = []
lows = []

def on_message(ws, message):
    data = json.loads(message)
    print(data)
    candle = data['k']
    is_candle_closed = candle['x']
    close = candle['c']
    high = candle['h']
    low = candle['l']
    vol = candle['v']

    if is_candle_closed:
        closes.append(float(close))
        highs.append(float(high))
        lows.append(float(low))
        print(closes)
        print(highs)
        print(lows)

def on_close(ws):
    print("close")

ws = websocket.WebSocketApp(url, on_message=on_message, on_close=on_close)
ws.run_forever()