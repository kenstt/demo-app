export const wsClient = (): WebSocket => {
  const url = `${import.meta.env.VITE_WS_BASE_URL}/echo`;
  const ws: WebSocket = new WebSocket(url);
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