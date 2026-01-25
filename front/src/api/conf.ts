import { type ApiConfig } from './types';

const apiConfig: ApiConfig = {
  basePath: import.meta.env.BASE_PATH || '/api', // TODO this will not work in React
};

export { apiConfig, type ApiConfig };
