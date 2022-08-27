<script lang="ts">
	import { storeJwt } from '$lib/store';
	import { browser } from '$app/env';
	import { get, type Writable } from 'svelte/store';
	import { session } from '$app/stores';
	import type { SessionData } from 'src/hooks';
	import { getUserJwt } from '$lib/user';

	let textBody = '';
	let message = '';
	$: textBody ? (message = '') : null;

	const { apiHost, apiPort } = get(session as Writable<SessionData>);

	const handleLogout = () => {
		if (browser) {
			window.localStorage.removeItem('jwt');
			storeJwt.set('');
		}
	};

	const userJwt = getUserJwt();
	if (!userJwt) {
		handleLogout();
	}

	const postTweet = async () => {
		const bodyData = {
			account_id: userJwt?.user?.id,
			body: textBody
		};
		const jwt = userJwt?.jwt;

		const res = await fetch(`http://${apiHost}:${apiPort}/tweet/create`, {
			method: 'post',
			headers: new Headers({
				Authorization: `Bearer ${jwt}`,
				'Content-Type': 'application/json'
			}),
			body: JSON.stringify(bodyData)
		});

		if (res.status === 201) {
			message = 'Tweeted!';
		}
	};
</script>

<main>
	<h1>You are logged in.</h1>
	<button on:click={handleLogout}>Logout</button>

	<h2>Add tweet</h2>
	<div><input bind:value={textBody} /></div>
	<div><button on:click={postTweet}>Tweet!</button></div>
	{#if !!message}
		<div>{message}</div>
	{/if}
</main>
