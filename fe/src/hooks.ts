import type { RequestEvent } from '@sveltejs/kit';

export type SessionData = {
  apiPort?: string;
}

export const getSession = (_: RequestEvent): SessionData => {
  console.log("### getSession", process.env.SECURE_API_PORT);

  return {
    apiPort: process.env.SECURE_API_PORT || '8000'
  }
}
