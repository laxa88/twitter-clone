<script lang="ts">
	let email = '';
	let username = '';
	let password = '';
	let registered = false;

	const handleRegister = async () => {
		const res = await fetch('http://localhost:8000/signup', {
			method: 'POST',
			body: JSON.stringify({
				email,
				username,
				password
			})
		});

		const data = await res.json();

		console.log('###', data);
	};
</script>

<main>
	<h1>Register new account</h1>
	<div>
		Email: <input bind:value={email} type="email" />
	</div>
	<div>
		Username: <input bind:value={username} />
	</div>
	<div>
		Password: <input bind:value={password} type="password" />
	</div>
	<div>
		{#if registered}
			Registered!
		{:else}
			<button on:click={handleRegister}>Register</button>
		{/if}
	</div>
</main>
