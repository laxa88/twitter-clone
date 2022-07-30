import type { RequestEvent } from '@sveltejs/kit';

export type SessionData = {
  apiHost?: string;
  apiPort?: string;
}

export const getSession = (_: RequestEvent): SessionData => {
  console.log("### getSession host", process.env.SECURE_API_HOST);
  console.log("### getSession port", process.env.SECURE_API_PORT);

  return {
    // Notes:
    // - 8000 is for local only
    // - 8002 is for docker
    apiHost: process.env.SECURE_API_HOST || 'localhost',
    apiPort: process.env.SECURE_API_PORT || '8000'
  }
}
