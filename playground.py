import json
import websocket

def on_message(ws, message):
    data = json.loads(message)
    if data['e'] == '24hrTicker':
        symbol = data['s']
        binance_price = float(data['c'])
        coinbase_price = float(get_coinbase_price(symbol))
        price_difference = coinbase_price - binance_price
        print(f'{symbol} diff between Binance and Coinbase: {price_difference}')

def on_error(ws, error):
    print(error)

def on_close(ws):
    print("connection closed.")

def on_open(ws):
    ws.send('{"event": "subscribe", "channel": "24hrTicker", "pair": "BTCUSDT"}')

def get_coinbase_price(symbol):
    pass

if __name__ == "__main__":
    ws = websocket.WebSocketApp("wss://stream.binance.com:9443/ws/btcusdt@ticker", 
                                on_message = on_message, 
                                on_error = on_error, 
                                on_close = on_close)
    ws.on_open = on_open
    ws.run_forever()