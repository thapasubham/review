<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let username = '';
	let password = '';
	let error = '';

	async function handleSubmit() {
		error = ''; // Clear previous error
		const response = await fetch('http://127.0.0.1:3000/admin/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		const data = await response.json();

		if (response.ok && data.sucess) {
			// Store token and admin_id in localStorage
			localStorage.setItem('token', data.token);
			localStorage.setItem('admin_id', data.admin_id);

			// Redirect to the home page
			window.location('admin/home');
		} else {
			// Show error message
			error = 'Login failed. Please check your username or password.';
		}
	}

	// Optional: Check if user is already logged in
	onMount(() => {
		const token = localStorage.getItem('token');
		if (token) {
			// If token exists, redirect to home
			goto('admin/home');
		}
	});
</script>

<div class="first-container">
	<h3>Login</h3>
	<h3>To continue</h3>
	<div class="container">
		<form on:submit|preventDefault={handleSubmit}>
			<label for="username">Username:</label>
			<input type="text" id="username" bind:value={username} required />

			<label for="password">Password:</label>
			<input type="password" id="password" bind:value={password} required />

			{#if error}
				<p class="error">{error}</p>
			{/if}

			<div class="aligner">
				<button type="submit">Login</button>
			</div>
		</form>
	</div>
</div>

<style>
	@import '../../body.css';
	@import '../../style.css';
	@import '../../login.css';

	h3 {
		text-align: center;
	}
	.error {
		color: red;
	}
</style>
