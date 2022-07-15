import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = ({ params }) => {
  return {
    status: 200,
    headers: {},
    body: {
      userId: params.userId,
      number: Math.random()
    }
  }
}