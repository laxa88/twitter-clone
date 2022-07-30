export const variables = {
  apiPort: import.meta.env.VITE_API_PORT || process.env.SECURE_API_PORT
}

console.log("variables: ", variables)