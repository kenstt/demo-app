import type { KyInstance } from '../../node_modules/ky/distribution/types/ky.js';
import ky from 'ky';

export const httpClient = (): KyInstance => {
  let api = ky.create({
    prefixUrl: import.meta.env.VITE_API_BASE_URL, // 取用env變數
    throwHttpErrors: false, // 4XX及5XX ky會拋Error，但我們要拿body的訊息
  });
  // TODO: 之後要加Authorization可以在這邊擴充
  return api;
};
