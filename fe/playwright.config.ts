import type { PlaywrightTestConfig } from '@playwright/test';

// console.log("#####", process.env.CI === '1')
// console.log("#####", process.env.CI)

const config: PlaywrightTestConfig = {
	webServer: {
		// Runs server (default port 4173)
		command: 'npm run build && npm run preview -- --port 1122',

		env: {
			SECURE_API_PORT: '8002',
		},

		// Explicitly tell Playwright which port to connect to server
		// url: 'http://localhost:1122'
		port: 1122
		// port: process.env.CI === '1' ? 1233 : 4173,
		// timeout: 10000
	},
};

export default config;
