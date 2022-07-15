import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = ({ params }) => {
  console.log("###", params)
  return {
    status: 200,
    headers: {},
    body: {
      userId: params.userId,
      tweetId: params.tweetId,
      number: Math.random()
    }
  }
}