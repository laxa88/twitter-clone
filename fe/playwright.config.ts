import type { PlaywrightTestConfig } from '@playwright/test';
import dotenv from 'dotenv';

// Load variables from .env file
dotenv.config();

console.log("#####", process.env.SECURE_API_HOST)
console.log("#####", process.env.SECURE_API_PORT)

const config: PlaywrightTestConfig = {
	webServer: {
		// Builds and runs app (default served on port 4173)
		command: 'npm run build && npm run preview -- --port 1122',

		env: {
			// Environment variables for connecting to Rust API
			SECURE_API_HOST: process?.env.SECURE_API_HOST || '',
			SECURE_API_PORT: process?.env.SECURE_API_PORT || '',
		},

		// Explicitly tell Playwright which port to connect to server
		// url: 'http://localhost:1122'
		port: 1122,

		// Default is 30 or 60 seconds, but we don't wanna wait long
		timeout: 10000,
	},
};

export default config;
