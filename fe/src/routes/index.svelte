<script lang="ts">
	import { storeJwt } from '$lib/store';
	import { browser } from '$app/env';
	import Login from './Login.svelte';
	import Register from './Register.svelte';
	import UserHome from './UserHome.svelte';

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
</script>

<main>
	{#if jwt}
		<UserHome />
	{:else}
		<Login />
		<Register />
	{/if}
</main>

<!--
  TODO:
  - if not logged in, show login page
  - if user is logged in, show current user's feed
-->
