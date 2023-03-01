import ccxt
import pandas as pd

'''
some issue occurred in bringing Binance API which is...
Binance Future api : APIError(code=-2015): Invalid API-key, IP, or permissions for action, request ip
so just brought market data
'''
# with open('api.txt') as f:
#     lines = f.readlines()
#     api_key = lines[0].strip()
#     secret = lines[1].strip()

# exchange = ccxt.binance({
#     # api 불러오기
#     'apiKey': 'YOUR_API_KEY',
#     'secret': 'YOUR_SECRET_KEY',

#     # enableRateLimit: 시장가로 주문이 제출되는 것을 방지하기 위한 post-only 설정값
#     'enableRateLimit': True,

#     # defaultType: future로 인해 이 코드는 선물시장에서만 동작하게 됨
#     'options': {
#         'defaultType': 'future',
#     },
# })

exchange = ccxt.binance()

'''
Larry William's volatility breakout strategy
Ref: 
https://www.whselfinvest.com/en-lu/trading-platform/free-trading-strategies/tradingsystem/56-volatility-break-out-larry-williams-free
'''
# 현재 코드는 1시간봉으로
# 가상의 지갑으로 백테스트
def larry_williams_strategy(symbol, timeframe):
    # 바이낸스에서 ohlcv 데이터 가져오기
    ohlcvs = exchange.fetch_ohlcv(symbol, timeframe)
    df = pd.DataFrame(ohlcvs, columns=['timestamp', 'open', 'high', 'low', 'close', 'volume'])
    df['timestamp'] = pd.to_datetime(df['timestamp'], unit='ms')
    df.set_index('timestamp', inplace=True)
    # print(df)
    
    # Calculate the Williams %R indicator
    period = 14
    highest_high = df['high'].rolling(window=period).max()
    lowest_low = df['low'].rolling(window=period).min()
    wr = -100 * (highest_high - df['close']) / (highest_high - lowest_low)
    
    # Calculate the average true range (ATR)
    atr_period = 14
    tr = pd.DataFrame()
    tr['h-l'] = df['high'] - df['low']
    tr['h-pc'] = abs(df['high'] - df['close'].shift())
    tr['l-pc'] = abs(df['low'] - df['close'].shift())
    tr['tr'] = tr.max(axis=1)
    atr = tr['tr'].rolling(window=atr_period).mean()
    
    # Calculate the entry and exit signals
    entry_price = df['close'][-1] + atr[-1]
    exit_price = df['close'][-1] - atr[-1]
    long_signal = (wr.iloc[-2] < -80) & (wr.iloc[-1] > -80) & (df['close'].iloc[-1] > entry_price)
    short_signal = (wr.iloc[-2] > -20) & (wr.iloc[-1] < -20) & (df['close'].iloc[-1] < exit_price)
    
    # Place a long or short order based on the entry and exit signals
    if long_signal:
        quantity = 1  # adjust this to your desired position size
        order = exchange.create_order(symbol, type='market', side='buy', amount=quantity)
        print(f'Placed a long order for {quantity} contracts at market price {order["price"]}')
    elif short_signal:
        quantity = 1  # adjust this to your desired position size
        order = exchange.create_order(symbol, type='market', side='sell', amount=quantity)
        print(f'Placed a short order for {quantity} contracts at market price {order["price"]}')

# Run the strategy on the BTC/USDT perpetual contract with 1-hour candles
larry_williams_strategy('BTC/USDT', '1h')