<script>
	import { redirect } from '@sveltejs/kit';

	let username = '';
	let password = '';

	async function handleSubmit() {
		const response = await fetch('http://127.0.0.1:3000/admin/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		const data = response.json();
		if (data.sucess) {
			redirect;
		}
	}
</script>

<div class="first-container">
	<h1>Login</h1>
	<div class="container">
		<!-- {#if error}
			<p>{error}</p>
		{/if} -->

		<form on:submit|preventDefault={handleSubmit}>
			<label for="username">Username:</label>
			<input type="text" id="username" bind:value={username} required />

			<label for="password">Password:</label>
			<input type="password" id="password" bind:value={password} required />

			<div class="algner">
				<button type="submit">Login</button>
				<p><a href="/">Login Later</a></p>
				<p><a href="/register">Register Now</a></p>
			</div>
		</form>
	</div>
</div>

<style>
	@import '../../body.css';
	@import '../../style.css';
	@import '../../login.css';
</style>
