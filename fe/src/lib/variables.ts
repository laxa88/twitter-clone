export const variables = {
  // process.env injected via env-cmd
  // Reference: https://joyofcode.xyz/sveltekit-environment-variables
  apiPort: import.meta.env.VITE_API_PORT || process.env.SECURE_API_PORT
}

console.log("variables: ", variables)