<script lang="ts">
	import type { ApiError } from 'src/lib/ApiError';
	import { browser } from '$app/env';
	import { storeJwt } from '$lib/store';
	import { get, type Writable } from 'svelte/store';
	import { session } from '$app/stores';
	import type { SessionData } from 'src/hooks';

	let username = '';
	let password = '';
	let error = '';

	// Every time username or password changes, reset error message.
	$: username || password ? (error = '') : null;

	const { apiHost, apiPort } = get(session as Writable<SessionData>);

	console.log('### apiHost', apiHost);
	console.log('### apiPort', apiPort);

	const handleLogin = async () => {
		const res = await fetch(`http://${apiHost}:${apiPort}/login`, {
			method: 'POST',
			body: JSON.stringify({
				username,
				password
			})
		});

		if (res.status === 200) {
			const jwt: string = await res.text();

			if (browser) {
				storeJwt.set(jwt);
			}
		} else {
			const data: ApiError = await res.json();
			error = data.details;
		}
	};
</script>

<main>
	<h1>Login</h1>
	<div>
		Username: <input bind:value={username} />
	</div>
	<div>
		Password: <input bind:value={password} type="password" />
	</div>
	<div>
		<button on:click={handleLogin}>Login</button>
	</div>
	{#if !!error}
		<div>
			{error}
		</div>
	{/if}
</main>
