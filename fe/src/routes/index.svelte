<script lang="ts">
	let email = '';
	let username = '';
	let password = '';

	const handleRegister = async () => {
		const res = await fetch('http://localhost:8000/account/create', {
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
	<button on:click={handleRegister}>Register</button>
</main>

<!--
  TODO:
  - if not logged in, show login page
  - if user is logged in, show current user's feed
-->
