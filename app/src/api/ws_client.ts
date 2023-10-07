let ws : WebSocket;

export const wsClient = (): WebSocket => {
  const url = `${import.meta.env.VITE_WS_BASE_URL}/echo`;

  // 連線物件不存在，或連線狀態不是OPEN，就重新建立一個連線物件
  if (!ws || ws.readyState !== 1) {
    ws = new WebSocket(url);
  }

  ws.onopen = () => {
    console.log('ws open');
  }
  ws.onclose = (): void => {
    console.log('ws close');
  }
  ws.onerror = (err: Event) => {
    console.log('ws error', err);
  }
  ws.onmessage = (msg: MessageEvent<any>) => {
    console.log('ws message', msg);
  }
  return ws;
}