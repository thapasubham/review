<script>
	import { goto } from '$app/navigation';

	let username = '';
	let password = '';
	let error = '';

	async function handleSubmit() {
		const response = await fetch('http://127.0.0.1:3000/members/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		const result = await response.json();

		if (result.success) {
			document.cookie = `token=${result.m_id}; path=/;`;
			goto(result.redirect_to);
		} else {
			error = result.error;
		}
	}
</script>

<div class="first-container">
	<h1>Login</h1>
	<div class="container">
		{#if error}
			<p>{error}</p>
		{/if}

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
	@import '../../style.css';
	@import '../../body.css';
	.algner {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	label {
		display: block;
		font-weight: bold;
		margin-top: 10px;
	}

	a {
		color: blue;
	}
	button {
		background-color: #007bff;
		color: #fff;
		padding: 10px 20px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-size: 16px;
		margin-top: 20px;
	}
	a:hover {
		color: #00ff99;
		cursor: pointer;
	}
</style>
