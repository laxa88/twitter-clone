export const variables = {
  apiPort: import.meta.env.VITE_API_PORT
}

// grabs from .env if prepended with VITE_
console.log('###1', variables);

// doesn't work after compiling
// console.log('###2', process.env.PORT);
