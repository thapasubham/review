<script>
	import { onMount } from 'svelte';

	let userData = ''; // Store user data here
	let username = '';
	let password1 = '';
	let password2 = '';
	let usernameError = '';
	let passwordError = '';
	onMount(async () => {
		const cookieHeader = document.cookie;
		// @ts-ignore
		const cookies = new Map(cookieHeader.split('; ').map((cookie) => cookie.split('=')));
		const token = cookies.get('token');
		const response = await fetch(`http://localhost:3000/member/username/${token}`);
		userData = await response.json();
		return userData;
	});
</script>

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

			<input type="text" id="username" name="username" value={userData.username} />

			<input class="btn" type="button" id="submitform" value="Update" />
			<label for="password1">Change Password</label>
			<input type="password" id="password1" name="password" />

			<label for=" password2">Confirm Password</label>
			<input type="password" id="password2" name="confirmp" />
			<input class="btn" type="button" id="passwordupdate" value="Confirm" />
			<p id="passwordcheck" class="error"></p>
			<a href="/logout" id="logout">Log Out</a>
		</form>
	</div>
</body>

<style>
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
