<script>
	// @ts-nocheck

	import { onMount } from 'svelte';

	let userName = '';
	let loggedIn = false;
	onMount(async () => {
		// Retrieve cookies from the document
		const cookieHeader = document.cookie;
		// @ts-ignore
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		const token = cookies.get('token');

		if (token) {
			try {
				// Fetch username from the server
				const response = await fetch(`http://localhost:3000/member/username/${token}`);
				if (response.ok) {
					userName = await response.json();
					loggedIn = true;
					// Update the store if used
				} else {
					// Handle cases where the token is invalid or expired
					loggedIn = false;
				}
			} catch (error) {
				console.error('Failed to fetch username:', error);
				loggedIn = false;
			}
		} else {
			// Token not found, user not logged in
			loggedIn = false;
		}
	});
</script>

<div class="box">
	<div class="top-bar">
		<div class="left-name">Review IT</div>
		<ul>
			<li><a href="/">Home</a></li>
			{#if loggedIn}
				<li><a href="/profile">{userName.firstname}</a></li>
				<li><a href="/logout">Logout</a></li>
			{:else}
				<li><a href="/login">Login</a></li>
				<li><a href="/register">Sign up</a></li>
			{/if}
		</ul>
	</div>
</div>

<style>
	.box {
		background-color: #333;
	}

	.top-bar {
		width: 50%;
		margin-left: auto;
		margin-right: auto;
		color: #fff;
		padding: 1em 0;
		text-align: center;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.left-name {
		text-align: left;
		width: 10%;
		font-size: 20px;
		display: flex;
	}

	ul {
		list-style-type: none;
		display: flex;
		padding: 0;
	}

	li {
		display: inline;
		margin-right: 10px;
	}

	a {
		text-align: right;
		text-decoration: none;
		color: #fff;
		font-weight: bold;
	}

	a:hover {
		color: #0577fa;
	}
</style>
