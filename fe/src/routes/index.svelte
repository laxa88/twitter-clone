<script lang="ts">
	import { storeJwt } from '$lib/store';
	import { browser } from '$app/env';
	import Login from './Login.svelte';
	import Register from './Register.svelte';

	if (browser) {
		const jwt = window.localStorage.getItem('jwt');
		storeJwt.set(jwt as string);
	}

	let jwt = '';
	storeJwt.subscribe((value) => {
		if (browser) {
			window.localStorage.setItem('jwt', value);
			jwt = value;
		}
	});

	const handleLogout = () => {
		if (browser) {
			window.localStorage.removeItem('jwt');
			storeJwt.set('');
		}
	};
</script>

<main>
	<div>
		{#if jwt}
			<button on:click={handleLogout}>Logout</button>
		{:else}
			<Login />
			<Register />
		{/if}
	</div>
</main>

<!--
  TODO:
  - if not logged in, show login page
  - if user is logged in, show current user's feed
-->
