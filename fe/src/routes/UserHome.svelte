<script lang="ts">
	import { storeJwt } from '$lib/store';
	import { browser } from '$app/env';

	const handleLogout = () => {
		if (browser) {
			window.localStorage.removeItem('jwt');
			storeJwt.set('');
		}
	};
</script>

<main>
	<h1>You are logged in.</h1>
	<button on:click={handleLogout}>Logout</button>
</main>
