import { goto } from '$app/navigation';
import { cleanJwt, getJwt, httpClient, setJwt } from './ky';

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
};

interface User {
  name: string;
  exp: number;
  permissions: number[];
}

export const currentUser = (): User|null => {
  const jwt = getJwt();
  console.log('jwt', jwt);
  if (jwt) {
    const payload = JSON.parse(atob(jwt.split('.')[1]));
    return {...payload, name: payload.sub};
  }
  return null;
};

export const route_guard = (): void => {
  if (typeof window === 'undefined') return;
  let location = window.location;
  let path = location.pathname;
  console.log('into path:', path);

  let user = currentUser();
  console.log('current user', user);

  // 進行權限判斷
  if (path === '/game/tic_tac_toe') {
    // 如果沒有權限，就導回首頁
    if (user?.permissions?.includes(1) ||
      user?.permissions?.includes(2)) {
      return;
    }
    goto('/game').then(() => console.log('redirect to /'));
  }
};
