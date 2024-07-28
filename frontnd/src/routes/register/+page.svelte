<script>
	let firstname = '';
	let lastname = '';
	let username = '';
	let password = '';
	let c_password = '';
	let phone = '';
	let email = '';
	function register() {}
</script>

<div>
	<div class="first-container">
		<h1>Register</h1>
		<div class="container">
			<form on:submit|preventDefault={register} method="post">
				<label
					><input
						type="text"
						placeholder="Firstname"
						name="firstname"
						bind:value={firstname}
					/></label
				>
				<label
					><input type="text" placeholder="Lastname" bind:value={lastname} name="lastname" /></label
				>
				<label
					><input type="text" placeholder="Username" bind:value={username} name="username" /></label
				>
				<label><input type="text" placeholder="Email" bind:value={email} name="email" /></label>
				<label
					><input
						type="number"
						placeholder="Phone Number"
						bind:value={phone}
						name="phoneNumber"
					/></label
				>
				<label
					><input
						type="password"
						placeholder="Password"
						bind:value={password}
						name="password"
					/></label
				>
				<label
					><input
						type="password"
						placeholder="Confirm Password"
						bind:value={c_password}
						name="passwordCon"
					/></label
				>
				<input type="submit" name="submit" />
				<a href="/login">Login</a>
			</form>
		</div>
	</div>
</div>

<style>
	@import '../../style.css';
	@import '../../body.css';
	.form_div {
		width: 80%;
		margin-left: auto;
		margin-right: auto;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
