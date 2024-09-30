<script>
	import { goto } from '$app/navigation';
	import { blur } from 'svelte/transition';
	import { checkCookie } from './check';

	let username = '';
	let password = '';
	let error = '';

	if (typeof window !== 'undefined') {
		const { token } = checkCookie();
		if (token) {
			alert('Aready logged in');

			goto('/');
		}
	}

	async function handleSubmit() {
		const response = await fetch('http://127.0.0.1:3000/members/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});
		const result = await response.json();
		if (response.ok) {
			document.cookie = `token=${result.m_id}; path=/;`;
			goto(result.redirect_to);
		} else {
			alert(result.error);
			error = result.error || 'An unknown error occurred.';
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
	@import '../../login.css';
</style>
