<script>
	import { onMount } from 'svelte';
	import HeaderComp from '../HeaderComp.svelte';

	let userData = ''; // Store user data here
	let username = '';
	let password1 = '';
	let password2 = '';
	let usernameError = '';
	let passwordError = '';
	let user_id;
	onMount(async () => {
		const cookieHeader = document.cookie;
		// @ts-ignore
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		const token = cookies.get('token');
		user_id = parseInt(token);
		const response = await fetch(`http://localhost:3000/member/username/${token}`);
		userData = await response.json();
		username = userData.username;
	});

	async function updateUsername() {
		if (username.trim() === '') {
			usernameError = 'Username cannot be empty';
			return;
		}
		if (username.length < 5) {
			usernameError = 'Username must be longer than 5';
			return;
		}
		console.log(username + ' ' + user_id);

		try {
			const response = await fetch(`http://localhost:3000/member/update/username`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ username: username, user_id: user_id }) // Ensure this is correct
			});

			if (!response.ok) {
				throw new Error('Failed to update Username');
			}

			const result = await response.json();
			alert('Username updated successfully');
			userData.username = username;
		} catch (error) {
			usernameError = error.message;
		}
	}

	// Function to handle password update
	async function updatePassword() {
		if (password1 !== password2) {
			passwordError = "The password doesn't match.";
			return;
		} else if (password1 === '' && password2 === '') {
			passwordError = 'No Password was entered.\n';
			return;
		} else if (password1.length < 6) {
			passwordError = 'Password must be at least 6 characters.\n';
			return;
		} else if (!/[a-z]/.test(password1) || !/[A-Z]/.test(password1) || !/[0-9]/.test(password1)) {
			passwordError =
				'Password requires at least one lowercase letter, one uppercase letter, and one digit.\n';
			return;
		}

		try {
			const response = await fetch(`http://localhost:3000/member/update/password`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ password: password1, user_id: user_id })
			});

			if (!response.ok) {
				throw new Error('Failed to update password');
			}

			alert('Password updated successfully');
			password1 = '';
			password2 = '';
			passwordError = '';
		} catch (error) {
			passwordError = error.message;
		}
	}
</script>

<HeaderComp />
<body>
	<header>
		<h2>Update Your Profile</h2>
	</header>

	<div class="container">
		<form>
			<label for="name">
				{userData.firstname}
				{userData.lastname}
			</label>

			<label for="phone">
				{userData.phone}
			</label>

			<label for="username">Username</label>
			<input type="text" id="username" name="username" bind:value={username} />
			<p class="error">{usernameError}</p>
			<input class="btn" type="button" id="submitform" value="Update" on:click={updateUsername} />

			<label for="password1">Change Password</label>
			<input type="password" id="password1" name="password" bind:value={password1} />

			<label for="password2">Confirm Password</label>
			<input type="password" id="password2" name="confirmp" bind:value={password2} />
			<p class="error">{passwordError}</p>
			<input
				class="btn"
				type="button"
				id="passwordupdate"
				value="Confirm"
				on:click={updatePassword}
			/>

			<a href="/logout" id="logout">Log Out</a>
		</form>
	</div>
</body>

<style>
	.error {
		color: red;
		font-size: 0.9em;
	}

	body {
		background-color: #042425;
		color: #ffffff;
		font-family: Arial, sans-serif;
		margin: 0;
		padding: 0;
	}
	.container a {
		color: #007bff;
		text-decoration: none;
		margin-right: 20px;
	}

	.container {
		display: flex;
		justify-content: center;
		margin-left: auto;
		margin-right: auto;
		align-items: center;
		height: 500px;
		background-color: #042425;
	}

	h2 {
		text-align: center;
	}

	label {
		display: block;
		font-weight: bold;
		margin-top: 10px;
	}

	input[type='text'],
	[type='password'] {
		width: 100%;
		padding: 10px;
		margin-top: 5px;
		border: 1px solid #ccc;
		border-radius: 5px;
		font-size: 16px;
	}

	#logging:hover {
		color: #fa7b00;
	}

	#delete {
		color: #ff0000;
	}

	#delete:hover {
		color: #aa0000;
	}

	.btn {
		background-color: #007bff;
		color: #fff;
		padding: 10px 20px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-size: 16px;
		margin-top: 10px;
	}

	.btn:hover {
		background-color: #0056b3;
	}

	.error {
		color: #ff0000;
		font-weight: bold;
		margin-top: 10px;
		font-size: 14px;
	}
</style>
