<script lang="ts">
	import { storeJwt } from '$lib/store';
	import { browser } from '$app/env';
	import { get, type Writable } from 'svelte/store';
	import { session } from '$app/stores';
	import type { SessionData } from 'src/hooks';
	import { getUserJwt } from '$lib/user';
	import type { Tweet } from '$lib/Tweet';

	let textBody = '';
	let message = '';
	let postingTweet = false;
	let loadingTweets = false;
	let tweets: Tweet[] = [];
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

	const getTweets = async () => {
		if (!userJwt) {
			return;
		}

		loadingTweets = true;

		const jwt = userJwt?.jwt;
		const username = userJwt?.user?.username || '';

		const res = await fetch(`http://${apiHost}:${apiPort}/tweets/${username}`, {
			method: 'get',
			headers: new Headers({
				Authorization: `Bearer ${jwt}`,
				'Content-Type': 'application/json'
			})
		});

		tweets = await res.json();

		loadingTweets = false;
	};

	const postTweet = async () => {
		if (!userJwt) {
			return;
		}

		postingTweet = true;

		const jwt = userJwt?.jwt;

		const res = await fetch(`http://${apiHost}:${apiPort}/tweet/create`, {
			method: 'post',
			headers: new Headers({
				Authorization: `Bearer ${jwt}`,
				'Content-Type': 'application/json'
			}),
			body: JSON.stringify({
				account_id: userJwt?.user?.id,
				body: textBody
			})
		});

		if (res.status === 201) {
			message = 'Tweeted!';
			getTweets();
		}

		postingTweet = false;
	};

	getTweets();
</script>

<main>
	<h1>You are logged in.</h1>
	<button on:click={handleLogout}>Logout</button>

	<h2>Add tweet</h2>
	<div><input bind:value={textBody} /></div>
	<div><button disabled={postingTweet} on:click={postTweet}>Tweet!</button></div>
	{#if !!message}
		<div>{message}</div>
	{/if}

	<h2>My tweets</h2>
	{#if loadingTweets}
		<div>Loading...</div>
	{:else}
		<div>
			<ul>
				{#each tweets as tweet}
					<li>{tweet.created_at}: {tweet.body}</li>
				{/each}
			</ul>
		</div>
	{/if}
</main>
