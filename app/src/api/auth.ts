import { cleanJwt, httpClient, setJwt } from './ky';

export const login = async (username: string, password: string): Promise<void> => {
  cleanJwt();
  const client = httpClient();
  const response = await client.post('login', { json: { username, password } });
  const status = response.status;
  if (status === 200) {
    const data: { access_token: string } = await response.json();
    const jwt = data.access_token;
    setJwt(jwt);
  }
}